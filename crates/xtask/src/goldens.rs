//! Extracts test cases from the rulebook so tests cannot drift from the specification.
//!
//! Reads the `Output` and `Examples` fields of every rule in `docs/grammar/` and emits
//! input-output pairs with the rule ID that licenses them. Nothing here interprets Hungarian; it
//! only recognises the shapes the rulebook uses to state a mapping.
//!
//! See `docs/design/02-generation.md`.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use crate::Result;

/// The note on a golden that is an accepted variant rather than the form generation gives.
pub(crate) const VARIANT: &str = "variant";

/// How the pair was stated in the rulebook.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Kind {
    /// `` `x` -> `y` ``, an explicit mapping.
    Arrow,
    /// A markdown table row whose cells are back-ticked forms.
    Table,
}

/// One extracted case.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Case {
    pub rule: String,
    pub chapter: String,
    pub kind: Kind,
    pub input: String,
    pub output: String,
    /// Row label or surrounding cell text, when a table supplied it.
    pub note: Option<String>,
}

/// One form the rulebook stars as ungrammatical: the checker must not accept it.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Reject {
    pub rule: String,
    pub chapter: String,
    pub form: String,
}

/// A starred form and the unstarred form it competes with, when the text names one.
///
/// `` `lovat` only, `*lót` `` forbids `lót` in the cell `lovat` fills; the string is still the verb
/// `lót`. The rival fixes which reading the star forbids.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Starred {
    pub row: Reject,
    pub rival: Option<String>,
}

/// One form a rule's examples show as a word: the checker must accept it.
pub(crate) type Accept = Reject;

/// Collects every unmarked single-word form in a rule's `Output` and `Examples` fields.
///
/// The arrow pairs test generation; these test acceptance, which is what a rule whose examples
/// are plain forms (`várt`, `kért`, `Kossuth-díj`) states.
fn accepts(text: &str, chapter: &str, out: &mut Vec<Accept>) {
    // The morphology chapters hyphenate segmentations (`ugor-jon`); the writing chapters
    // hyphenate words (`Kossuth-díj`). The writing-system and repair chapters show damaged input
    // beside its repair, so their examples state nothing about acceptance.
    if chapter == "01" || chapter == "10" {
        return;
    }
    let segmenting = ("02"..="06").contains(&chapter);
    for block in text.split("\n### ").skip(1) {
        let Some(rule) = rule_id(block) else { continue };
        let fields = ["Examples", "Output"]
            .into_iter()
            .filter_map(|name| field(block, name));
        // A parenthesised list here is examples, not a gloss of an arrow's input, so the plain
        // split serves: it never hides a list inside a gloss.
        for segment in fields.flat_map(|t| t.split([';', '\n', '|'])) {
            let segment = unbroken(&unattested(&unglossed(&unqualified(segment))));
            for (span, starred_outside) in ticked_starred(&segment) {
                let Some(form) = unmarked(span) else { continue };
                // A `#` marks a blocked formation or a segmentation, never a word; a `~` joins
                // the doublets the norm accepts, each of which is one.
                if starred_outside || form.contains('#') {
                    continue;
                }
                for form in form.split('~') {
                    let form = form.trim_matches([
                        '.', ',', '?', '!', '\u{201E}', '\u{201D}', '\u{00BB}', '\u{00AB}',
                        '\u{2026}', '\u{2019}', '\'', '"',
                    ]);
                    // A feature name or a label has a capital inside (`PronType`) or is a bare
                    // ASCII word under three letters; a word has neither.
                    let inner_capital = form.chars().skip(1).any(char::is_uppercase)
                        && !form.chars().all(|c| !c.is_lowercase());
                    if form.contains(char::is_whitespace)
                        || form.contains("->")
                        || form.contains('+')
                        || form.chars().any(|c| c.is_ascii_digit())
                        || (segmenting && form.contains(['-', '.']))
                        || inner_capital
                        || form.chars().count() < 3
                        || !is_form(form)
                    {
                        continue;
                    }
                    out.push(Accept {
                        rule: rule.to_owned(),
                        chapter: chapter.to_owned(),
                        form: form.to_owned(),
                    });
                }
            }
        }
    }
}

/// `segment` without the forms the rulebook marks as unattested.
///
/// An attestation count follows a form, and a dash in ticks or a zero says the corpus has none:
/// `` `pénzt` [ATT 18] vs `pénzet` `-` ``, `` `ötödikek` 1,635 : `ötödikök` 0 ``, `[ATT 0]`. The
/// unattested form is cited as what the language does not do, so it is no example of acceptance.
fn unattested(segment: &str) -> String {
    let zero = |tail: &str| {
        let t = tail.trim_start();
        if t.starts_with("`-`") {
            return true;
        }
        let t = t
            .strip_prefix("[ATT")
            .or_else(|| t.strip_prefix('('))
            .unwrap_or(t)
            .trim_start();
        let Some(r) = t.strip_prefix('0') else {
            return false;
        };
        // A bare zero, not the start of `0.03`, `0,4` or `01`.
        let mut next = r.chars();
        match (next.next(), next.next()) {
            (Some(d), _) if d.is_ascii_digit() => false,
            (Some('.' | ','), Some(d)) => !d.is_ascii_digit(),
            _ => true,
        }
    };
    let mut out = String::with_capacity(segment.len());
    let mut rest = segment;
    while let Some(a) = rest.find('`') {
        let Some(len) = rest[a + 1..].find('`') else {
            break;
        };
        let tail = &rest[a + len + 2..];
        out.push_str(&rest[..a]);
        if zero(tail) {
            let t = tail.trim_start();
            rest = t.strip_prefix("`-`").unwrap_or(tail);
        } else {
            out.push_str(&rest[a..a + len + 2]);
            rest = tail;
        }
    }
    out.push_str(rest);
    out
}

