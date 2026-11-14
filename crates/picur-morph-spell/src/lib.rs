//! Hungarian spellchecking.
//!
//! A thin layer over `picur-morph-core`: the morphology already decides whether a word is well
//! formed, so this crate supplies the search space, the token handling and the suggestions.
//!
//! What makes it different from an affix-file checker is the three-way verdict. Hungarian suffixes
//! productively, and it compounds productively, so no word list is ever complete; a checker that
//! can only say "in the dictionary or not" must therefore either reject correct words or be padded
//! until it accepts wrong ones. Here an unknown-but-well-formed word is its own answer
//! ([`Status::Unknown`]), distinct from a real error ([`Status::Wrong`]), so a false rejection is
//! never silenced by adding a word.
//!
//! See `docs/design/03-analysis.md`.

use std::hash::{DefaultHasher, Hash, Hasher};

use picur_morph_core::{
    Case, Definite, DerId, Mood, Morph, Number, Person, Role, Stacks, Tense, Verdict,
    accent_confusable, analyse, analyse_with, is_known, licenses_nominal, segment_surface,
};
use picur_morph_lexicon::{Trait, frequency, lexicon};
use picur_morph_ortho::case::{is_latin_letter, lower, lower_initial, starts_upper};
use picur_morph_ortho::{Letter, letters, tokens};

/// What the checker concluded about one token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Status {
    /// Analysed: the word is correct.
    Ok,
    /// Well formed, but its stem is not in the lexicon.
    ///
    /// Neologisms, unlisted proper names and technical vocabulary land here. A checker should not
    /// underline these in red; at most it offers to learn them.
    Unknown,
    /// No legal analysis: this is an error.
    Wrong,
}

/// Every case a nominal can take, for the default search space.
const CASES: [Case; 22] = [
    Case::Acc,
    Case::Dat,
    Case::Ins,
    Case::Cau,
    Case::Tra,
    Case::Ter,
    Case::Ess,
    Case::Ine,
    Case::Ill,
    Case::Ela,
    Case::Sup,
    Case::Sbl,
    Case::Del,
    Case::Ade,
    Case::All,
    Case::Abl,
    Case::Tem,
    // NOM-CASE-016: the lexicon marks the towns that take it, so it costs nothing elsewhere.
    Case::Loc,
    Case::Formalis,
    // The case-like suffixes of `NOM-CASE-001`'s third class: `mindenképp`, `hatféleképpen`,
    // `naponta`, `házanként`. `-képpen` is the modalis plus the superessive on the inflected
    // stem, which `NOM-STEM-017` licenses.
    Case::Modalis,
    Case::Distributivus,
    Case::Sociativus,
];

/// The bases the anaphoric `-é` and the associative `-ék` attach to, and what follows them.
///
/// `NOM-ANP-001` takes an unsuffixed, plural or possessed nominal; `NOM-ANP-002` adds the
/// possessed plural `-i` and a case after either. `NOM-ASSOC-001` takes a bare or possessed
/// singular and never a plural `-k`; `NOM-ASSOC-002` lets its output take `-é` and every case.
fn anaphoric_stacks(out: &mut Vec<Vec<Morph>>) {
    let mut bases: Vec<Vec<Morph>> = vec![Vec::new()];
    for person in [Person::First, Person::Second, Person::Third] {
        for number in [Number::Sing, Number::Plur] {
            let poss = Morph::Possessive { person, number };
            bases.push(vec![poss]);
            bases.push(vec![Morph::PossessedPlural, poss]);
        }
    }
    // `NOM-ASSOC-001`, `DER-MODEL-001`: the associative follows a bare, a possessed and a
    // possessed-plural stem (`Szabóék`, `anyámék`, `szomszédaimék`).
    for base in bases.clone() {
        let assoc = [base.as_slice(), &[Morph::Associative]].concat();
        bases.push(assoc.clone());
        out.push(assoc.clone());
        for c in CASES {
            out.push([assoc.as_slice(), &[Morph::Case(c)]].concat());
        }
    }
    bases.push(vec![Morph::Plural]);
    for base in &bases {
        for tail in [
            vec![Morph::Anaphoric],
            vec![Morph::Anaphoric, Morph::Plural],
        ] {
            let stack = [base.as_slice(), tail.as_slice()].concat();
            out.push(stack.clone());
            for c in CASES {
                out.push([stack.as_slice(), &[Morph::Case(c)]].concat());
            }
        }
    }
}

