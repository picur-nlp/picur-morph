//! Document-level quality scoring of Hungarian text.
//!
//! Ranking scraped text asks a different question from checking a word. A checker says whether one
//! token is correct; a ranker says whether a document is worth keeping, and it has to stay useful
//! when most of the document is fine and a little of it is broken.
//!
//! The score is deliberately made of parts that are reported separately, because they mean
//! different things to a caller building a training set: text that is *not Hungarian* should be
//! dropped, text that is *damaged* can often be repaired and kept, and text that is merely
//! *unusual* (names, jargon, new words) should be kept as it stands. Collapsing those into one
//! number loses the distinction, so [`Quality`] keeps them and [`Quality::score`] is only a
//! convenience over them.
//!
//! Measured by `cargo xtask rank` over 250 `NerKor` paragraphs and the same paragraphs damaged
//! three ways (2026-09-18): clean 0.968 mean, accents stripped 0.171, OCR shapes 0.256, mojibake
//! 0.390, separating at 0.73 with 1 of the 1,000 on the wrong side.
//!
//! That one is a page of film credits, 41.5% names and 1.9% actually wrong, and it is the cost of
//! [`Quality::score`] counting an unknown word in full. It is a stated trade rather than a hidden
//! one: a discount that carried such a page would also pay for damage, which is the larger
//! error. A caller that wants to keep name-dense text reads [`Quality::unknown`] and decides.
//!
//! The honest limit on the figure is that the negatives are synthetic damage of the positives, so
//! this is the easy version of the problem: real bad text is spam and low-quality prose, neither
//! represented here.
//!
//! On real documents the same run separates languages by a wide margin. Ten English linguistics
//! papers score 0.079 to 0.121 against 0.690 to 0.979 for Hungarian, the low end of the Hungarian
//! range being that same name-dense page.
//!
//! # Real OCR is barely visible
//!
//! The synthetic OCR class above is unrepresentative, and measuring against real OCR shows by how
//! much. Sixteen pages of a Hungarian orthography handbook were rendered at 300 dpi and read back
//! with `tesseract -l hun`, against the PDF's own text layer as ground truth: a 4.62% word error
//! rate, and 0.888 for the scan against 0.891 for the truth. The order is right and the distance
//! is almost nothing, where the synthetic class suggested a gulf.
//!
//! This is what exposed the scoring hole that [`Quality::score`] now closes: while an unknown word
//! counted for less than a rejected one, the scan scored 0.925 against the truth's 0.920, because
//! a misread letter leaves a word the checker cannot place rather than one it can call wrong. The
//! score went up as the text got worse.
//!
//! What remains is that good OCR of a clean scan really is mostly correct Hungarian, and this
//! crate measures correctness. The obvious sharpening does not help: counting the tokens that one
//! `REP-OCR-001` confusion turns into a known word gives 1.99% of the clean text against 2.63% of
//! the scan, which gates nothing. Hungarian inflects densely enough that ordinary unknown words
//! sit one substitution from a real form, and the confusion inventory is the wrong model anyway.
//! Classified against the ground truth, the scan's bad tokens are 38% single-character edits, 27%
//! fragments of split words and 36% merges, so shape confusion is barely a third of what scanning
//! does. Detecting the rest means modelling where scanning breaks words apart and glues them
//! together, which is `REP-SPACE` territory and not a letter table.
//!
//! # What this cannot see
//!
//! Translated Hungarian. Measured on `NerKor` by genre, 120 paragraphs each: the legal subcorpus,
//! which is EU legislative translation from the DGT memory, scores 0.978 against 0.966 for
//! Wikipedia, 0.980 for news and 0.989 for fiction, and across eight chunks apiece it spans
//! 0.940 to 0.997 where fiction spans 0.981 to 0.997. It overlaps native Hungarian entirely and
//! its best chunks beat fiction's.
//!
//! That is not a defect to fix here. Translationese is orthographically and morphologically
//! correct; it is unnatural, not incorrect, and this crate measures correctness. Separating it
//! needs a distributional signal this project does not have and is not built for: an n-gram or
//! neural model of what native Hungarian sounds like, or at least a register profile over
//! function words and the bureaucratic postpositional constructions (`tekintetében`, `esetében`,
//! `keretében`, `értelmében`) that translated legalese leans on.
//!
//! A caller building a training set should treat a high score as "this is correct Hungarian", not
//! as "this is worth training on".

