//! Checks the rulebook's own hygiene.
//!
//! The gate that matters is provenance: a rule whose `Lexical input` names a list without saying
//! where the list comes from is an override waiting to happen. The check fails only when the count
//! *grows*; see `docs/design/05-lexicon.md` for the worklist.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::Result;
use crate::coverage::primary_token;

/// The number of unsourced list references at the time the gate was introduced.
///
/// Lower this as the worklist is cleared. Never raise it.
const UNSOURCED_BUDGET: usize = 154;

/// Fields every rule must carry, in this order.
const FIELDS: [&str; 11] = [
    "Statement",
    "Conditions",
    "Output",
    "Examples",
    "Exceptions",
    "Predictability",
    "Lexical input",
    "Interactions",
    "Sources",
    "Validation",
    "Confidence",
];

/// The values a `Predictability` field may open with.
const PREDICTABILITY: [&str; 3] = ["rule", "tendency", "lexical"];

/// Whether a `Lexical input` value names a list, table or inventory.
fn names_a_list(v: &str) -> bool {
    let v = v.to_lowercase();
    ["table", "list", "inventory", "mapping"]
        .iter()
        .any(|k| v.contains(k))
}

/// Whether the value says where the list comes from.
///
/// Accepted: an inline inventory declared closed, a generated `.tsv`, or an explicit source key.
fn has_provenance(v: &str) -> bool {
    let low = v.to_lowercase();
    low.contains("this rule")
        || low.contains("closed")
        || low.contains(".tsv")
        || low.contains(".ron")
        || v.contains('[')
}

/// Parsed view of one rule.
struct Rule {
    id: String,
    chapter: String,
    fields: BTreeMap<String, String>,
}

/// Reads one `- **Field**:` entry.
///
/// The field name may carry a parenthetical qualifier before the colon, as in
/// `- **Output** (standard name, then variants):`, so the value starts at the first colon on that
/// line rather than at a fixed offset.
fn field_value(block: &str, name: &str) -> Option<String> {
    let head = format!("- **{name}**");
    let start = block.find(&head)? + head.len();
    let rest = &block[start..];
    let line_end = rest.find('\n').unwrap_or(rest.len());
    let colon = rest[..line_end].find(':')? + 1;
    let rest = &rest[colon..];
    let end = rest.find("\n- **").unwrap_or(rest.len());
    Some(rest[..end].trim().to_owned())
}

fn is_rule_id(s: &str) -> bool {
    let parts: Vec<&str> = s.split('-').collect();
    parts.len() == 3
        && (2..=4).contains(&parts[0].len())
        && parts[0].chars().all(|c| c.is_ascii_uppercase())
        && parts[2].chars().all(|c| c.is_ascii_digit())
}

fn parse(text: &str, chapter: &str) -> Vec<Rule> {
    let mut out = Vec::new();
    for block in text.split("\n### ").skip(1) {
        let Some(id) = block.split_whitespace().next() else {
            continue;
        };
        if !is_rule_id(id) {
            continue;
        }
        let fields = FIELDS
            .iter()
            .filter_map(|f| field_value(block, f).map(|v| ((*f).to_owned(), v)))
            .collect();
        out.push(Rule {
            id: id.to_owned(),
            chapter: chapter.to_owned(),
            fields,
        });
    }
    out
}

/// Runs the checks over `dir`, printing a report and failing on a regression.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn check(dir: &Path) -> Result {
    let mut files: Vec<_> = fs::read_dir(dir)?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.extension().is_some_and(|e| e == "md")
                && p.file_stem()
                    .and_then(|s| s.to_str())
                    .is_some_and(|s| s.starts_with(|c: char| c.is_ascii_digit()))
        })
        .collect();
    files.sort();

    let mut rules = Vec::new();
    let mut texts = Vec::new();
    for path in &files {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let ch = stem.get(..2).unwrap_or(stem).to_owned();
        let text = fs::read_to_string(path)?;
        rules.extend(parse(&text, &ch));
        texts.push((ch, text));
    }
    if rules.is_empty() {
        return Err(format!("no rules found in {}", dir.display()).into());
    }

    let mut problems: Vec<String> = Vec::new();

    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for r in &rules {
        *seen.entry(r.id.as_str()).or_default() += 1;
    }
    for (id, n) in seen.iter().filter(|&(_, &n)| n > 1) {
        problems.push(format!("duplicate rule ID {id} ({n} times)"));
    }

    for r in &rules {
        for f in FIELDS {
            if !r.fields.contains_key(f) {
                problems.push(format!("{} [{}] missing field {f}", r.id, r.chapter));
            }
        }
    }

    for r in &rules {
        if let Some(v) = r.fields.get("Predictability") {
            let token = primary_token(v);
            if !PREDICTABILITY.contains(&token.as_str()) {
                problems.push(format!(
                    "{} [{}] Predictability opens with `{token}`, not one of {PREDICTABILITY:?}",
                    r.id, r.chapter
                ));
            }
        }
    }

    for (ch, text) in &texts {
        for r in dangling(text, &rules) {
            problems.push(format!("[{ch}] cites `{r}`, which no rule defines"));
        }
    }

    let unsourced: Vec<&Rule> = rules
        .iter()
        .filter(|r| {
            r.fields
                .get("Lexical input")
                .is_some_and(|v| names_a_list(v) && !has_provenance(v))
        })
        .collect();

    println!("{} rules across {} chapters", rules.len(), files.len());
    println!(
        "unsourced list references: {} (budget {UNSOURCED_BUDGET})",
        unsourced.len()
    );
    let mut by_chapter: BTreeMap<&str, usize> = BTreeMap::new();
    for r in &unsourced {
        *by_chapter.entry(r.chapter.as_str()).or_default() += 1;
    }
    for (ch, n) in &by_chapter {
        println!("  chapter {ch}: {n}");
    }

    if unsourced.len() > UNSOURCED_BUDGET {
        problems.push(format!(
            "unsourced list references grew to {} (budget {UNSOURCED_BUDGET}); \
             give the list a provenance or derive it away",
            unsourced.len()
        ));
    }

    if problems.is_empty() {
        println!("ok");
        return Ok(());
    }
    for p in problems.iter().take(40) {
        println!("  {p}");
    }
    Err(format!("{} problem(s)", problems.len()).into())
}

