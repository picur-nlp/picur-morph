//! Memoising a pure function of a string that sits under the checker's hot path.

use std::collections::HashMap;
use std::sync::RwLock;

/// A shared, unbounded table from a string key to a computed value.
///
/// Meant for functions of a lemma, whose domain is the finite lexicon. A poisoned lock is treated
/// as a miss, so the table can only ever slow a caller down, never change an answer.
#[derive(Debug, Default)]
pub(crate) struct Memo<V>(RwLock<HashMap<Box<str>, V>>);

impl<V: Clone> Memo<V> {
    /// The value for `key`, computing and storing it on a miss.
    pub(crate) fn get_or_compute(&self, key: &str, compute: impl FnOnce() -> V) -> V {
        if let Ok(m) = self.0.read()
            && let Some(hit) = m.get(key)
        {
            return hit.clone();
        }
        let value = compute();
        if let Ok(mut m) = self.0.write() {
            m.insert(key.into(), value.clone());
        }
        value
    }
}
