//! The feature model.
//!
//! Variants are named after UD values where UD has them (`Case::Ine`, `Mood::Pot`), so the
//! projection is mostly mechanical. Our model is deliberately richer than its projection: it keeps
//! distinctions UD cannot express, which is the correct direction for a source-of-truth library.
//!
//! See `docs/design/01-core-model.md` and `docs/grammar/03-nominal.md`.

use serde::{Deserialize, Serialize};

/// Grammatical case.
///
/// The inventory is `NOM-CASE-001`, a closed list the language actually has. Four values carry our
/// own names because UD has no home for them; see [`Case::ud`].
///
/// Rules: `NOM-CASE-001`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Case {
    /// Nominative, zero-marked.
    Nom,
    /// Accusative `-(O)t`.
    Acc,
    /// Dative `-nAk`.
    Dat,
    /// Instrumental-comitative `-vAl`.
    Ins,
    /// Causal-final `-ért`.
    Cau,
    /// Translative-factive `-vÁ`.
    Tra,
    /// Terminative `-ig`.
    Ter,
    /// Essive `-Ul`.
    Ess,
    /// Inessive `-bAn`.
    Ine,
    /// Illative `-bA`.
    Ill,
    /// Elative `-bÓl`.
    Ela,
    /// Superessive `-(O)n`.
    Sup,
    /// Sublative `-rA`.
    Sbl,
    /// Delative `-rÓl`.
    Del,
    /// Adessive `-nÁl`.
    Ade,
    /// Allative `-hOz`.
    All,
    /// Ablative `-tÓl`.
    Abl,
    /// Temporal `-kor`.
    Tem,
    /// Locative `-t`/`-Ott`, lexically restricted (`Pécsett`).
    Loc,
    /// Essive-formal `-ként`. Our name; UD projects it as `Abs`.
    Formalis,
    /// Distributive `-(O)nként`. Our name; UD projects it as `Dis`.
    Distributivus,
    /// Sociative `-(O)stUl`. Our name; **no UD `Case` value exists** for it.
    Sociativus,
    /// Modal-formal `-képp`/`-képpen`. Our name; UD projects it as `Abs`.
    Modalis,
    /// Distributive-temporal `-(O)ntA`. Our name; UD has no value.
    DistributivusTemporalis,
}

impl UdCase {
    /// The value UD writes for this case, as in `Case=Ine`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Nom => "Nom",
            Self::Acc => "Acc",
            Self::Dat => "Dat",
            Self::Ins => "Ins",
            Self::Cau => "Cau",
            Self::Tra => "Tra",
            Self::Ter => "Ter",
            Self::Ess => "Ess",
            Self::Ine => "Ine",
            Self::Ill => "Ill",
            Self::Ela => "Ela",
            Self::Sup => "Sup",
            Self::Sbl => "Sbl",
            Self::Del => "Del",
            Self::Ade => "Ade",
            Self::All => "All",
            Self::Abl => "Abl",
            Self::Tem => "Tem",
            Self::Loc => "Loc",
            Self::Abs => "Abs",
            Self::Dis => "Dis",
        }
    }
}

impl Case {
    /// Every case, in declaration order.
    pub const ALL: [Self; 24] = [
        Self::Nom,
        Self::Acc,
        Self::Dat,
        Self::Ins,
        Self::Cau,
        Self::Tra,
        Self::Ter,
        Self::Ess,
        Self::Ine,
        Self::Ill,
        Self::Ela,
        Self::Sup,
        Self::Sbl,
        Self::Del,
        Self::Ade,
        Self::All,
        Self::Abl,
        Self::Tem,
        Self::Loc,
        Self::Formalis,
        Self::Distributivus,
        Self::Sociativus,
        Self::Modalis,
        Self::DistributivusTemporalis,
    ];
}

