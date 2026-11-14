//! The substitution tables the repair rules name, read from `data/glyphs.ron`.

use std::sync::LazyLock;

use picur_morph_lexicon::Tables;

static GLYPHS: LazyLock<Tables> =
    LazyLock::new(|| Tables::parse(include_str!("../data/glyphs.ron")));

/// A sequence rewrite, with the rule that states it.
pub(crate) struct Rewrite {
    pub from: &'static str,
    pub to: &'static str,
    pub rule: &'static str,
}

/// The rewrites of a `from, to, rule` table.
pub(crate) fn rewrites(role: &str) -> Vec<Rewrite> {
    GLYPHS
        .rows(role)
        .iter()
        .map(|r| Rewrite {
            from: &r[0],
            to: &r[1],
            rule: &r[2],
        })
        .collect()
}

/// The words of a plain list.
pub(crate) fn words(role: &str) -> Vec<&'static str> {
    GLYPHS.words(role).iter().map(AsRef::as_ref).collect()
}

/// The pairs of a string substitution table.
pub(crate) fn pairs(role: &str) -> Vec<(&'static str, &'static str)> {
    GLYPHS
        .pairs(role)
        .iter()
        .map(|(a, b)| (a.as_ref(), b.as_ref()))
        .collect()
}

/// The first character of a cell, which a character table's key is.
fn first(cell: &str) -> char {
    cell.chars()
        .next()
        .unwrap_or_else(|| panic!("an empty character cell"))
}

/// The pairs of a table keyed by a character, each expanding to a string.
pub(crate) fn char_strs(role: &str) -> Vec<(char, &'static str)> {
    GLYPHS
        .pairs(role)
        .iter()
        .map(|(a, b)| (first(a), b.as_ref()))
        .collect()
}

/// The pairs of a table mapping a character to a character; an empty cell maps to nothing.
pub(crate) fn char_chars(role: &str) -> Vec<(char, char)> {
    GLYPHS
        .pairs(role)
        .iter()
        .map(|(a, b)| (first(a), first(b)))
        .collect()
}

/// Whether some table citing `rule` maps `from` to `to`: the rule's own substitution, stated
/// as data, which the evaluation checks against the rulebook's rows.
#[must_use]
pub fn tabulated(rule: &str, from: &str, to: &str) -> bool {
    GLYPHS.roles().into_iter().any(|role| {
        GLYPHS.rules(role).iter().any(|r| r.as_ref() == rule)
            && (GLYPHS
                .pairs(role)
                .iter()
                .any(|(a, b)| a.as_ref() == from && b.as_ref() == to)
                || GLYPHS
                    .rows(role)
                    .iter()
                    .any(|r| r.len() >= 2 && r[0].as_ref() == from && r[1].as_ref() == to))
    })
}

/// The rows of a table as single characters.
pub(crate) fn char_rows(role: &str) -> Vec<Vec<char>> {
    GLYPHS
        .rows(role)
        .iter()
        .map(|r| r.iter().map(|c| first(c)).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_table_reads() {
        for role in [
            "Archaic",
            "Assimilation",
            "Lookalike",
            "Compatibility",
            "LongDigraph",
            "Confusion",
            "OcrConfusion",
            "Diacritic",
            "LigatureSlot",
            "EarlyPrint",
            "SpacingAccent",
            "FontShift",
            "DoubleAcute",
            "Cwi2",
            "Homoglyph",
            "SevenBitDigit",
            "Telegram",
            "MailDomain",
        ] {
            let t = &*super::GLYPHS;
            assert!(
                !t.rows(role).is_empty() || !t.pairs(role).is_empty() || !t.words(role).is_empty(),
                "{role}"
            );
        }
        assert_eq!(super::char_strs("LigatureSlot")[0].0, '\u{F000}');
    }
}
