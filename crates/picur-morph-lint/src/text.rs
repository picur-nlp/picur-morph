//! The character window every rule scans.

/// A character with the byte range it occupies.
///
/// Rules index this slice by position and emit byte spans, so an accented letter can never be cut
/// in half by a fix.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Ch {
    /// Byte offset of the character.
    pub at: usize,
    /// Byte offset just past the character.
    pub end: usize,
    /// The character itself.
    pub c: char,
}

/// Indexes `text` by character position.
pub(crate) fn scan(text: &str) -> Vec<Ch> {
    text.char_indices()
        .map(|(at, c)| Ch {
            at,
            end: at + c.len_utf8(),
            c,
        })
        .collect()
}

/// The character at `i`, or `None` past either end.
pub(crate) fn at(chars: &[Ch], i: usize) -> Option<char> {
    chars.get(i).map(|c| c.c)
}

/// Whether the character at `i` satisfies `f`.
pub(crate) fn is(chars: &[Ch], i: usize, f: impl Fn(char) -> bool) -> bool {
    at(chars, i).is_some_and(f)
}

/// A space that separates words: the ASCII space, the tab and the no-break variants.
///
/// Newlines are excluded throughout: a line break is layout, and collapsing one would reflow the
/// text rather than punctuate it.
pub(crate) fn is_space(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\u{a0}' | '\u{202f}' | '\u{2009}')
}

/// The end of a run of spaces starting at `i`.
pub(crate) fn skip_spaces(chars: &[Ch], i: usize) -> usize {
    let mut j = i;
    while is(chars, j, is_space) {
        j += 1;
    }
    j
}

/// The start of the run of spaces ending just before `i`.
pub(crate) fn back_spaces(chars: &[Ch], i: usize) -> usize {
    let mut j = i;
    while j > 0 && is(chars, j - 1, is_space) {
        j -= 1;
    }
    j
}

/// An empty span at the start of the character at `i`, where a fix inserts text before it.
///
/// `span(chars, i, i)` looks like it means this but computes the end from `i - 1`, which only
/// coincides because characters are contiguous. This states the intent.
pub(crate) fn insert_before(chars: &[Ch], i: usize) -> core::ops::Range<usize> {
    let at = chars
        .get(i)
        .map_or_else(|| chars.last().map_or(0, |c| c.end), |c| c.at);
    at..at
}

/// The byte span covered by character positions `start..end`.
pub(crate) fn span(chars: &[Ch], start: usize, end: usize) -> core::ops::Range<usize> {
    let from = chars.get(start).map_or(0, |c| c.at);
    let to = if end == 0 {
        from
    } else {
        chars.get(end - 1).map_or(from, |c| c.end)
    };
    from..to
}
