//! Stem alternation: the shape a suffix actually attaches to.
//!
//! Alternants are computed from the alternation type, never stored. Storing both `ló` and `lov`
//! would store what the type already determines, and would let them disagree.
//!
//! The trigger is uniform: a class I suffix sees the bound alternant, everything else sees the
//! lemma. That is `NOM-STEM-010`'s "before class I suffixes", and it makes epenthesis fall out for
//! free, because `bokor` -> `bokr` turns a bare-class coda into a cluster and the cluster is what
//! forces the linking vowel in `bokrot`.

use std::borrow::Cow;

use picur_morph_lexicon::{Entry, Trait};
use picur_morph_ortho::{Letter, letters};

use crate::features::Person;
use crate::suffix::{Suffix, SuffixClass};

/// Index and letter of the last vowel in `stem`.
fn last_vowel(stem: &str) -> Option<(usize, Letter, usize)> {
    letters(stem)
        .filter(|s| s.letter.is_vowel())
        .last()
        .map(|s| (s.at, s.letter, s.text.len()))
}

/// Replaces the last vowel of `stem` with `to`.
fn replace_last_vowel(stem: &str, to: &str) -> Option<String> {
    let (at, _, len) = last_vowel(stem)?;
    Some(format!("{}{to}{}", &stem[..at], &stem[at + len..]))
}

/// Shortens the last vowel, if it is long.
///
/// Rules: `NOM-STEM-008`
fn shorten_last_vowel(stem: &str) -> Option<String> {
    let (at, letter, len) = last_vowel(stem)?;
    let short = letter.short()?;
    Some(format!(
        "{}{}{}",
        &stem[..at],
        short.as_str(),
        &stem[at + len..]
    ))
}

/// Drops the last vowel, used for epenthetic and u-deleting stems.
fn drop_last_vowel(stem: &str) -> Option<String> {
    let (at, _, len) = last_vowel(stem)?;
    Some(format!("{}{}", &stem[..at], &stem[at + len..]))
}

/// The low counterpart of a long vowel, for the vowel-changing v-stems.
///
/// `tó` -> `tav-`, `lé` -> `lev-`: a back long vowel lowers to `a`, a front one to `e`.
///
/// Rules: `NOM-STEM-010`
const fn lowered(l: Letter) -> Option<&'static str> {
    match l {
        Letter::OAcute | Letter::UAcute | Letter::AAcute => Some("a"),
        Letter::ODblAcute | Letter::UDblAcute | Letter::EAcute => Some("e"),
        _ => None,
    }
}

/// The stem of an epenthetic lexeme with its unstable vowel dropped: `jelez` -> `jelz`.
///
/// Exposed because the verb path cannot go through [`alternant`]: its gate asks whether the suffix
/// begins with an archiphoneme, and the definite `-i` is a literal. Widening that gate would make
/// `-ig` and `-ért` vowel-initial too and give `kezig` for `kézig`.
///
/// Rules: `NOM-STEM-004`, `VRB-STEM-007`
#[must_use]
pub fn syncopated(lemma: &str) -> Option<String> {
    drop_last_vowel(lemma).map(|s| metathesised(&s))
}

/// `NOM-STEM-005`: the metathesis stems. Syncope leaves `h` before the final consonant, a coda
/// no Hungarian word has, and the two swap: `teher` -> `terh-`, `pehely` -> `pelyh-`,
/// `kehely` -> `kelyh-`.
fn metathesised(stem: &str) -> String {
    let spans: Vec<_> = letters(stem).collect();
    if let [.., h, last] = spans.as_slice()
        && h.letter == Letter::H
        && last.letter.is_consonant()
    {
        return format!("{}{}{}", &stem[..h.at], last.text, h.text);
    }
    stem.to_owned()
}

