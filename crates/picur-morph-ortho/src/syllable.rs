//! Hungarian syllabification and hyphenation.
//!
//! Rules: PH-SYL-001, PH-SYL-002, ORT-SPLIT-001, ORT-SPLIT-002, ORT-SPLIT-003, ORT-SPLIT-005,
//! ORT-SPLIT-006, ORT-SPLIT-007, ORT-SPLIT-008, ORT-SPLIT-009, ORT-SPLIT-015, ORT-SPLIT-020.

use crate::letter::{Letter, Off, raw_spans, raw_spans_seamed, vowel_char};

/// Token-index boundaries of `spans`' syllables: `[0, b1, b2, ..., spans.len()]`.
///
/// Every vowel is the nucleus of exactly one syllable (`PH-SYL-001`). Consonants between two
/// nuclei split so that only the last one opens the next syllable; the rest close the previous one
/// (`ka-sza`, `ab-lak`, `temp-lom`: `ORT-SPLIT-003`, `ORT-SPLIT-005`, `ORT-SPLIT-008`,
/// `PH-SYL-002`). Leading consonants join the first syllable, trailing consonants the last.
fn syllable_boundaries(spans: &[(Letter, Off, Off)]) -> Vec<usize> {
    let nuclei: Vec<usize> = spans
        .iter()
        .enumerate()
        .filter_map(|(i, (l, ..))| l.is_vowel().then_some(i))
        .collect();

    if nuclei.is_empty() {
        return vec![0, spans.len()];
    }

    let mut boundaries = vec![0];
    for pair in nuclei.windows(2) {
        let (prev_nucleus, next_nucleus) = (pair[0], pair[1]);
        let cluster_len = next_nucleus - prev_nucleus - 1;
        // Only the last consonant of the cluster opens the next syllable (0 if the vowels are
        // adjacent); everything before it closes the previous one.
        boundaries.push(next_nucleus - cluster_len.min(1));
    }
    boundaries.push(spans.len());
    boundaries
}

/// The number of syllables in `word`.
///
/// One per vowel: Hungarian has no syllabic consonants and no diphthongs, so every syllable has
/// exactly one vowel as its nucleus, and every vowel is a single character. Counting them needs
/// neither the boundaries [`syllables`] computes nor any segmentation.
///
/// Rules: PH-SYL-001.
#[must_use]
pub fn syllable_count(word: &str) -> usize {
    word.chars().filter(|&c| vowel_char(c).is_some()).count()
}

/// Splits `word` into syllables.
///
/// One syllable per vowel; a single intervocalic consonant joins the following vowel (`ka-sza`,
/// `ORT-SPLIT-003`); in a consonant cluster only the last consonant joins the following vowel,
/// the rest close the preceding syllable (`ab-lak`, `temp-lom`: `ORT-SPLIT-005`, `ORT-SPLIT-008`,
/// `PH-SYL-002`). A word with no vowels is returned as a single piece.
///
/// The returned pieces are byte-range slices of `word` and concatenate back to it exactly; unlike
/// [`hyphenate`], a simplified doubled digraph that happens to split is left in its written
/// (simplified) form, e.g. `asszony` -> `["as", "szony"]`.
///
/// Rules: PH-SYL-001, PH-SYL-002, ORT-SPLIT-001, ORT-SPLIT-003, ORT-SPLIT-005, ORT-SPLIT-008.
#[must_use]
pub fn syllables(word: &str) -> Vec<&str> {
    let spans = raw_spans(word);
    let boundaries = syllable_boundaries(&spans);
    boundaries
        .windows(2)
        .map(|w| {
            let (start, end) = (w[0], w[1]);
            &word[spans[start].1 as usize..spans[end - 1].2 as usize]
        })
        .collect()
}

/// Whether the token at `spans[i]` is the truncated first half of a simplified doubled digraph
/// whose full form opens the next syllable (`WS-DIG-002`).
fn is_split_doubled_digraph(spans: &[(Letter, Off, Off)], i: usize) -> bool {
    let (letter, start, end) = spans[i];
    if (end - start) as usize >= letter.as_str().len() {
        return false;
    }
    spans
        .get(i + 1)
        .is_some_and(|&(next_letter, next_start, next_end)| {
            next_letter == letter && (next_end - next_start) as usize == letter.as_str().len()
        })
}

/// Hyphenates `word` at its syllable boundaries (`ORT-SPLIT-001`).
///
/// Follows the same syllable boundaries as [`syllables`], except for `ORT-SPLIT-007`: when the
/// break falls inside a simplified doubled digraph, both halves are written out in full
/// (`asszony` -> `["asz", "szony"]`, `loccsan` -> `["locs", "csan"]`, `eddzük` -> `["edz",
/// "dzük"]`). The pieces this produces are **not** a valid re-spelling of the word when rejoined
/// without hyphens; only the simplified original is (`ORT-SPLIT-007` Interactions).
///
/// Rules: ORT-SPLIT-001, ORT-SPLIT-007.
#[must_use]
pub fn hyphenate(word: &str) -> Vec<String> {
    hyphenate_at(word, &[])
}