/// The feature stacks the checker searches.
///
/// This is the checker's whole notion of "a possible word", so it is built once and shared, and
/// anything missing from it is a false rejection: `írjuk`, `tenni` and `teszünk` are ordinary
/// words, and a checker that only knows nouns calls all of them errors.
#[expect(
    clippy::too_many_lines,
    reason = "the inventory of stacks a checker searches, in one place"
)]
fn search_space() -> Vec<Vec<Morph>> {
    let mut out: Vec<Vec<Morph>> = vec![
        vec![],
        vec![Morph::Plural],
        vec![Morph::Comparative],
        vec![Morph::Superlative],
        vec![Morph::Excessive],
        vec![Morph::Collective],
        vec![Morph::Multiplicative],
        vec![Morph::Comparative, Morph::Plural],
        vec![Morph::Superlative, Morph::Plural],
    ];
    anaphoric_stacks(&mut out);
    for c in CASES {
        out.push(vec![Morph::Case(c)]);
        out.push(vec![Morph::Plural, Morph::Case(c)]);
        out.push(vec![Morph::Comparative, Morph::Case(c)]);
        out.push(vec![Morph::Superlative, Morph::Case(c)]);
    }
    for id in picur_morph_core::DerId::ALL {
        out.push(vec![Morph::Derive(id)]);
        out.push(vec![Morph::Derive(id), Morph::Plural]);
        for c in CASES {
            out.push(vec![Morph::Derive(id), Morph::Case(c)]);
            out.push(vec![Morph::Derive(id), Morph::Plural, Morph::Case(c)]);
        }
    }
    // A derived noun takes a possessive and the relational `-i` like any other, and the deverbal `-Ás` does both
    // constantly. Only noun-forming suffixes stack here; the search space is paid on every unfound token.
    // `PND-NUMINF-008`: the distributive is `-Os` with the possessive and the instrumental
    // (`hármasával`), and an `-Os` noun is possessed like any other (`kettese`).
    for id in [DerId::As, DerId::Sag, DerId::O, DerId::Os] {
        for person in [Person::First, Person::Second, Person::Third] {
            for number in [Number::Sing, Number::Plur] {
                let poss = Morph::Possessive { person, number };
                out.push(vec![Morph::Derive(id), poss]);
                // `létrehozásáról` was the commonest unread token of the corpus, a delative.
                for c in CASES {
                    out.push(vec![Morph::Derive(id), poss, Morph::Case(c)]);
                }
            }
        }
        out.push(vec![Morph::Derive(id), Morph::Derive(DerId::I)]);
        out.push(vec![
            Morph::Derive(id),
            Morph::Derive(DerId::I),
            Morph::Plural,
        ]);
    }
    // An adverb built on a derived adjective, which is two derivations deep: `kezdődően` is
    // `kezdődik` plus the participle `-Ó` plus the manner `-An`. Only the adverb-forming suffixes
    // stack here, because that is what running text uses and the pair space is otherwise large.
    for first in [
        DerId::O,
        DerId::Hato,
        DerId::Talan,
        DerId::U,
        DerId::I,
        DerId::Os,
    ] {
        for second in [DerId::An, DerId::Lag] {
            out.push(vec![Morph::Derive(first), Morph::Derive(second)]);
        }
    }
    // `DER-ADJ-006`: `-(O)s` on an `-i` adjective (`párizsias`, `családiasan`).
    let ias = [Morph::Derive(DerId::I), Morph::Derive(DerId::Os)];
    out.push(ias.to_vec());
    out.push([ias.as_slice(), &[Morph::Plural]].concat());
    out.push([ias.as_slice(), &[Morph::Derive(DerId::An)]].concat());
    for c in CASES {
        out.push([ias.as_slice(), &[Morph::Case(c)]].concat());
    }
    // `DER-VERB-012`, `DER-VERB-020`: `-(O)skOdik` and the passive in their citation form.
    for chain in KODIK_CHAINS {
        out.push(chain.to_vec());
    }
    out.push(vec![Morph::Derive(DerId::Passive)]);
    // `DER-ADJ-017`: `-Ós` is the `-(O)s` adjective of the present
    // participle, so it is `-Ó` plus `-(O)s` and inflects as an adjective (`nézegetős`,
    // `üldögélősek`).
    let habitual = [Morph::Derive(DerId::O), Morph::Derive(DerId::OsHabitual)];
    out.push(habitual.to_vec());
    out.push([habitual.as_slice(), &[Morph::Plural]].concat());
    for c in CASES {
        out.push([habitual.as_slice(), &[Morph::Case(c)]].concat());
        out.push([habitual.as_slice(), &[Morph::Plural, Morph::Case(c)]].concat());
    }
    // A graded manner adverb is the adverb of the comparative, not a lemma of its own:
    // `gyorsabban`, `szebben`, `korábban`, `legjobban`. `NOM-ADJ-010` on a `NOM-ADJ-004` stem.
    // `PND-NUMINF-005`: `-szOrOs` and its inflection (`ötszörös`, `négyszeresére`).
    out.push(vec![Morph::Multiplicative, Morph::Derive(DerId::Os)]);
    out.push(vec![
        Morph::Multiplicative,
        Morph::Derive(DerId::Os),
        Morph::Plural,
    ]);
    let third = Morph::Possessive {
        person: Person::Third,
        number: Number::Sing,
    };
    out.push(vec![Morph::Multiplicative, Morph::Derive(DerId::Os), third]);
    // `DER-ADV-010`, `DER-ADJ-025`: the multiplicative takes `-i` and, through `-(O)s`, the manner
    // adverb (`hatszori`, `nyolcszori`, `ötszörösen`).
    out.push(vec![Morph::Multiplicative, Morph::Derive(DerId::I)]);
    out.push(vec![
        Morph::Multiplicative,
        Morph::Derive(DerId::Os),
        Morph::Derive(DerId::An),
    ]);
    for c in CASES {
        out.push(vec![
            Morph::Multiplicative,
            Morph::Derive(DerId::I),
            Morph::Case(c),
        ]);
        out.push(vec![
            Morph::Multiplicative,
            Morph::Derive(DerId::Os),
            Morph::Case(c),
        ]);
        out.push(vec![
            Morph::Multiplicative,
            Morph::Derive(DerId::Os),
            third,
            Morph::Case(c),
        ]);
        // `NOM-STEM-017`: a graded stem pluralises and inflects (`újabbakkal`, `legszebbeket`).
        out.push(vec![Morph::Comparative, Morph::Plural, Morph::Case(c)]);
        out.push(vec![Morph::Superlative, Morph::Plural, Morph::Case(c)]);
        // `DER-ADJ-019` graded: `érthetőbb`, `élvezhetőbbek`.
        out.push(vec![
            Morph::Potential,
            Morph::PresentParticiple,
            Morph::Comparative,
            Morph::Case(c),
        ]);
    }
    out.push(vec![
        Morph::Potential,
        Morph::PresentParticiple,
        Morph::Comparative,
    ]);
    out.push(vec![
        Morph::Potential,
        Morph::PresentParticiple,
        Morph::Comparative,
        Morph::Plural,
    ]);
    // `NOM-CASE-020`: `-képpen` is the modalis plus the superessive (`semmiképpen`).
    out.push(vec![Morph::Case(Case::Modalis), Morph::Case(Case::Sup)]);
    // `DER-ADJ-001`: `-i` after `-nként` and `-kor` (`fejenkénti`, `hajnalkori`).
    out.push(vec![
        Morph::Case(Case::Distributivus),
        Morph::Derive(DerId::I),
    ]);
    out.push(vec![Morph::Case(Case::Tem), Morph::Derive(DerId::I)]);
    for grade in [Morph::Comparative, Morph::Superlative, Morph::Excessive] {
        out.push(vec![grade, Morph::Derive(DerId::An)]);
        // The other suffixes a graded stem takes: `többször`, `legtöbbször`, `régebbi`,
        // `későbbi`. `NOM-STEM-017` makes the comparative a stem like any other.
        out.push(vec![grade, Morph::Multiplicative]);
        out.push(vec![grade, Morph::Derive(DerId::I)]);
        out.push(vec![grade, Morph::Collective]);
    }
    // The same adverb on a participle rather than on a derived noun: `kezdődően` is the manner
    // adverb of the present participle `kezdődő`, and `-Ó` as a noun (`DER-NOUN-008`) cannot take
    // it because the category is wrong.
    for part in [Morph::PresentParticiple, Morph::PastParticiple] {
        out.push(vec![part, Morph::Derive(DerId::An)]);
    }
    // `DER-ADV-004`: `-lAg` on a present participle (`hallgatólag`, `remélhetőleg`).
    out.push(vec![Morph::PresentParticiple, Morph::Derive(DerId::Lag)]);
    out.push(vec![
        Morph::Potential,
        Morph::PresentParticiple,
        Morph::Derive(DerId::Lag),
    ]);
    // `DER-ADJ-022`: a participle is an adjective and inflects as one: `értendők`, `tervezettnél`,
    // `fenntarthatóság` (the potential participle's `-sÁg`) and its inflection.
    for part in [
        Morph::PresentParticiple,
        Morph::PastParticiple,
        Morph::FutureParticiple,
    ] {
        out.push(vec![part, Morph::Plural]);
        for c in CASES {
            out.push(vec![part, Morph::Case(c)]);
            out.push(vec![part, Morph::Plural, Morph::Case(c)]);
        }
    }
    let potential_noun = vec![
        Morph::Potential,
        Morph::PresentParticiple,
        Morph::Derive(DerId::Sag),
    ];
    out.push(potential_noun.clone());
    out.push([potential_noun.as_slice(), &[Morph::Derive(DerId::I)]].concat());
    for c in CASES {
        out.push([potential_noun.as_slice(), &[Morph::Case(c)]].concat());
        out.push(
            [
                potential_noun.as_slice(),
                &[
                    Morph::Possessive {
                        person: Person::Third,
                        number: Number::Sing,
                    },
                    Morph::Case(c),
                ],
            ]
            .concat(),
        );
    }
    out.push(
        [
            potential_noun.as_slice(),
            &[Morph::Possessive {
                person: Person::Third,
                number: Number::Sing,
            }],
        ]
        .concat(),
    );
    // `NOM-ADJ-009`: `-ik` on a graded adjective, and its cases: `nagyobbik`, `kisebbiket`.
    for grade in [Morph::Comparative, Morph::Superlative] {
        out.push(vec![grade, Morph::Derive(DerId::Ik)]);
        for c in CASES {
            out.push(vec![grade, Morph::Derive(DerId::Ik), Morph::Case(c)]);
        }
    }
    // `VRB-PART-004`: the privative on the potential stem, `láthatatlan`, and its inflection.
    out.push(vec![Morph::Potential, Morph::Derive(DerId::Talan)]);
    out.push(vec![
        Morph::Potential,
        Morph::Derive(DerId::Talan),
        Morph::Plural,
    ]);
    for c in CASES {
        out.push(vec![
            Morph::Potential,
            Morph::Derive(DerId::Talan),
            Morph::Case(c),
        ]);
    }
    for person in [Person::First, Person::Second, Person::Third] {
        for number in [Number::Sing, Number::Plur] {
            let poss = Morph::Possessive { person, number };
            out.push(vec![poss]);
            out.push(vec![Morph::PossessedPlural, poss]);
            // NOM-STEM-017: a graded stem is a stem, so `legjobbja` and `kisebbik` alike.
            for grade in [Morph::Comparative, Morph::Superlative] {
                out.push(vec![grade, poss]);
                for c in CASES {
                    out.push(vec![grade, poss, Morph::Case(c)]);
                }
            }
            // VRB-INF-002: the personal infinitive, `tudnom` and `lennie`.
            out.push(vec![Morph::Infinitive, poss]);
            for c in CASES {
                out.push(vec![poss, Morph::Case(c)]);
                // NOM-PPL-001: the possessed plural inflects for case like any other stem, so
                // `adatai` needs `adatait` and `adataiban` beside it.
                out.push(vec![Morph::PossessedPlural, poss, Morph::Case(c)]);
            }
        }
    }
    for person in [Person::First, Person::Second, Person::Third] {
        for number in [Number::Sing, Number::Plur] {
            for mood in [Mood::Ind, Mood::Cnd, Mood::Imp] {
                for tense in [Tense::Pres, Tense::Past] {
                    if tense == Tense::Past && mood != Mood::Ind {
                        continue;
                    }
                    for definite in [Definite::Ind, Definite::Def, Definite::Two] {
                        let v = Morph::Verbal {
                            mood,
                            tense,
                            person,
                            number,
                            definite,
                        };
                        out.push(vec![v]);
                        out.push(vec![Morph::Potential, v]);
                        for id in DERIVED_VERBS {
                            out.push(vec![Morph::Derive(id), v]);
                        }
                        if definite == Definite::Ind {
                            for chain in KODIK_CHAINS {
                                out.push([chain, &[v]].concat());
                            }
                        }
                        // `DER-VERB-020`: the passive has the third person only (`adatott`,
                        // `adassék`).
                        if person == Person::Third && definite == Definite::Ind {
                            out.push(vec![Morph::Derive(DerId::Passive), v]);
                        }
                    }
                }
            }
        }
    }
    for m in [
        Morph::Infinitive,
        Morph::PresentParticiple,
        Morph::PastParticiple,
        Morph::Converb,
        Morph::FutureParticiple,
    ] {
        out.push(vec![m]);
        out.push(vec![Morph::Potential, m]);
        for id in DERIVED_VERBS {
            out.push(vec![Morph::Derive(id), m]);
        }
        for chain in KODIK_CHAINS {
            out.push([chain, &[m]].concat());
        }
    }
    out
}

