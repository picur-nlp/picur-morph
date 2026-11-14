//! Compound splitting.
//!
//! Hungarian compounds solidly and productively, so no word list can be complete and a spellchecker
//! that only looks words up will reject ordinary writing. `DER-COMP-021` to `DER-COMP-023` set out
//! the candidate generation, the constraints and the ranking, and mark themselves an engineering
//! proposal rather than a source rule; this is that proposal.
//!
//! Ambiguity is returned, not resolved. `karóra` is both `kar|óra` 'wristwatch' and `karó+ra` 'onto
//! a stake', and only context decides, so both come back ranked (`DER-COMP-023`).
//!
//! See `docs/grammar/06-derivation-compounding.md`.

use picur_morph_lexicon::{Pos, Role, Trait, entries, lexicon, lists};
use picur_morph_ortho::case::starts_upper;
use picur_morph_ortho::letters;

/// One member of a segmentation.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Member {
    /// The member as written.
    pub text: Box<str>,
    /// Its part of speech.
    pub pos: Pos,
}

/// One candidate segmentation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Split {
    /// Members, left to right. A single member means the word is not a compound.
    pub members: Vec<Member>,
    /// Whether the whole word is a lexicon entry in its own right.
    pub listed: bool,
}

impl Split {
    /// Whether this analysis has more than one member.
    #[must_use]
    pub fn is_compound(&self) -> bool {
        self.members.len() > 1
    }
}

/// Categories that may stand as a non-final member.
///
/// `DER-COMP-022` rejects `V + N` outright, and a verbal prefix is only a member before a verb or a
/// deverbal head, which the head check below enforces.
fn can_be_nonfinal(pos: Pos) -> bool {
    // PND-INDF-003: the quantifying determiner opens a numeral compound (`mindkét`).
    matches!(
        pos,
        Pos::Noun | Pos::ProperNoun | Pos::Adjective | Pos::Numeral | Pos::Preverb
    )
}

/// Whether `pos` may head a compound whose preceding member is or is not a verbal prefix.
///
/// Exposed because analysis builds its own heads: it hands the remainder of the word to the suffix
/// parser, and the readings that come back must be filtered by the same rule.
///
/// Rules: `DER-COMP-021`
#[must_use]
pub fn head_allows(pos: Pos, after_preverb: bool) -> bool {
    can_be_head(pos, after_preverb)
}

/// Whether a reflexive pronoun heads the compound: `PND-REFL-003` builds 'with N-1 others' on
/// an ordinal stem (`másodmagammal`, `harmadmagával`), the one place a pronoun is a head.
///
/// Rules: `PND-REFL-003`, `PND-ORD-005`
#[must_use]
pub fn reflexive_head(pos: Pos, members: &[Member]) -> bool {
    pos == Pos::Pronoun
        && members
            .last()
            .is_some_and(|m| m.pos == Pos::Numeral && crate::numeral::is_fraction(&m.text))
}

/// Whether a head of category `pos` may follow `members`, as far as numerals go.
///
/// A numeral heads a coined compound only after numerals and the quantifying determiners
/// (`kétszáz`, `félezer`, `mindkét`): after anything else the split is a coincidence, `házhat`
/// being `ház` before `hat`.
///
/// Rules: `PND-CARD-003`, `PND-FRAC-004`, `PND-INDF-003`, `DER-COMP-021`
#[must_use]
pub fn numeral_head_allows(pos: Pos, members: &[Member]) -> bool {
    pos != Pos::Numeral || members.iter().all(|m| m.pos == Pos::Numeral)
}

/// Whether `lemma` may head a compound after a verbal prefix, by its own shape.
///
/// A deverbal adjective is an adjective by category and a verb form underneath, and the prefix
/// belongs to that verb: `meglévő`, `elmondott`, `bejövő`. `can_be_head` cannot see this, because
/// such a word is often a stored lexeme rather than a live participle reading (`lévő` is both a
/// noun and an adjective in the lexicon, with no verb of its own in the analysis).
///
/// Rules: `DER-COMP-021`, `VRB-PREV-001`
#[must_use]
pub fn participle_shaped(lemma: &str) -> bool {
    lemma.ends_with(['ó', 'ő']) || lemma.ends_with('t')
}

/// Whether `member` has the shape of a deverbal noun or adjective: a participle or an `-Ás` noun.
fn deverbal_shaped(member: &str) -> bool {
    participle_shaped(member) || member.ends_with("ás") || member.ends_with("és")
}

