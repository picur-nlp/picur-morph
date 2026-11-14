//! Lexical properties that no rule predicts, and a bitset over them.

use serde::{Deserialize, Serialize};

/// A property of a lexeme that cannot be computed from its spelling or pronunciation.
///
/// Admission test: if `harmony_of`, the coda, or the alternation type already determines it, it is
/// not a trait. Defaults are chosen so the unmarked case is the productive one, which is what makes
/// the unknown-word guesser correct by construction (`NOM-LOW-001`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Trait {
    /// Back harmony against the spelling (`híd` -> `hídnak`). Rules: `PH-HARM-011`.
    Back,
    /// Front harmony against the spelling. Rules: `PH-HARM-011`.
    Front,
    /// Front rounded harmony against the spelling. Rules: `PH-HARM-004`.
    FrontRounded,

    /// Lowering stem: lowerable suffixes take low `A` (`ház` -> `házak`). Rules: `NOM-LOW-001`.
    Low,

    /// Accusative `-t` attaches bare, against the computed coda default. Rules: `NOM-ACC-009`.
    BareAccusative,
    /// Accusative takes a linking vowel, against the computed default (`orr` -> `orrot`).
    /// Rules: `NOM-ACC-006`.
    LinkedAccusative,

    /// Third person possessive `-a`/`-e` (`ablaka`). Rules: `NOM-POSS-005`.
    PossessiveA,
    /// Third person possessive `-ja`/`-je` (`kalapja`). Rules: `NOM-POSS-005`.
    PossessiveJa,

    /// Stem vowel drops before a vowel-initial suffix (`bokor` ~ `bokr-`). Rules: `NOM-STEM-004`.
    Epenthetic,
    /// Stem vowel shortens (`kéz` ~ `kez-`). Rules: `NOM-STEM-006`.
    Shortening,
    /// `v` stem (`ló` ~ `lov-`). Rules: `NOM-STEM-008`.
    VStem,
    /// Final `ó`/`ő` alternates with `a`/`e` (`tó` ~ `tav-`). Rules: `NOM-STEM-010`.
    OaStem,
    /// Final `u`/`ü` drops before `v` (`daru` ~ `darv-`). Rules: `NOM-STEM-012`.
    UStem,
    /// Final `ó`/`ő` becomes `a`/`e` plus `j` before the third person possessive only.
    ///
    /// `ajtó` ~ `ajtaja`, `idő` ~ `ideje`. `NOM-STEM-013` confines it to 3sg and 3pl: the stem is
    /// invariant before `-m`, `-d`, `-nk`, `-tOk` and before every other suffix (`időt`, `idők`),
    /// which is what separates it from a v-stem.
    ///
    /// Rules: `NOM-STEM-013`
    PossessiveAlt,
    /// Takes the `ajtó` stem change before the possessed plural, whatever the singular takes.
    ///
    /// The cells come apart: `szülő` has the plain singular `szülője` and the alternating
    /// possessed plural `szülei` 179,772, which `NOM-STEM-013` records as lexicalised. Measured,
    /// like the singular cell it sits beside.
    ///
    /// Rules: `NOM-STEM-013`, `NOM-PPL-002`
    PossessedPluralAlt,

    /// An epenthetic verb that keeps its full stem before the definite `-i` (`képez` ~ `képezi`).
    ///
    /// The definite cell and the indefinite one disagree for a third of the class, which is why
    /// they are measured and marked apart: `énekel` gives `énekli` 27,146 : `énekeli` 595 but
    /// `énekelek` 7,957 : `éneklek` 1,904 `[WC2]`.
    ///
    /// Rules: `VRB-STEM-007`
    DefiniteFullStem,

    /// An epenthetic verb that syncopates before the definite `-i` although the rule computes that
    /// it should not.
    ///
    /// `VRB-STEM-007` calls the syncope obligatory only after `g`, `z` and `m` and item-specific
    /// after `l` and `r`. The corpus settles the items, and it settles them the other way for the
    /// commonest of them: `közli` 120,588 : `közöli` 757, `törli` 89,954 : `töröli` 351 `[WC2]`.
    ///
    /// Rules: `VRB-STEM-007`
    DefiniteSyncope,

    /// An epenthetic verb that keeps its full stem in the past.
    ///
    /// The third measured cell of the same axis. `jegyez` syncopates here and not in the
    /// infinitive: `jegyzett` 13,079 : `jegyezett` 206 `[WC2]`.
    ///
    /// Rules: `VRB-STEM-007`, `VRB-PAST-001`
    PastFullStem,

    /// As [`Self::DefiniteSyncope`], for the past.
    ///
    /// Rules: `VRB-STEM-007`, `VRB-PAST-001`
    PastSyncope,

    /// As [`Self::DefiniteSyncope`], for the vowel-initial indefinite endings.
    ///
    /// Rules: `VRB-STEM-007`
    PresentSyncope,

    /// An epenthetic verb that keeps its full stem in the indefinite present (`énekel` ~
    /// `énekelek`).
    ///
    /// `VRB-STEM-007` calls the syncope obligatory after `g`, `z` and `m`, but the corpus
    /// disagrees for a fifth of them and the split is by cell: the same verb gives `képzés` and
    /// `képezi`. The syncope stays everywhere else, so this marks the present alone.
    ///
    /// Rules: `VRB-STEM-007`
    PresentFullStem,

    /// `sz`/`d` verb stem (`alszik` ~ `aludt`). Rules: `VRB-STEM-012`.
    SzDStem,
    /// `sz`/`d`/`v` verb stem (`fekszik` ~ `feküdt` ~ `fekvő`). Rules: `VRB-STEM-012`.
    SzDVStem,

    /// Past `-t` word-finally. Rules: `VRB-PAST-001`.
    PastFinalT,
    /// Past `-Vtt` word-finally. Rules: `VRB-PAST-001`.
    PastFinalVtt,
    /// Past `-t` before a person suffix. Rules: `VRB-PAST-001`.
    PastNonfinalT,
    /// Past `-Vtt` before a person suffix. Rules: `VRB-PAST-001`.
    PastNonfinalVtt,

    /// Keeps the free stem before the accusative and the superessive, although it alternates
    /// elsewhere.
    ///
    /// `NOM-STEM-010` names the class: v-stems take the bound alternant (`lovat`, `követ`) except
    /// `szó`, `jó` and `szú`, which give `szót`, `jót`, `szút` while still pluralising `szavak`.
    /// Corpus: `szót` 534,651 : `szavat` 1,408 `[WC2]`.
    ///
    /// Rules: `NOM-STEM-010`, `NOM-ACC-001`
    FreeStemAccSup,

    /// Keeps the free stem before the plural, although it alternates elsewhere.
    ///
    /// The plural counterpart of [`Self::FreeStemAccSup`], and measured the same way. `odú`
    /// pluralises `odúk` 2,668 against `odvak`, and takes the accusative `odút` 2,571, while its
    /// possessive really is `odva` `[WC2]`: the cells are independent, so they get independent
    /// flags rather than one verdict for the lexeme.
    ///
    /// Rules: `NOM-STEM-008`, `NOM-STEM-010`, `NOM-STEM-012`
    FreeStemPlural,

    /// Also takes the free stem before the third person possessive, although it alternates
    /// there by default.
    ///
    /// `PH-ZERO-002` names the third singular possessive as the third unstable cell beside the
    /// accusative and the superessive: `sarka` is generated and `sarokja` accepted beside it,
    /// `retke` and `retekje`. Analysis reads the mark; generation keeps the alternant.
    ///
    /// Rules: `PH-ZERO-002`, `NOM-POSS-009`
    FreeStemPossessive,

    /// The alternation is real but optional: the unalternated stem inflects too.
    ///
    /// Marked where the corpus shows both alternants in quantity rather than one winning, which is
    /// 96 of the marked stems. `mag` pluralises `magok` 42,607 and `magvak` 32,449 `[WC2]`, so
    /// generating only `magvak` leaves the commoner form unanalysable while dropping the v-stem
    /// loses the other. Generation keeps the alternant and analysis accepts either, in the same way
    /// the open `-ja/-a` possessive is handled.
    ///
    /// Rules: `NOM-STEM-008`, `NOM-STEM-010`, `NOM-STEM-012`
    AlternationOptional,

    /// Takes both third person possessive allomorphs, `-ja/-je` beside `-a/-e`.
    ///
    /// `NOM-POSS-006` makes the choice lexical, and where the corpus shows the losing allomorph in
    /// quantity both are words: `utóda` 23,241 beside `utódja` 38,421 `[WC2]`. Generation takes
    /// the marked allomorph and analysis accepts the other. A stem doublet is a separate axis
    /// ([`Self::AlternationOptional`]): `faluk` beside `falvak` licenses no `*falva` beside
    /// `faluja` (`NOM-STEM-018`).
    ///
    /// Rules: `NOM-POSS-006`, `NOM-STEM-018`
    PossessiveOptional,

    /// The possessed plural takes `-ai/-ei`, although the singular possessive takes `-ja/-je`.
    ///
    /// `NOM-PPL-001` builds the possessed plural on the same allomorph choice as the singular, but
    /// the cells part company for a large class: `barátja` is right beside `barátai` 116,592 to
    /// `barátjai` 591 `[WC2]`. Measured, like the singular choice it overrides.
    ///
    /// Rules: `NOM-PPL-001`, `NOM-POSS-006`
    PossessedPluralA,

    /// The mirror of [`Self::PossessedPluralA`]: `-jai/-jei` although the singular takes `-a/-e`.
    ///
    /// Rules: `NOM-PPL-001`, `NOM-POSS-006`
    PossessedPluralJa,

    /// Lowers, but not before the plural.
    ///
    /// `NOM-LOW-001` marks lowering per lexeme, and the accusative and the plural come apart for a
    /// measurable class: `olaj` takes `olajat` 156,574 against `olajot` 12, and `olajok` 43,083
    /// against `olajak` 3,789 `[WC2]`. The lexeme is lowering, and the plural is the exception.
    ///
    /// Rules: `NOM-LOW-001`, `NOM-PL-001`
    LowPluralExempt,

    /// Lowers, but not before a possessive suffix.
    ///
    /// The third measured cell of the same axis, and it parts company with the others: `pár`
    /// takes the low `párat` and the plain `párom` 91,800 `[WC2]`.
    ///
    /// Rules: `NOM-LOW-001`, `NOM-POSS-001`
    LowPossessiveExempt,

    /// Lowers, but not before the accusative.
    ///
    /// The mirror of [`Self::LowPluralExempt`], and measured the same way: 149 of the 2,859
    /// attested lowering nouns take the plain accusative while still lowering in the plural.
    ///
    /// Rules: `NOM-LOW-001`, `NOM-ACC-005`
    LowAccusativeExempt,

    /// Takes a low linking vowel before the plural although the stem ends in a vowel.
    ///
    /// `NOM-PL-003` gives the rule for adjectives and names `férfi` as the noun that follows it:
    /// `férfiak`, 633,484 against `férfik` 285 `[WC2]`.
    ///
    /// Rules: `NOM-PL-003`
    LinkedPlural,

    /// A geminate-final verb stem that counts as heavy, taking the linking vowel.
    ///
    /// `VRB-LINK-002` says the geminates go both ways and nothing in the shape decides: `hall` is
    /// heavy (`hallani` 270,582 against `hallni` 165 `[WC2]`) while `áll` is not. Only the heavy
    /// ones carry this, because light is what the shape computes.
    ///
    /// Rules: `VRB-LINK-002`
    HeavyStem,

    /// An abbreviation or symbol, which takes a suffix only after a hyphen.
    ///
    /// Rules: `ORT-ABBR-009`, `ORT-ABBR-019`.
    Abbreviation,

    /// Final `z` assimilates totally to a consonant-initial suffix: `ez` + `-bAn` is `ebben`.
    ///
    /// Only the demonstratives in `-ez`/`-az` have it, and it is a property of the stem rather than
    /// of any suffix, which is why it lives here. Rules: `PND-DEM-002`.
    DemonstrativeZ,

    /// Lexeme has no plural (singulare tantum).
    ///
    /// Carried from morphdb's `sing` flag. The rulebook covers pluralia tantum in `SYN-AGR-010`
    /// and states nothing about singulare tantum, so no rule consumes this yet.
    SingularOnly,
    /// Lexeme takes no possessive suffix.
    ///
    /// Carried from morphdb's `non_poss` flag, with no rulebook backing; no rule consumes it yet,
    /// as for [`Trait::SingularOnly`].
    NoPossessive,
    /// Takes a direct object. Rules: `SYN-GOV-001`.
    Transitive,
    /// The first compound part is a verbal prefix (`meg` + `old`). Rules: `ORT-TOG-026`.
    Preverb,
    /// The lexeme is an article rather than another kind of determiner.
    ///
    /// Only `a` and `az` carry it. The distinction is not derivable from the category, because
    /// `eme`, `ama`, `ahány`, `amennyi` and `némi` are determiners and not articles, and it is not
    /// a class that grows: a language does not acquire a new definite article.
    Article,

    /// A foreign-spelled word whose suffix follows its pronunciation, written after a hyphen.
    ///
    /// `ORT-FOR-012` makes the decision lexical: a silent final letter or an unusual final letter
    /// group is not in the spelling, and `Goethe` says its `e` where `Cambridge` does not. The
    /// pronunciation, in Hungarian letters, goes in `alt` (`kembridzs`, `sekszpír`, `glaszgó`),
    /// and every allomorph then follows from the ordinary rules on that stem: `Cambridge-ben`,
    /// `Shakespeare-rel`, `Glasgow-val`, `cambridge-i`.
    ///
    /// A suffix consumes only three facts of that pronunciation, and attested forms show them, so
    /// the entry may carry the facts instead of a respelling: the harmony class as `Back`, `Front`
    /// or `FrontRounded`, and the last sound in `last_sound` as one consonant letter (`g`, `dzs`)
    /// or absent for a vowel. `League-ben`, `League-et` and `League-gel` follow from `Front` and
    /// `g`; `Chelsea-t` and `Chelsea-vel` from `Front` alone.
    ///
    /// Rules: `ORT-FOR-012`
    Spoken,

    /// A place name that takes the locative `-t`/`-Ott` (`NOM-CASE-016`).
    ///
    /// The case survives on a closed set of about ten Hungarian towns and nowhere else, which the
    /// rule states as a lexical flag: `Pécsett`, `Győrött`, `Kaposvárott`.
    Locative,
    /// A preverb that stands in front of a verb already carrying one (`ORT-TOG-026`).
    ///
    /// A verb takes one preverb; the aspectual `újra` and `tovább` are the exceptions the
    /// rulebook's list does not single out: `újraeloszt`, `továbbfeldolgoz`.
    Stacking,
    /// A mixed stem whose suffixes the norm allows in both harmonies (`PH-HARM-011`).
    ///
    /// Generation takes the marked or computed class; analysis accepts the other too:
    /// `szlovének` beside `szlovénok`, `norvégok` beside `norvégek`, `Athénban` beside `Athénben`.
    HarmonyOptional,
    /// A verb stem whose linking vowel before a consonant-initial ending is optional
    /// (`VRB-LINK-010`, `VRB-STEM-006`).
    ///
    /// Generation takes the measured or computed heaviness; analysis accepts the other too:
    /// `függenek` beside `függnek`, `állsz` beside `állasz`.
    LinkOptional,
    /// A family name kept in its pre-1832 spelling, read with the archaic letter values
    /// (`PH-G2P-004`).
    ///
    /// The letter groups are possible in modern words too, so the rule makes the reading lexical:
    /// `Madách` is \[madács\] and `Zürich` is not, `Végh` is \[vég\] and `Edinburgh` is not. A flagged
    /// name takes the ambiguous groups of the table and is suffixed by its archaic value rather
    /// than refused as an unusual final group: `Babitscsal`, `Móriczcal`, `Véghgel`.
    ///
    /// Rules: `PH-G2P-004`, `ORT-FOR-012`
    ArchaicSpelling,
    /// A quantifier adjective, which alone among adjectives takes the collective and the
    /// multiplicative (`PND-INDF-004`): `sokan`, `kevésszer`, `többször`.
    Quantity,
    /// An occupational or functional person noun, the domain in which `-sÁg` is productive on a
    /// noun (`menedzserség`, `brókerség`, `vezérség`).
    ///
    /// `DER-NOUN-003` names the feature as the rule's lexical input and the rule states the
    /// default: an unmarked noun takes no `-sÁg` (`*székség`, `*könyvség`).
    ///
    /// Rules: `DER-NOUN-003`, `DER-MODEL-012`
    Occupation,
    /// A stem-final `h` the norm may write as silent, so `-vAl`/`-vÁ` does not assimilate to it
    /// (`csehvel`, `pléhvé`, `méhvé`). Rules: `NOM-STEM-004`.
    SilentH,
    /// An attributive-only form, which takes no suffix at all: `kis ház` but `*kisek`, with the
    /// predicative `kicsi` inflecting instead. Rules: `NOM-ADJ-002`.
    AttributiveOnly,
}