/// `DER-VERB-012`: `-kOdik`, on a listed adjective, a noun's `-(O)skOdik` (`katonáskodik`) and an
/// `-i` adjective's (`petőfieskedik`). The verb is intransitive, so it conjugates indefinite only.
const KODIK_CHAINS: [&[Morph]; 3] = [
    &[Morph::Derive(DerId::Kodik)],
    &[Morph::Derive(DerId::Os), Morph::Derive(DerId::Kodik)],
    &[
        Morph::Derive(DerId::I),
        Morph::Derive(DerId::Os),
        Morph::Derive(DerId::Kodik),
    ],
];

/// The verb-forming derivations whose output conjugates in the search space.
const DERIVED_VERBS: [DerId; 7] = [
    DerId::Ogat,
    DerId::Causative,
    DerId::It,
    DerId::Oz,
    DerId::OdikV,
    // `DER-VERB-018`: the onomatopoeic verbs conjugate like any other (`kopogott`, `zuttyant`).
    DerId::Og,
    DerId::Momentaneous,
];

/// The feature stacks the checker searches, which is its whole notion of a possible word.
///
/// Exposed so that a caller can enumerate what the checker would accept for a lemma, which is
/// the paradigm a dataset generator wants.
#[must_use]
pub fn stacks() -> &'static Stacks {
    space()
}

/// The shared search space, built on first use.
fn space() -> &'static Stacks {
    use std::sync::OnceLock;
    static SPACE: OnceLock<Stacks> = OnceLock::new();
    SPACE.get_or_init(|| Stacks::from(search_space()))
}

/// Slots in the verdict cache, one token each.
///
/// Running text is Zipf-distributed, so a table this size holds the forms that actually recur.
const SLOTS: usize = 1 << 20;

/// One cache slot: the token it holds and the verdict for it.
type Slot = Option<(Box<str>, Status)>;

/// Memoised verdicts.
///
/// Running text repeats its words heavily, and a verdict is a pure function of the token, so the
/// second occurrence of a word is free. This is a cache, not a correctness mechanism: dropping an
/// entry changes nothing but speed.
///
/// Direct-mapped and fixed in size, where an unbounded map grows for as long as the corpus does:
/// a 9-billion-word corpus has tens of millions of distinct forms, and keeping every one of them
/// costs gigabytes to answer for words that never recur. A slot holds whichever token last hashed
/// to it, so no eviction policy is needed.
///
/// Shared across threads rather than held per thread. That is measured, not assumed: a per-thread
/// table was slower at 1, 4 and 16 threads alike, because threads checking the same corpus miss on
/// the same words and the lock is not what the time goes on.
fn cache() -> &'static std::sync::RwLock<Vec<Slot>> {
    use std::sync::{OnceLock, RwLock};
    static CACHE: OnceLock<RwLock<Vec<Slot>>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(vec![None; SLOTS]))
}

/// The slot a token memoises to.
fn slot_of(word: &str) -> usize {
    let mut h = DefaultHasher::new();
    word.hash(&mut h);
    #[expect(
        clippy::cast_possible_truncation,
        reason = "an index into a fixed-size table"
    )]
    let bits = h.finish() as usize;
    bits % SLOTS
}

/// Every reading of one token, against the checker's own search space.
///
/// The search space is what makes a reading reachable at all, so an evaluation against annotated
/// text has to use the same one the checker does rather than build its own.
#[must_use]
pub fn analyses(word: &str) -> Vec<picur_morph_core::Analysis> {
    // The same paths `check` takes, or the two disagree about what a word is: `1991-ben` and
    // `HVG-nek` are accepted and must have a reading, or an evaluation against annotated text
    // counts them as unanalysed.
    // A tokeniser that splits the clitic off leaves it as a token of its own, which is how UD
    // writes it: `-e`, tagged an interrogative adverb with itself as the lemma. It is a word in
    // that tokenisation even though it is a clitic in the orthography.
    if matches!(word, "-e" | "-E") {
        return vec![picur_morph_core::Analysis {
            lemma: "-e".into(),
            pos: picur_morph_lexicon::Pos::Adverb,
            stack: Vec::new(),
        }];
    }
    // SYN-Q-002 attaches the interrogative clitic to a word that stands on its own, so the host's
    // readings are the answer: `van-e` is the verb `van`, not a noun.
    if interrogative_clitic(word)
        && let Some((host, _)) = word.rsplit_once('-')
    {
        return analyses(host);
    }
    // A full reading first: a name suffixed by its pronunciation generates with its hyphen, so
    // `Cambridge-ben` carries its case. The hyphen path is the fallback for a token the rules
    // accept without generating it, `1991-ben` or `HVG-nek`, whose reading is the base alone.
    if let picur_morph_core::Verdict::Known(readings) = picur_morph_core::analyse(word, space()) {
        return readings;
    }
    hyphenated_reading(word).into_iter().collect()
}

