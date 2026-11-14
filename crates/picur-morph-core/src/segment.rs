//! Cutting a surface form into the pieces its morphs are written as.
//!
//! Analysis says which morphs a form carries; this says where each one is written. The two are not
//! the same question, and the second has no answer in general: Hungarian is not purely
//! concatenative, and a morph can rewrite the stem instead of attaching to it.
//!
//! Nothing here reads the surface. The pieces come from generation, by building the form one morph
//! at a time and seeing what each step did to the string, so a piece is only ever claimed where
//! generation actually produced it. A step can only do three things, and which one it did is read
//! off the strings rather than off a list of suffixes: it extends the form at the end, it extends
//! it at the front, or it rewrites what was already there. The third case is reported as such
//! rather than guessed at, because a cut that cannot be justified is worse than no cut.

use picur_morph_lexicon::{Entry, Pos, Trait, lexicon};

use crate::compound::{head_allows, member_prefixes};
use crate::features::Morph;
use crate::generate::generate;
use crate::verbal::preverb_base;

/// What a written piece of a form realises.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Role {
    /// A preverb, written in front of the stem it belongs to.
    Preverb,
    /// A non-final member of a compound, morphologically inert (`DER-COMP-007`).
    Member,
    /// The lemma's stem, as written in this form.
    ///
    /// Carries the morphs that could not be cut from it. A morph lands here when its step rewrote
    /// the form rather than extending it, which is what an alternating stem does: `szép` plus the
    /// comparative is `szebb`, and no cut of `szebb` is the lemma.
    Stem(Vec<Morph>),
    /// One morph, written as a piece of its own.
    Affix(Morph),
}

/// One written piece of a surface form.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    /// The characters, exactly as the form writes them.
    pub text: Box<str>,
    /// What they realise.
    pub role: Role,
}

/// The pieces `stack` is written as on `entry`, in the order they appear.
///
/// `None` when the stack does not generate, which is the same condition analysis uses to reject a
/// reading: a form that cannot be built cannot be cut.
///
/// The pieces concatenate back to the generated form exactly.
#[must_use]
pub fn segment(entry: &Entry, stack: &[Morph]) -> Option<Vec<Piece>> {
    let mut form = generate(entry, &[]).ok()?;
    let mut front: Vec<Piece> = Vec::new();
    let mut back: Vec<Piece> = Vec::new();
    let mut stem = form.clone();
    let mut fused: Vec<Morph> = Vec::new();

    for (i, morph) in stack.iter().enumerate() {
        let next = generate(entry, &stack[..=i]).ok()?;
        // `NOM-LENG-001`: a final `a`/`e` lengthens before a suffix. The lengthened vowel stays with
        // the piece it ends and the suffix is cut after it: `szoba` + `-bAn` is `szobá` + `ban`,
        // `háza` + `-bAn` is `ház` + `á` + `ban`.
        let lengthened = crate::generate::lengthen_final(&form);
        if lengthened != form
            && let Some(added) = next.strip_prefix(lengthened.as_str())
        {
            match back.last_mut() {
                Some(last) => last.text = crate::generate::lengthen_final(&last.text).into(),
                None => stem.clone_from(&lengthened),
            }
            back.push(Piece {
                text: added.into(),
                role: Role::Affix(*morph),
            });
        } else if let Some(added) = next.strip_prefix(form.as_str()) {
            back.push(Piece {
                text: added.into(),
                role: Role::Affix(*morph),
            });
        } else if let Some(added) = next.strip_suffix(form.as_str()) {
            front.insert(
                0,
                Piece {
                    text: added.into(),
                    role: Role::Affix(*morph),
                },
            );
        } else if let Some((head, tail)) = next
            .find(form.as_str())
            .map(|at| (&next[..at], &next[at + form.len()..]))
        {
            front.insert(
                0,
                Piece {
                    text: head.into(),
                    role: Role::Affix(*morph),
                },
            );
            back.push(Piece {
                text: tail.into(),
                role: Role::Affix(*morph),
            });
        } else {
            // The step rewrote the form. Whatever the pieces around the stem still are, the stem
            // is what is left between them, and this morph is written into it.
            stem = between(&next, &front, &back).unwrap_or_else(|| {
                front.clear();
                back.clear();
                fused.extend(stack[..i].iter().copied());
                next.clone()
            });
            fused.push(*morph);
        }
        form = next;
    }

    let mut out = front;
    out.extend(split_stem(entry, &stem, fused));
    out.extend(back);
    Some(out)
}

