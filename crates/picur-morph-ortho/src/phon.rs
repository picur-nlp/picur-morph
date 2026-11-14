//! Phonological features of the letters.
//!
//! One const table over the 44-letter alphabet, which is a definition of the language's sound
//! system rather than an override list. It replaces several rule tables at once: the accusative
//! coda classes of `NOM-ACC-003`, the `-vAl` assimilation target of `NOM-CASE-003`, and the voicing
//! assimilation of `PH-ASSIM`.
//!
//! See `docs/design/02-generation.md` ("The bare accusative is a natural class, not a list").

use crate::Letter;

/// Place of articulation.
///
/// `Postalveolar` and `Palatal` are kept distinct from `Coronal` because the language needs the
/// difference elsewhere, even though several rules group all three.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Place {
    /// `b p m f v`
    Labial,
    /// `d t n l r c dz sz z`
    Coronal,
    /// `cs dzs s zs`
    Postalveolar,
    /// `gy ty ny j ly`
    Palatal,
    /// `g k`
    Velar,
    /// `h`
    Glottal,
}

/// Manner of articulation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Manner {
    /// `b p d t gy ty g k`
    Stop,
    /// `c cs dz dzs`
    Affricate,
    /// `f v sz z s zs h`
    Fricative,
    /// `m n ny`
    Nasal,
    /// `l`
    Lateral,
    /// `r`
    Trill,
    /// `j ly`
    Approximant,
}

/// The phonological features of one consonant letter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Consonant {
    /// Place of articulation.
    pub place: Place,
    /// Manner of articulation.
    pub manner: Manner,
    /// Whether the consonant is voiced.
    pub voiced: bool,
}

impl Consonant {
    /// Whether this is a sonorant: a nasal, liquid or glide.
    #[must_use]
    pub const fn is_sonorant(self) -> bool {
        matches!(
            self.manner,
            Manner::Nasal | Manner::Lateral | Manner::Trill | Manner::Approximant
        )
    }

    /// Whether this is a sibilant fricative: `s sz z zs`.
    ///
    /// The affricates `c cs dz dzs` have a sibilant release but are **not** sibilants for the rules
    /// that matter here, which is why `perec` takes `perecet` and not `*perect`.
    #[must_use]
    pub const fn is_sibilant(self) -> bool {
        matches!(self.manner, Manner::Fricative)
            && matches!(self.place, Place::Coronal | Place::Postalveolar)
            && !matches!(self.manner, Manner::Stop)
    }
}

const fn c(place: Place, manner: Manner, voiced: bool) -> Consonant {
    Consonant {
        place,
        manner,
        voiced,
    }
}

/// The features of a letter, or `None` for a vowel or a letter with no native sound value.
///
/// `q`, `w`, `x` and `y` occur only in foreign spellings and names; `x` is pronounced as a cluster
/// `ks`, so it has no single set of features and is deliberately `None`.
///
/// Rules: `PH-CONS-001`, `NOM-ACC-003`
#[must_use]
pub const fn features(l: Letter) -> Option<Consonant> {
    use Manner::{Affricate, Approximant, Fricative, Lateral, Nasal, Stop, Trill};
    use Place::{Coronal, Glottal, Labial, Palatal, Postalveolar, Velar};
    match l {
        Letter::B => Some(c(Labial, Stop, true)),
        Letter::P => Some(c(Labial, Stop, false)),
        Letter::M => Some(c(Labial, Nasal, true)),
        Letter::F => Some(c(Labial, Fricative, false)),
        Letter::V => Some(c(Labial, Fricative, true)),

        Letter::D => Some(c(Coronal, Stop, true)),
        Letter::T => Some(c(Coronal, Stop, false)),
        Letter::N => Some(c(Coronal, Nasal, true)),
        Letter::L => Some(c(Coronal, Lateral, true)),
        Letter::R => Some(c(Coronal, Trill, true)),
        Letter::C => Some(c(Coronal, Affricate, false)),
        Letter::Dz => Some(c(Coronal, Affricate, true)),
        Letter::Sz => Some(c(Coronal, Fricative, false)),
        Letter::Z => Some(c(Coronal, Fricative, true)),

        Letter::Cs => Some(c(Postalveolar, Affricate, false)),
        Letter::Dzs => Some(c(Postalveolar, Affricate, true)),
        Letter::S => Some(c(Postalveolar, Fricative, false)),
        Letter::Zs => Some(c(Postalveolar, Fricative, true)),

        Letter::Gy => Some(c(Palatal, Stop, true)),
        Letter::Ty => Some(c(Palatal, Stop, false)),
        Letter::Ny => Some(c(Palatal, Nasal, true)),
        Letter::J | Letter::Ly => Some(c(Palatal, Approximant, true)),

        Letter::G => Some(c(Velar, Stop, true)),
        Letter::K => Some(c(Velar, Stop, false)),
        Letter::H => Some(c(Glottal, Fricative, false)),

        _ => None,
    }
}