/// Categories that may stand as the head.
///
/// `DER-COMP-021` admits a verbal head **only** directly after a verbal prefix. Without that
/// condition every noun plus an obscure verb parses: `ház` + `csen` 'pilfer' would make `házcsen` a
/// word, and an edit-distance checker would then offer it as a correction of `házben`.
fn can_be_head(pos: Pos, after_preverb: bool) -> bool {
    match pos {
        // A preverb modifies a verb and nothing else, so it may not open a nominal compound:
        // `*elalma`, `*beház`, `*kiszék`. Lexicalised nominals built on a preverb (`bejárat`,
        // `összetétel`) are entries of their own and never reach this path.
        Pos::Noun | Pos::ProperNoun | Pos::Adjective | Pos::Numeral => !after_preverb,
        Pos::Verb => after_preverb,
        _ => false,
    }
}

/// Letters in a member that needs no listing to be admitted.
const MIN_MEMBER_LETTERS: usize = 3;

/// The semi-words, each with the category it closes a compound as.
///
/// Rules: `DER-COMP-017`
#[must_use]
pub fn semi_words() -> &'static [(Box<str>, Pos)] {
    entries(Role::SemiWord)
}

/// The category `semi` closes a compound as, if it is a semi-word.
fn semi_word_pos(semi: &str) -> Option<Pos> {
    semi_words()
        .iter()
        .find(|(s, _)| s.as_ref() == semi)
        .map(|(_, p)| *p)
}

/// `lemma` compounded with a semi-word: `kutya` and `szerű` give `kutyaszerű`.
///
/// A non-final member is morphologically inert (`DER-COMP-007`), so the join is plain
/// concatenation, and the doubled letter it can create is written out in full: `víz` plus `szerű`
/// is `vízszerű`. `ORT-HYPH-004` writes a hyphen after a proper name, which keeps its capital,
/// and `ORT-ASSIM-016` after a geminate that the member would make a triple: `expressz-szerű`,
/// `puff-féle`. A name of several elements cases them as its `-i` derivative does
/// (`Ipoly eurorégióbeli`).
///
/// `None` when `semi` is not one of them, or does not join `lemma` ([`semi_word_joins`]).
///
/// Rules: `DER-COMP-017`, `DER-ADJ-014`, `ORT-HYPH-004`, `ORT-ASSIM-016`
#[must_use]
pub fn with_semi_word(lemma: &str, semi: &str) -> Option<String> {
    semi_word_pos(semi)?;
    if !semi_word_joins(lemma, semi) {
        return None;
    }
    let proper = lemma.chars().next().is_some_and(char::is_uppercase);
    if proper && let Some(named) = crate::derivation::name_with_semi_word(lemma, semi) {
        return Some(named);
    }
    let spans: Vec<_> = letters(lemma).collect();
    let triple = matches!(spans.as_slice(), [.., a, b] if a.letter == b.letter)
        && letters(semi)
            .next()
            .is_some_and(|s| Some(s.letter) == spans.last().map(|l| l.letter));
    Some(if proper || triple {
        format!("{lemma}-{semi}")
    } else {
        format!("{lemma}{semi}")
    })
}

/// Whether `semi` may follow `member` as written.
///
/// A semi-word spelled as a lengthening suffix is that suffix (`-né`, `DER-NOUN-024`), and a
/// lengthening suffix never follows a short final `a`/`e` (`DER-MODEL-005`): `Katonáné`, not
/// `*katonané`.
///
/// Rules: `DER-COMP-017`, `DER-MODEL-005`, `DER-NOUN-024`
#[must_use]
pub fn semi_word_joins(member: &str, semi: &str) -> bool {
    let lengthens = crate::derivation::PRODUCTIVE
        .iter()
        .any(|d| d.lvl && matches!(d.template, [crate::suffix::Seg::Lit(t)] if *t == semi));
    !(lengthens && member.ends_with(['a', 'e']))
}

