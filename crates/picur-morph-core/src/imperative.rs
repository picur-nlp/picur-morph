//! The imperative-subjunctive stem.
//!
//! The marker is `-j`, but it assimilates to the stem's final consonant in ways that are written,
//! not merely pronounced. This is the one place in the nominal and verbal system where the stem and
//! the marker fuse into a single spelled unit, so the fused stem is computed first and the person
//! endings attach to it.
//!
//! See `docs/grammar/05-verbal.md` (`VRB-IMP-002` onward).

use picur_morph_lexicon::{Role, words};
use picur_morph_ortho::{Letter, letters};

/// Fuses a `j` marker with a stem-final sibilant, written as a doubled letter.
///
/// Shared by the imperative and the present definite, which assimilate identically: `olvas` gives
/// both `olvass` and `olvassa`. The two differ only in what they do elsewhere, so only the sibilant
/// case lives here.
///
/// `None` when the stem does not end in one of `s`, `sz`, `z`, `dz`; `zs`, `cs` and `c` are not in
/// the set.
///
/// Rules: `VRB-IMP-003`, `VRB-PRES-003`
#[must_use]
pub fn sibilant_j(stem: &str) -> Option<String> {
    let span = letters(stem).last()?;
    if span.at + span.text.len() != stem.len() {
        return None;
    }
    let trunk = &stem[..span.at];
    Some(match span.letter {
        Letter::S => format!("{trunk}ss"),
        Letter::Sz => format!("{trunk}ssz"),
        Letter::Z => format!("{trunk}zz"),
        Letter::Dz => format!("{trunk}ddz"),
        _ => return None,
    })
}

/// The imperative stem: the verb stem with the `-j` marker already fused.
///
/// Rules: `VRB-IMP-002`, `VRB-IMP-003`, `VRB-IMP-004`, `VRB-IMP-005`, `VRB-IMP-006`
#[must_use]
pub fn imperative_stem(stem: &str) -> String {
    let ls: Vec<(Letter, usize, usize)> = letters(stem)
        .map(|s| (s.letter, s.at, s.text.len()))
        .collect();
    let Some(&(last, at, len)) = ls.last() else {
        return stem.to_owned();
    };
    let prev = ls.len().checked_sub(2).map(|i| ls[i].0);
    let trunk = &stem[..at];
    let tail = &stem[at..at + len];

    // VRB-IMP-006: `st` and `szt` drop the `t` and the marker assimilates to the sibilant.
    if last == Letter::T {
        if prev == Some(Letter::S) {
            let base = &stem[..ls[ls.len() - 2].1];
            return format!("{base}ss");
        }
        if prev == Some(Letter::Sz) {
            let base = &stem[..ls[ls.len() - 2].1];
            return format!("{base}ssz");
        }
        // VRB-IMP-004: short vowel + `t`, and the listed long-vowel stems, give `ss`.
        let short_vowel = prev.is_some_and(|p| p.is_vowel() && !p.is_long_vowel());
        let ss_exception = words(Role::ImperativeSs)
            .iter()
            .any(|s| stem.ends_with(s.as_ref()));
        if short_vowel || ss_exception {
            return format!("{trunk}ss");
        }
        // VRB-IMP-005: long vowel + `t`, and consonant + `t`, give `ts`.
        return format!("{trunk}ts");
    }

    // VRB-IMP-003: sibilants assimilate totally, written doubled.
    if let Some(fused) = sibilant_j(stem) {
        return fused;
    }

    // VRB-IMP-002: everything else keeps a written `j`. After a stem-final `j` the two are written
    // as a geminate (`fúj` -> `fújj`).
    let _ = tail;
    format!("{stem}j")
}

/// The short definite second singular imperative: `lásd`, `mutasd`, `tartsd`, `mondd`.
///
/// Derived from the fused imperative stem rather than listed, which is what keeps it in step with
/// `VRB-IMP-002..006`: whatever those do to the marker, this undoes by one letter and adds `-d`.
/// Three shapes fall out of that:
///
/// - a vowel stem takes `-dd` (`lő` -> `lődd`)
/// - a stem whose marker is still a plain `j` drops it (`vár` -> `várd`, `ad` -> `add`)
/// - anything else de-geminates the fused consonant (`olvass` -> `olvasd`, `köss` -> `kösd`),
///   except the `ts` stems, which keep both letters (`taníts` -> `tanítsd`)
///
/// Rules: `VRB-IMP-007`
#[must_use]
pub fn short_definite_2sg(stem: &str) -> String {
    // ORT-ASSIM-017: a stem already ending in `dd` writes no third `d` (`fedd`).
    if stem.ends_with("dd") {
        return stem.to_owned();
    }
    let fused = imperative_stem(stem);
    // AkH12 s66: after a vowel the `d` is written double.
    if letters(stem).last().is_some_and(|s| s.letter.is_vowel()) {
        return format!("{stem}dd");
    }
    // AkH12 s76: the `ts` stems keep both letters before the `d`.
    if fused.ends_with("ts") {
        return format!("{fused}d");
    }
    // A plain written `j` is simply dropped; the `d`-final stems then double by themselves.
    if fused == format!("{stem}j") {
        return format!("{stem}d");
    }
    // De-geminate by rebuilding from the first copy's offset. Slicing past that offset would be
    // wrong for a simplified doubled digraph, where the first copy is written with one character:
    // `ossz` is `sz` + `sz` over four bytes, and the single letter is `sz`.
    let ls: Vec<(Letter, usize)> = letters(&fused).map(|s| (s.letter, s.at)).collect();
    match (ls.len().checked_sub(2).map(|i| ls[i]), ls.last()) {
        (Some((a, at)), Some(&(b, _))) if a == b => {
            format!("{}{}d", &fused[..at], a.as_str())
        }
        _ => format!("{fused}d"),
    }
}

