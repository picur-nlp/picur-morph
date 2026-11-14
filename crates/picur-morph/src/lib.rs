//! Hungarian morphology, orthography, spellchecking, text repair and document ranking.
//!
//! The one dependency to declare. Each area is a module that re-exports the crate implementing
//! it, and the handful of entry points most callers want are at the top level. Everything is
//! compiled in: there is no configuration, no data file and no environment variable to provide.
//!
//! - [`check`] says whether a word is correct, well formed but unlisted, or wrong.
//! - [`analyses`] gives every reading of a word: lemma, category and feature stack.
//! - [`generate`](fn@generate) builds a form from a lexicon entry and a feature stack.
//! - [`repair`](fn@repair) proposes fixes for damaged text, and [`quality`] scores a document.
//!
//! Every function is safe to call from any thread; the library keeps only read-mostly caches.

pub use picur_morph_core as core;
pub use picur_morph_lexicon as lexicon;
pub use picur_morph_lint as lint;
pub use picur_morph_ortho as ortho;
pub use picur_morph_rank as rank;
pub use picur_morph_repair as repair;
pub use picur_morph_spell as spell;

pub use picur_morph_core::{
    Analysis, Morph, Ud, generate, project, segment_reading, segment_surface,
};
pub use picur_morph_lexicon::{Entry, Pos};
pub use picur_morph_rank::{Quality, quality};
pub use picur_morph_repair::{Fix, Mode, repair};
pub use picur_morph_spell::{Finding, Status, analyses, check, check_text, guesses};