/// Whether `lemma` may head a coined compound, by the lemma alone.
///
/// The length floor applies to the head too, so `kerttye` is not `kert` plus the letter `ty`
/// with a possessive. A name is never the solid head of a coined compound, because a common
/// member before a name takes a hyphen (`ORT-NAME-023`) and a solid one (`Kiskunhalas`) is
/// listed, so `oldalrolis` is not `oldal` before `Roli`. A verb takes one preverb, so after a
/// preverb member the head may not be prefixed itself (`elébeáll` is not `elé` on `beáll`) unless
/// that member is one the lexicon marks as stacking (`újraeloszt`). And the compound as a whole
/// must be [`coinable`].
///
/// `members` are the non-final members the head follows.
///
/// Rules: `DER-COMP-021`, `ORT-NAME-023`, `ORT-TOG-026`
#[must_use]
pub fn admissible_head(lemma: &str, members: &[Member]) -> bool {
    // After a preverb the head is a verb and the preverbs are a closed class, so a short verb
    // costs nothing (`végigég`).
    (admissible_as_head(lemma) || preverb_member(members).is_some())
        // A pronoun is not coined from its members: `egymás` takes no possessive, where `egy`
        // plus `más` would (`PND-REFL-004`).
        && !lexicon()
            .get(&format!("{}{lemma}", members.iter().map(|m| m.text.as_ref()).collect::<String>()))
            .iter()
            .any(|e| e.pos == Pos::Pronoun)
        && !starts_upper(lemma)
        && !preverb_member(members)
            .is_some_and(|p| !crate::verbal::stacking(p) && crate::verbal::prefixed(lemma))
        && unit_head(members, lemma)
        && coinable(members, lemma)
}

/// Whether a head after a decimal prefix is a unit name (`PND-MEAS-002`): `kilobájt`, not
/// `*kiloház`.
fn unit_head(members: &[Member], head: &str) -> bool {
    !members
        .last()
        .is_some_and(|m| lists(Role::UnitPrefix, &m.text))
        || lists(Role::UnitName, head)
}

/// Whether a coined compound of these members and head has at most one coined seam.
///
/// Compounding is productive, but not without limit: a word coined on the spot joins two words,
/// and a longer one is built on a compound that is already a word (`DER-COMP-023` counts a listed
/// sub-compound as one member). Three unrelated members are what let `ingzensört` and
/// `vizsgálatlés` through: no adjacent pair is a word. So the pieces must fall into at most two
/// groups, each a single piece or a listed run of pieces, and a preverb joins the piece after it
/// for free, since that join is a rule rather than a coinage (`ORT-TOG-026`).
///
/// Rules: `DER-COMP-022`, `DER-COMP-023`
#[must_use]
pub fn coinable(members: &[Member], head: &str) -> bool {
    let pieces: Vec<&str> = members
        .iter()
        .map(|m| m.text.as_ref())
        .chain(std::iter::once(head))
        .collect();
    let preverb = |i: usize| members.get(i).is_some_and(|m| m.pos == Pos::Preverb);
    let group = |i: usize, j: usize| {
        (i..j - 1).all(preverb) || !lexicon().get(&pieces[i..j].concat()).is_empty()
    };
    let n = pieces.len();
    let mut fewest = vec![usize::MAX; n + 1];
    fewest[n] = 0;
    for i in (0..n).rev() {
        fewest[i] = (i + 1..=n)
            .filter(|&j| group(i, j) && fewest[j] != usize::MAX)
            .map(|j| fewest[j] + 1)
            .min()
            .unwrap_or(usize::MAX);
    }
    fewest[0] <= 2
}

/// Where a surface stands when it is analysed.
#[derive(Clone, Copy, Debug)]
pub enum Position<'a> {
    /// The whole word.
    Whole,
    /// The head of a coined compound, after these non-final members.
    Head(&'a [Member]),
}

/// The preverb the head directly follows, if the last member is one.
#[must_use]
pub fn preverb_member(members: &[Member]) -> Option<&str> {
    members
        .last()
        .filter(|m| m.pos == Pos::Preverb)
        .map(|m| m.text.as_ref())
}

/// Whether this slice may be a member at all.
fn admissible(text: &str) -> bool {
    letters(text).count() >= MIN_MEMBER_LETTERS || lists(Role::ShortMember, text)
}

/// Whether this slice may close a coined compound.
fn admissible_as_head(text: &str) -> bool {
    letters(text).count() >= MIN_MEMBER_LETTERS || lists(Role::ShortHead, text)
}

