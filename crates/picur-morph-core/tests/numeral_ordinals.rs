//! Ordinals read back from their spelling.
//!
//! `PND-ORD-001`, `PND-CARD-005`.

use picur_morph_core::numeral;

#[test]
fn an_ordinal_is_read_back_from_its_spelling() {
    // PND-ORD-001: the ordinals are unbounded, so the checker inverts the spelling instead of
    // storing a sample of it.
    for (word, n) in [
        ("ezeregyedik", 1001u64),
        ("hatmilliomodik", 6_000_000),
        ("milliomodik", 1_000_000),
        ("négyezer-hatszázötödik", 4605),
        ("nyolcszázkilencvenhatodik", 896),
    ] {
        assert_eq!(numeral::parse_ordinal(word), Some(n), "{word}");
    }
    assert_eq!(numeral::parse_ordinal("harmadika"), None);
    assert_eq!(numeral::parse_ordinal("házadik"), None);
    assert_eq!(numeral::parse_ordinal("húszegyedik"), None);
}
