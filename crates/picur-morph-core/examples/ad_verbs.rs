//! Lists every `-ad`/`-ed` verb in the lexicon, one per line.
//!
//! Input for `tools/measure_past.py`, which measures the `VRB-PAST-008` split. The class is
//! open, so it has to come from the lexicon rather than from a list.

use picur_morph_lexicon::{Pos, lexicon};

#[expect(clippy::print_stdout, reason = "the output is the point")]
fn main() {
    for e in lexicon().iter() {
        if e.pos == Pos::Verb && (e.lemma.ends_with("ad") || e.lemma.ends_with("ed")) {
            println!("{}", e.lemma);
        }
    }
}