/// A UD `Case` value permitted for Hungarian.
///
/// `[UD-feats]` sets `Case.lvalues = []` for `hu`, so **no language-specific value may be emitted**.
/// That constraint is why `Case=Frm`, `Case=Com` and `Case=Inl` are absent here: emitting them is a
/// validation error, not a stylistic choice.
///
/// Rules: `NOM-CASE-001`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[non_exhaustive]
pub enum UdCase {
    /// `Case=Nom`
    Nom,
    /// `Case=Acc`
    Acc,
    /// `Case=Dat`
    Dat,
    /// `Case=Ins`
    Ins,
    /// `Case=Cau`
    Cau,
    /// `Case=Tra`
    Tra,
    /// `Case=Ter`
    Ter,
    /// `Case=Ess`
    Ess,
    /// `Case=Ine`
    Ine,
    /// `Case=Ill`
    Ill,
    /// `Case=Ela`
    Ela,
    /// `Case=Sup`
    Sup,
    /// `Case=Sbl`
    Sbl,
    /// `Case=Del`
    Del,
    /// `Case=Ade`
    Ade,
    /// `Case=All`
    All,
    /// `Case=Abl`
    Abl,
    /// `Case=Tem`
    Tem,
    /// `Case=Loc`
    Loc,
    /// `Case=Abs`, used for `-ként` and `-képp(en)`.
    Abs,
    /// `Case=Dis`, used for `-nként`.
    Dis,
}

impl Case {
    /// The UD projection, which is lossy and one-way.
    ///
    /// `None` is a recorded decision, not an omission. `-stUl` has no UD `Case` home at all, and
    /// `-(O)ntA` has none either; the projection omits the feature rather than inventing a value.
    ///
    /// Rules: `NOM-CASE-001`
    #[must_use]
    pub const fn ud(self) -> Option<UdCase> {
        Some(match self {
            Self::Nom => UdCase::Nom,
            Self::Acc => UdCase::Acc,
            Self::Dat => UdCase::Dat,
            Self::Ins => UdCase::Ins,
            Self::Cau => UdCase::Cau,
            Self::Tra => UdCase::Tra,
            Self::Ter => UdCase::Ter,
            Self::Ess => UdCase::Ess,
            Self::Ine => UdCase::Ine,
            Self::Ill => UdCase::Ill,
            Self::Ela => UdCase::Ela,
            Self::Sup => UdCase::Sup,
            Self::Sbl => UdCase::Sbl,
            Self::Del => UdCase::Del,
            Self::Ade => UdCase::Ade,
            Self::All => UdCase::All,
            Self::Abl => UdCase::Abl,
            Self::Tem => UdCase::Tem,
            Self::Loc => UdCase::Loc,
            Self::Formalis | Self::Modalis => UdCase::Abs,
            Self::Distributivus => UdCase::Dis,
            Self::Sociativus | Self::DistributivusTemporalis => return None,
        })
    }
}

/// Grammatical person.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Person {
    /// First person.
    First,
    /// Second person.
    Second,
    /// Third person.
    Third,
}

impl Person {
    /// Every variant.
    pub const ALL: [Self; 3] = [Self::First, Self::Second, Self::Third];
}

/// Grammatical number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Number {
    /// Singular.
    Sing,
    /// Plural.
    Plur,
}

impl Number {
    /// Every variant.
    pub const ALL: [Self; 2] = [Self::Sing, Self::Plur];
}

/// Object definiteness on a finite verb.
///
/// Rules: `SYN-DEF-001`
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Definite {
    /// Indefinite (alanyi) conjugation.
    Ind,
    /// Definite (targyas) conjugation.
    Def,
    /// The `-lak`/`-lek` form, first person subject acting on a second person object.
    /// UD writes this `Definite=2`, not `Person[obj]=2`.
    Two,
}

/// Verbal mood.
///
/// The potential is **not** a value here. `-hAt` occupies its own slot before mood and tense
/// (`VRB-MODEL-001`), and it combines with every mood: `olvashatnék` is potential *and*
/// conditional, `olvashasson` potential *and* imperative. A `Pot` variant alongside `Cnd` would
/// make those two forms inexpressible. See [`Morph::Potential`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Mood {
    /// Indicative.
    Ind,
    /// Conditional.
    Cnd,
    /// Imperative-subjunctive.
    Imp,
}

/// Verbal tense.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Tense {
    /// Present.
    Pres,
    /// Past.
    Past,
}

