//! Offline tooling: data conversion, golden extraction, rulebook checks.

use std::error::Error;
use std::path::Path;
use std::process::ExitCode;

mod accept;
mod analyse;
mod coverage;
mod crawl;
mod eval;
mod frequency;
mod gold;
mod goldens;
mod ledger;
mod lint_rules;
mod lint_text;
mod morphdb;
mod punct;
mod rank;
mod roundtrip;
mod suggest;

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

const USAGE: &str = "usage: cargo xtask (goldens | lint-rules | coverage | ledger [<status>] | punct | lint-text <file> [<rule>] | rank <file> [<n>] | crawl <file> [<n>] | score <file>... | accept <list> | gold <dir> | analyse <word>... | eval [<chapter>] | roundtrip [<n>] [<pos>] | suggest | morphdb <dir> | frequency [<corpus-dir>]) [<grammar-dir>]";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let flags: Vec<&str> = args.iter().map(String::as_str).collect();
    if let Some(("analyse", words)) = flags.split_first().map(|(c, rest)| (*c, rest))
        && !words.is_empty()
    {
        analyse::report(words);
        return ExitCode::SUCCESS;
    }
    if let Some(("score", files)) = flags.split_first().map(|(c, rest)| (*c, rest))
        && !files.is_empty()
    {
        return finish(rank::score_files(files));
    }
    let result = match flags[..] {
        ["goldens"] => goldens::extract(Path::new("docs/grammar")),
        ["goldens", dir] => goldens::extract(Path::new(dir)),
        ["lint-rules"] => lint_rules::check(Path::new("docs/grammar")),
        ["lint-rules", dir] => lint_rules::check(Path::new(dir)),
        ["coverage"] => coverage::report(Path::new("docs/grammar"), Path::new("crates")),
        ["ledger"] => ledger::report(Path::new("docs/grammar"), Path::new("crates"), None),
        ["ledger", status] => {
            ledger::report(Path::new("docs/grammar"), Path::new("crates"), Some(status))
        }
        ["accept", list] => accept::report(Path::new(list)),
        ["gold", dir] => gold::report(Path::new(dir)),
        ["lint-text", file] => lint_text::report(Path::new(file), None),
        ["lint-text", file, rule] => lint_text::report(Path::new(file), Some(rule)),
        ["crawl", file] => crawl::report(Path::new(file), 6),
        ["crawl", file, n] => n
            .parse()
            .map_err(|e| format!("crawl takes a count: {e}").into())
            .and_then(|n| crawl::report(Path::new(file), n)),
        ["rank", file] => rank::report(Path::new(file), 500),
        ["rank", file, n] => n
            .parse()
            .map_err(|e| format!("rank takes a count: {e}").into())
            .and_then(|n| rank::report(Path::new(file), n)),
        ["punct"] => punct::report(Path::new("docs/grammar")),
        ["punct", dir] => punct::report(Path::new(dir)),
        ["morphdb", dir] => morphdb::convert(Path::new(dir)),
        ["frequency"] => frequency::convert(Path::new("local/corpora")),
        ["frequency", dir] => frequency::convert(Path::new(dir)),
        ["suggest"] => suggest::report(Path::new("docs/grammar/goldens.ron"), None),
        ["suggest", n] => match n.parse() {
            Ok(n) => suggest::report(Path::new("docs/grammar/goldens.ron"), Some(n)),
            Err(_) => Err("suggest takes a count".into()),
        },
        ["roundtrip"] => roundtrip::run(None, None),
        ["roundtrip", n] => roundtrip::run(Some(n), None),
        ["roundtrip", n, pos] => roundtrip::run(Some(n), Some(pos)),
        ["eval"] => eval::run(Path::new("docs/grammar/goldens.ron"), None),
        ["eval", ch] => eval::run(Path::new("docs/grammar/goldens.ron"), Some(ch)),
        _ => Err(USAGE.into()),
    };
    finish(result)
}

/// Turns a task's outcome into an exit code, reporting the error.
#[expect(clippy::print_stderr, reason = "CLI diagnostics")]
fn finish(result: Result) -> ExitCode {
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