/// `segment` without the halves of a line break, which the rulebook writes as two ticked spans
/// around a slash, the first ending in the hyphen or dash of the break (`` `Dumas-` / `nak` ``):
/// neither half is a word. A slash between two whole words (`` `süllyed` / `süllyeszt` ``) pairs
/// them and stays.
fn unbroken(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len());
    let mut rest = segment;
    while let Some(at) = rest.find(['-', '\u{2013}']) {
        let dash = rest[at..].chars().next().map_or(1, char::len_utf8);
        let Some(after) = rest[at + dash..].strip_prefix("` / `") else {
            out.push_str(&rest[..at + dash]);
            rest = &rest[at + dash..];
            continue;
        };
        let open = rest[..at].rfind('`').unwrap_or(at);
        out.push_str(&rest[..open]);
        rest = after.find('`').map_or("", |c| &after[c + 1..]);
    }
    out.push_str(rest);
    out
}

/// Collects every phrase or sentence a punctuation or syntax rule shows as correct: the lint
/// must stay silent on it.
fn silences(text: &str, chapter: &str, out: &mut Vec<Accept>) {
    if chapter != "08" && chapter != "09" {
        return;
    }
    for block in text.split("\n### ").skip(1) {
        let Some(rule) = rule_id(block) else { continue };
        let Some(t) = field(block, "Examples") else {
            continue;
        };
        for (span, starred, mapped) in ticked_in_context(t) {
            let span = span.trim();
            if starred
                || mapped
                || !span.contains(char::is_whitespace)
                || span.starts_with(['*', '?', '%', '#'])
                || span.contains("->")
            {
                continue;
            }
            out.push(Accept {
                rule: rule.to_owned(),
                chapter: chapter.to_owned(),
                form: span.to_owned(),
            });
        }
    }
}

/// A form is rejected as a golden if it is not a single word-like token.
///
/// The rulebook uses back-ticks for prose, feature names and glosses as well as for forms, so this
/// is the filter that keeps `` `Case=Ine` `` and `` `not recommended` `` out of the test set.
fn is_form(s: &str) -> bool {
    let s = s.trim();
    if s.is_empty() || s.chars().count() > 40 {
        return false;
    }
    // A form may contain a hyphen, apostrophe or a leading/trailing marker, but no spaces beyond
    // one (compounds written apart), and no markup or feature syntax.
    if s.contains([
        '=', '|', '/', '\\', '(', ')', '[', ']', '{', '}', '<', '>', '"', ':', ';',
    ]) {
        return false;
    }
    if s.split_whitespace().count() > 2 {
        return false;
    }
    // A leading hyphen marks a suffix template (`-t`, `-ének`), and a trailing one a bound stem or
    // the first half of a line-break (`Sim-`, `huszon-`, `forogt-`, `asz-`). Neither is a word form
    // a generator returns, so neither is a golden.
    if s.starts_with('-') || s.ends_with('-') {
        return false;
    }
    // A rule ID that leaked out of a cross-reference cell.
    if s.len() >= 7
        && s.split('-').count() == 3
        && s.chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-')
    {
        return false;
    }
    // Must contain at least one letter, and no `_`, which only a lexicon field name has
    // (`p_back`, `lex_compound`).
    if !s.chars().any(char::is_alphabetic) || s.contains('_') {
        return false;
    }
    // A Hungarian word form always contains a vowel; the class labels that head spelling tables
    // (`cs`, `dzs`, `sz`) do not. A few letter names in chapter 01 are lost with them.
    s.chars()
        .any(|c| "aáeéiíoóöőuúüűAÁEÉIÍOÓÖŐUÚÜŰ".contains(c))
}

/// Strips the markers the rulebook attaches to forms: `*` ungrammatical, `%` dialectal,
/// `?` questionable, `#` blocked or potential, `'` prosodic stress. Returns `None` for a marked
/// form, which is not a generation golden: a stress-marked string is a prosodic annotation
/// (`DER-COMP-016`), not a spelling the engine writes.
fn unmarked(s: &str) -> Option<&str> {
    let t = s.trim();
    if t.starts_with(['*', '%', '?', '#', '\'']) {
        return None;
    }
    Some(t)
}

/// Back-ticked spans in order.
fn ticked(line: &str) -> Vec<&str> {
    ticked_marked(line).into_iter().map(|t| t.span).collect()
}

/// Back-ticked spans in order, each with whether a `*` stands right before its opening tick:
/// the rulebook also stars a form from outside the ticks (``*`lőjjön` ``).
fn ticked_starred(line: &str) -> Vec<(&str, bool)> {
    ticked_in_context(line)
        .into_iter()
        .map(|(t, starred, _)| (t, starred))
        .collect()
}

/// Back-ticked spans in order, each with whether a `*` precedes it and whether an arrow follows
/// it, which makes it the input of a mapping rather than a form in its own right.
///
/// Markdown bold also ends in `*` (``**`szú`** is not a v-stem``), so only an odd run of them
/// before the tick is a star.
fn ticked_in_context(line: &str) -> Vec<(&str, bool, bool)> {
    let mut out = Vec::new();
    let mut rest = line;
    while let Some(a) = rest.find('`') {
        let after = &rest[a + 1..];
        let Some(b) = after.find('`') else { break };
        let starred = rest[..a].chars().rev().take_while(|&c| c == '*').count() % 2 == 1;
        let tail = &after[b + 1..];
        let mapped = tail.trim_start().starts_with("->");
        out.push((&after[..b], starred, mapped));
        rest = tail;
    }
    out
}

/// One back-ticked span and the text between it and the span before it.
#[derive(Clone, Copy, Debug)]
struct Tick<'a> {
    span: &'a str,
    /// A `~` joins it to the span before it.
    tilde: bool,
    /// The text since the previous span closed; empty for the first span.
    gap: &'a str,
}

/// Back-ticked spans in order.
///
/// `` `ugrani` ~ `ugorni` `` states a doublet: the first form is the one generation gives, the
/// later ones are accepted variants.
fn ticked_marked(line: &str) -> Vec<Tick<'_>> {
    let mut out = Vec::new();
    let mut rest = line;
    let mut gap = "";
    while let Some(a) = rest.find('`') {
        let after = &rest[a + 1..];
        let Some(b) = after.find('`') else { break };
        let g = gap.trim();
        if !is_comparison(g) {
            out.push(Tick {
                span: &after[..b],
                tilde: g == "~",
                gap,
            });
        }
        rest = &after[b + 1..];
        gap = &rest[..rest.find('`').unwrap_or(rest.len())];
    }
    out
}