#[cfg(test)]
mod tests {
    use super::{imperative_stem, short_definite_2sg};

    #[test]
    fn plain_j_after_most_consonants() {
        // VRB-IMP-002.
        assert_eq!(imperative_stem("vár"), "várj");
        assert_eq!(imperative_stem("ad"), "adj");
        assert_eq!(imperative_stem("hagy"), "hagyj");
        assert_eq!(imperative_stem("lép"), "lépj");
        assert_eq!(imperative_stem("mond"), "mondj");
        assert_eq!(imperative_stem("küld"), "küldj");
        assert_eq!(imperative_stem("dob"), "dobj");
        assert_eq!(imperative_stem("fon"), "fonj");
    }

    #[test]
    fn stem_final_j_doubles() {
        assert_eq!(imperative_stem("fúj"), "fújj");
    }

    #[test]
    fn after_a_vowel_the_j_stays_single() {
        // AkH12 s66: lojon, never *lojjon.
        assert_eq!(imperative_stem("lő"), "lőj");
        assert_eq!(imperative_stem("ró"), "rój");
    }

    #[test]
    fn sibilants_assimilate_totally() {
        // VRB-IMP-003.
        assert_eq!(imperative_stem("olvas"), "olvass");
        assert_eq!(imperative_stem("mos"), "moss");
        assert_eq!(imperative_stem("néz"), "nézz");
        assert_eq!(imperative_stem("hoz"), "hozz");
        assert_eq!(imperative_stem("edz"), "eddz");
    }

    #[test]
    fn short_vowel_plus_t_gives_ss() {
        // VRB-IMP-004: kossek, fusson, nyissa, szeress.
        assert_eq!(imperative_stem("köt"), "köss");
        assert_eq!(imperative_stem("fut"), "fuss");
        assert_eq!(imperative_stem("nyit"), "nyiss");
        assert_eq!(imperative_stem("szeret"), "szeress");
        assert_eq!(imperative_stem("alkot"), "alkoss");
    }

    #[test]
    fn lat_is_an_ss_exception_despite_its_long_vowel() {
        // VRB-IMP-004 names lat, bocsat and lot explicitly.
        assert_eq!(imperative_stem("lát"), "láss");
        assert_eq!(imperative_stem("bocsát"), "bocsáss");
    }

    #[test]
    fn long_vowel_plus_t_and_consonant_plus_t_give_ts() {
        // VRB-IMP-005.
        assert_eq!(imperative_stem("tanít"), "taníts");
        assert_eq!(imperative_stem("fűt"), "fűts");
        assert_eq!(imperative_stem("tart"), "tarts");
        assert_eq!(imperative_stem("hajt"), "hajts");
        assert_eq!(imperative_stem("önt"), "önts");
        assert_eq!(imperative_stem("ért"), "érts");
    }

    #[test]
    fn st_and_szt_stems_drop_the_t() {
        // VRB-IMP-006.
        assert_eq!(imperative_stem("fest"), "fess");
        assert_eq!(imperative_stem("oszt"), "ossz");
        assert_eq!(imperative_stem("ereszt"), "eressz");
        assert_eq!(imperative_stem("választ"), "válassz");
    }

    #[test]
    fn short_definite_2sg_by_stem_type() {
        // VRB-IMP-007, one case per bullet of the rule.
        for (stem, want) in [
            ("vár", "várd"),
            ("kér", "kérd"),
            ("dob", "dobd"),
            ("hagy", "hagyd"),
            ("hív", "hívd"),
            ("tanul", "tanuld"),
            ("ad", "add"),
            ("véd", "védd"),
            ("tud", "tudd"),
            ("mond", "mondd"),
            ("küld", "küldd"),
            ("kezd", "kezdd"),
            ("old", "oldd"),
            ("lő", "lődd"),
            ("ró", "ródd"),
            ("sző", "sződd"),
            ("olvas", "olvasd"),
            ("néz", "nézd"),
            ("hoz", "hozd"),
            ("edz", "edzd"),
            ("köt", "kösd"),
            ("nyit", "nyisd"),
            ("mutat", "mutasd"),
            ("lát", "lásd"),
            ("bocsát", "bocsásd"),
            ("tanít", "tanítsd"),
            ("tart", "tartsd"),
            ("ment", "mentsd"),
            ("ért", "értsd"),
            ("tát", "tátsd"),
            ("fest", "fesd"),
            ("oszt", "oszd"),
            ("választ", "válaszd"),
        ] {
            assert_eq!(short_definite_2sg(stem), want, "{stem}");
        }
    }

    #[test]
    fn a_doubled_digraph_degeminates_to_one_letter() {
        // The first copy of a simplified doubled digraph is written with one character, so the
        // cut cannot be made by byte length: `ossz` -> `oszd`, never `*ossd`.
        assert_eq!(short_definite_2sg("játsz"), "játszd");
        assert_eq!(short_definite_2sg("metsz"), "metszd");
    }
}