/// Candidate boundaries: every letter boundary, plus the inside of every multi-character letter.
///
/// The interior cuts are not noise. `WS-DIG-006` exists because concatenation creates sequences
/// that read as a single letter by default: `vízsugár` is `víz` + `sugár`, but its `z`+`s` parses
/// as the letter `zs`, so a splitter that respects the default parse can never find the seam.
/// `DER-COMP-022` therefore opens the boundary and guards it by requiring both sides to be members,
/// which is what rules out `ví|zsugár` while keeping `víz|sugár`.
pub(crate) fn boundaries(word: &str) -> Vec<usize> {
    let mut cuts = Vec::new();
    for span in letters(word) {
        cuts.push(span.at);
        for (off, _) in span.text.char_indices().skip(1) {
            cuts.push(span.at + off);
        }
    }
    cuts.push(word.len());
    cuts.sort_unstable();
    cuts.dedup();
    cuts
}

/// Entries whose citation form is exactly `text` and which may occupy this position.
fn members_at(text: &str, final_position: bool, after_preverb: bool) -> Vec<Pos> {
    // ORT-NAME-014: a name before a common noun takes a hyphen (`Kossuth-díj`), so a capitalised
    // piece never opens a solid coined compound; `Kossuthhal` is not `Kossuth` with a fish.
    if !final_position && starts_upper(text) {
        return Vec::new();
    }
    let mut out: Vec<Pos> = lexicon()
        .get(text)
        .iter()
        // `ORT-ABBR-009`: an abbreviation is not a word and joins nothing (`köv` in `*kövkemény`).
        .filter(|e| !e.has(Trait::Abbreviation))
        .filter(|e| {
            if final_position {
                can_be_head(e.pos, after_preverb)
            } else {
                can_be_nonfinal(e.pos)
            }
        })
        .map(|e| e.pos)
        .collect();
    // DER-COMP-017: a semi-word closes a compound and derives its own category.
    if final_position && let Some(pos) = semi_word_pos(text) {
        out.push(pos);
    }
    // DER-PREF-005: a bound first member opens a compound and nothing else.
    if !final_position && lists(Role::BoundMember, text) {
        out.push(Pos::Noun);
    }
    // PND-ORD-005: the ordinal stem without `-ik` opens a compound (`harmadfél`, `hetedízben`,
    // `másodmagával`); the stems are generated, so no entry lists them.
    if !final_position && crate::numeral::is_fraction(text) {
        out.push(Pos::Numeral);
    }
    // PND-MEAS-002: a decimal prefix opens a unit name; `unit_head` holds it to that.
    if !final_position && lists(Role::UnitPrefix, text) {
        out.push(Pos::Noun);
    }
    // Homographs of the same category are the same segmentation, so only the categories matter.
    out.sort_unstable();
    out.dedup();
    out
}

/// Every segmentation of `word` into lexicon members.
///
/// The head is matched as a citation form here. A head carrying inflection is the caller's job:
/// [`crate::analysis`] owns suffix parsing, and this feeds it the last member.
///
/// Rules: `DER-COMP-021`, `DER-COMP-022`
#[must_use]
pub fn split(word: &str) -> Vec<Split> {
    let cuts = boundaries(word);
    let mut out = Vec::new();
    let listed = !lexicon().get(word).is_empty();
    walk(word, &cuts, 0, &mut Vec::new(), &mut out);
    for s in &mut out {
        s.listed = listed && s.members.len() == 1;
    }
    rank(&mut out);
    out
}

/// Every way to consume a prefix of `word` as non-final compound members.
///
/// Returns the members consumed and the byte offset where the head begins. This is what analysis
/// needs: the non-final members of a compound are morphologically inert (`DER-COMP-007`), so only
/// the remainder has to be parsed for suffixes, and it can be handed straight to the analyser.
///
/// The empty prefix is included, because "not a compound" is always a candidate.
///
/// Rules: `DER-COMP-007`, `DER-COMP-021`
#[must_use]
pub fn member_prefixes(word: &str) -> Vec<(Vec<Member>, usize)> {
    let cuts = boundaries(word);
    let mut out = vec![(Vec::new(), 0usize)];
    prefix_walk(word, &cuts, 0, &mut Vec::new(), &mut out);
    out
}