/// The possessive stem of an `ajtó`-type noun: `ajtó` -> `ajta`, `idő` -> `ide`, `szülő` -> `szüle`.
///
/// The change is the final `ó`/`ő` becoming `a`/`e`, and nothing more. `NOM-STEM-013` writes the
/// class as "`a`/`e` plus `j`", but the `j` is the ordinary `-jA` possessive allomorph doing its
/// normal job after a vowel, so putting it in the stem is what forced exceptions: the stem `ajtaj`
/// cannot produce `szülei`, which is this same change plus the possessed plural's bare `-i`.
///
/// The result is a bound alternant, so its final vowel is not a free one and `NOM-LENG-001` does
/// not lengthen it: `szüle` plus `-i` is `szülei`, not `*szüléi`.
///
/// Rules: `NOM-STEM-013`
#[must_use]
pub fn possessive_alt(lemma: &str) -> Option<String> {
    let (at, letter, len) = last_vowel(lemma)?;
    let low = match letter {
        Letter::OAcute => "a",
        Letter::ODblAcute => "e",
        _ => return None,
    };
    Some(format!("{}{low}{}", &lemma[..at], &lemma[at + len..]))
}

/// Whether a personal possessive keeps the free stem of a high-vowel-to-v noun.
///
/// `PH-VSTEM-004`: the `v` alternant of `falu`, `daru`, `odú`, `tetű` appears before the plural
/// `-k` and the possessed plural `-i`; the first and second person possessives and the third
/// plural `-jUk` take the vowel stem (`falum` 3,585 : `falvam` 17, `falujuk` 1,448 : `falvuk` 3
/// `[WC2]`). The third singular `-A` of a `-A` noun keeps the alternant (`tetve`).
///
/// Rules: `PH-VSTEM-004`
#[must_use]
pub fn free_personal_possessive(entry: &Entry, person: Person) -> bool {
    entry.has(Trait::VStem)
        && entry.has(Trait::UStem)
        && (person != Person::Third || !entry.has(Trait::PossessiveA))
}

/// The stem shape `suffix` attaches to.
///
/// Rules: `NOM-STEM-006`, `NOM-STEM-008`, `NOM-STEM-010`, `NOM-STEM-012`
#[must_use]
pub fn alternant<'a>(entry: &'a Entry, suffix: &Suffix) -> Cow<'a, str> {
    // What selects the bound alternant is a vowel meeting the stem, not the attachment class as
    // such. Class I is the inflectional case, where the vowel is the optional link; a derivational
    // suffix that carries its own vowel selects it just the same, which is why `mozog` + `-Ás` is
    // `mozgás` (`DER-MODEL-006`).
    if suffix.class != SuffixClass::One && !suffix.starts_with_vowel() {
        return Cow::Borrowed(&entry.lemma);
    }
    bound_alternant(entry, suffix)
}

/// The bound stem of `entry`, without asking whether `suffix` selects it.
///
/// `DER-MODEL-006` states the stem per derivational suffix rather than leaving it to the suffix's
/// shape, so a caller that has read the rule's stem column decides and this applies it.
///
/// Rules: `NOM-STEM-006`, `NOM-STEM-008`, `NOM-STEM-010`, `NOM-STEM-012`
#[must_use]
pub fn bound_alternant<'a>(entry: &'a Entry, suffix: &Suffix) -> Cow<'a, str> {
    let lemma: &str = &entry.lemma;
    let t = entry.traits;

    if t.has(Trait::OaStem) {
        if let Some((_, letter, _)) = last_vowel(lemma)
            && let Some(low) = lowered(letter)
            && let Some(s) = replace_last_vowel(lemma, low)
        {
            return Cow::Owned(format!("{s}v"));
        }
        return Cow::Borrowed(lemma);
    }

    // v-stem: `mű` -> `műv-`, with Shortening `ló` -> `lov-`, and with UStem `daru` -> `darv-`,
    // where the `v` replaces the final vowel instead of following it, so UStem is read first.
    if t.has(Trait::VStem) {
        let base = if t.has(Trait::UStem) {
            drop_last_vowel(lemma).unwrap_or_else(|| lemma.to_owned())
        } else if t.has(Trait::Shortening) {
            shorten_last_vowel(lemma).unwrap_or_else(|| lemma.to_owned())
        } else {
            lemma.to_owned()
        };
        return Cow::Owned(format!("{base}v"));
    }

    if t.has(Trait::UStem) {
        if let Some(s) = drop_last_vowel(lemma) {
            return Cow::Owned(s);
        }
        return Cow::Borrowed(lemma);
    }

    if t.has(Trait::Epenthetic) {
        let Some(dropped) = drop_last_vowel(lemma).map(|s| metathesised(&s)) else {
            return Cow::Borrowed(lemma);
        };
        // The alternations compose in this order: `lélek` drops the epenthetic vowel to
        // `lélk` and then shortens what is now the last vowel, giving `lelk-`. Returning after the
        // drop leaves every inflected form of the `lélek` family unanalysable.
        if t.has(Trait::Shortening)
            && let Some(s) = shorten_last_vowel(&dropped)
        {
            return Cow::Owned(s);
        }
        return Cow::Owned(dropped);
    }

    // Shortening: `kéz` -> `kez-`, `nyár` -> `nyar-`. The superessive is the exception and keeps
    // the long vowel: `kézen`, `nyáron`, `úton`.
    if t.has(Trait::Shortening)
        && !suffix.free_stem
        && let Some(s) = shorten_last_vowel(lemma)
    {
        return Cow::Owned(s);
    }
    Cow::Borrowed(lemma)
}

