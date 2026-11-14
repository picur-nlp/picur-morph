//! Typing slips: the keyboard errors one edit undoes (`REP-KEY-007`).
//!
//! `REP-KEY-007` also lists swapped neighbours, and that half is not implemented: a transposition
//! is one edit away from most short tokens (`data` as `adta`, a line-break fragment `ságra` as
//! `sárga`), so the oracle licenses it everywhere and nothing but the keyboard geometry of Table
//! T-KEY-2 under a language gate would separate those from `mnidig`.

use picur_morph_ortho::case::{capitalise, lower};

use crate::vouched_as_written;

/// The two case patterns a Shift or `CapsLock` slip writes (`REP-KEY-007`).
///
/// `hOGY` is `CapsLock` inverted, `HOgy` is Shift held through the second key. Both are patterns no
/// ordinary word has, so the token itself is the evidence and the lexicon decides what it spells.
/// An acronym cannot match either: both shapes require a lowercase letter where an acronym has a
/// capital.
///
/// Rules: `REP-KEY-007`
pub(crate) fn case_slip(word: &str) -> Option<(String, &'static str)> {
    let letters: Vec<char> = word.chars().filter(|c| c.is_alphabetic()).collect();
    let [first, second, rest @ ..] = letters.as_slice() else {
        return None;
    };
    let inverted =
        first.is_lowercase() && [second].into_iter().chain(rest).all(|c| c.is_uppercase());
    let held = first.is_uppercase()
        && second.is_uppercase()
        && !rest.is_empty()
        && rest.iter().all(|c| c.is_lowercase());
    if !inverted && !held {
        return None;
    }
    let candidate = capitalise(&lower(word));
    (candidate != word && vouched_as_written(&candidate)).then_some((candidate, "REP-KEY-007"))
}

#[cfg(test)]
mod tests {
    use super::case_slip;

    /// `REP-KEY-007`: the rule's own case patterns.
    #[test]
    fn rep_key_007_a_case_slip_is_undone() {
        for (typed, want) in [("hOGY", "Hogy"), ("HOgy", "Hogy"), ("hÁZ", "Ház")] {
            assert_eq!(
                case_slip(typed).map(|(w, _)| w).as_deref(),
                Some(want),
                "{typed}"
            );
        }
    }

    /// An acronym, a name and an ordinary word carry none of the patterns.
    #[test]
    fn rep_key_007_ordinary_case_is_left_alone() {
        for word in ["NATO", "Hogy", "hogy", "USA", "Kovács", "camelCase", "PhD"] {
            assert_eq!(case_slip(word), None, "{word}");
        }
    }
}