/// Whether the accusative `-t` attaches bare after this letter.
///
/// `NOM-ACC-003` lists `j`/`ly`, `l`, `r`, `n`, `ny`, `s`, `sz`, `z`, `zs`. Read phonologically that
/// is exactly **the coronal and palatal sonorants plus the sibilants**, so this is a predicate over
/// features rather than a lookup table. Everything taking a linking vowel falls outside it: `m` is a
/// nasal but labial (`nyomot`), `f v h` are non-sibilant fricatives (`ivet`, `dohot`), and the stops
/// and affricates are obstruents (`botot`, `perecet`).
///
/// Rules: `NOM-ACC-003`
#[must_use]
pub const fn bare_t_class(l: Letter) -> bool {
    match features(l) {
        Some(f) => {
            (f.is_sonorant() && matches!(f.place, Place::Coronal | Place::Palatal))
                || f.is_sibilant()
        }
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{Manner, Place, bare_t_class, features};
    use crate::Letter;

    /// Every letter in `NOM-ACC-003`'s bare column.
    const BARE: [Letter; 9] = [
        Letter::J,
        Letter::Ly,
        Letter::L,
        Letter::R,
        Letter::N,
        Letter::Ny,
        Letter::S,
        Letter::Sz,
        Letter::Z,
    ];

    /// Letters `NOM-ACC-003` puts in the linking-vowel column.
    const LINKED: [Letter; 14] = [
        Letter::P,
        Letter::B,
        Letter::T,
        Letter::D,
        Letter::K,
        Letter::G,
        Letter::Ty,
        Letter::Gy,
        Letter::F,
        Letter::V,
        Letter::H,
        Letter::M,
        Letter::C,
        Letter::Cs,
    ];

    #[test]
    fn bare_column_of_nom_acc_003() {
        for l in BARE {
            assert!(bare_t_class(l), "{l:?} must take bare -t");
        }
        assert!(bare_t_class(Letter::Zs));
    }

    #[test]
    fn linking_vowel_column_of_nom_acc_003() {
        for l in LINKED {
            assert!(!bare_t_class(l), "{l:?} must take a linking vowel");
        }
        assert!(!bare_t_class(Letter::Dzs));
    }

    #[test]
    fn m_is_a_nasal_but_labial() {
        // The case that proves the class is "coronal sonorant", not just "sonorant": nyom -> nyomot.
        let m = features(Letter::M).expect("m is a consonant");
        assert!(m.is_sonorant());
        assert_eq!(m.place, Place::Labial);
        assert!(!bare_t_class(Letter::M));
    }

    #[test]
    fn affricates_are_not_sibilants() {
        // perec -> perecet, not *perect.
        for l in [Letter::C, Letter::Cs, Letter::Dz, Letter::Dzs] {
            let f = features(l).expect("affricate");
            assert_eq!(f.manner, Manner::Affricate);
            assert!(!f.is_sibilant(), "{l:?} must not count as a sibilant");
        }
    }

    #[test]
    fn the_four_sibilants() {
        for l in [Letter::S, Letter::Sz, Letter::Z, Letter::Zs] {
            assert!(features(l).expect("sibilant").is_sibilant(), "{l:?}");
        }
        for l in [Letter::F, Letter::V, Letter::H] {
            assert!(!features(l).expect("fricative").is_sibilant(), "{l:?}");
        }
    }

    #[test]
    fn vowels_and_foreign_letters_have_no_features() {
        for l in [Letter::A, Letter::E, Letter::I, Letter::O, Letter::U] {
            assert!(features(l).is_none(), "{l:?} is a vowel");
        }
        // x is pronounced as the cluster ks, so it has no single feature set.
        assert!(features(Letter::X).is_none());
        assert!(!bare_t_class(Letter::X));
    }

    #[test]
    fn voicing_pairs_agree_in_place_and_manner() {
        // PH-ASSIM depends on this being consistent.
        for (voiced, voiceless) in [
            (Letter::B, Letter::P),
            (Letter::D, Letter::T),
            (Letter::G, Letter::K),
            (Letter::Gy, Letter::Ty),
            (Letter::Z, Letter::Sz),
            (Letter::Zs, Letter::S),
            (Letter::Dz, Letter::C),
            (Letter::Dzs, Letter::Cs),
        ] {
            let a = features(voiced).expect("voiced");
            let b = features(voiceless).expect("voiceless");
            assert_eq!(a.place, b.place, "{voiced:?}/{voiceless:?} place");
            assert_eq!(a.manner, b.manner, "{voiced:?}/{voiceless:?} manner");
            assert!(a.voiced && !b.voiced, "{voiced:?}/{voiceless:?} voicing");
        }
    }
}

/// Whether `word` ends the way a monomorphemic Hungarian word may (`PH-SYL-007`).
///
/// A final vowel or a single consonant is always well-formed. A final pair is well-formed when
/// sonority falls into it (a sonorant before an obstruent, a liquid before a nasal, `r` or `j`
/// before `l`), when it is a geminate, when an obstruent pair is a coronal fricative before a
/// coronal stop (`st`, `szt`, `zd`), when a nasal precedes a homorganic stop or affricate, or when
/// `v` or `j` follows a liquid or nasal (`érv`, `könyv`, `férj`). Three consonants are never
/// well-formed. The
/// clusters outside these patterns occur only in listed irregular words, so a stem nobody has
/// listed cannot end in one.
#[must_use]
pub fn well_formed_final_cluster(word: &str) -> bool {
    let tail: Vec<Letter> = crate::letters(word)
        .map(|s| s.letter)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .take_while(|l| l.is_consonant())
        .collect();
    let (Some(&last), Some(&first)) = (tail.first(), tail.get(1)) else {
        return true;
    };
    if tail.len() > 2 {
        return false;
    }
    let (Some(a), Some(b)) = (features(first), features(last)) else {
        return false;
    };
    let liquid = |x: Consonant| matches!(x.manner, Manner::Lateral | Manner::Trill);
    let coronal_fricative = |x: Consonant| {
        x.manner == Manner::Fricative && matches!(x.place, Place::Coronal | Place::Postalveolar)
    };
    first == last
        || (a.is_sonorant() && !b.is_sonorant())
        || (liquid(a) && b.manner == Manner::Nasal)
        || ((first == Letter::R || first == Letter::J) && last == Letter::L)
        || (coronal_fricative(a) && b.manner == Manner::Stop && b.place == Place::Coronal)
        || (a.manner == Manner::Nasal
            && matches!(b.manner, Manner::Stop | Manner::Affricate)
            && a.place == b.place)
        // `j` after a liquid or nasal is among the rule's regular examples: `férj`, `sarj`, `szomj`.
        || ((liquid(a) || a.manner == Manner::Nasal) && matches!(last, Letter::V | Letter::J))
}

#[cfg(test)]
mod cluster_tests {
    use super::well_formed_final_cluster;

    #[test]
    fn ph_syl_007_the_rules_examples() {
        for w in [
            "part", "kard", "halk", "lomb", "ront", "rend", "ponty", "gyöngy", "lánc", "kincs",
            "test", "liszt", "most", "film", "kert", "fürj", "férj", "majd", "csepp", "sakk",
            "meggy", "vicc", "friss", "érv", "könyv", "ház", "alma",
        ] {
            assert!(well_formed_final_cluster(w), "{w}");
        }
        for w in ["multb", "Lillianr", "boszankodásb", "nyomtk", "kapd"] {
            assert!(!well_formed_final_cluster(w), "{w}");
        }
    }
}