#[cfg(test)]
mod tests {
    use picur_morph_lexicon::{Entry, Pos, Trait, TraitSet};

    use super::alternant;
    use crate::features::Case;
    use crate::suffix::case_suffix;

    fn entry(lemma: &str, traits: &[Trait]) -> Entry {
        let mut e = Entry::new(lemma, Pos::Noun);
        e.traits = TraitSet::from_iter_of(traits.iter().copied());
        e
    }

    fn bound(lemma: &str, traits: &[Trait]) -> String {
        let acc = case_suffix(Case::Acc).expect("acc");
        alternant(&entry(lemma, traits), &acc).into_owned()
    }

    fn free(lemma: &str, traits: &[Trait]) -> String {
        let ine = case_suffix(Case::Ine).expect("ine");
        alternant(&entry(lemma, traits), &ine).into_owned()
    }

    #[test]
    fn class_two_suffixes_see_the_lemma() {
        // NOM-STEM-010: alternation is a class I phenomenon. kezben, loban, bokorban.
        assert_eq!(free("kéz", &[Trait::Shortening]), "kéz");
        assert_eq!(free("ló", &[Trait::VStem, Trait::Shortening]), "ló");
        assert_eq!(free("bokor", &[Trait::Epenthetic]), "bokor");
    }

    #[test]
    fn epenthetic_drops_the_unstable_vowel() {
        assert_eq!(bound("bokor", &[Trait::Epenthetic]), "bokr");
        assert_eq!(bound("retek", &[Trait::Epenthetic]), "retk");
        assert_eq!(bound("majom", &[Trait::Epenthetic]), "majm");
        assert_eq!(bound("álom", &[Trait::Epenthetic]), "álm");
    }

    #[test]
    fn shortening_shortens_the_last_long_vowel() {
        assert_eq!(bound("kéz", &[Trait::Shortening]), "kez");
        assert_eq!(bound("nyár", &[Trait::Shortening]), "nyar");
        assert_eq!(bound("madár", &[Trait::Shortening]), "madar");
        assert_eq!(bound("tél", &[Trait::Shortening]), "tel");
    }

    #[test]
    fn non_shortening_v_stems_keep_their_vowel() {
        // NOM-STEM-010: mu -> muv-, bo -> bov-.
        assert_eq!(bound("mű", &[Trait::VStem]), "műv");
    }

    #[test]
    fn shortening_v_stems_shorten_and_add_v() {
        assert_eq!(bound("ló", &[Trait::VStem, Trait::Shortening]), "lov");
        assert_eq!(bound("kő", &[Trait::VStem, Trait::Shortening]), "köv");
        assert_eq!(bound("cső", &[Trait::VStem, Trait::Shortening]), "csöv");
        assert_eq!(bound("fű", &[Trait::VStem, Trait::Shortening]), "füv");
    }

    #[test]
    fn vowel_changing_v_stems_lower_and_add_v() {
        assert_eq!(bound("tó", &[Trait::OaStem]), "tav");
        assert_eq!(bound("hó", &[Trait::OaStem]), "hav");
        assert_eq!(bound("szó", &[Trait::OaStem]), "szav");
        assert_eq!(bound("lé", &[Trait::OaStem]), "lev");
    }

    #[test]
    fn u_deleting_stems_drop_the_final_vowel() {
        assert_eq!(bound("borjú", &[Trait::UStem]), "borj");
        assert_eq!(bound("varjú", &[Trait::UStem]), "varj");
    }

    #[test]
    fn plain_stems_are_unchanged() {
        assert_eq!(bound("ház", &[]), "ház");
        assert_eq!(bound("alma", &[]), "alma");
    }
}