/// Readings of a word the lexicon cannot vouch for, from its suffix alone.
///
/// Empty for a word that has proper readings. The verdict of [`check`] is unchanged by this: a
/// guessed reading says how the suffix parses, not that the stem is a word.
#[must_use]
pub fn guesses(word: &str) -> Vec<picur_morph_core::Analysis> {
    // A hyphenated name's fallback reading is its bare base, which says nothing about the suffix;
    // a guess that does is the better answer.
    let bare = |a: &picur_morph_core::Analysis| a.stack.is_empty();
    let proper = analyses(word);
    let only_bare_hyphen = word.contains('-') && proper.iter().all(bare);
    if !proper.is_empty() && !only_bare_hyphen {
        return Vec::new();
    }
    picur_morph_core::guess(word, space())
}

/// The reading of a suffixed acronym, abbreviation or number, whose lemma is the host.
///
/// `ORT-ABBR-009` attaches the suffix after a hyphen and leaves the host unchanged, so the host is
/// the lemma and the suffix is what the stack would carry. The stack is left empty rather than
/// guessed: the written suffix is not parsed here, and claiming a feature we did not identify
/// would be worse than claiming none.
///
/// Rules: `ORT-ABBR-009`, `PND-NUMINF-010`
fn hyphenated_reading(word: &str) -> Option<picur_morph_core::Analysis> {
    if !hyphenated_abbreviation(word) {
        return None;
    }
    let (base, _) = word.rsplit_once('-')?;
    Some(picur_morph_core::Analysis {
        lemma: base.into(),
        pos: picur_morph_lexicon::Pos::Noun,
        stack: Vec::new(),
    })
}

/// Checks one token.
///
/// The token is expected to be a word: strip punctuation first, or use [`check_text`].
#[must_use]
pub fn check(word: &str) -> Status {
    if word.is_empty() {
        return Status::Ok;
    }
    let slot = slot_of(word);
    if let Ok(c) = cache().read()
        && let Some(Some((key, status))) = c.get(slot)
        && &**key == word
    {
        return *status;
    }
    let status = check_uncached(word);
    if let Ok(mut c) = cache().write() {
        c[slot] = Some((word.into(), status));
    }
    status
}

/// Whether the checker accepts `word` as a listed word or a form of one.
///
/// Acceptance alone is wider: a hyphenated capitalised base is accepted as a possible foreign
/// name and a coined compound by the compounding rules. A reading whose lemma the lexicon lists
/// is what a repair may land on. Memoised like [`check`], because a text repair asks it of every
/// token.
#[must_use]
pub fn vouches(word: &str) -> bool {
    use std::sync::{OnceLock, RwLock};
    type Vouched = Option<(Box<str>, bool)>;
    static CACHE: OnceLock<RwLock<Vec<Vouched>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| RwLock::new(vec![None; SLOTS]));
    let slot = slot_of(word);
    if let Ok(c) = cache.read()
        && let Some(Some((key, vouched))) = c.get(slot)
        && &**key == word
    {
        return *vouched;
    }
    // The same spellings `check` tries, judged without the coined-compound path.
    let mut candidates = vec![word.to_owned()];
    if starts_upper(word) {
        candidates.push(lower_initial(word));
    }
    if word.chars().all(|c| !c.is_lowercase()) && word.chars().count() > 1 {
        candidates.push(lower(word));
    }
    let vouched = check(word) == Status::Ok
        && candidates
            .iter()
            .any(|c| picur_morph_core::is_vouched(c, space()));
    if let Ok(mut c) = cache.write() {
        c[slot] = Some((word.into(), vouched));
    }
    vouched
}

/// Whether the token is a host plus the interrogative clitic `-e`.
///
/// `SYN-Q-002` attaches `-e` with a hyphen to the finite verb, or to the predicate in a verbless
/// clause, and takes no lexical input at all: the host simply has to be a word. So the test is
/// whether what precedes the hyphen is itself accepted.
///
/// Rules: `SYN-Q-002`
fn interrogative_clitic(word: &str) -> bool {
    word.strip_suffix("-e")
        .is_some_and(|host| !host.is_empty() && check(host) == Status::Ok)
}

/// Whether the token is an acronym or abbreviation carrying a hyphenated suffix.
///
/// `ORT-ABBR-014` writes the suffix after a hyphen in the form the acronym's pronunciation
/// requires, and `ORT-ABBR-018` makes that pronunciation lexical, so the checker accepts any tail a
/// licensed reading would give rather than guessing one. `MÁV-val` and `BKV-val` are both correct
/// and neither is derivable.
fn hyphenated_abbreviation(word: &str) -> bool {
    let Some((base, tail)) = word.rsplit_once('-') else {
        return false;
    };
    if base.is_empty() || tail.is_empty() {
        return false;
    }
    // A numeric tail is a designation, not a suffix: `COVID-19`, `Airbus-320`. `ORT-HYPH` writes
    // it with a hyphen and there is nothing to inflect, so the host alone decides.
    if tail.chars().all(char::is_numeric) {
        return check(base) == Status::Ok
            || base.chars().all(|c| c.is_uppercase() || c.is_numeric());
    }
    // PH-G2P-013: a symbol is suffixed by the word it is read as, whatever precedes it.
    if picur_morph_core::acronym::licenses_expansion(base, tail) {
        return true;
    }
    // An acronym is written in capitals; an abbreviation or unit is short and lowercase (`kg-nak`,
    // `tbc-s`). Anything longer is an ordinary hyphenated compound and is not this rule's business.
    let acronym = base.chars().all(|c| c.is_uppercase() || c.is_numeric());
    // A lowercase host counts only if the lexicon records it as an abbreviation; any short token would admit `van-e`,
    // the interrogative clitic.
    // Listed small, written with a capital in a company name: `Zrt-vel`, `Kft-nek`.
    let abbreviation = [base.to_owned(), lower_initial(base)]
        .iter()
        .flat_map(|b| lexicon().get(b))
        .any(|e| e.has(Trait::Abbreviation));
    // `ORT-FOR-012` writes the suffix after a hyphen when a foreign name ends in a silent letter
    // (`Montreux-ig`, `Glasgow-ban`), and whether it does is a fact about the pronunciation that
    // no spelling shows. Acceptance is therefore permissive here, as it is for the acronym's
    // allomorph: a capitalised base that is not a common word licenses the hyphen, and the
    // unhyphenated `Glasgowban` stays accepted too, because for most such names it is right.
    // Listed with its capital, or not listed without it: `Shakespeare` is both a name and, in lowercase, the base of
    // `shakespeare-i`, so a lowercase-only test calls it a common word.
    let proper_name = starts_upper(base)
        && (lexicon().get(&lower_initial(base)).is_empty() || !lexicon().get(base).is_empty());
    if !acronym && !abbreviation && !proper_name {
        return false;
    }
    // A number's pronunciation is not lexical, so `PND-NUMINF-010` computes its suffix exactly; the acronym probe has no
    // final letter to work from and cannot offer `1-et` or the day form `1-jén`.
    licenses_nominal(base, tail)
        || picur_morph_core::numeral::licenses_number(base, tail)
        || (proper_name && !acronym && picur_morph_core::acronym::licenses_foreign(tail))
}