/// Rejoins a word broken at a line end, undoing the doubling that hyphenation wrote out
/// (`WS-DIG-012`).
///
/// `left` is the piece before the break, hyphen included; `right` the piece after it. The
/// candidates come back in rule order: the simplified spelling first, which is what a suffix
/// boundary or a stem-internal split needs (`asz-` plus `szony` is `asszony`), then the full
/// spelling, which is what a compound boundary needs (`kulcs-` plus `csomó` is `kulcscsomó`).
/// Only morphology tells the two apart, so both are offered and the caller chooses by lexicon.
///
/// A break that did not split a doubled multi-character letter has one candidate.
///
/// Rules: WS-DIG-012, ORT-SPLIT-007.
#[must_use]
pub fn rejoin(left: &str, right: &str) -> Vec<String> {
    let stem = left.strip_suffix('-').unwrap_or(left);
    let full = format!("{stem}{right}");
    let Some((letter, _, end)) = raw_spans(right).first().copied() else {
        return vec![full];
    };
    let spelt = letter.as_str();
    if spelt.len() == 1 || end as usize != spelt.len() || !stem.to_lowercase().ends_with(spelt) {
        return vec![full];
    }
    let keep = stem.len() - spelt.len() + 1;
    let simplified = format!("{}{right}", &stem[..keep]);
    if simplified == full {
        vec![full]
    } else {
        vec![simplified, full]
    }
}

/// Whether every break in `split` (`ka-to-na`, `holna-pig`, `asz-szony`) is a break one of the
/// hyphenations `hyphenate` gives allows in the word it spells.
///
/// A break may merge syllables (`holna-pig`), and a doubled digraph written out at a break is
/// undone to find the word (`WS-DIG-012`). The hyphenator is a parameter because a seam break
/// (`ORT-SPLIT-015`, `ház-sor`) needs the morphology this crate does not have.
///
/// Rules: ORT-SPLIT-001, ORT-SPLIT-003, ORT-SPLIT-007.
#[must_use]
pub fn splits_by(split: &str, hyphenate: impl Fn(&str) -> Vec<Vec<String>>) -> bool {
    let pieces: Vec<&str> = split.split('-').collect();
    if pieces.len() < 2 || pieces.iter().any(|p| p.is_empty()) {
        return false;
    }
    let words = pieces[1..]
        .iter()
        .fold(vec![pieces[0].to_owned()], |words, p| {
            words
                .iter()
                .flat_map(|w| rejoin(&format!("{w}-"), p))
                .collect()
        });
    words
        .iter()
        .any(|w| hyphenate(w).iter().any(|h| merges_into(h, &pieces)))
}

/// Whether consecutive runs of `syllables`, each rejoined, give exactly `pieces`.
fn merges_into(syllables: &[String], pieces: &[&str]) -> bool {
    let mut it = syllables.iter();
    pieces.iter().all(|want| {
        let Some(first) = it.next() else {
            return false;
        };
        let mut got = first.clone();
        while got.len() < want.len() {
            let Some(next) = it.next() else {
                return false;
            };
            got = rejoin(&format!("{got}-"), next).swap_remove(0);
        }
        got == *want
    }) && it.next().is_none()
}

/// `spans` with each `c` + `h` that no seam divides read as one consonant (`ORT-SPLIT-010`):
/// `ar-chívum`, `almana-chot`, and long `cch` as `c` + `ch` (`Bac-chus`). The alphabet has no
/// `ch`; only line breaking counts it as one letter.
///
/// Rules: ORT-SPLIT-010.
fn with_ch(spans: &[(Letter, Off, Off)], seams: &[usize]) -> Vec<(Letter, Off, Off)> {
    let mut out: Vec<(Letter, Off, Off)> = Vec::with_capacity(spans.len());
    for &(letter, start, end) in spans {
        if letter == Letter::H
            && let Some(last) = out.last_mut()
            && last.0 == Letter::C
            && last.2 == start
            && last.2 - last.1 == 1
            && !seams.contains(&(start as usize))
        {
            last.2 = end;
            continue;
        }
        out.push((letter, start, end));
    }
    out
}