impl Trait {
    /// Every variant, in declaration order. The bit index of a trait is its position here.
    pub const ALL: [Self; 56] = [
        Self::Back,
        Self::Front,
        Self::FrontRounded,
        Self::Low,
        Self::BareAccusative,
        Self::LinkedAccusative,
        Self::PossessiveA,
        Self::PossessiveJa,
        Self::Epenthetic,
        Self::Shortening,
        Self::VStem,
        Self::OaStem,
        Self::UStem,
        Self::PossessiveAlt,
        Self::PossessedPluralAlt,
        Self::DefiniteFullStem,
        Self::DefiniteSyncope,
        Self::PastFullStem,
        Self::PastSyncope,
        Self::PresentSyncope,
        Self::PresentFullStem,
        Self::SzDStem,
        Self::SzDVStem,
        Self::PastFinalT,
        Self::PastFinalVtt,
        Self::PastNonfinalT,
        Self::PastNonfinalVtt,
        Self::FreeStemAccSup,
        Self::FreeStemPlural,
        Self::FreeStemPossessive,
        Self::AlternationOptional,
        Self::PossessiveOptional,
        Self::PossessedPluralA,
        Self::PossessedPluralJa,
        Self::LowPluralExempt,
        Self::LowPossessiveExempt,
        Self::LowAccusativeExempt,
        Self::LinkedPlural,
        Self::HeavyStem,
        Self::Abbreviation,
        Self::DemonstrativeZ,
        Self::SingularOnly,
        Self::NoPossessive,
        Self::Transitive,
        Self::Preverb,
        Self::Article,
        Self::Spoken,
        Self::Locative,
        Self::Stacking,
        Self::HarmonyOptional,
        Self::LinkOptional,
        Self::ArchaicSpelling,
        Self::Quantity,
        Self::Occupation,
        Self::SilentH,
        Self::AttributiveOnly,
    ];