/// [`check`] without the memo.
fn check_uncached(word: &str) -> Status {
    if hyphenated_abbreviation(word) || interrogative_clitic(word) {
        return Status::Ok;
    }
    if writes_three_identical_consonants(word) {
        return Status::Wrong;
    }
    // PUN-NUMF-010: a Roman numeral is a number, not a word with no vowel.
    if picur_morph_core::numeral::is_roman(word) {
        return Status::Ok;
    }
    // A capitalised word may be a sentence-initial common noun, so both spellings are tried before
    // the word is called wrong. Rejecting `Kutya` because the lexicon lists `kutya` would be a
    // false rejection on the first word of every sentence. An all-capital word is a word set in
    // capitals before it is anything else (`WS-CASE-004`), so its lowercase is tried too.
    let mut candidates = vec![word.to_owned()];
    if starts_upper(word) {
        candidates.push(lower_initial(word));
    }
    if word.chars().all(|c| !c.is_lowercase()) && word.chars().count() > 1 {
        candidates.push(lower(word));
    }
    // The fast path answers the overwhelmingly common case, a correct word, without building
    // readings at all.
    if candidates.iter().any(|c| is_known(c, space())) {
        return Status::Ok;
    }
    // ORT-ABBR-011: a letter acronym is well formed as written and its expansion is lexical, so
    // it is unvouched rather than wrong. A single letter is a letter, named rather than spelled,
    // and is likewise not a word to judge: a list marker, an initial, a variable.
    if picur_morph_core::acronym::is_letter_acronym(word) || letters(word).count() == 1 {
        return Status::Unknown;
    }
    let mut best = Status::Wrong;
    for c in &candidates {
        match analyse(c, space()) {
            Verdict::Known(_) => return Status::Ok,
            Verdict::WellFormed => best = Status::Unknown,
            _ => {}
        }
    }
    if best == Status::Wrong && (foreign_name(word) || name_compound(word)) {
        return Status::Unknown;
    }
    // An unlisted word that is an accent away from an accepted one is that word with its accents
    // damaged, which is what a checker exists to catch: `hosszu`, `ugy`, `útat`. A capitalised
    // word is exempt, since a name is not a misspelling of whatever it resembles.
    if best == Status::Unknown && !starts_upper(word) && accent_confusable(word, space()) {
        return Status::Wrong;
    }
    best
}

/// Whether the token is a capitalised word spelled with a Latin letter the Hungarian alphabet does
/// not have.
///
/// `ORT-FOR-005`: a Latin-script name keeps its own letters, so `Čapek` and `Jovanović` are
/// correctly written as they stand and are names the lexicon does not vouch for, not wrong words.
fn foreign_name(word: &str) -> bool {
    let hungarian: usize = letters(word).map(|s| s.text.len()).sum();
    let latin: usize = word.chars().filter(|c| *c != '-').map(char::len_utf8).sum();
    starts_upper(word) && hungarian < latin && word.chars().all(|c| is_latin_letter(c) || c == '-')
}

/// Whether the token is a capitalised member and one more joined by a hyphen, each of which is a
/// word or an unvouched name: `Brogan-Moore`, `Wi-Fi`, `Apollo-program`.
///
/// `ORT-NAME-007` writes a double family name with a hyphen and `ORT-NAME-014` a name before the
/// noun it compounds with, and the name need not be listed. Each member is judged on its own, so
/// a member the checker calls wrong still makes the whole wrong, and a lowercase member must be a
/// word outright, or `Péter-tel` would pass on an unvouched `tel`. A member of one letter is a
/// suffix written wrong (`Péter-t`), not a member, and is left to the suffix rules.
fn name_compound(word: &str) -> bool {
    let vouched = |m: &str| match check(m) {
        Status::Ok => true,
        Status::Unknown => starts_upper(m),
        _ => false,
    };
    let mut members = word.split('-');
    members.next().is_some_and(starts_upper)
        && word.split('-').count() == 2
        && word
            .split('-')
            .all(|m| letters(m).count() >= 2 && vouched(m))
}

/// Whether the token writes three identical consonant letters in a row.
///
/// `ORT-ASSIM-017` leaves nowhere for such a run to be standard: where suffixation would bring
/// three of the same consonant letter together, a common word writes two (`toll` plus `-lal` is
/// `tollal`), and a proper name or a compound boundary keeps all three sounds but separates them
/// with a hyphen (`Széll-lel`, `sakk-kör`, `balett-táncos`). Written solid it is an error whatever
/// the morphology behind it, which is why this needs no lexicon.
///
/// An all-capital token is an acronym rather than a Hungarian word, and is left alone.
///
/// Rules: `ORT-ASSIM-017`, `ORT-HYPH-001`
fn writes_three_identical_consonants(word: &str) -> bool {
    if word.chars().any(|c| !c.is_alphabetic()) {
        return false;
    }
    let lower: Vec<char> = word.chars().flat_map(char::to_lowercase).collect();
    if word
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(char::is_uppercase)
    {
        return false;
    }
    lower
        .windows(3)
        .any(|w| w[0] == w[1] && w[1] == w[2] && !"aáeéiíoóöőuúüű".contains(w[0]))
}

/// One flagged token in a text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Finding {
    /// The token as written.
    pub word: Box<str>,
    /// Byte offset of the token in the input.
    pub at: usize,
    /// What the checker concluded.
    pub status: Status,
}

/// Checks a whole text, returning one finding per token that is not [`Status::Ok`].
///
/// The text must already be in NFC (`WS-NORM-001`). This does not compose it, because the
/// offsets it returns index the caller's own string and composing would shift them; a caller
/// holding text from PDF extraction, macOS file names or some input methods should pass it
/// through `picur_morph_ortho::norm::compose` first. Skipping that is silent and expensive: a
/// document with `é` written as `e` plus U+0301 has every such word rejected.
///
/// Tokenisation is deliberately simple: a token is a run of alphabetic characters and internal
/// hyphens. Sentence and punctuation rules belong to chapter 08 and are not applied here.
#[must_use]
pub fn check_text(text: &str) -> Vec<Finding> {
    let mut out = Vec::new();
    let words = tokens(text);
    for (i, &(at, word)) in words.iter().enumerate() {
        let generic = words
            .get(i + 1)
            .filter(|&&(next, _)| text.get(at + word.len()..next) == Some(" "))
            .map(|&(_, g)| g);
        let status = match generic {
            Some(g) if picur_morph_core::derivation::hyphenated_in_street_name(word, g) => {
                Status::Wrong
            }
            _ => check(word),
        };
        let solid_street_name = generic.is_some_and(|g| {
            picur_morph_core::derivation::solid_in_street_name(word, g, |w| check(w) == Status::Ok)
        });
        if status != Status::Ok && !solid_street_name && !lowercased_pair_member(text, &words[i..])
        {
            out.push(Finding {
                word: word.into(),
                at,
                status,
            });
        }
    }
    out
}

/// Whether the first of `words` is a name lowercased inside an en-dash pair whose last member is
/// the derivative of a name.
///
/// `AkH12` §264 c) suffixes the last member of a pair joined by an unspaced en dash, and the
/// derivative of a name is lowercase (§161), so every member lowercases: `budapest–bécsi`,
/// `moszkva–párizsi`.
///
/// Rules: `ORT-NAME-027`, `ORT-HYPH-013`
fn lowercased_pair_member(text: &str, words: &[(usize, &str)]) -> bool {
    const EN_DASH: &str = "\u{2013}";
    let Some(&(_, first)) = words.first() else {
        return false;
    };
    let joined = words
        .windows(2)
        .take_while(|w| text.get(w[0].0 + w[0].1.len()..w[1].0) == Some(EN_DASH))
        .count();
    let Some(&(_, last)) = words.get(joined).filter(|_| joined > 0) else {
        return false;
    };
    let name = picur_morph_ortho::case::upper_initial(first);
    !starts_upper(last)
        && name.is_some_and(|n| check(&n) == Status::Ok && !lexicon().get(&n).is_empty())
        && analyses(last).iter().any(|r| starts_upper(&r.lemma))
}