/// The list `ticks` opens with: the first prose gap ends it, and forms after it belong to the
/// commentary (`` `Magyarország` -> `Magyarországon` and some islands (`Máltán` ``).
fn list_head(mut ticks: Vec<Tick<'_>>) -> Vec<Tick<'_>> {
    if let Some(end) = ticks.iter().skip(1).position(|t| !is_list_gap(t.gap)) {
        ticks.truncate(end + 1);
    }
    ticks
}

/// Whether `gap` is a count comparison (`` `brosst` 508 : `brossot` ``): a count, then `:` or
/// `vs`, with only attestation brackets besides. The form after it is evidence, not a second
/// output.
fn is_comparison(gap: &str) -> bool {
    let bare = without_brackets(gap);
    let words: Vec<&str> = bare.split_whitespace().collect();
    let count = |w: &str| {
        w.chars()
            .all(|c| c.is_ascii_digit() || c == ',' || c == '.')
    };
    words
        .iter()
        .position(|w| matches!(*w, ":" | "vs" | "vs."))
        .is_some_and(|at| at > 0 && words[..at].iter().all(|w| count(w)) && at == words.len() - 1)
}

/// `gap` without its bracketed attestation tags (`[WC2]`, `[ATT 0]`).
fn without_brackets(gap: &str) -> String {
    let mut bare = String::with_capacity(gap.len());
    let mut depth = 0usize;
    for c in gap.chars() {
        match c {
            '[' => depth += 1,
            ']' => depth = depth.saturating_sub(1),
            _ if depth == 0 => bare.push(c),
            _ => {}
        }
    }
    bare
}

/// Whether `gap` only separates the members of one list: commas, `~`, `/`, attestation brackets
/// and counts, and at most two words of connective (`but`, `pl.`, `imperative`). A longer stretch
/// is prose, which ends the list.
fn is_list_gap(gap: &str) -> bool {
    without_brackets(gap)
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|w| {
            !w.is_empty() && !matches!(*w, "~" | "/") && !w.chars().all(|c| c.is_ascii_digit())
        })
        .count()
        <= 2
}

/// `text` cut at `;`, newlines and table bars, but never at a `;` inside a parenthesis: a gloss
/// may hold one (`` `fő` (verb; ... `fövő` 909 : `fővő` 671) -> `fővök` ``) and still belongs to
/// the form before it.
fn segments(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut start = 0;
    for (at, c) in text.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            '\n' | '|' => {
                out.push(&text[start..at]);
                start = at + 1;
                depth = 0;
            }
            ';' if depth == 0 => {
                out.push(&text[start..at]);
                start = at + 1;
            }
            _ => {}
        }
    }
    out.push(&text[start..]);
    out
}

/// Words that put a form outside the present-day norm: neither a golden nor an example of it.
const QUALIFIERS: [&str; 6] = [
    "archaic",
    "traditional",
    "obsolete",
    "dated",
    "dialectal",
    "regional",
];

/// `segment` without the forms a qualifier marks: one followed by a parenthesised qualifier
/// (`` `lakol` (archaic) ``), and the list a qualifier heads (`` traditional `esném`, `esnék` ``).
/// A name keeps its spelling however old it is (`` archaic family names (`Széchenyi` ``), so a
/// capitalised form is never dropped.
fn unqualified(segment: &str) -> String {
    let qualifier = |w: &str| {
        let w = w.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
        QUALIFIERS.contains(&w.as_str())
    };
    let mut out = String::with_capacity(segment.len());
    let mut rest = segment;
    let mut run = false;
    while let Some(a) = rest.find('`') {
        let Some(len) = rest[a + 1..].find('`') else {
            break;
        };
        let gap = &rest[..a];
        let tail = &rest[a + len + 2..];
        let headed = gap.split_whitespace().last().is_some_and(qualifier);
        run = headed || (run && is_list_gap(gap));
        let trailing = tail
            .trim_start()
            .strip_prefix('(')
            .and_then(|t| t.split_whitespace().next())
            .is_some_and(qualifier);
        let name = rest[a + 1..].starts_with(char::is_uppercase);
        out.push_str(gap);
        if name || (!run && !trailing) {
            out.push_str(&rest[a..a + len + 2]);
        }
        rest = tail;
    }
    out.push_str(rest);
    out
}

/// The offset of the closer that matches the opener at the start of `span`.
///
/// A gloss may nest (`(r = .892 on BN(N) and N stems)`), and closing at the first `)` would leave
/// the tail of the gloss in the segment, where its back-ticks read as forms.
fn balanced(span: &str, close: char) -> Option<usize> {
    let open = span.chars().next()?;
    let mut depth = 0usize;
    for (at, c) in span.char_indices() {
        if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return Some(at);
            }
        }
    }
    None
}

/// A segment with its glosses removed.
///
/// A parenthesised or braced span that carries no arrow is commentary on the form before it:
/// a translation, a count, a contrast, a feature set. Left in, its
/// back-ticked words read as further outputs of the pair. Letters in parentheses glued to a
/// word are an optional ending, as in the `AkH12` notation `fehérl(ik)` and `-képp(en)`, and the
/// full form is kept: the bare stem of an `-ik` verb is no word.
fn unglossed(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len());
    let mut rest = segment;
    while let Some(open) = opener(rest) {
        let close = if rest.as_bytes()[open] == b'(' {
            ')'
        } else {
            '}'
        };
        let Some(len) = balanced(&rest[open..], close) else {
            break;
        };
        let inner = &rest[open..=open + len];
        out.push_str(&rest[..open]);
        let body = &inner[1..inner.len() - 1];
        let ending = close == ')'
            && rest[..open]
                .chars()
                .next_back()
                .is_some_and(char::is_alphabetic)
            && !body.is_empty()
            && body.chars().all(char::is_alphabetic);
        if ending {
            out.push_str(body);
        } else if inner.contains("->") || (close == '}' && is_set(body)) {
            out.push_str(inner);
        }
        rest = &rest[open + len + 1..];
    }
    out.push_str(rest);
    out
}