/// The pieces of one reading.
///
/// A convenience over [`segment`] for a caller holding an [`Analysis`](crate::analysis::Analysis): the reading names its
/// lemma and category, and those pick the entry to generate from. A compound the lexicon does not
/// carry whole is generated from its head, with its members as seams, the way analysis read it.
/// `None` when neither is possible.
#[must_use]
pub fn segment_reading(reading: &crate::analysis::Analysis) -> Option<Vec<Piece>> {
    let listed = lexicon().get_pos(&reading.lemma, reading.pos);
    let entry = listed
        .cloned()
        .or_else(|| compound_entry(&reading.lemma, reading.pos))
        // A guessed reading has no entry anywhere; its lemma is the stem the guess was built on.
        .unwrap_or_else(|| Entry::new(reading.lemma.clone(), reading.pos));
    segment(&entry, &reading.stack)
}

/// The pieces of `surface` under `reading`, for a reading that may be a guess.
///
/// [`segment_reading`] cuts the form the reading's entry generates, which for a guessed name is
/// its letters' form; a name read with its suffix after a hyphen (`Chilcote-ot`) is generated by
/// a pronunciation, so the entry is rebuilt the way [`guess`](crate::analysis::guess) built it,
/// and only a cut that reproduces `surface` exactly is returned.
#[must_use]
pub fn segment_surface(surface: &str, reading: &crate::analysis::Analysis) -> Option<Vec<Piece>> {
    let rebuilds =
        |pieces: &[Piece]| pieces.iter().map(|p| p.text.as_ref()).collect::<String>() == surface;
    if let Some(pieces) = segment_reading(reading).filter(|p| rebuilds(p)) {
        return Some(pieces);
    }
    let (left, tail) = surface.split_once('-')?;
    if left != reading.lemma.as_ref() {
        return None;
    }
    let consonant = picur_morph_ortho::letters(tail)
        .next()
        .filter(|s| s.letter.is_consonant())
        .map_or("b", |s| s.letter.as_str());
    for h in [Trait::Back, Trait::Front, Trait::FrontRounded] {
        for last in [None, Some(consonant)] {
            let mut entry = Entry::new(left, reading.pos);
            entry.traits = entry.traits.with(Trait::Spoken).with(h);
            entry.last_sound = last.map(Into::into);
            if let Some(pieces) = segment(&entry, &reading.stack).filter(|p| rebuilds(p)) {
                return Some(pieces);
            }
        }
    }
    None
}

/// An entry for an unlisted compound: its head's, under the whole lemma, with the member seams.
///
/// The longest prefix of listed members with a listed head wins, which is how [`generate`]'s
/// compound path chooses too, so the pieces come from the same form analysis verified.
fn compound_entry(lemma: &str, pos: Pos) -> Option<Entry> {
    let (members, at) = member_prefixes(lemma)
        .into_iter()
        .filter(|(members, at)| !members.is_empty() && *at < lemma.len())
        .filter(|(_, at)| {
            lexicon()
                .get_pos(&lemma[*at..], pos)
                .is_some_and(|e| head_allows(e.pos, false))
        })
        .max_by_key(|(_, at)| *at)?;
    let mut entry = lexicon().get_pos(&lemma[at..], pos)?.clone();
    entry.lemma = lemma.into();
    let mut seams: Vec<u32> = Vec::new();
    let mut offset = 0usize;
    for m in &members {
        offset += m.text.len();
        seams.push(u32::try_from(offset).ok()?);
    }
    entry.parts = seams.into_boxed_slice();
    Some(entry)
}

/// The text of `form` left between the pieces already known to sit around the stem.
fn between(form: &str, front: &[Piece], back: &[Piece]) -> Option<String> {
    let mut rest = form;
    for p in front {
        rest = rest.strip_prefix(&*p.text)?;
    }
    for p in back.iter().rev() {
        rest = rest.strip_suffix(&*p.text)?;
    }
    Some(rest.to_owned())
}

/// The stem, cut at the seams the lemma carries: compound members and a preverb.
///
/// The seams are byte offsets into the lemma, and the non-final members are inert
/// (`DER-COMP-007`), so they still hold in the stem as written in this form provided the stem
/// begins with the lemma up to the last seam; a stem rewritten before that point is left whole.
/// A verb's first member that is a listed preverb is reported as one. Where the lemma records no
/// seam, [`preverb_base`] still supplies the preverb cut, so `megaludt` splits where `megalszik`
/// does even though `alud` is listed nowhere.
fn split_stem(entry: &Entry, stem: &str, fused: Vec<Morph>) -> Vec<Piece> {
    let seams: Vec<usize> = entry.parts.iter().map(|&at| at as usize).collect();
    let cut_at: Vec<usize> = match seams.last() {
        Some(&last) if stem.len() > last && stem.starts_with(&entry.lemma[..last]) => seams,
        _ => preverb_base(&entry.lemma)
            .map(|base| entry.lemma.len() - base.len())
            .filter(|&at| stem.starts_with(&entry.lemma[..at]) && stem.len() > at)
            .into_iter()
            .collect(),
    };
    let mut out = Vec::new();
    let mut start = 0;
    for at in cut_at {
        let text = &stem[start..at];
        let preverb = entry.pos == Pos::Verb && start == 0 && lexicon().has_pos(text, Pos::Preverb);
        out.push(Piece {
            text: text.into(),
            role: if preverb { Role::Preverb } else { Role::Member },
        });
        start = at;
    }
    out.push(Piece {
        text: stem[start..].into(),
        role: Role::Stem(fused),
    });
    out
}

