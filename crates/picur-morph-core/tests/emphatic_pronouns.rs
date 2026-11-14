//! The emphatic doubled pronoun forms of `PND-PERS-007`.

use picur_morph_core::pronoun::{Emphatic, emphatic};

#[test]
fn the_rulebook_forms_are_read() {
    // PND-PERS-007: the nominative pronoun before the case-stem form of the same person.
    for word in [
        "énnekem",
        "teneked",
        "őneki",
        "minekünk",
        "tinektek",
        "őnekik",
        "énvelem",
        "ővele",
        "ővelük",
        "énbennem",
        "tenálad",
        "őtőle",
        "mihozzánk",
        "őrá",
        "énrólam",
    ] {
        assert!(
            matches!(emphatic(word), Some(Emphatic::Pronoun(_))),
            "{word}"
        );
    }
    for word in [
        "énmiattam",
        "teáltalad",
        "őutána",
        "mimellettünk",
        "tinélkületek",
        "őközöttük",
    ] {
        assert!(
            matches!(emphatic(word), Some(Emphatic::Postposition(_))),
            "{word}"
        );
    }
}

#[test]
fn the_exceptions_are_refused() {
    // PND-PERS-007: `ők` prefixes nothing, nothing assimilates or harmonises across the
    // boundary, and the accusative has no emphatic form.
    for word in [
        "őkvelük",
        "énnelem",
        "őrőlük",
        "énengem",
        "teténged",
        "ővel",
        "énnekünk",
        "nekem",
    ] {
        assert_eq!(emphatic(word), None, "{word}");
    }
}