/// The first `(`, or `{` outside a code span: a brace inside one is a character of the form
/// (`\H{o}r`), while a set of forms is braced around its code spans.
fn opener(text: &str) -> Option<usize> {
    let mut in_code = false;
    for (at, c) in text.char_indices() {
        match c {
            '`' => in_code = !in_code,
            '(' => return Some(at),
            '{' if !in_code => return Some(at),
            _ => {}
        }
    }
    None
}

/// A braced set of forms, `{`kerek`, `kerék`, `kérek`}`, which are all outputs.
fn is_set(inner: &str) -> bool {
    inner.split(',').all(|item| {
        let item = item.trim();
        item.len() > 2
            && item.starts_with('`')
            && item.ends_with('`')
            && item[1..].find('`') == Some(item.len() - 2)
    })
}

/// A segment with the rulebook's code point notation turned into the character.
///
/// `REP-ENC-012` writes a replacement character as `` `vis`U+FFFD`sza` `` and `REP-FONT-002` a
/// trailing combining mark as `` `kő`+U+0301 ``; `WS-CHAR-008` writes `` `vissza<U+00AD>tér` ``.
/// Each is one form with the character in it.
fn with_code_points(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len());
    let mut rest = segment;
    while let Some(at) = rest.find("U+") {
        let hex: String = rest[at + 2..]
            .chars()
            .take_while(char::is_ascii_hexdigit)
            .collect();
        let Some(c) = u32::from_str_radix(&hex, 16)
            .ok()
            .filter(|_| (4..=6).contains(&hex.len()))
            .and_then(char::from_u32)
        else {
            out.push_str(&rest[..at + 2]);
            rest = &rest[at + 2..];
            continue;
        };
        let after = &rest[at + 2 + hex.len()..];
        let before = &rest[..at];
        if let Some(b) = before
            .strip_suffix("`+")
            .or_else(|| before.strip_suffix("` + "))
        {
            out.push_str(b);
            out.push(c);
            out.push('`');
            rest = after;
        } else if let (Some(b), Some(a)) = (before.strip_suffix('`'), after.strip_prefix('`')) {
            out.push_str(b);
            out.push(c);
            rest = a;
        } else if let (Some(b), Some(a)) = (before.strip_suffix('<'), after.strip_prefix('>')) {
            out.push_str(b);
            out.push(c);
            rest = a;
        } else {
            out.push_str(&rest[..at + 2 + hex.len()]);
            rest = after;
        }
    }
    out.push_str(rest);
    out
}

/// The forms an output names: `a~b` states a doublet.
///
/// A later half may abbreviate to what differs: `Huron-tói~-tavi` and `Fertő tói~tavi` both
/// replace the last member of the first form.
fn alternatives(output: &str) -> Vec<String> {
    let mut halves = output.split('~');
    let Some(first) = halves.next() else {
        return Vec::new();
    };
    let mut out = vec![first.to_owned()];
    for alt in halves {
        let expanded = if let Some(tail) = alt.strip_prefix('-')
            && let Some(at) = first.rfind('-')
        {
            format!("{}-{tail}", &first[..at])
        } else if let Some(at) = first.rfind(' ')
            && !alt.contains(' ')
        {
            format!("{} {alt}", &first[..at])
        } else {
            alt.to_owned()
        };
        out.push(expanded);
    }
    out
}

/// The run of comma-separated back-ticked forms that ends `left`, in order.
fn listed_inputs(left: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut rest = left.trim_end();
    while let Some(stripped) = rest.strip_suffix('`') {
        let Some(open) = stripped.rfind('`') else {
            break;
        };
        out.push(&stripped[open + 1..]);
        rest = stripped[..open].trim_end();
        match rest.strip_suffix(',') {
            Some(before) => rest = before.trim_end(),
            None => break,
        }
    }
    out.reverse();
    out
}

/// Whether two forms begin with the same letter, ignoring case and vowel length.
///
/// Suffixation and stem alternation both preserve the initial letter, so this is a cheap test for
/// "could be a form of".
fn shares_initial(a: &str, b: &str) -> bool {
    shared_prefix(a, b) > 0
}

/// The number of leading letters `a` and `b` share, ignoring case and vowel length.
fn shared_prefix(a: &str, b: &str) -> usize {
    a.chars()
        .map(fold)
        .zip(b.chars().map(fold))
        .take_while(|(x, y)| x == y)
        .count()
}

/// A letter lowercased, with its vowel length and rounding dropped.
fn fold(c: char) -> char {
    match c.to_lowercase().next().unwrap_or(c) {
        'á' => 'a',
        'é' => 'e',
        'í' => 'i',
        'ó' | 'ö' | 'ő' => 'o',
        'ú' | 'ü' | 'ű' => 'u',
        other => other,
    }
}

/// Both operands of a two-operand left side, joined with `+`.
///
/// `WS-DIG-012` states a binary operation (`` `asz-` + `szony` -> `asszony` ``) and `REP-SPACE-004`
/// the same one with a slash for the line break (`` `asz-` / `szony` ``). Only the last
/// back-ticked form would leave the unanswerable `szony -> asszony`. The
/// `+` written into the golden matches the seam notation `NOM-CASE-005` already uses
/// (`orr+ra -> orra`).
fn operands(left: &str) -> Option<String> {
    let ticks = ticked(left);
    let [.., a, b] = ticks.as_slice() else {
        return None;
    };
    let (a, b) = (unmarked(a)?, unmarked(b)?);
    // Only a bare `+` or `/` between the two forms joins them; `WS-NORM-002` writes a code point
    // there (`` `ko` + U+0308 + `r` ``) and means something else.
    let tail = left.trim_end();
    ["+", "/"]
        .iter()
        .any(|sep| tail.ends_with(&format!("`{a}` {sep} `{b}`")))
        .then(|| format!("{a}+{b}"))
}

/// Extracts `` `x` -> `y`, `z` `` pairs, where a comma-separated tail shares the same input.
/// Whether a mapped span is a golden in this chapter: the writing chapters map text, with its
/// spaces and punctuation, and every other chapter maps word forms.
fn is_mapped_form(s: &str, chapter: &str) -> bool {
    if chapter == "08" || chapter == "10" {
        let s = s.trim();
        return !s.is_empty()
            && s.chars().count() <= 80
            && s.chars().any(char::is_alphanumeric)
            && !s.contains("->");
    }
    is_form(s)
}

