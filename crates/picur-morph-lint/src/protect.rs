//! Regions no punctuation rule may touch.
//!
//! `PUN-SPACE-003` exempts internet addresses from the spacing rules, and `PUN-SPACE-006` repeats
//! the exemption for URLs, e-mail addresses and code. Those are the only regions that need marking:
//! the remaining exemptions (times, decimals, version numbers) fall out of the rules themselves,
//! which require letter context and so never look at a digit-punctuation-digit run.

/// Byte ranges that no rule may report inside.
///
/// Rules: `PUN-SPACE-003`, `PUN-SPACE-006`
#[derive(Clone, Debug, Default)]
pub struct Protected(Vec<core::ops::Range<usize>>);

impl Protected {
    /// Finds the protected regions of `text`.
    #[must_use]
    pub fn of(text: &str) -> Self {
        let mut out = Vec::new();
        let mut at = 0;
        while at < text.len() {
            let rest = &text[at..];
            let Some(start) = rest.find(|c: char| !c.is_whitespace()) else {
                break;
            };
            let from = at + start;
            let len = text[from..]
                .find(char::is_whitespace)
                .unwrap_or(text.len() - from);
            let span = from..from + len;
            if is_address(&text[span.clone()]) {
                out.push(span);
            }
            at = from + len;
        }
        Self(out)
    }

    /// Whether any protected region overlaps `span`.
    #[must_use]
    pub fn covers(&self, span: &core::ops::Range<usize>) -> bool {
        self.0
            .iter()
            .any(|p| p.start < span.end && span.start < p.end)
    }
}

/// Whether a whitespace-delimited token is an internet address or a code identifier.
///
/// Deliberately narrow. A token is exempt only on positive evidence of a scheme, a host, a mailbox
/// or an identifier convention; everything else is ordinary text, because a guard that fires too
/// often silently disables the whole lint.
fn is_address(token: &str) -> bool {
    let bare = token.trim_end_matches(['.', ',', ';', ':', '!', '?', ')', ']', '”', '»']);
    if bare.contains("://") || bare.starts_with("www.") || bare.contains('_') {
        return true;
    }
    // A mailbox needs a dotted host after the `@`, which is what separates `nev@pelda.hu` from an
    // `@` used as a handle or a price marker.
    matches!(bare.split_once('@'), Some((user, host))
        if !user.is_empty() && host.contains('.') && !host.starts_with('.') && !host.ends_with('.'))
}

#[cfg(test)]
mod tests {
    use super::Protected;

    fn protected(text: &str) -> Vec<String> {
        Protected::of(text)
            .0
            .into_iter()
            .map(|r| text[r].to_string())
            .collect()
    }

    #[test]
    fn addresses_are_protected() {
        assert_eq!(protected("Lásd a www.mta.hu oldalt."), ["www.mta.hu"]);
        assert_eq!(protected("Írj a nev@pelda.hu címre."), ["nev@pelda.hu"]);
        assert_eq!(
            protected("Nyisd meg: http://example.hu/oldal ma."),
            ["http://example.hu/oldal"]
        );
        assert_eq!(protected("A snake_case nevű."), ["snake_case"]);
    }

    #[test]
    fn ordinary_text_is_not_protected() {
        // A guard that swallowed these would disable the lint on the text it exists to check.
        for text in [
            "Ez egy mondat.",
            "Kedves Péter, szia!",
            "A 15. §-ban.",
            "Kb. 20-30 fő.",
            "Az e-mail cím kell.",
        ] {
            assert!(protected(text).is_empty(), "{text}");
        }
    }

    #[test]
    fn a_trailing_sentence_period_does_not_defeat_the_guard() {
        // PUN-SPACE-003: the address keeps its exemption when a sentence ends on it.
        assert_eq!(protected("Nézd meg a www.mta.hu."), ["www.mta.hu."]);
    }
}
