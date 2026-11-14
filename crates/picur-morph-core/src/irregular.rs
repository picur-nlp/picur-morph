//! The suppletive verbs.
//!
//! These are the one place the rulebook itself gives up on rules: `VRB-IRR-001` to `VRB-IRR-010`
//! each print a full paradigm and say `Predictability: lexical`. What is stated here is therefore
//! data, but it is the *smallest* data that reproduces those tables: a stem per slot, not a form
//! per cell. Everything after the stem is the ordinary machinery, so `tesz` gets `tesszük` from the
//! same j-assimilation as `olvas` and `jöjjetek` from the same rounding rule as `ütöttetek`.
//!
//! Ten verbs, closed for good. See `docs/grammar/05-verbal.md` (`VRB-IRR-*`).

use std::sync::LazyLock;

use picur_morph_lexicon::{Role, rows};

/// The stems of one suppletive verb, one per slot of `VRB-MODEL-001`.
#[derive(Clone, Copy, Debug)]
pub struct Irregular {
    /// The lemma this describes.
    pub lemma: &'static str,
    /// Present stem, inflected by the regular endings. Empty when [`Irregular::present_forms`]
    /// supplies the whole table instead.
    pub present: &'static str,
    /// The present indicative indefinite, when no stem generates it.
    ///
    /// `van`, `megy` and `jön` are suppletive *within* the present, so there is nothing to inflect.
    pub present_forms: Option<[&'static str; 6]>,
    /// The third singular indefinite past, which is also the past participle.
    ///
    /// Separate from [`Irregular::past`] because the two differ for `eszik` and `iszik`: `evett`
    /// against `ettem`.
    pub past_3sg: &'static str,
    /// The past stem the person endings attach to, marker included.
    pub past: &'static str,
    /// The stem the conditional marker and the infinitive attach to.
    pub bound: &'static str,

    /// The converb stem, where the verb has an irregular one. Empty means the bound stem serves.
    ///
    /// `VRB-ADVP-001` lists them: `téve`, `véve`, `víve`, `menve`. `eszik` has no current form at
    /// all, and `iszik` and `hisz` are marginal, so those are left alone.
    pub converb: &'static str,

    /// The conditional stem, where it differs from [`Self::bound`]. Empty means it does not.
    ///
    /// `VRB-IRR-001` gives `van` two conditionals: `lenne` on the bound stem and the
    /// counterfactual `volna` on the past stem. Only the second is unreachable any other way,
    /// because `lenne` is also the conditional of `lesz`.
    pub conditional: &'static str,
    /// The imperative stem, with the marker already fused.
    pub imperative: &'static str,
    /// The second singular indefinite imperative, which is not the bare stem here.
    pub imperative_2sg: &'static str,
    /// The short definite second singular imperative, empty when the verb has no definite cells.
    ///
    /// Not derivable from [`Irregular::imperative`]: `hisz` fuses to `higgy` but shortens to
    /// `hidd`, not `*higyd`.
    ///
    /// Rules: `VRB-IMP-007`
    pub imperative_2sg_def: &'static str,
    /// The stem `-hAt` attaches to: `le` gives `lehet`, not `*vanhat`.
    pub potential: &'static str,
    /// The stem the present participle attaches to (`tev` -> `tevő`).
    pub participle: &'static str,
    /// The future participle, where the verb has one.
    ///
    /// Stated per verb because nothing derives it: `tesz` gives `teendő` on the bare stem while
    /// `jön` gives `jövendő` on the `v` stem, and `megy` and `eszik` have none at all. Each comes
    /// from its own rule's nonfinite list.
    ///
    /// Rules: `VRB-PART-003`, `VRB-IRR-001` .. `VRB-IRR-010`
    pub future_participle: &'static str,

    /// Whether the verb has a definite conjugation at all.
    ///
    /// `van`, `lesz`, `megy` and `jön` are intransitive, so their definite cells and `-lAk` do not
    /// exist and must be refused rather than invented.
    pub definite: bool,
}

impl Irregular {
    /// Every distinct stem this verb inflects on.
    ///
    /// Analysis needs these: the stem index keys on the lemma, and no prefix of `ettem` resembles
    /// `eszik`, so without them a suppletive verb can be generated but never recognised.
    pub fn stems(&self) -> Vec<&'static str> {
        let mut out = vec![
            self.present,
            self.past_3sg,
            self.past,
            self.bound,
            self.conditional,
            self.converb,
            self.imperative,
            self.imperative_2sg,
            self.potential,
            self.participle,
        ];
        if let Some(forms) = self.present_forms {
            out.extend_from_slice(&forms);
        }
        out.retain(|s| !s.is_empty());
        out.sort_unstable();
        out.dedup();
        out
    }
}