fn arrows(text: &str, rule: &str, chapter: &str, out: &mut Vec<Case>) {
    // A mapping never spans a table cell or a line, so those are hard boundaries alongside `;`.
    for segment in segments(text) {
        let segment = with_code_points(&unglossed(&unqualified(segment)));
        // A cell may chain several pairs: `baj` -> `bajt`, `báj` -> `bájt`. Splitting on the
        // arrows and pairing adjacent parts keeps them apart.
        let parts: Vec<&str> = segment.split("->").collect();
        for i in 0..parts.len().saturating_sub(1) {
            let joined = operands(parts[i]);
            let input = if let Some(both) = &joined {
                both.as_str()
            } else {
                let Some(input) = ticked(parts[i]).last().copied() else {
                    continue;
                };
                let Some(input) = unmarked(input) else {
                    continue;
                };
                if !is_mapped_form(input, chapter) {
                    continue;
                }
                input
            };
            let mut outputs = ticked_marked(parts[i + 1]);
            if i + 1 < parts.len() - 1 {
                outputs.pop();
            }
            let outputs = list_head(outputs);
            // A list maps onto a list of the same length member by member: `PH-ASSIM-005`
            // writes `` `hűt`, `műt` -> `hűts`, `műtsd` ``.
            let inputs = listed_inputs(parts[i]);
            if parts.len() == 2 && inputs.len() > 1 && inputs.len() == outputs.len() {
                for (input, output) in inputs.iter().zip(&outputs) {
                    if let (Some(input), Some(output)) = (unmarked(input), unmarked(output.span))
                        && is_mapped_form(input, chapter)
                        && is_mapped_form(output, chapter)
                        && input != output
                    {
                        out.push(Case {
                            rule: rule.to_owned(),
                            chapter: chapter.to_owned(),
                            kind: Kind::Arrow,
                            input: input.to_owned(),
                            output: output.to_owned(),
                            note: None,
                        });
                    }
                }
                continue;
            }
            let outputs: Vec<(usize, String, bool)> = outputs
                .iter()
                .enumerate()
                .flat_map(|(n, t)| {
                    alternatives(t.span)
                        .into_iter()
                        .enumerate()
                        .map(move |(k, a)| (n, a, t.tilde || k > 0))
                })
                .collect();
            for (n, output, variant) in &outputs {
                let n = *n;
                let Some(output) = unmarked(output) else {
                    continue;
                };
                // The first form after the arrow is always this input's output. A comma-separated
                // tail may continue the same mapping (`fix` -> `fixszel`, `fixszé`) or may simply
                // be the next example in the list (`oxigén`, `szexuális`, `taxi`), and only the
                // shared initial letter tells them apart. Position 1 is exempt so that suppletive
                // pairs survive: `sok` -> `több` and `van` -> `lesz` share nothing.
                if n > 0 && !shares_initial(input, output) {
                    continue;
                }
                // A `+` in an output is segmentation notation (`legelő+re`), not a form.
                if !is_mapped_form(output, chapter) || output == input || output.contains('+') {
                    continue;
                }
                // A one-character input is a letter or an archiphoneme label, and a letter can only
                // map to another letter: `PH-VSTEM-002` writes its classes as `` `é`->`e`+`v` ``
                // on the same line as its examples. Letter-to-letter pairs such as `a` -> `á` and
                // the OCR confusions are kept.
                if input.chars().count() == 1
                    && (output.chars().count() > 2
                        || input.starts_with(|c: char| c.is_ascii_uppercase()))
                {
                    continue;
                }
                out.push(Case {
                    rule: rule.to_owned(),
                    chapter: chapter.to_owned(),
                    kind: Kind::Arrow,
                    input: input.to_owned(),
                    output: output.to_owned(),
                    note: variant.then(|| VARIANT.to_owned()),
                });
            }
        }
    }
}

/// Extracts table rows of the shape `| label | `input` | `output` |` and wider paradigm tables.
///
/// A paradigm row names a stem in one cell and its inflected forms in the others; the first
/// back-ticked form in the row is taken as the input and the rest as outputs, which is the shape
/// `NOM-CASE-001` and the `NOM-ACC` coda tables use.
fn tables(body: &str, rule: &str, chapter: &str, out: &mut Vec<Case>) {
    for line in body.lines() {
        let line = line.trim();
        if !line.starts_with('|') || line.contains("---") {
            continue;
        }
        let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() < 2 {
            continue;
        }
        let forms: Vec<&str> = cells
            .iter()
            .flat_map(|c| list_head(ticked_marked(c)))
            .map(|t| t.span)
            .collect();
        if forms.len() < 2 {
            continue;
        }
        // A single-character first cell is a class label (`NOM-ACC-003` keys rows by final
        // consonant), not a stem. Those rows would otherwise yield `r` -> `kort`.
        let Some(input) = unmarked(forms[0]).filter(|s| is_form(s) && s.chars().count() >= 2)
        else {
            continue;
        };
        let note = cells
            .first()
            .map(|c| c.trim().to_owned())
            .filter(|c| !c.is_empty() && !c.contains('`'));
        for (k, output) in forms[1..]
            .iter()
            .flat_map(|o| alternatives(o).into_iter().enumerate())
        {
            let Some(output) = unmarked(&output).filter(|s| is_form(s)) else {
                continue;
            };
            if output == input {
                continue;
            }
            out.push(Case {
                rule: rule.to_owned(),
                chapter: chapter.to_owned(),
                kind: Kind::Table,
                input: input.to_owned(),
                output: output.to_owned(),
                note: if k > 0 {
                    Some(VARIANT.to_owned())
                } else {
                    note.clone()
                },
            });
        }
    }
}

/// The text of one `- **Field**:` entry, including any indented continuation and table rows.
fn field<'a>(block: &'a str, name: &str) -> Option<&'a str> {
    let head = format!("- **{name}**:");
    let start = block.find(&head)? + head.len();
    let rest = &block[start..];
    let end = rest.find("\n- **").unwrap_or(rest.len());
    Some(&rest[..end])
}