/// The rule IDs and families `text` cites in ticks that no rule in `rules` defines.
///
/// A family (`ORT-TOG`) is defined by any rule in it; only the rulebook's own prefixes are
/// checked, so source keys (`ENY-DATE`) and open-question IDs pass.
fn dangling(text: &str, rules: &[Rule]) -> BTreeSet<String> {
    let ids: BTreeSet<&str> = rules.iter().map(|r| r.id.as_str()).collect();
    let families: BTreeSet<&str> = ids
        .iter()
        .filter_map(|id| id.rsplit_once('-').map(|(f, _)| f))
        .collect();
    let prefixes: BTreeSet<&str> = ids.iter().filter_map(|id| id.split('-').next()).collect();
    text.split('`')
        .skip(1)
        .step_by(2)
        .filter(|t| {
            let parts: Vec<&str> = t.split('-').collect();
            let shaped = parts.iter().all(|p| {
                !p.is_empty()
                    && p.chars()
                        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
            });
            shaped
                && prefixes.contains(parts[0])
                && match parts.len() {
                    2 => !families.contains(*t),
                    3 => parts[2].len() == 3 && is_rule_id(t) && !ids.contains(*t),
                    _ => false,
                }
        })
        .map(str::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        PREDICTABILITY, dangling, has_provenance, is_rule_id, names_a_list, parse, primary_token,
    };

    #[test]
    fn finds_dangling_references() {
        let md = "\n### ORT-TOG-001 A\n- **Statement**: see `ORT-TOG-002`, `ORT-TOG`, `SYN-SEG`, `ENY-DATE`.\n";
        let rules = parse(md, "07");
        let got: Vec<String> = dangling(md, &rules).into_iter().collect();
        assert_eq!(got, ["ORT-TOG-002"]);
    }

    #[test]
    fn recognises_rule_ids() {
        assert!(is_rule_id("NOM-ACC-003"));
        assert!(is_rule_id("WS-DIG-010"));
        assert!(!is_rule_id("Not a rule"));
        assert!(!is_rule_id("X-Y-001"));
    }

    #[test]
    fn detects_list_references() {
        assert!(names_a_list("abbreviation list"));
        assert!(names_a_list("pronoun table"));
        assert!(!names_a_list("none"));
        assert!(!names_a_list("harmony"));
    }

    #[test]
    fn provenance_accepts_declared_sources() {
        assert!(has_provenance("letter name table (this rule)"));
        assert!(has_provenance("closed list, 26 entries"));
        assert!(has_provenance("shortening-stems.tsv, 497 members"));
        assert!(has_provenance("table [AkH12 §15]"));
        assert!(!has_provenance("brand-name list for exceptions"));
    }

    #[test]
    fn predictability_opens_with_a_known_value() {
        assert!(
            PREDICTABILITY.contains(&primary_token("`rule` (form), `lexical` (usage)").as_str())
        );
        assert!(PREDICTABILITY.contains(&primary_token("`lexical`/semantic").as_str()));
        assert!(!PREDICTABILITY.contains(&primary_token("as owning rules").as_str()));
        assert!(!PREDICTABILITY.contains(&primary_token("as `ORT`").as_str()));
    }

    #[test]
    fn parses_fields() {
        let md = "\n### NOM-ACC-003 Coda\n- **Statement**: x\n- **Lexical input**: none\n- **Confidence**: normative\n";
        let rules = parse(md, "03");
        assert_eq!(rules.len(), 1);
        assert_eq!(rules[0].id, "NOM-ACC-003");
        assert_eq!(
            rules[0].fields.get("Lexical input").map(String::as_str),
            Some("none")
        );
    }
}