/// Hyphenates `word`, breaking at each morpheme seam as well as at the syllable boundaries.
///
/// `ORT-SPLIT-015` breaks a compound at its member boundary even where that is not a syllable
/// boundary, so `ház` plus `sor` is `ház-sor` and not `há-zsor`; `ORT-SPLIT-020` is why that is
/// two letters and not a `zs` in the first place, and the same holds at a stem-suffix junction
/// (`köz-ség`, not `kö-zség`). Without the seams a purely phonological split has no way to know.
///
/// `seams` are byte offsets into `word`. An offset that does not fall on a letter boundary is
/// ignored, so a caller may pass what it has.
///
/// Rules: ORT-SPLIT-001, ORT-SPLIT-007, ORT-SPLIT-015, ORT-SPLIT-020.
#[must_use]
pub fn hyphenate_at(word: &str, seams: &[usize]) -> Vec<String> {
    let spans = with_ch(&raw_spans_seamed(word, seams), seams);
    let mut boundaries = syllable_boundaries(&spans);
    // A seam moves the break between the two vowels around it, which is what makes `ház-sor` and
    // `meg-ér` come out at all. Each piece keeps one vowel, so a seam outside the first and last
    // vowel moves nothing (`vas-út`, not `va-s-ú-t`).
    let nuclei: Vec<usize> = spans
        .iter()
        .enumerate()
        .filter_map(|(i, (l, ..))| l.is_vowel().then_some(i))
        .collect();
    for &b in seams {
        let Some(i) = spans.iter().position(|&(_, start, _)| start as usize == b) else {
            continue;
        };
        if let Some(k) = nuclei.windows(2).position(|w| w[0] < i && i <= w[1]) {
            boundaries[k + 1] = i;
        }
    }
    boundaries
        .windows(2)
        .map(|w| {
            let (start, end) = (w[0], w[1]);
            let mut piece = String::new();
            for i in start..end {
                let (letter, b0, b1) = spans[i];
                if i == end - 1 && is_split_doubled_digraph(&spans, i) {
                    piece.push_str(letter.as_str());
                } else {
                    piece.push_str(&word[b0 as usize..b1 as usize]);
                }
            }
            piece
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{hyphenate, hyphenate_at, splits_by, syllables};

    #[test]
    fn ort_split_001_checks_each_break() {
        let legal = |s| splits_by(s, |w| vec![hyphenate(w)]);
        assert!(legal("ka-to-na"));
        assert!(legal("holna-pig"));
        assert!(legal("asz-szony"));
        assert!(!legal("kat-ona"));
        assert!(!legal("katona"));
        assert!(legal("ar-chívum"));
        assert!(legal("Bac-chus"));
        assert!(legal("pech-hel"));
    }

    #[test]
    fn ort_split_015_and_020_break_at_a_seam() {
        // `ház` + `sor`: the `z`+`s` is not a `zs`, and the member boundary is the break.
        assert_eq!(hyphenate_at("házsor", &[4]), vec!["ház", "sor"]);
        // `köz` + `ség` at a stem-suffix junction, which ORT-SPLIT-009 otherwise ignores.
        assert_eq!(hyphenate_at("község", &[4]), vec!["köz", "ség"]);
        // Without the seam the same string is read with its digraph and splits phonologically.
        assert_eq!(hyphenate("házsor"), vec!["há", "zsor"]);
        // A seam that is not a letter boundary is ignored rather than trusted.
        assert_eq!(hyphenate_at("kasza", &[99]), vec!["ka", "sza"]);
    }

    #[test]
    fn ort_split_002_one_vowel_is_not_split() {
        for w in ["föld", "hajt", "nyolc", "strand", "sztrájk"] {
            assert_eq!(hyphenate(w), vec![w.to_owned()], "{w}");
        }
    }

    #[test]
    fn ort_split_006_a_doubled_letter_splits_in_the_middle() {
        assert_eq!(syllables("tonna"), vec!["ton", "na"]);
        assert_eq!(syllables("hallak"), vec!["hal", "lak"]);
        assert_eq!(syllables("jobban"), vec!["job", "ban"]);
    }

    #[test]
    fn ort_split_003_single_consonant_joins_next_vowel() {
        assert_eq!(syllables("kasza"), vec!["ka", "sza"]);
        assert_eq!(syllables("betű"), vec!["be", "tű"]);
        assert_eq!(syllables("rózsa"), vec!["ró", "zsa"]);
        assert_eq!(syllables("golyó"), vec!["go", "lyó"]);
    }

    #[test]
    fn ort_split_005_two_different_consonants_split_between() {
        assert_eq!(syllables("ablak"), vec!["ab", "lak"]);
        assert_eq!(syllables("amper"), vec!["am", "per"]);
        assert_eq!(syllables("morzsa"), vec!["mor", "zsa"]);
    }

    #[test]
    fn ort_split_008_three_or_more_consonants_only_the_last_moves() {
        assert_eq!(syllables("templom"), vec!["temp", "lom"]);
        assert_eq!(syllables("centrum"), vec!["cent", "rum"]);
        assert_eq!(syllables("ostrom"), vec!["ost", "rom"]);
    }

    #[test]
    fn ph_syl_002_onsetless_syllables() {
        assert_eq!(syllables("tea"), vec!["te", "a"]);
        assert_eq!(syllables("ideális"), vec!["i", "de", "á", "lis"]);
    }

    #[test]
    fn multi_vowel_words_split_at_every_nucleus() {
        assert_eq!(syllables("katona"), vec!["ka", "to", "na"]);
        assert_eq!(syllables("magányos"), vec!["ma", "gá", "nyos"]);
    }

    #[test]
    fn ws_dig_002_simplified_doubling_splits_as_written() {
        // syllables() does not restore the full digraph, unlike hyphenate() (ORT-SPLIT-007).
        assert_eq!(syllables("asszony"), vec!["as", "szony"]);
        assert_eq!(syllables("loccsan"), vec!["loc", "csan"]);
    }

    #[test]
    fn syllables_reconstruct_the_original_word() {
        for w in [
            "kasza",
            "ablak",
            "templom",
            "tea",
            "asszony",
            "magányos",
            "híd",
        ] {
            assert_eq!(syllables(w).concat(), w, "{w}");
        }
    }

    #[test]
    fn one_vowel_words_are_not_split() {
        assert_eq!(syllables("híd"), vec!["híd"]);
        assert_eq!(syllables("font"), vec!["font"]);
    }

    #[test]
    fn no_vowels_is_a_single_piece() {
        assert_eq!(syllables("brr"), vec!["brr"]);
    }

    #[test]
    fn ort_split_007_doubled_digraph_splits_full_on_both_sides() {
        assert_eq!(hyphenate("asszony"), vec!["asz", "szony"]);
        assert_eq!(hyphenate("loccsan"), vec!["locs", "csan"]);
        assert_eq!(hyphenate("eddzük"), vec!["edz", "dzük"]);
        assert_eq!(hyphenate("meggyel"), vec!["megy", "gyel"]);
    }

    #[test]
    fn ort_split_007_more_examples() {
        assert_eq!(hyphenate("mennyi"), vec!["meny", "nyi"]);
        assert_eq!(hyphenate("ággyal"), vec!["ágy", "gyal"]);
        assert_eq!(hyphenate("könnyes"), vec!["köny", "nyes"]);
    }

    #[test]
    fn ort_split_002_one_vowel_words_are_never_hyphenated() {
        // meggy has a single vowel, so it is not split at all (ORT-SPLIT-002), even though it
        // contains a simplified doubled digraph.
        assert_eq!(hyphenate("meggy"), vec!["meggy"]);
    }

    #[test]
    fn hyphenate_matches_syllables_when_nothing_is_doubled() {
        for w in ["kasza", "ablak", "templom", "tea", "katona"] {
            assert_eq!(hyphenate(w), syllables(w));
        }
    }

    #[test]
    fn ws_dig_012_rejoining_offers_the_simplified_form_first() {
        use super::rejoin;
        for (left, right, want) in [
            ("asz-", "szony", "asszony"),
            ("kulcs-", "csal", "kulccsal"),
            ("Arany-", "nyal", "Arannyal"),
            ("edz-", "dze", "eddze"),
            ("bridzs-", "dzsel", "briddzsel"),
        ] {
            assert_eq!(rejoin(left, right).first().map(String::as_str), Some(want));
        }
    }

    #[test]
    fn ws_dig_012_a_compound_boundary_keeps_the_full_doubling() {
        use super::rejoin;
        for (left, right, want) in [
            ("kulcs-", "csomó", "kulcscsomó"),
            ("jegy-", "gyűrű", "jegygyűrű"),
        ] {
            assert!(rejoin(left, right).contains(&want.to_owned()), "{want}");
        }
    }

    #[test]
    fn ws_dig_012_an_ordinary_break_has_one_candidate() {
        use super::rejoin;
        assert_eq!(rejoin("ab-", "lak"), vec!["ablak".to_owned()]);
        assert_eq!(rejoin("ka-", "lap"), vec!["kalap".to_owned()]);
    }

    #[test]
    fn ws_dig_012_inverts_hyphenation() {
        use super::{hyphenate, rejoin};
        for word in ["asszony", "meggyes", "kalapos", "hosszú"] {
            let pieces = hyphenate(word);
            for i in 1..pieces.len() {
                let left = format!("{}-", pieces[..i].concat());
                let right = pieces[i..].concat();
                assert!(
                    rejoin(&left, &right).contains(&word.to_owned()),
                    "{word} split as {left}|{right}"
                );
            }
        }
    }
}
