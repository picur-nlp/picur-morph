//! Hungarian lexicon schema.
//!
//! Holds only what no rule predicts. Anything derivable from the written or spoken shape of a word
//! belongs in `picur-morph-ortho` or the generation engine, not here.
//!
//! See `docs/design/05-lexicon.md`.

mod entry;
mod inventory;
mod store;
mod traits;

pub use entry::{Entry, Pos};
pub use inventory::{Role, Tables, entries, lists, pairs, rows, words};
pub use store::{Lexicon, frequency, lexicon};
pub use traits::{Trait, TraitSet};