/// Extracts every case from one chapter file.
/// The rule ID a `###` block is headed by, when it is one.
fn rule_id(block: &str) -> Option<&str> {
    let rule = block.split_whitespace().next()?;
    let ok = rule.len() >= 7
        && rule.split('-').next().is_some_and(|p| {
            (2..=4).contains(&p.len()) && p.chars().all(|c| c.is_ascii_uppercase())
        })
        && rule
            .rsplit('-')
            .next()
            .is_some_and(|n| n.chars().all(|c| c.is_ascii_digit()));
    ok.then_some(rule)
}

/// Collects every `` `*form` `` a rule block stars, wherever in the block it stands.
///
/// A star marks a form the rule forbids, so each is a negative golden: the checker must reject
/// it. Only single words count: a starred phrase is a syntax judgement, and a starred `+`
/// segmentation (`*épít+et`, `DER-NOUN-009`) names a morpheme sequence rather than a spelling.
///
/// The rival of a star is the nearest unstarred form before it in the same segment that shares
/// its first two letters: the form that fills the cell the star competes for.
fn rejects(text: &str, chapter: &str, out: &mut Vec<Starred>) {
    for block in text.split("\n### ").skip(1) {
        let Some(rule) = rule_id(block) else { continue };
        for segment in segments(block) {
            let mut rival = None;
            for (span, starred_outside) in ticked_starred(segment) {
                let span = span.trim();
                let form = match span.strip_prefix('*') {
                    Some(f) => f.trim(),
                    None if starred_outside => span,
                    None => {
                        rival = unmarked(span).filter(|f| is_form(f)).or(rival);
                        continue;
                    }
                };
                if !form.contains(char::is_whitespace)
                    && !form.contains('+')
                    && !form.ends_with('.')
                    && is_form(form)
                {
                    out.push(Starred {
                        row: Reject {
                            rule: rule.to_owned(),
                            chapter: chapter.to_owned(),
                            form: form.to_owned(),
                        },
                        rival: rival
                            .filter(|r| *r != form && shared_prefix(r, form) >= 2)
                            .map(str::to_owned),
                    });
                }
            }
        }
    }
}

fn chapter(text: &str, chapter: &str, out: &mut Vec<Case>) {
    for block in text.split("\n### ").skip(1) {
        let Some(rule) = rule_id(block) else { continue };
        for name in ["Output", "Examples"] {
            if let Some(t) = field(block, name) {
                arrows(t, rule, chapter, out);
                tables(t, rule, chapter, out);
            }
        }
    }
}

/// Reads `dir`, writes `tests/goldens.ron` next to it, and reports the yield.
#[expect(clippy::print_stdout, reason = "CLI report")]
pub(crate) fn extract(dir: &Path) -> Result {
    let mut cases = Vec::new();
    let mut files: Vec<_> = fs::read_dir(dir)?
        .filter_map(std::result::Result::ok)
        .map(|e| e.path())
        .filter(|p| {
            p.extension().is_some_and(|e| e == "md")
                && p.file_stem()
                    .and_then(|s| s.to_str())
                    .is_some_and(|s| s.chars().next().is_some_and(|c| c.is_ascii_digit()))
        })
        .collect();
    files.sort();
    if files.is_empty() {
        return Err(format!("no chapter files in {}", dir.display()).into());
    }
    let mut rejected = Vec::new();
    let mut accepted = Vec::new();
    let mut silent = Vec::new();
    for path in &files {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let ch = stem.get(..2).unwrap_or(stem).to_owned();
        let text = fs::read_to_string(path)?;
        chapter(&text, &ch, &mut cases);
        rejects(&text, &ch, &mut rejected);
        accepts(&text, &ch, &mut accepted);
        silences(&text, &ch, &mut silent);
    }

    cases.sort();
    cases.dedup();
    rejected.sort_by(|a, b| a.row.cmp(&b.row).then_with(|| b.rival.cmp(&a.rival)));
    rejected.dedup_by(|a, b| a.row == b.row);
    write_rejects(dir, &rejected)?;
    accepted.sort();
    accepted.dedup();
    write_forms(
        dir,
        "accepts.ron",
        "a word a rule's examples show: the checker must accept it",
        &accepted,
    )?;
    silent.sort();
    silent.dedup();
    write_forms(
        dir,
        "silences.ron",
        "a text a punctuation or syntax rule shows as correct: the lint must not flag it",
        &silent,
    )?;

    let rules: BTreeSet<&str> = cases.iter().map(|c| c.rule.as_str()).collect();
    write_set(dir, "goldens.ron", &cases, Kind::Arrow)?;
    write_set(dir, "goldens-review.ron", &cases, Kind::Table)?;

    let arrow = cases.iter().filter(|c| c.kind == Kind::Arrow).count();
    println!("{} cases from {} rules", cases.len(), rules.len());
    println!("  goldens.ron        {arrow} arrow cases, authoritative");
    println!(
        "  goldens-review.ron {} table cases, need curation",
        cases.len() - arrow
    );
    println!(
        "  rejects.ron        {} starred forms, must be rejected",
        rejected.len()
    );
    println!(
        "  accepts.ron        {} example forms, must be accepted",
        accepted.len()
    );
    println!(
        "  silences.ron       {} example texts, must pass the lint",
        silent.len()
    );
    Ok(())
}

/// Writes the starred forms to `rejects.ron` under `dir`.
fn write_rejects(dir: &Path, rejects: &[Starred]) -> Result {
    let mut ron = String::from(
        "// Generated by `cargo xtask goldens`. Do not edit.\n// Each row is a form the rulebook \
         stars as ungrammatical: the checker must not accept it in the reading its rival fills.\n[\n",
    );
    for Starred { row: r, rival } in rejects {
        let rival = rival
            .as_ref()
            .map(|v| format!(",rival:{v:?}"))
            .unwrap_or_default();
        writeln!(
            ron,
            "    (rule:{:?},chapter:{:?},form:{:?}{rival}),",
            r.rule, r.chapter, r.form
        )?;
    }
    ron.push_str("]\n");
    fs::write(dir.join("rejects.ron"), ron)?;
    Ok(())
}

