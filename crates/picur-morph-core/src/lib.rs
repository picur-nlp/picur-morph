//! Hungarian morphological generation and analysis.
//!
//! Generation is the single source of truth; analysis is search over it, verified by regenerating
//! and comparing. See `docs/design/02-generation.md` and `docs/design/03-analysis.md`.

pub mod acronym;
pub mod alternation;
pub mod analysis;
pub mod article;
pub mod compound;
pub mod derivation;
pub mod features;
pub mod generate;
pub mod imperative;
pub mod irregular;
mod memo;
pub mod numeral;
pub mod possessive;
pub mod postposition;
pub mod pronoun;
pub mod segment;
pub mod stem;
pub mod suffix;
pub mod ud;
pub mod verbal;

pub use acronym::{Reading, licenses, licenses_nominal, reading};
pub use alternation::{alternant, syncopated};
pub use analysis::{
    Analysis, Stacks, Verdict, accent_confusable, analyse, analyse_with, forms, guess, is_known,
    is_vouched, stable_prefix,
};
pub use article::{Article, article};
pub use compound::{Member, Split, member_prefixes, split};
pub use derivation::{Der, DerId, Link, Prod, derive, derived};
pub use features::{Case, Definite, Mood, Morph, Number, Person, Tense, UdCase};
pub use generate::{
    GenError, attach, generate, generate_compound, generate_seam, generate_toward, harmony,
    lengthen_final,
};
pub use imperative::imperative_stem;
pub use irregular::{Irregular, irregular};
pub use numeral::{cardinal, ordinal, year};
pub use possessive::{possessive, takes_j};
pub use postposition::PostpositionForm;
pub use pronoun::{PronounForm, oblique, possessive_pronoun};
pub use segment::{Piece, Role, segment, segment_reading, segment_surface};
pub use stem::{Coda, coda, linking_vowel, needs_link};
pub use suffix::{Arch, Seg, Suffix, SuffixClass, case_suffix};
pub use ud::{Ud, project};
pub use verbal::{heavy, present_indefinite, present_stem};
