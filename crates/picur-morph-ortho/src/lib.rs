//! Hungarian orthography: letters, syllables, hyphenation, capitalisation, tokens.
//!
//! Every function here is a pure function over text.
//!
//! See `docs/design/01-core-model.md` and `docs/grammar/01-writing-system.md`.

pub mod case;
pub mod encoding;
mod harmony;
mod letter;
pub mod norm;
mod phon;
mod syllable;
mod token;

pub use harmony::{Harmony, harmony_of, pick2, pick3};
pub use letter::{Letter, Span, letters};
pub use phon::{Consonant, Manner, Place, bare_t_class, features, well_formed_final_cluster};
pub use syllable::{hyphenate, hyphenate_at, rejoin, splits_by, syllable_count, syllables};
pub use token::tokens;