/// Writes one-form rows to `name` under `dir`.
fn write_forms(dir: &Path, name: &str, what: &str, rows: &[Reject]) -> Result {
    let mut ron =
        format!("// Generated by `cargo xtask goldens`. Do not edit.\n// Each row is {what}.\n[\n");
    for r in rows {
        writeln!(
            ron,
            "    (rule:{:?},chapter:{:?},form:{:?}),",
            r.rule, r.chapter, r.form
        )?;
    }
    ron.push_str("]\n");
    fs::write(dir.join(name), ron)?;
    Ok(())
}

/// Writes the cases of one kind to `name` under `dir`.
fn write_set(dir: &Path, name: &str, cases: &[Case], kind: Kind) -> Result {
    let header = match kind {
        Kind::Arrow => {
            "// Generated by `cargo xtask goldens`. Do not edit.\n                        // Authoritative: each row is an explicit `x -> y` mapping in the rulebook.\n"
        }
        Kind::Table => {
            "// Generated by `cargo xtask goldens`. Do not edit.\n                        // NOT authoritative. Table rows may pair sibling forms (`magame` / `magade`)\n                        // rather than an input and its output. Curate before using as goldens.\n"
        }
    };
    let mut ron = String::from(header);
    ron.push_str("[\n");
    for c in cases.iter().filter(|c| c.kind == kind) {
        writeln!(
            ron,
            "    (rule:{:?},chapter:{:?},input:{:?},output:{:?},note:{}),",
            c.rule,
            c.chapter,
            c.input,
            c.output,
            c.note
                .as_ref()
                .map_or_else(|| "None".to_owned(), |n| format!("Some({n:?})"))
        )?;
    }
    ron.push_str("]\n");
    fs::write(dir.join(name), ron)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Case, Kind, chapter, is_form, rejects, ticked, unattested, unmarked};

    #[test]
    fn rejects_non_forms() {
        assert!(is_form("házat"));
        assert!(is_form("Van Gogh"));
        assert!(!is_form("Case=Ine"));
        assert!(!is_form("not recommended (in print)"));
        assert!(!is_form(""));
        assert!(!is_form("a b c d"));
    }

    #[test]
    fn strips_markers() {
        assert_eq!(unmarked("házat"), Some("házat"));
        assert_eq!(unmarked("*házot"), None);
        assert_eq!(unmarked("%Tomiéknek"), None);
    }

    #[test]
    fn finds_ticked_spans() {
        assert_eq!(ticked("a `x` b `y`"), vec!["x", "y"]);
        assert_eq!(ticked("no ticks"), Vec::<&str>::new());
    }

    fn pairs(md: &str) -> Vec<(String, String)> {
        let mut out = Vec::new();
        chapter(md, "03", &mut out);
        out.into_iter().map(|c| (c.input, c.output)).collect()
    }

    #[test]
    fn bold_is_no_star() {
        let md = "\n### PH-VSTEM-002 V\n- **Examples**: **`szú`** is not a v-stem, ***`szuvak`**\n";
        let mut out = Vec::new();
        rejects(md, "02", &mut out);
        let forms: Vec<_> = out.iter().map(|r| r.row.form.as_str()).collect();
        assert_eq!(forms, ["szuvak"]);
    }

    #[test]
    fn a_star_takes_its_rival() {
        let md = "\n### NOM-STEM-018 V\n- **Exceptions**: `lovat` only, `*lót` as the accusative of `ló`; `kéz`, `*házék`\n";
        let mut out = Vec::new();
        rejects(md, "03", &mut out);
        let got: Vec<_> = out
            .iter()
            .map(|s| (s.row.form.as_str(), s.rival.as_deref()))
            .collect();
        assert_eq!(got, [("lót", Some("lovat")), ("házék", None)]);
    }

    #[test]
    fn arrow_list_ends_at_prose() {
        let md = "\n### NOM-PROP-006 P\n- **Output**: `Magyarország` -> `Magyarországon` and islands (`Krétán`, `Máltán`\n";
        assert_eq!(
            pairs(md),
            [("Magyarország".into(), "Magyarországon".into())]
        );
        let md = "\n### VRB-STEM-009 D\n- **Examples**: `kétli` -> `kétlem`, `kétled`, past cells by the lemma `kétell`\n";
        assert!(pairs(md).iter().all(|(_, o)| o != "kétell"));
    }

    #[test]
    fn gloss_keeps_its_semicolon() {
        let md = "\n### PH-VSTEM-001 V\n- **Examples**: `fő` (verb; `fövő` 909 : `fővő` 671) -> `fővök`\n";
        assert_eq!(pairs(md), [("fő".into(), "fővök".into())]);
    }

    #[test]
    fn qualified_forms_are_dropped() {
        let md = "\n### VRB-IK-004 C\n- **Examples**: `esik` -> `esnék`, `esne` ~ traditional `esném`, `esnék`; `lakik` -> `laksz` (standard), `lakol` (archaic)\n";
        let got = pairs(md);
        assert!(
            got.iter().all(|(_, o)| o != "esném" && o != "lakol"),
            "{got:?}"
        );
        assert!(got.contains(&("lakik".into(), "laksz".into())));
    }

    #[test]
    fn digits_in_prose_are_no_comparison() {
        let md = "\n### PH-HARM-018 V\n- **Examples**: `alibi`->`alibijének` (`alibije` 920 : `alibija` 4) RST12 judges: `?agilisság`->`?agilisságnak`, and `fotel`->`foteljének`, `brosst` 508 : `brossot` 32.\n";
        let got = pairs(md);
        assert!(
            got.contains(&("alibi".into(), "alibijének".into())),
            "{got:?}"
        );
        assert!(
            got.contains(&("fotel".into(), "foteljének".into())),
            "{got:?}"
        );
        assert!(!got.iter().any(|(_, o)| o == "brossot"), "{got:?}");
    }

    #[test]
    fn zero_counts_are_unattested() {
        let kept = |s: &str| ticked(&unattested(s)).join(" ");
        assert_eq!(kept("`ötödikek` 1,635 : `ötödikök` 0."), "ötödikek");
        assert_eq!(kept("`a` [ATT 0], `b` 0.4"), "b");
        assert_eq!(kept("`pénzet` `-`, `x`"), "x");
    }

    #[test]
    fn extracts_arrow_pairs() {
        let md = "\n### NOM-ACC-003 Coda\n- **Examples**: `bot` -> `botot`; `rab` -> `rabot`\n";
        let mut out = Vec::new();
        chapter(md, "03", &mut out);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].input, "bot");
        assert_eq!(out[0].output, "botot");
        assert_eq!(out[0].kind, Kind::Arrow);
    }

    #[test]
    fn pairs_do_not_bleed_across_table_cells() {
        let md = "\n### NOM-ACC-003 T\n- **Output**:\n  | `j` | `-t` | `akadály` -> `akadályt` |\n  | `l` | `-t` | `dal` -> `dalt` |\n";
        let mut out = Vec::new();
        chapter(md, "03", &mut out);
        let arrows: Vec<_> = out.iter().filter(|c| c.kind == Kind::Arrow).collect();
        assert!(
            arrows.iter().all(|c| c.input.chars().count() > 1),
            "class labels leaked in as inputs: {arrows:?}"
        );
        assert!(
            arrows
                .iter()
                .any(|c| c.input == "akadály" && c.output == "akadályt")
        );
    }

    #[test]
    fn rejects_vowelless_class_labels() {
        assert!(!is_form("cs"));
        assert!(!is_form("dzs"));
        assert!(is_form("ló"));
        assert!(is_form("mű"));
    }

    #[test]
    fn chained_pairs_do_not_bleed() {
        // `NOM-ACC-003` lists several pairs per cell; pairing across them gives `baj` -> `báj`.
        let md = "\n### NOM-ACC-003 T\n- **Examples**: `baj` -> `bajt`, `báj` -> `bájt`\n";
        let mut out = Vec::new();
        chapter(md, "03", &mut out);
        assert_eq!(out.len(), 2, "{out:?}");
        assert_eq!(
            (out[0].input.as_str(), out[0].output.as_str()),
            ("baj", "bajt")
        );
        assert_eq!(
            (out[1].input.as_str(), out[1].output.as_str()),
            ("báj", "bájt")
        );
    }

    #[test]
    fn arrow_shares_input_across_comma_list() {
        let md = "\n### VRB-PAST-001 Past\n- **Examples**: `vár` -> `várt`, `vártam`\n";
        let mut out = Vec::new();
        chapter(md, "05", &mut out);
        assert_eq!(out.len(), 2);
        assert!(out.iter().all(|c| c.input == "vár"));
    }

    #[test]
    fn starred_forms_are_rejects() {
        let text = "\n### NOM-X-001 T\n- **Conditions**: `-stul` after a singular (`*házakostul`, `*ház ostul`); `házastul`.\n";
        let mut out = Vec::new();
        super::rejects(text, "03", &mut out);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].row.form, "házakostul");
        assert_eq!(out[0].row.rule, "NOM-X-001");
    }

    #[test]
    fn ignores_ungrammatical_targets() {
        let md = "\n### NOM-ACC-001 T\n- **Examples**: `ház` -> `házat`, `*házot`\n";
        let mut out = Vec::new();
        chapter(md, "03", &mut out);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].output, "házat");
    }

    #[test]
    fn extracts_paradigm_table_rows() {
        let md = "\n### NOM-PARA-001 T\n- **Output**:\n  | Stem | Acc | Pl |\n  |---|---|---|\n  | `alma` | `almát` | `almák` |\n";
        let mut out = Vec::new();
        chapter(md, "03", &mut out);
        assert_eq!(out.len(), 2);
        assert!(
            out.iter()
                .all(|c| c.input == "alma" && c.kind == Kind::Table)
        );
    }

    #[test]
    fn skips_class_label_table_rows() {
        // `NOM-ACC-003` keys rows by final consonant; the label is not a stem.
        let md = "\n### NOM-ACC-003 T\n- **Output**:\n  | Final C | Acc | Ex |\n  |---|---|---|\n  | `l` | `dalt` | `bált` |\n";
        let mut out = Vec::new();
        chapter(md, "03", &mut out);
        assert!(out.is_empty(), "class-label rows are not goldens: {out:?}");
    }

    #[test]
    fn skips_suffix_templates() {
        let md = "\n### NOM-CASE-002 T\n- **Examples**: `ház` -> `-nak`\n";
        let mut out = Vec::new();
        chapter(md, "03", &mut out);
        assert_eq!(out, [] as [Case; 0]);
    }

    #[test]
    fn glosses_are_not_outputs() {
        let md = "\n### DER-NOUN-024 T\n- **Examples**: `orvos` -> `orvosné` (contrast `orvosnő`); `MTA` (`em-té-á`) -> `MTA-nak`\n";
        let mut out = Vec::new();
        chapter(md, "06", &mut out);
        let pairs: Vec<(&str, &str)> = out
            .iter()
            .map(|c| (c.input.as_str(), c.output.as_str()))
            .collect();
        assert_eq!(pairs, vec![("orvos", "orvosné"), ("MTA", "MTA-nak")]);
    }

    #[test]
    fn doublets_split_and_code_points_join() {
        let md = "\n### ORT-NAME-022 T\n- **Examples**: `Huron-tó` -> `Huron-tói~-tavi`; `Fertő tó` -> `Fertő tói~tavi`; `vis`U+FFFD`sza` -> `vissza`; `kó`+U+0301 -> `kő`; `kerek` -> {`kerek`, `kerék`}\n";
        let mut out = Vec::new();
        chapter(md, "07", &mut out);
        let pairs: Vec<(&str, &str)> = out
            .iter()
            .map(|c| (c.input.as_str(), c.output.as_str()))
            .collect();
        assert_eq!(
            pairs,
            vec![
                ("Huron-tó", "Huron-tói"),
                ("Huron-tó", "Huron-tavi"),
                ("Fertő tó", "Fertő tói"),
                ("Fertő tó", "Fertő tavi"),
                ("vis\u{FFFD}sza", "vissza"),
                ("kó\u{301}", "kő"),
                ("kerek", "kerék"),
            ]
        );
    }

    #[test]
    fn skips_non_rule_headings() {
        let md = "\n### Not a rule\n- **Examples**: `a` -> `b`\n";
        let mut out = Vec::new();
        chapter(md, "01", &mut out);
        assert_eq!(out, [] as [Case; 0]);
    }
}