    const fn bit(self) -> u64 {
        1 << (self as u64)
    }
}

/// A set of [`Trait`]s, stored as a bitset so membership is a single mask test.
///
/// Membership is asked in the inner loop of generation, which a linear scan over a `Vec<Trait>`
/// would make the hot spot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TraitSet(u64);

impl TraitSet {
    /// The empty set.
    pub const EMPTY: Self = Self(0);

    /// Builds a set from an iterator of traits.
    pub fn from_iter_of(traits: impl IntoIterator<Item = Trait>) -> Self {
        traits.into_iter().fold(Self::EMPTY, Self::with)
    }

    /// Returns this set with `t` added.
    #[must_use]
    pub const fn with(self, t: Trait) -> Self {
        Self(self.0 | t.bit())
    }

    /// Returns this set with `t` removed.
    #[must_use]
    pub const fn without(self, t: Trait) -> Self {
        Self(self.0 & !t.bit())
    }

    /// Whether `t` is present.
    #[must_use]
    pub const fn has(self, t: Trait) -> bool {
        self.0 & t.bit() != 0
    }

    /// Whether the set is empty.
    ///
    /// Takes `&self` so serde's `skip_serializing_if` can call it.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Whether any of `traits` is present.
    pub fn has_any(self, traits: impl IntoIterator<Item = Trait>) -> bool {
        traits.into_iter().any(|t| self.has(t))
    }