/// Every suppletive verb, read from the inventory's rows in the column order it documents.
///
/// Rules: `VRB-IRR-001` .. `VRB-IRR-010`
static TABLE: LazyLock<Vec<Irregular>> = LazyLock::new(|| {
    rows(Role::IrregularVerb)
        .iter()
        .map(|r| {
            let s = |i: usize| -> &'static str { &r[i] };
            let present_forms = (!r[2].is_empty()).then(|| {
                let forms: Vec<&'static str> = r[2].split(' ').collect();
                forms
                    .try_into()
                    .unwrap_or_else(|_| panic!("{}: six present forms", r[0]))
            });
            Irregular {
                lemma: s(0),
                present: s(1),
                present_forms,
                past_3sg: s(3),
                past: s(4),
                bound: s(5),
                converb: s(6),
                conditional: s(7),
                imperative: s(8),
                imperative_2sg: s(9),
                imperative_2sg_def: s(10),
                potential: s(11),
                participle: s(12),
                future_participle: s(13),
                definite: r[14].as_ref() == "true",
            }
        })
        .collect()
});

/// The suppletive paradigm for this lemma, if it has one.
#[must_use]
pub fn irregular(lemma: &str) -> Option<&'static Irregular> {
    TABLE.iter().find(|i| i.lemma == lemma)
}

/// Every listed present indefinite form of that cell, across the suppletive verbs.
///
/// `VRB-IRR-001`, `VRB-IRR-003`, `VRB-IRR-004` write these cells whole, so the surface ends in the
/// form itself and there is no suffix for the analyser to key on (`vagy`, `megy`).
///
/// Rules: `VRB-IRR-001`, `VRB-IRR-003`, `VRB-IRR-004`
#[must_use]
pub fn present_forms(cell: usize) -> Vec<&'static str> {
    TABLE
        .iter()
        .filter_map(|i| i.present_forms.map(|f| f[cell]))
        .filter(|f| !f.is_empty())
        .collect()
}

impl Irregular {
    /// The stem the converb suffix attaches to.
    #[must_use]
    pub fn converb_stem(&self) -> &'static str {
        if self.converb.is_empty() {
            self.bound
        } else {
            self.converb
        }
    }

    /// The stem the conditional marker attaches to.
    #[must_use]
    pub fn conditional_stem(&self) -> &'static str {
        if self.conditional.is_empty() {
            self.bound
        } else {
            self.conditional
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TABLE, irregular};

    #[test]
    fn the_class_is_closed_and_addressable() {
        assert_eq!(TABLE.len(), 10);
        assert_eq!(irregular("tesz").expect("tesz").past_3sg, "tett");
        assert_eq!(irregular("eszik").expect("eszik").past, "ett");
        assert!(irregular("vár").is_none());
    }

    #[test]
    fn intransitives_have_no_definite_conjugation() {
        // VRB-IRR-001: `van` has no definite cells and no -lAk.
        for lemma in ["van", "lesz", "megy", "jön"] {
            assert!(!irregular(lemma).expect(lemma).definite, "{lemma}");
        }
        for lemma in ["tesz", "vesz", "visz", "hisz", "eszik", "iszik"] {
            assert!(irregular(lemma).expect(lemma).definite, "{lemma}");
        }
    }

    #[test]
    fn a_past_3sg_differs_from_the_past_stem_only_for_the_v_verbs() {
        // VRB-PAST-010: `evett` against `ettem`, but `tett` against `tettem`.
        for lemma in ["eszik", "iszik"] {
            let i = irregular(lemma).expect(lemma);
            assert_ne!(i.past_3sg, i.past);
        }
        for lemma in ["tesz", "vesz", "visz", "hisz", "van", "megy", "jön", "lesz"] {
            let i = irregular(lemma).expect(lemma);
            assert_eq!(i.past_3sg, i.past);
        }
    }
}