use picur_morph_ortho::encoding::demojibake;
use picur_morph_ortho::norm::compose;
use picur_morph_ortho::tokens;
use picur_morph_repair::{expand_compatibility, is_accent_stripped};
use picur_morph_spell::{Status, check};

/// What a document looks like to the checker.
///
/// Shares are of the alphabetic tokens, so they are comparable across documents of any length.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Quality {
    /// Alphabetic tokens considered.
    pub tokens: usize,
    /// Share accepted as written.
    pub accepted: f64,
    /// Share the checker calls wrong.
    pub rejected: f64,
    /// Share it can neither accept nor reject: well formed, stem unlisted.
    pub unknown: f64,
    /// Whether the text is Hungarian written without its accents (`REP-DIAC-001`).
    pub accent_stripped: bool,
    /// Share of tokens carrying code-page damage (`WS-ENC-006`).
    pub mojibake: f64,
    /// Share of tokens carrying a compatibility ligature (`WS-CHAR-010`).
    pub ligatures: f64,
}

/// The recommended floor on [`Quality::tokens`] before a score is trusted.
///
/// `REP-DIAC-001` abstains below a paragraph for the same reason: a handful of tokens carries too
/// little of the language to separate bad Hungarian from text that is not Hungarian at all. The
/// shares are still computed and reported below it; the floor is the caller's to apply.
pub const MIN_TOKENS: usize = 30;

impl Quality {
    /// A single number in `0.0..=1.0`, for a caller that wants one.
    ///
    /// The accepted share, less the damage the document carries. An unknown word costs exactly as
    /// much as a rejected one, and that is forced rather than chosen: discounting it would pay for
    /// the conversion between the two, and damage is what performs that conversion. A misread or
    /// mistyped letter rarely leaves a word the checker can call wrong, it leaves one the checker
    /// cannot place. While an unknown counted a quarter, moving a token from rejected to unknown
    /// paid three quarters of a point, and a scan of a Hungarian book scored above the text layer
    /// it was made from.
    ///
    /// Any discount keeps that hole open, and counting an unknown for more than a rejection would
    /// make an unfamiliar word worse than an outright error, so parity is the only setting that
    /// neither rewards damage nor punishes novelty more than error.
    ///
    /// A page of names is therefore not free, and should not be: it is correct Hungarian that the
    /// tool cannot vouch for. A caller who wants to keep it reads [`Quality::unknown`] and decides
    /// for itself, which is what the parts are for.
    ///
    /// Accent stripping is reported, not charged: every unaccented word is already rejected or
    /// unknown, so a further term would count the same damage twice. The score of such a document
    /// is what the checker could accept of it, and [`Quality::accent_stripped`] says why the rest
    /// was not. Length is not charged either: the shares are what they are on any document, and a
    /// caller that wants a floor compares [`Quality::tokens`] with [`MIN_TOKENS`].
    #[must_use]
    pub fn score(self) -> f64 {
        let damage = self.mojibake + self.ligatures;
        (self.accepted - damage * 0.5).clamp(0.0, 1.0)
    }

    /// Whether the document carries damage a repair pass would act on.
    #[must_use]
    pub fn is_damaged(self) -> bool {
        self.accent_stripped || self.mojibake > 0.0 || self.ligatures > 0.0
    }
}