    /// Iterates the traits present, in [`Trait::ALL`] order.
    pub fn iter(self) -> impl Iterator<Item = Trait> {
        Trait::ALL.into_iter().filter(move |&t| self.has(t))
    }
}

impl FromIterator<Trait> for TraitSet {
    fn from_iter<I: IntoIterator<Item = Trait>>(iter: I) -> Self {
        Self::from_iter_of(iter)
    }
}

impl Serialize for TraitSet {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        self.iter().collect::<Vec<_>>().serialize(s)
    }
}

impl<'de> Deserialize<'de> for TraitSet {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::from_iter_of(Vec::<Trait>::deserialize(d)?))
    }
}

#[cfg(test)]
mod tests {
    use super::{Trait, TraitSet};

    #[test]
    fn all_variants_fit_the_bitset() {
        assert!(Trait::ALL.len() <= 64, "TraitSet is a u64");
        // Declaration order must match ALL, or bit indices drift from the enum discriminants.
        for (i, t) in Trait::ALL.into_iter().enumerate() {
            assert_eq!(t as usize, i, "Trait::ALL out of order at {i}");
        }
    }

    #[test]
    fn membership_round_trips() {
        let s = TraitSet::from_iter_of([Trait::Low, Trait::Epenthetic]);
        assert!(s.has(Trait::Low));
        assert!(s.has(Trait::Epenthetic));
        assert!(!s.has(Trait::VStem));
        assert_eq!(
            s.iter().collect::<Vec<_>>(),
            vec![Trait::Low, Trait::Epenthetic]
        );
    }

    #[test]
    fn empty_set_has_nothing() {
        assert!(TraitSet::EMPTY.is_empty());
        assert!(Trait::ALL.into_iter().all(|t| !TraitSet::EMPTY.has(t)));
    }
}