/// One layer of the feature stack, innermost first.
///
/// Slot order is enforced by the morphotactics, not by this type; see `NOM-MODEL-003`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Morph {
    /// Plural of the noun itself.
    Plural,
    /// Possessive, carrying the person and number of the *possessor*.
    Possessive {
        /// Person of the possessor.
        person: Person,
        /// Number of the possessor.
        number: Number,
    },
    /// Plural of the possessed thing (`-i` in `házai`).
    PossessedPlural,
    /// Anaphoric possessive `-é`, and `-éi` when combined with [`Morph::PossessedPlural`].
    Anaphoric,
    /// Comparative `-(A)bb` of a gradable adjective or adverb.
    Comparative,
    /// Superlative `leg-` + comparative. Rules: `NOM-ADJ-006`.
    Superlative,
    /// Excessive `legesleg-` + comparative. Rules: `NOM-ADJ-006`.
    Excessive,
    /// Associative plural `-ék`, `X and their family or group`.
    Associative,
    /// Collective `-An` on a numeral or quantifier: `hárman`, `öten`, `sokan`.
    ///
    /// Rules: `PND-NUMINF-005`
    Collective,
    /// Multiplicative `-szOr`: `háromszor`, `ötször`, `hússzor`.
    ///
    /// Rules: `PND-NUMINF-006`
    Multiplicative,
    /// A derivational suffix, which builds a new stem that then inflects.
    ///
    /// Rules: `DER-MODEL-003`
    Derive(crate::derivation::DerId),
    /// Potential `-hAt`, which builds a new verb stem that then inflects normally.
    ///
    /// A slot of its own rather than a mood, so it can precede any of them (`VRB-MODEL-001`). UD
    /// writes `Mood=Pot` when no other mood marker follows.
    Potential,
    /// Infinitive `-ni`.
    Infinitive,
    /// Present participle `-Ó` (`váró`).
    PresentParticiple,
    /// Past participle `-t` / `-Ott` (`várt`).
    PastParticiple,
    /// Adverbial participle `-vA` (`várva`).
    Converb,
    /// Future participle `-AndÓ` (`olvasandó`), 'to be V-ed'.
    ///
    /// Rules: `VRB-PART-003`
    FutureParticiple,
    /// Case.
    Case(Case),
    /// Finite verb features.
    Verbal {
        /// Mood.
        mood: Mood,
        /// Tense.
        tense: Tense,
        /// Subject person.
        person: Person,
        /// Subject number.
        number: Number,
        /// Object definiteness.
        definite: Definite,
    },
}

/// A feature value by its name in the data files, which is its `Debug` name.
pub(crate) fn named<T: std::fmt::Debug + Copy>(all: &[T], name: &str) -> T {
    all.iter()
        .copied()
        .find(|v| format!("{v:?}") == name)
        .unwrap_or_else(|| panic!("inventory.ron names no feature {name}"))
}

#[cfg(test)]
mod tests {
    use super::{Case, Definite, Morph, Number, Person, UdCase};

    #[test]
    fn sociative_has_no_ud_case() {
        // NOM-CASE-001: a recorded gap in UD, not an omission here.
        assert_eq!(Case::Sociativus.ud(), None);
        assert_eq!(Case::DistributivusTemporalis.ud(), None);
    }

    #[test]
    fn kent_and_kepp_share_abs() {
        // NOM-CASE-001: the two are not distinguished in the UD projection.
        assert_eq!(Case::Formalis.ud(), Some(UdCase::Abs));
        assert_eq!(Case::Modalis.ud(), Some(UdCase::Abs));
    }

    #[test]
    fn distributive_is_separate_from_formalis() {
        assert_eq!(Case::Distributivus.ud(), Some(UdCase::Dis));
        assert_ne!(Case::Distributivus.ud(), Case::Formalis.ud());
    }

    #[test]
    fn ordinary_cases_project_directly() {
        assert_eq!(Case::Ine.ud(), Some(UdCase::Ine));
        assert_eq!(Case::Acc.ud(), Some(UdCase::Acc));
    }

    #[test]
    fn possessive_records_the_possessor() {
        let m = Morph::Possessive {
            person: Person::Third,
            number: Number::Plur,
        };
        assert_eq!(
            m,
            Morph::Possessive {
                person: Person::Third,
                number: Number::Plur
            }
        );
    }

    #[test]
    fn lak_lek_is_a_definiteness_value() {
        // UD writes this Definite=2, not Person[obj]=2; getting it wrong breaks validation.
        assert_ne!(Definite::Two, Definite::Def);
    }
}