/// Scores `text` as a document.
///
/// The text is composed to NFC first (`WS-NORM-001`), because every rule below assumes it.
///
#[must_use]
pub fn quality(text: &str) -> Quality {
    // `WS-NORM-001`: every rule assumes NFC, and scraped text does not always arrive that way.
    // PDF extraction is the case that matters here, and it is silent: a document set with `é` as
    // `e` plus a combining acute has every such word rejected and scores as another language.
    let composed = compose(text);
    let text = composed.as_str();
    let words: Vec<&str> = tokens(text).into_iter().map(|(_, w)| w).collect();
    let n = words.len().max(1);
    let (mut ok, mut wrong, mut unknown, mut moji, mut lig) = (0usize, 0usize, 0usize, 0usize, 0);
    for w in &words {
        match check(w) {
            Status::Ok => ok += 1,
            Status::Wrong => wrong += 1,
            _ => unknown += 1,
        }
        if !demojibake(w).is_empty() {
            moji += 1;
        }
        if expand_compatibility(w) != *w {
            lig += 1;
        }
    }
    #[expect(
        clippy::cast_precision_loss,
        reason = "counts are far below the f64 mantissa"
    )]
    let share = |k: usize| k as f64 / n as f64;
    Quality {
        tokens: words.len(),
        accepted: share(ok),
        rejected: share(wrong),
        unknown: share(unknown),
        accent_stripped: is_accent_stripped(text),
        mojibake: share(moji),
        ligatures: share(lig),
    }
}

#[cfg(test)]
mod tests {
    use super::{MIN_TOKENS, quality};

    const CLEAN: &str = "A tagállamok törekednek arra, hogy a szolgáltatások liberalizálásának \
         mértékén túl is vállalják a további lépéseket, amennyiben gazdasági helyzetük azt \
         lehetővé teszi. A bizottság jelentést nyújt be az Európai Parlamentnek és a Tanácsnak \
         a pénzügyi helyzetre vonatkozóan, különös tekintettel a vállalt kötelezettségekre.";

    #[test]
    fn a_clean_document_scores_well() {
        let q = quality(CLEAN);
        assert!(q.tokens >= MIN_TOKENS, "{} tokens", q.tokens);
        assert!(q.accepted > 0.9, "accepted {}", q.accepted);
        assert!(!q.is_damaged());
        assert!(q.score() > 0.9, "score {}", q.score());
    }

    #[test]
    fn a_short_document_is_still_measured() {
        let q = quality("Rövid szöveg, nem elég hosszú.");
        assert!(q.tokens < MIN_TOKENS);
        assert!(q.accepted > 0.9, "accepted {}", q.accepted);
        assert!(quality("").score() < f64::EPSILON);
    }

    #[test]
    fn accent_stripping_is_reported_and_not_charged_twice() {
        let stripped: String = CLEAN
            .chars()
            .map(|c| match c {
                'á' => 'a',
                'é' => 'e',
                'í' => 'i',
                'ó' | 'ö' | 'ő' => 'o',
                'ú' | 'ü' | 'ű' => 'u',
                _ => c,
            })
            .collect();
        let q = quality(&stripped);
        assert!(q.accent_stripped);
        assert!(q.is_damaged());
        assert!(q.score() > 0.0, "score {}", q.score());
        assert!(q.score() < quality(CLEAN).score());
        assert!((q.score() - q.accepted).abs() < f64::EPSILON);
    }

    #[test]
    fn mojibake_is_seen_and_counted_as_damage() {
        let damaged: String = CLEAN
            .split(' ')
            .map(|w| {
                picur_morph_ortho::encoding::mojibake(
                    w,
                    picur_morph_ortho::encoding::Page::Windows1252,
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        let q = quality(&damaged);
        assert!(q.mojibake > 0.0, "no mojibake seen");
        assert!(q.is_damaged());
        assert!(q.score() < quality(CLEAN).score());
    }

    #[test]
    fn ws_norm_001_a_decomposed_document_is_composed_first() {
        let decomposed: String = CLEAN
            .chars()
            .flat_map(|c| match c {
                'á' => vec!['a', '\u{301}'],
                'é' => vec!['e', '\u{301}'],
                'í' => vec!['i', '\u{301}'],
                'ó' => vec!['o', '\u{301}'],
                'ö' => vec!['o', '\u{308}'],
                'ő' => vec!['o', '\u{30B}'],
                'ú' => vec!['u', '\u{301}'],
                'ü' => vec!['u', '\u{308}'],
                'ű' => vec!['u', '\u{30B}'],
                other => vec![other],
            })
            .collect();
        assert_ne!(decomposed, CLEAN);
        assert!(
            (quality(&decomposed).score() - quality(CLEAN).score()).abs() < 1e-9,
            "decomposed {} against composed {}",
            quality(&decomposed).score(),
            quality(CLEAN).score()
        );
    }
}