/// Depth-first walk collecting non-final member prefixes.
fn prefix_walk(
    word: &str,
    cuts: &[usize],
    from: usize,
    acc: &mut Vec<Member>,
    out: &mut Vec<(Vec<Member>, usize)>,
) {
    if acc.len() >= MAX_MEMBERS {
        return;
    }
    for &cut in cuts {
        if cut <= from || cut >= word.len() {
            continue;
        }
        let piece = &word[from..cut];
        // A preverb is exempt from the length rule. `MIN_MEMBER_LETTERS` caps the noise from
        // stray nominal members, but the preverbs are a closed lexical class, so admitting them
        // costs nothing, and `be` in `bejön` is two letters where `meg` is three.
        let long_enough = admissible(piece);
        for pos in members_at(piece, false, false) {
            // The numerals are a closed class like the preverbs, and `egy` and `öt` open
            // compounds as freely as `két`: `egypárti`, `ötágú`. A bound member is admitted by
            // its list.
            if !long_enough
                && !matches!(pos, Pos::Preverb | Pos::Numeral)
                && !lists(Role::BoundMember, piece)
            {
                continue;
            }
            // A preverb belongs to a verb (`DER-COMP-022`), so what follows a preverb member is
            // the head or a deverbal member of the inner word (`ellátó` in `ellátólánc`), never a
            // plain noun: `bentársas` is not `bent`, `ár` and `sas`. A verb takes one preverb,
            // so a preverb never follows another (`megbeáll` is not `meg`, `be` and `áll`)
            // unless the first is marked as stacking (`újra` before `el` and `osztás`). After a
            // nominal member a preverb opens the inner word, as in `karbon`, `ki` and `égés`.
            if preverb_member(acc).is_some_and(|p| match pos {
                Pos::Preverb => !crate::verbal::stacking(p),
                _ => !deverbal_shaped(piece),
            }) {
                continue;
            }
            // No `heads_a_verb` check here, unlike the citation-form walk: this path hands the
            // remainder to the analyser, so the head is an inflected form and not a lemma. The
            // constraint is enforced instead by `head_allows`, which admits a verbal head only
            // after a prefix. That is what lets `megettük` parse as `meg` plus a finite verb.

            acc.push(Member {
                text: piece.into(),
                pos,
            });
            out.push((acc.clone(), cut));
            prefix_walk(word, cuts, cut, acc, out);
            acc.pop();
        }
    }
}

/// Depth-first walk over the cut points, collecting complete segmentations.
fn walk(word: &str, cuts: &[usize], from: usize, acc: &mut Vec<Member>, out: &mut Vec<Split>) {
    // Caps the search: long words with many short candidate members explode combinatorially.
    if acc.len() >= MAX_MEMBERS {
        return;
    }
    for &cut in cuts {
        if cut <= from {
            continue;
        }
        let piece = &word[from..cut];
        if !admissible(piece) {
            continue;
        }
        let final_position = cut == word.len();
        let after_preverb = acc.last().is_some_and(|m: &Member| m.pos == Pos::Preverb);
        for pos in members_at(piece, final_position, after_preverb) {
            // DER-COMP-022: a verbal prefix is a member only before a verb or a deverbal head.
            if !final_position && pos == Pos::Preverb && !heads_a_verb(word, cuts, cut) {
                continue;
            }
            if final_position && !unit_head(acc, piece) {
                continue;
            }
            acc.push(Member {
                text: piece.into(),
                pos,
            });
            if final_position {
                out.push(Split {
                    members: acc.clone(),
                    listed: false,
                });
            } else {
                walk(word, cuts, cut, acc, out);
            }
            acc.pop();
        }
    }
}

/// Most members a candidate may have.
const MAX_MEMBERS: usize = 6;

/// Whether anything from `at` onward can be a verbal or deverbal head.
///
/// Rules: `DER-COMP-022`
fn heads_a_verb(word: &str, cuts: &[usize], at: usize) -> bool {
    cuts.iter()
        .any(|&c| c > at && lexicon().has_pos(&word[at..c], Pos::Verb))
}

/// Orders candidates by the criteria of `DER-COMP-023`.
///
/// Fewest members first, because an analysis that needs more pieces is making a stronger claim; a
/// whole-word listing outranks any split of the same length.
fn rank(splits: &mut Vec<Split>) {
    // A candidate is identified by its segmentation and its head category. `DER-COMP-002` makes the
    // head decide the compound's category, so two readings that differ only in what category a
    // non-final member was matched as are the same analysis, not an ambiguity worth reporting.
    let key = |s: &Split| {
        (
            s.members
                .iter()
                .map(|m| m.text.to_string())
                .collect::<Vec<_>>(),
            s.members.last().map(|m| m.pos),
        )
    };
    splits.sort_by(|a, b| {
        (a.members.len(), !a.listed)
            .cmp(&(b.members.len(), !b.listed))
            .then_with(|| key(a).cmp(&key(b)))
    });
    splits.dedup_by(|a, b| key(a) == key(b));
}