/// Corrections for a token, best first.
///
/// Candidates are generated by single-letter edits **over letters, not characters**, which matters:
/// `sz` is one letter, so deleting the `s` of `ész` must not be offered as a correction any more
/// than deleting half of any other letter would be. Every candidate is then verified by
/// [`check`], so a suggestion is never a word the checker would itself reject.
#[must_use]
pub fn suggest(word: &str) -> Vec<String> {
    if check(word) == Status::Ok {
        return Vec::new();
    }
    let mut scored: Vec<((u8, usize, u8), String)> = Vec::new();
    for c in edits(word) {
        if !is_word(&c) || scored.iter().any(|(_, s)| *s == c) {
            continue;
        }
        // Longer shared prefix first: a real misspelling usually goes wrong late in the word, so `tökhez` goes to `tökhöz`, not `tékhez`.
        // Frequency breaks prefix ties: `látja`, `kertje` and `bánja` each share their stem with five rivals.
        // Frequency ahead of prefix scores better on pairs (14 first of 60 against 11; 54% reached against 44%) but puts `tékhez`
        // before `tökhöz` and drops `lássunk` from the list.
        let rank = (
            strength(&c),
            usize::MAX - shared_prefix(word, &c),
            u8::MAX - frequency_of(&c),
        );
        scored.push((rank, c));
    }
    // A compound-only correction is a last resort. Any string can be cut into members somehow, so
    // offering `házcsen` beside `házban` would bury the answer.
    if scored.iter().any(|((k, _, _), _)| *k == 0) {
        scored.retain(|((k, _, _), _)| *k == 0);
    }
    // A stable sort keeps the edit order inside each tier, so a substitution still beats an
    // insertion that scores the same.
    scored.sort_by_key(|(rank, _)| *rank);
    scored
        .into_iter()
        .take(MAX_SUGGESTIONS)
        .map(|(_, s)| s)
        .collect()
}

/// Hyphenates a word, using the morpheme seams its own analysis finds.
///
/// This is what separates a Hungarian hyphenator that works from one that does not: the written
/// form alone reads `házsor` with a `zs` and breaks it `há-zsor`, while the word is `ház` plus
/// `sor` and breaks `ház-sor` (`ORT-SPLIT-015`, `ORT-SPLIT-020`). Only morphology knows.
///
/// A word with no reading is hyphenated phonologically, which is the best available answer.
///
/// Rules: `ORT-SPLIT-001`, `ORT-SPLIT-015`, `ORT-SPLIT-020`
///
/// A homograph has one hyphenation per reading (`gép-elem`, `gé-pelem`); this is the one with the
/// most seams, so a word read both bare and suffixed (`község`) breaks at its seam (`köz-ség`).
#[must_use]
pub fn hyphenate(word: &str) -> Vec<String> {
    hyphenations(word).swap_remove(0)
}

/// Every hyphenation of `word`, one per distinct set of seams its readings give, most seams
/// first. A word with no reading is hyphenated phonologically.
///
/// Rules: `ORT-SPLIT-015`
#[must_use]
pub fn hyphenations(word: &str) -> Vec<Vec<String>> {
    let mut sets = seams(word);
    sets.sort_by_key(|s| std::cmp::Reverse(s.len()));
    sets.dedup();
    if sets.is_empty() {
        sets.push(Vec::new());
    }
    let mut out: Vec<Vec<String>> = Vec::new();
    for set in &sets {
        let pieces = picur_morph_ortho::hyphenate_at(word, set);
        if !out.contains(&pieces) {
            out.push(pieces);
        }
    }
    out
}

/// Whether every break in `split` (`vas-út`, `ka-to-na`) is a legal line break, with the seams
/// the word's analysis finds.
///
/// Rules: `ORT-SPLIT-001`, `ORT-SPLIT-015`
#[must_use]
pub fn is_legal_split(split: &str) -> bool {
    picur_morph_ortho::splits_by(split, hyphenations)
}

/// The morpheme seams of `word`, one set per reading: every piece boundary of its segmentation.
///
/// A member, a preverb and a prefix such as `leg-` always end at a seam (`ORT-SPLIT-015`). `ORT-SPLIT-009`: a suffix splits by syllable
/// (`pén-zért`, `há-zak`), so its seam only counts before a consonant, where it settles the letters
/// (`köz-ség`, `ház-ban`); before a vowel the syllable rule decides (`meny-nyi`).
fn seams(word: &str) -> Vec<Vec<usize>> {
    let Verdict::Known(readings) = analyse(word, space()) else {
        return Vec::new();
    };
    let consonant_at = |at: usize| {
        word[at..]
            .chars()
            .next()
            .is_some_and(|c| !"aáeéiíoóöőuúüű".contains(c.to_lowercase().next().unwrap_or(c)))
    };
    let mut sets = Vec::new();
    for reading in &readings {
        let Some(pieces) = segment_surface(word, reading) else {
            continue;
        };
        let mut out = Vec::new();
        let mut at = 0;
        let mut past_stem = false;
        for pair in pieces.windows(2) {
            at += pair[0].text.len();
            past_stem |= matches!(pair[0].role, Role::Stem(_));
            let suffix = past_stem && matches!(pair[1].role, Role::Affix(_));
            if !suffix || consonant_at(at) {
                out.push(at);
            }
        }
        out.retain(|&b| b > 0 && b < word.len() && word.is_char_boundary(b));
        out.sort_unstable();
        out.dedup();
        sets.push(out);
    }
    sets
}

/// Whether a candidate correction is a word, without deciding why it is not.
///
/// [`check`] costs two full searches on a word it rejects: `is_known` fails, and then `analyse`
/// runs again to tell an unlisted-but-well-formed word from a real error. A suggester throws away
/// that distinction, and it asks about some seven hundred candidates per token, so paying for it
/// took `suggest` to 5.4s for `házben` and 7.2s for `kertban`. A candidate whose stem is real is
/// what costs: `házben` needs 225ms to be rejected where `xqzben` needs 0.6ms.
///
/// The set accepted is `is_known`'s, which is what `check`'s own fast path uses, so a correction
/// only reachable through the reverse tables is not offered.
fn is_word(word: &str) -> bool {
    if hyphenated_abbreviation(word) || interrogative_clitic(word) {
        return true;
    }
    if is_known(word, space()) {
        return true;
    }
    starts_upper(word) && is_known(&lower_initial(word), space())
}

/// How common a candidate is, as a band, taking the commonest band of it or its stem.
///
/// An inflected form is usually absent from the frequency list while its lemma is there, so the
/// form alone would score most candidates zero and break no ties at all.
fn frequency_of(word: &str) -> u8 {
    let direct = frequency(word);
    let Verdict::Known(readings) = analyse_with(word, space()) else {
        return direct;
    };
    readings
        .iter()
        .map(|r| frequency(&r.lemma))
        .chain(std::iter::once(direct))
        .max()
        .unwrap_or(direct)
}

/// How many leading letters two words share.
fn shared_prefix(a: &str, b: &str) -> usize {
    letters(a)
        .zip(letters(b))
        .take_while(|(x, y)| x.text == y.text)
        .count()
}

/// How strong a candidate correction is, lower being better.
///
/// A word that analyses on its own is a far better suggestion than one that only works as a
/// compound. The lexicon holds 124k stems, so almost any string can be cut into members somehow,
/// and without this `házben` is "corrected" to `házcsen` rather than to `házban`.
fn strength(word: &str) -> u8 {
    u8::from(!matches!(analyse_with(word, space()), Verdict::Known(_)))
}

