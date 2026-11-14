//! Total readings over a corpus sample, for comparing a change that prunes the search.
use picur_morph_spell::analyses;

#[test]
#[ignore = "instrumentation"]
fn total_readings() {
    let Ok(path) = std::env::var("CORPUS") else {
        return;
    };
    let text = std::fs::read_to_string(path).expect("readable");
    let words: Vec<&str> = text
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| w.chars().any(char::is_alphabetic))
        .take(20_000)
        .collect();
    let mut total = 0usize;
    let mut with = 0usize;
    for w in &words {
        let n = analyses(w).len();
        total += n;
        with += usize::from(n > 0);
    }
    #[expect(clippy::print_stdout, reason = "instrumentation")]
    {
        println!("{} words: {total} readings, {with} analysable", words.len());
    }
}