#[cfg(test)]
mod tests {
    use super::{Split, split};

    fn shapes(word: &str) -> Vec<String> {
        split(word)
            .iter()
            .map(|s| {
                s.members
                    .iter()
                    .map(|m| m.text.to_string())
                    .collect::<Vec<_>>()
                    .join("|")
            })
            .collect()
    }

    fn has(word: &str, want: &str) -> bool {
        shapes(word).iter().any(|s| s == want)
    }

    #[test]
    fn splits_a_transparent_compound() {
        assert!(has("vízsugár", "víz|sugár"), "{:?}", shapes("vízsugár"));
    }

    #[test]
    fn a_digraph_boundary_needs_members_on_both_sides() {
        // DER-COMP-022: the seam inside `zs` is opened, which is the only way to find `víz|sugár`,
        // but `ví|zsugár` dies on the left member rather than on the boundary itself.
        assert!(has("vízsugár", "víz|sugár"));
        assert!(!has("vízsugár", "ví|zsugár"));
        for word in ["rózsa", "vizsga", "kocsi", "harcsa"] {
            assert!(
                split(word).iter().all(|c: &Split| !c.is_compound()),
                "{word} split as {:?}",
                shapes(word)
            );
        }
    }

    #[test]
    fn genuine_ambiguity_is_returned_rather_than_resolved() {
        // DER-COMP-023: `karóra` is both `kar|óra` and the simplex `karó` plus a case suffix. The
        // compound reading must be present; the inflected one belongs to the suffix analyser.
        assert!(has("karóra", "kar|óra"), "{:?}", shapes("karóra"));
    }

    #[test]
    fn the_whole_word_is_always_a_candidate() {
        let s = split("kerékpár");
        assert_ne!(s, [] as [Split; 0]);
        assert_eq!(s[0].members.len(), 1, "{:?}", shapes("kerékpár"));
        assert!(s[0].listed);
    }

    #[test]
    fn fewer_members_rank_first() {
        // DER-COMP-023.
        let s = split("vízsugár");
        assert!(
            s.windows(2)
                .all(|w| w[0].members.len() <= w[1].members.len())
        );
    }

    #[test]
    fn short_noise_members_are_excluded() {
        for word in ["asztal", "kenyér", "ablak"] {
            let s = split(word);
            assert!(
                s.iter().all(|c: &Split| !c.is_compound()),
                "{word} split as {:?}",
                shapes(word)
            );
        }
    }

    #[test]
    fn an_unknown_word_yields_nothing() {
        assert_eq!(split("xyzzyqwerty"), [] as [Split; 0]);
    }

    #[test]
    fn a_semi_word_closes_a_compound() {
        // DER-COMP-017, DER-COMP-022: `-szerű` is a member, not a suffix, so `kutyaszerű` is a
        // two-member compound even though `szerű` is no free word.
        assert!(
            has("kutyaszerű", "kutya|szerű"),
            "{:?}",
            shapes("kutyaszerű")
        );
    }

    #[test]
    fn the_rulebook_examples_split() {
        // DER-COMP-021 and DER-COMP-022, including the longest worked example.
        for (word, want) in [
            ("vízsugár", "víz|sugár"),
            ("almafa", "alma|fa"),
            ("folyóirat", "folyó|irat"),
            ("kerékpár", "kerék|pár"),
            ("szárazföld", "száraz|föld"),
            ("karóra", "kar|óra"),
        ] {
            assert!(has(word, want), "{word} gave {:?}", shapes(word));
        }
        let s = shapes("rendőrfőtörzsőrmester");
        assert!(
            s.iter().any(|x| x.starts_with("rendőr|")),
            "no listed-member reading: {s:?}"
        );
    }

    #[test]
    fn readings_differing_only_in_a_non_final_category_are_one_analysis() {
        // DER-COMP-002: the head decides the compound's category, so a non-final member matching
        // two categories is not two segmentations.
        let s = shapes("kutyaszerű");
        let mut sorted = s.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(s.len(), sorted.len(), "duplicate segmentations: {s:?}");
    }

    #[test]
    fn a_verbal_prefix_needs_a_verbal_head() {
        // DER-COMP-022: `meg|int` is licensed because `int` is a verb.
        assert!(has("megint", "meg|int"), "{:?}", shapes("megint"));
    }
}