#[cfg(test)]
mod tests {
    use picur_morph_lexicon::{Pos, lexicon};

    use super::{Role, segment};
    use crate::features::{Case, Morph, Number, Person};

    /// The pieces of `lemma` read as `pos`.
    fn pieces(lemma: &str, pos: Pos, stack: &[Morph]) -> Vec<(String, Role)> {
        let lx = lexicon();
        let entry = lx
            .get_pos(lemma, pos)
            .expect("lemma is listed under that category");
        segment(entry, stack)
            .expect("generates")
            .into_iter()
            .map(|p| (p.text.into_string(), p.role))
            .collect()
    }

    #[test]
    fn a_suffixed_noun_is_cut_at_every_suffix() {
        let got = pieces(
            "ház",
            Pos::Noun,
            &[
                Morph::PossessedPlural,
                Morph::Possessive {
                    person: Person::First,
                    number: Number::Sing,
                },
                Morph::Case(Case::Ine),
            ],
        );
        let text: Vec<&str> = got.iter().map(|(t, _)| t.as_str()).collect();
        assert_eq!(text, ["ház", "ai", "m", "ban"], "{got:?}");
        assert_eq!(got[0].1, Role::Stem(Vec::new()));
    }

    #[test]
    fn a_lengthened_final_vowel_stays_with_its_piece() {
        let got = pieces("szoba", Pos::Noun, &[Morph::Case(Case::Ine)]);
        let text: Vec<&str> = got.iter().map(|(t, _)| t.as_str()).collect();
        assert_eq!(text, ["szobá", "ban"], "{got:?}");
        assert_eq!(got[0].1, Role::Stem(Vec::new()));
        let got = pieces(
            "ház",
            Pos::Noun,
            &[
                Morph::Possessive {
                    person: Person::Third,
                    number: Number::Sing,
                },
                Morph::Case(Case::Ine),
            ],
        );
        let text: Vec<&str> = got.iter().map(|(t, _)| t.as_str()).collect();
        assert_eq!(text, ["ház", "á", "ban"], "{got:?}");
    }

    #[test]
    fn a_listed_compound_is_cut_at_its_seam() {
        let got = pieces("vasútállomás", Pos::Noun, &[Morph::Case(Case::Sup)]);
        let text: Vec<&str> = got.iter().map(|(t, _)| t.as_str()).collect();
        assert_eq!(text, ["vasút", "állomás", "on"], "{got:?}");
        assert_eq!(got[0].1, Role::Member);
        assert_eq!(got[1].1, Role::Stem(Vec::new()));
    }

    #[test]
    fn an_unlisted_compound_is_cut_from_its_head() {
        let reading = crate::analysis::Analysis {
            lemma: "autópályadíj".into(),
            pos: Pos::Noun,
            stack: vec![Morph::Plural],
        };
        let got = super::segment_reading(&reading).expect("a compound with a listed head");
        let text: Vec<&str> = got.iter().map(|p| p.text.as_ref()).collect();
        assert_eq!(text, ["autópálya", "díj", "ak"], "{got:?}");
    }

    #[test]
    fn the_pieces_concatenate_back_to_the_form() {
        for (lemma, pos, stack) in [
            ("ház", Pos::Noun, vec![Morph::Case(Case::Ine)]),
            (
                "szép",
                Pos::Adjective,
                vec![Morph::Superlative, Morph::Plural],
            ),
            ("olvas", Pos::Verb, vec![Morph::Derive(crate::DerId::Talan)]),
        ] {
            let joined: String = pieces(lemma, pos, &stack)
                .into_iter()
                .map(|(t, _)| t)
                .collect();
            let lx = lexicon();
            let entry = lx.get_pos(lemma, pos).expect("listed");
            let want = crate::generate::generate(entry, &stack).expect("generates");
            assert_eq!(joined, want, "{lemma} {stack:?}");
        }
    }

    /// An alternating stem has no honest cut, and says so instead of inventing one.
    #[test]
    fn a_morph_that_rewrites_the_stem_is_reported_in_the_stem() {
        let got = pieces("szép", Pos::Adjective, &[Morph::Superlative, Morph::Plural]);
        assert_eq!(got.last().expect("a suffix").0, "ek");
        let stem = &got[got.len() - 2];
        assert_eq!(stem.1, Role::Stem(vec![Morph::Superlative]), "{got:?}");
    }

    #[test]
    fn a_preverb_is_cut_from_the_verb() {
        let got = pieces("megnéz", Pos::Verb, &[Morph::Potential]);
        assert_eq!(got[0].1, Role::Preverb);
        assert_eq!(got[0].0, "meg");
        assert_eq!(got[1].0, "néz");
    }
}