/// Most suggestions returned for one token.
const MAX_SUGGESTIONS: usize = 8;

/// Single-letter edits of `word`, over the Hungarian alphabet.
fn edits(word: &str) -> Vec<String> {
    let spans: Vec<(usize, usize, bool)> = letters(word)
        .map(|s| (s.at, s.text.len(), s.letter.is_vowel()))
        .collect();
    let mut out = Vec::new();
    // Substitution first: the commonest real error is a wrong vowel, not a missing letter. A vowel
    // is only ever replaced by a vowel and a consonant by a consonant, which is both what real
    // errors look like (harmony and length, `házben` for `házban`) and a third of the work.
    for &(at, len, vowel) in &spans {
        for (r, rv) in ALPHABET.iter() {
            if *rv == vowel && &word[at..at + len] != *r {
                out.push(format!("{}{r}{}", &word[..at], &word[at + len..]));
            }
        }
    }
    for &(at, len, _) in &spans {
        out.push(format!("{}{}", &word[..at], &word[at + len..]));
    }
    for w in spans.windows(2) {
        let ((a, la, _), (b, lb, _)) = (w[0], w[1]);
        out.push(format!(
            "{}{}{}{}",
            &word[..a],
            &word[b..b + lb],
            &word[a..a + la],
            &word[b + lb..]
        ));
    }
    let mut points: Vec<usize> = spans.iter().map(|&(at, _, _)| at).collect();
    points.push(word.len());
    for at in points {
        for (r, _) in ALPHABET.iter() {
            out.push(format!("{}{r}{}", &word[..at], &word[at..]));
        }
    }
    out
}

/// The letters an edit may introduce.
///
/// Every letter of the alphabet, taken from the shared inventory rather than restated, so a change
/// there cannot leave this list stale.
static ALPHABET: std::sync::LazyLock<Vec<(&'static str, bool)>> = std::sync::LazyLock::new(|| {
    Letter::ALL
        .iter()
        .map(|l| (l.as_str(), l.is_vowel()))
        .collect()
});

#[cfg(test)]
mod tests {

    #[test]
    fn analyses_agrees_with_check_on_a_hyphenated_host() {
        // ORT-ABBR-009 attaches the suffix after a hyphen and leaves the host unchanged, and
        // SYN-Q-002's clitic does the same. `analyses` must take the paths `check` takes, or
        // `1991-ben` and `HVG-nek` are accepted with no reading.
        for word in ["1991-ben", "HVG-nek", "van-e", "jön-e"] {
            assert_eq!(check(word), Status::Ok, "{word}");
            assert!(!super::analyses(word).is_empty(), "{word} has no reading");
        }
        // The clitic's host keeps its own category: `van-e` is the verb, not a noun.
        assert!(
            super::analyses("van-e")
                .iter()
                .any(|a| a.pos == picur_morph_lexicon::Pos::Verb)
        );
    }
    use super::{Status, analyses, check, check_text, hyphenate, suggest};
    use picur_morph_ortho::tokens;

    /// Chapter 07's writing-together principles, each with the form that keeps the gap.
    #[test]
    fn chapter_07_solid_writing() {
        for w in [
            // ORT-TOG-036: two derivational suffixes on a phrase.
            "házhozszállítási",
            // ORT-TOG-037: `leg-` on a phrase's comparative.
            "legharcrakészebb",
            "legszívhezszólóbb",
            // ORT-TOG-032: `se-` before a non-`mi` base.
            "semekkora",
            // ORT-NUM-010: a spelled-out year.
            "kétezertizennégy",
            "kétezer-tizennégy",
            // ORT-FOR-010: a lengthening derivative of a spoken name.
            "goethés",
            // ORT-HYPH-005: a common-noun blend before a derived head.
            "trafótekercselés",
        ] {
            assert_eq!(check(w), Status::Ok, "{w}");
        }
        for w in ["szívhezszólóbb", "semmekkora"] {
            assert_ne!(check(w), Status::Ok, "{w}");
        }
    }

    /// ORT-NAME-027: an en-dash pair lowercases with the derivative of its last member only.
    #[test]
    fn en_dash_pairs_lowercase_with_their_derivative() {
        for kept in ["a budapest–bécsi vonat", "a moszkva–párizsi járat"] {
            assert_eq!(check_text(kept).len(), 0, "{kept}");
        }
        assert_eq!(check_text("a budapest–bécs vonal").len(), 2);
    }

    /// The suffix sequences chapter 06 licenses beyond one derivation.
    #[test]
    fn chapter_03_inflection_logic() {
        for w in [
            // DER-MODEL-010: an adjective output lowers, the front semi-word in the accusative.
            "sósat",
            "ködösek",
            "hibamenteset",
            "gluténmentesek",
            // NOM-STEM-018: the stem doublet of `falu`.
            "faluk",
            "falvak",
            "faluja",
            // NOM-POSS-006, NOM-POSS-009: measured or listed possessive doublets.
            "utóda",
            "utódja",
            "fia",
            "fiúja",
            // NOM-ADJ-005: the one irregular comparative with a regular doublet.
            "lassabb",
            "lassúbb",
            "leglassúbbat",
            // NOM-LOW-008, NOM-ACC-009: lexemes shadowed by an abbreviation or a conversion homograph.
            "májak",
            "májat",
            "protestánst",
            "légideszantost",
        ] {
            assert_eq!(check(w), Status::Ok, "{w}");
        }
        for w in [
            "sóst",
            "ködösök",
            "hibamentest",
            "hosszúbb",
            "pajzst",
            "orrt",
            "oldalat",
        ] {
            assert_ne!(check(w), Status::Ok, "{w}");
        }
        // NOM-STEM-018: `faluk` licenses no `-a` possessive; `falva` is the converb of `fal`.
        assert!(analyses("falva").iter().all(|a| &*a.lemma != "falu"));
    }

    #[test]
    fn chapter_06_suffix_sequences() {
        for w in [
            // DER-ADJ-017: `-Ó` plus `-(O)s`.
            "kacsintós",
            "viszketős",
            // DER-ADV-010, DER-ADJ-025: `-i` and `-(O)s` plus `-An` after the multiplicative.
            "hatszori",
            "nyolcszori",
            "ötszörösen",
            // NOM-ASSOC-001, DER-MODEL-001: the associative after a possessed plural.
            "szomszédaimék",
            // DER-MODEL-008: `-képp` is an opaque front domain, so the superessive is `-en`
            // whatever the base does.
            "válaszképpen",
            "példaképpen",
            // DER-MODEL-009: the possessive allomorph and the harmony are chosen apart.
            "fotelja",
            "fotelje",
            "foteljének",
            // DER-NOUN-003: `-sÁg` inside its occupational domain.
            "menedzserség",
            "brókerség",
            "vezérség",
        ] {
            assert_eq!(check(w), Status::Ok, "{w}");
        }
        for w in [
            // DER-NOUN-003: a noun outside the occupational domain takes no `-sÁg`.
            "székség",
            "könyvség",
            // DER-ADJ-010: the privative needs a transitive base.
            "ugrálatlan",
            "repületlen",
            // DER-ADV-003: a potential participle takes `-An`, not the essive.
            "olvashatóul",
            "mérhetőül",
            // DER-DIM-001 rule 6: a monosyllabic consonant-final base takes `-(O)cskA` only.
            "szívke",
            "lábka",
            // DER-NOUN-024: `-né` lengthens a final `a`/`e`.
            "katonané",
            // DER-VERB-014: the medial suffixes.
            "ölelkezget",
            "gazdagodgat",
        ] {
            assert_ne!(check(w), Status::Ok, "{w}");
        }
        for w in [
            "megoldatlan",
            "olvasatlan",
            "angolul",
            "feleségül",
            "bátortalanul",
            "szívecske",
            "lábacska",
            "lányka",
            "madárka",
            "nézegetős",
            // DER-ADJ-017: the event reading of any activity verb.
            "ballagós",
            "lövöldözős",
            // DER-VERB-014: a denominal `-(O)z` verb is not medial.
            "emlékezget",
            // DER-PREF-003, DER-COMP-015: reduplicated preverb, suffixed coordination.
            "meg-megjön",
            "rohantak-szaladtak",
            "üldögélős",
        ] {
            assert_eq!(check(w), Status::Ok, "{w}");
        }
        // DER-VERB-018: the onomatopoeic pair on a sound-imitating root, and nothing on a
        // root of the wrong shape.
        for w in [
            "zuttyog", "zuttyan", "bummog", "bumman", "koppan", "csattan", "kopog",
        ] {
            assert_eq!(check(w), Status::Ok, "{w}");
        }
        for w in ["jajog", "hátog", "házog", "házan"] {
            assert_ne!(check(w), Status::Ok, "{w}");
        }
        // DER-MODEL-009: the alternant a harmonic suffix picked fixes what follows.
        for w in ["agilisságnak", "agilisségnek", "karibinak", "karibinek"] {
            assert_eq!(check(w), Status::Ok, "{w}");
        }
        for w in ["agilisságnek", "agilisségnak"] {
            assert_ne!(check(w), Status::Ok, "{w}");
        }
    }

    #[test]
    fn ordinary_words_pass() {
        for w in [
            "ház", "házban", "házak", "kutya", "olvas", "szép", "asztalok",
        ] {
            assert_eq!(check(w), Status::Ok, "{w}");
        }
    }

    #[test]
    fn harmony_errors_are_wrong_not_merely_unknown() {
        for w in ["házben", "kertban", "tökhez", "gázak"] {
            assert_eq!(check(w), Status::Wrong, "{w}");
        }
    }

    #[test]
    fn unlisted_compounds_pass() {
        for w in ["vízbicikli", "macskaeledel", "autókulcs", "kutyaházat"] {
            assert_eq!(check(w), Status::Ok, "{w}");
        }
    }

    #[test]
    fn a_sentence_initial_common_noun_is_not_an_error() {
        assert_eq!(check("Kutya"), Status::Ok);
        assert_eq!(check("Házban"), Status::Ok);
    }

    #[test]
    fn an_unknown_but_well_formed_word_is_its_own_verdict() {
        // Neither Ok nor Wrong: the checker says "I do not know this", which is the answer that
        // lets a false rejection be fixed without padding the dictionary.
        assert_eq!(check("blorvasztény"), Status::Unknown);
    }

    #[test]
    fn tokenisation_keeps_offsets() {
        let t = tokens("A ház nagy.");
        assert_eq!(t, vec![(0, "A"), (2, "ház"), (7, "nagy")]);
    }

    #[test]
    fn check_text_reports_only_problems() {
        let f = check_text("A ház nagy, de a kertban hideg.");
        assert_eq!(f.len(), 1, "{f:?}");
        assert_eq!(f[0].word.as_ref(), "kertban");
        assert_eq!(f[0].status, Status::Wrong);
    }

    #[test]
    fn a_suffixed_number_or_symbol_is_one_token() {
        // A digit is wordy, or the token breaks at the hyphen and the checker is asked about
        // `ban` and `os`.
        for text in [
            "A 10 legjobb film 2020-ban.",
            "Az MTA-nak 5-kor, 1-jén.",
            "50%-os emelés, 3,5 millió forint.",
            "A COVID-19 járvány 2020 óta tart.",
            "A hőmérséklet 20 °C-on stabil.",
        ] {
            assert!(
                check_text(text).is_empty(),
                "{text}: {:?}",
                check_text(text)
            );
        }
    }

    #[test]
    fn hyphenation_uses_the_morpheme_seams() {
        // The written form alone reads a `zs` here and would give `há-zsor`.
        assert_eq!(hyphenate("házsor").join("-"), "ház-sor");
        assert_eq!(hyphenate("vízsugár").join("-"), "víz-su-gár");
        // A suffix boundary counts too, and so does ORT-SPLIT-007's digraph restoration.
        assert_eq!(hyphenate("házban").join("-"), "ház-ban");
        assert_eq!(hyphenate("mennyi").join("-"), "meny-nyi");
        for (word, want) in [
            ("megérkezett", "meg-ér-ke-zett"),
            ("kisasszony", "kis-asz-szony"),
            ("vasút", "vas-út"),
            ("szemüveg", "szem-ü-veg"),
            ("legerősebb", "leg-e-rő-sebb"),
            ("tanár", "ta-nár"),
        ] {
            assert_eq!(hyphenate(word).join("-"), want);
        }
        assert!(super::is_legal_split("gép-elem") && super::is_legal_split("gé-pelem"));
        assert!(!super::is_legal_split("há-zsor"));
        assert_eq!(hyphenate("templom").join("-"), "temp-lom");
        assert_eq!(hyphenate("kasza").join("-"), "ka-sza");
    }

    #[test]
    fn suggestions_correct_a_harmony_error() {
        let s = suggest("házben");
        assert!(s.contains(&"házban".to_owned()), "{s:?}");
    }

    #[test]
    fn a_correct_word_gets_no_suggestions() {
        assert_eq!(suggest("házban"), [] as [String; 0]);
    }

    #[test]
    fn every_suggestion_passes_the_checker() {
        // A suggestion the checker would itself reject is worse than none.
        for w in ["házben", "kertban", "gázak"] {
            for s in suggest(w) {
                assert_eq!(check(&s), Status::Ok, "{w} -> {s}");
            }
        }
    }

    #[test]
    fn a_suffixed_acronym_is_accepted_under_any_licensed_reading() {
        // ORT-ABBR-014, ORT-ABBR-018: the pronunciation is lexical, so both harmonies are allowed.
        for w in [
            "MÁV-val",
            "BKV-val",
            "BKV-vel",
            "OTP-vel",
            "CNN-nek",
            "ENSZ-szel",
            "MTA-tól",
            "NATO-hoz",
            "ELTE-re",
            "tbc-s",
            "kg-nak",
        ] {
            assert_eq!(check(w), Status::Ok, "{w}");
        }
    }

    #[test]
    fn a_suffixed_acronym_is_still_checked() {
        // Permissive about the reading, not about the morphology.
        for w in ["MÁV-xyz", "BKV-qqq", "ENSZ-zzz"] {
            assert_eq!(check(w), Status::Wrong, "{w}");
        }
    }

    #[test]
    fn the_interrogative_clitic_attaches_to_any_host() {
        // SYN-Q-002 takes no lexical input: the host has only to be a word. A finite verb and a
        // nominal predicate both qualify.
        for w in ["halad-e", "van-e", "olvas-e", "beteg-e", "igaz-e"] {
            assert_eq!(check(w), Status::Ok, "{w}");
        }
        assert_ne!(check("blorvasz-e"), Status::Ok);
    }

    #[test]
    fn a_comparative_inflects() {
        // NOM-ADJ-007 through the checker.
        for w in [
            "kevesebbet",
            "legkevesebbet",
            "nagyobbat",
            "legjobbat",
            "szebbek",
        ] {
            assert_eq!(check(w), Status::Ok, "{w}");
        }
    }
}
