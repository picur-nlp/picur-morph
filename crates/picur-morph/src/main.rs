//! `picur-morph`: the library on the command line.
//!
//! Reads text from files or standard input, writes one line per result, plain or as JSON. There
//! is no configuration file and no environment variable: the flags below are the whole surface.

use std::io::{self, BufRead, BufWriter, Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use picur_morph::core::Role;
use picur_morph::lexicon::lexicon;
use picur_morph::{
    Mode, Status, analyses, check_text, generate, project, quality, repair, segment_surface,
};
use serde_json::{Value, json};

#[derive(Parser)]
#[command(
    name = "picur-morph",
    version,
    about = "Hungarian morphology, spellchecking, repair and ranking"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Spellcheck text: one line per token that is not accepted.
    Check {
        /// Files to read, or standard input when none.
        files: Vec<PathBuf>,
        /// One JSON object per finding.
        #[arg(long)]
        json: bool,
    },
    /// Every reading of each word: lemma, category, features, UD tag and segmentation. A word
    /// with no reading gets the suffix rules' guess, marked as such.
    Analyse {
        /// Words to analyse, or one per line on standard input when none.
        words: Vec<String>,
        /// One JSON object per word.
        #[arg(long)]
        json: bool,
    },
    /// Repair damaged text: encoding, spacing, confusions, accents.
    Repair {
        /// Files to read, or standard input when none.
        files: Vec<PathBuf>,
        /// Also modernise historical spelling.
        #[arg(long)]
        modernise: bool,
        /// List the fixes as JSON instead of printing the repaired text.
        #[arg(long)]
        fixes: bool,
    },
    /// Score documents for well-formed Hungarian: one JSON object per document.
    Rank {
        /// Files to read, one document each, or standard input when none.
        files: Vec<PathBuf>,
        /// Treat each input line as a JSON object and score this field.
        #[arg(long, value_name = "FIELD")]
        jsonl: Option<String>,
        /// Worker threads; defaults to the available parallelism.
        #[arg(long)]
        threads: Option<usize>,
    },
    /// Punctuation and typography findings.
    Lint {
        /// Files to read, or standard input when none.
        files: Vec<PathBuf>,
        /// Print the text with every rule-licensed fix applied instead of the findings.
        #[arg(long)]
        fix: bool,
        /// One JSON object per finding.
        #[arg(long)]
        json: bool,
    },
    /// Every form of a lemma the checker would accept, with its feature stack.
    Paradigm {
        /// The lemma, as listed in the lexicon.
        lemma: String,
        /// Restrict to one category, such as `noun` or `verb`.
        #[arg(long)]
        pos: Option<String>,
        /// One JSON object per form.
        #[arg(long)]
        json: bool,
    },
}

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;

#[expect(clippy::print_stderr, reason = "CLI diagnostics")]
fn main() -> ExitCode {
    match run(Cli::parse().command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e)
            if e.downcast_ref::<io::Error>()
                .is_some_and(|e| e.kind() == io::ErrorKind::BrokenPipe) =>
        {
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("picur: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(command: Command) -> Result {
    let mut out = BufWriter::new(io::stdout().lock());
    match command {
        Command::Check { files, json } => check_cmd(&mut out, &files, json)?,
        Command::Analyse { words, json } => analyse_cmd(&mut out, words, json)?,
        Command::Repair {
            files,
            modernise,
            fixes,
        } => repair_cmd(&mut out, &files, modernise, fixes)?,
        Command::Rank {
            files,
            jsonl,
            threads,
        } => {
            let threads = threads
                .or_else(|| std::thread::available_parallelism().ok().map(usize::from))
                .unwrap_or(1)
                .max(1);
            match jsonl {
                Some(field) => rank_jsonl(&mut out, &files, &field, threads)?,
                None => {
                    for (name, text) in inputs(&files)? {
                        let mut q = quality_json(&text);
                        q["file"] = Value::String(name);
                        emit(&mut out, &q)?;
                    }
                }
            }
        }
        Command::Lint { files, fix, json } => lint_cmd(&mut out, &files, fix, json)?,
        Command::Paradigm { lemma, pos, json } => {
            paradigm_cmd(&mut out, &lemma, pos.as_deref(), json)?;
        }
    }
    out.flush()?;
    Ok(())
}

fn check_cmd(out: &mut impl Write, files: &[PathBuf], json: bool) -> Result {
    for (name, text) in inputs(files)? {
        for f in check_text(&text) {
            let status = status_name(f.status);
            if json {
                emit(
                    out,
                    &json!({"file": name, "at": f.at, "word": f.word, "status": status}),
                )?;
            } else {
                writeln!(out, "{name}:{}\t{}\t{status}", f.at, f.word)?;
            }
        }
    }
    Ok(())
}

fn analyse_cmd(out: &mut impl Write, words: Vec<String>, json: bool) -> Result {
    let words = if words.is_empty() {
        stdin_lines()?
    } else {
        words
    };
    for word in words {
        let mut readings: Vec<Value> = analyses(&word)
            .iter()
            .map(|a| reading_json(&word, a))
            .collect();
        let guessed_readings: Vec<Value> = picur_morph::guesses(&word)
            .iter()
            .map(|a| reading_json(&word, a))
            .collect();
        let guessed = !guessed_readings.is_empty();
        if guessed {
            readings = guessed_readings;
        }
        if json {
            emit(
                out,
                &json!({"word": word, "readings": readings, "guessed": guessed && !readings.is_empty()}),
            )?;
            continue;
        }
        writeln!(
            out,
            "{word}{}",
            if guessed && !readings.is_empty() {
                "  (guessed)"
            } else {
                ""
            }
        )?;
        for r in &readings {
            let col = |k: &str| r[k].as_str().unwrap_or("").to_owned();
            writeln!(
                out,
                "  {}\t{}\t{}\t{}\t{}",
                col("lemma"),
                col("pos"),
                col("upos"),
                col("feats"),
                col("segments")
            )?;
        }
    }
    Ok(())
}

fn repair_cmd(out: &mut impl Write, files: &[PathBuf], modernise: bool, list: bool) -> Result {
    let mode = if modernise {
        Mode::Modernise
    } else {
        Mode::Conservative
    };
    for (name, text) in inputs(files)? {
        let found = repair(&text, mode);
        if list {
            for f in &found {
                emit(
                    out,
                    &json!({"file": name, "at": f.at, "from": f.from, "to": f.to, "rule": f.rule}),
                )?;
            }
        } else {
            out.write_all(picur_morph::repair::apply(&text, &found).as_bytes())?;
        }
    }
    Ok(())
}

fn lint_cmd(out: &mut impl Write, files: &[PathBuf], fix: bool, json: bool) -> Result {
    for (name, text) in inputs(files)? {
        if fix {
            out.write_all(picur_morph::lint::normalise(&text).as_bytes())?;
            continue;
        }
        let found = picur_morph::lint::check(&text);
        for d in &found {
            let severity = format!("{:?}", d.severity);
            if json {
                emit(
                    out,
                    &json!({"file": name, "at": d.span.start, "end": d.span.end, "rule": d.rule, "severity": severity, "message": d.message, "fix": d.fix}),
                )?;
            } else {
                writeln!(
                    out,
                    "{name}:{}\t{}\t{severity}\t{}",
                    d.span.start, d.rule, d.message
                )?;
            }
        }
    }
    Ok(())
}

fn paradigm_cmd(out: &mut impl Write, lemma: &str, pos: Option<&str>, json: bool) -> Result {
    let mut entries: Vec<picur_morph::Entry> = lexicon()
        .get(lemma)
        .iter()
        .filter(|e| pos.is_none_or(|p| format!("{:?}", e.pos).eq_ignore_ascii_case(p)))
        .cloned()
        .collect();
    // An unlisted lemma is generated on its category alone, which is what the library does for
    // any caller; the rules that need a lexical fact refuse and the rest apply.
    if entries.is_empty() {
        let Some(p) = pos else {
            return Err(format!(
                "`{lemma}` is not in the lexicon; give --pos to generate it anyway"
            )
            .into());
        };
        let pos = picur_morph::Pos::ALL
            .into_iter()
            .find(|c| format!("{c:?}").eq_ignore_ascii_case(p))
            .ok_or_else(|| format!("unknown category `{p}`"))?;
        entries.push(picur_morph::Entry::new(lemma, pos));
    }
    for entry in &entries {
        let pos = format!("{:?}", entry.pos);
        for stack in picur_morph::spell::stacks().iter() {
            let Ok(form) = generate(entry, stack) else {
                continue;
            };
            let stack_text = format!("{stack:?}");
            if json {
                emit(
                    out,
                    &json!({"lemma": lemma, "pos": pos, "form": form, "stack": stack_text}),
                )?;
            } else {
                writeln!(out, "{form}\t{pos}\t{stack_text}")?;
            }
        }
    }
    Ok(())
}

/// Scores one JSON document per input line, in input order, across `threads` workers.
fn rank_jsonl(out: &mut impl Write, files: &[PathBuf], field: &str, threads: usize) -> Result {
    let lines: Vec<String> = inputs(files)?
        .into_iter()
        .flat_map(|(_, text)| text.lines().map(str::to_owned).collect::<Vec<_>>())
        .collect();
    let chunk = lines.len().div_ceil(threads).max(1);
    let scored: Vec<Vec<Value>> = std::thread::scope(|s| {
        let workers: Vec<_> = lines
            .chunks(chunk)
            .map(|part| {
                s.spawn(move || {
                    part.iter()
                        .map(|line| match serde_json::from_str::<Value>(line) {
                            Ok(doc) => {
                                let id = doc.get("id").cloned().unwrap_or(Value::Null);
                                match doc[field].as_str() {
                                    Some(text) => {
                                        let mut q = quality_json(text);
                                        q["id"] = id;
                                        q
                                    }
                                    None => json!({"error": format!("no field {field}"), "id": id}),
                                }
                            }
                            Err(e) => json!({"error": e.to_string()}),
                        })
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|w| w.join().unwrap_or_else(|e| std::panic::resume_unwind(e)))
            .collect()
    });
    for value in scored.iter().flatten() {
        emit(out, value)?;
    }
    Ok(())
}

/// A document's quality, every field the ranker computes and the single score.
fn quality_json(text: &str) -> Value {
    let q = quality(text);
    json!({
        "score": q.score(),
        "tokens": q.tokens,
        "accepted": q.accepted,
        "rejected": q.rejected,
        "unknown": q.unknown,
        "accent_stripped": q.accent_stripped,
        "mojibake": q.mojibake,
        "ligatures": q.ligatures,
        "damaged": q.is_damaged(),
    })
}

/// One reading: lemma, category, stack, UD projection and surface segmentation.
fn reading_json(word: &str, a: &picur_morph::Analysis) -> Value {
    let ud = project(a);
    let segments = segment_surface(word, a).map(|pieces| {
        pieces
            .iter()
            .map(|p| match &p.role {
                Role::Preverb => format!("{}=", p.text),
                Role::Member => format!("{}|", p.text),
                Role::Affix(m) => format!("{}:{}", p.text, short(*m)),
                _ => p.text.to_string(),
            })
            .collect::<Vec<_>>()
            .join(" + ")
    });
    json!({
        "lemma": a.lemma,
        "pos": format!("{:?}", a.pos),
        "stack": format!("{:?}", a.stack),
        "upos": ud.upos,
        "feats": ud.feats(),
        "segments": segments,
    })
}

/// A morph's name without its fields: `Verbal` for the whole finite cell.
fn short(morph: picur_morph::Morph) -> String {
    let text = format!("{morph:?}");
    text.split([' ', '(']).next().unwrap_or(&text).to_owned()
}

const fn status_name(status: Status) -> &'static str {
    match status {
        Status::Ok => "ok",
        Status::Unknown => "unknown",
        Status::Wrong => "wrong",
        _ => "other",
    }
}

fn emit(out: &mut impl Write, value: &Value) -> Result {
    serde_json::to_writer(&mut *out, value)?;
    out.write_all(b"\n")?;
    Ok(())
}

/// The named inputs: each file, or standard input as `-` when no file is given.
fn inputs(files: &[PathBuf]) -> Result<Vec<(String, String)>> {
    if files.is_empty() {
        let mut text = String::new();
        io::stdin().lock().read_to_string(&mut text)?;
        return Ok(vec![("-".to_owned(), text)]);
    }
    files
        .iter()
        .map(|f| {
            let text = std::fs::read_to_string(f).map_err(|e| format!("{}: {e}", f.display()))?;
            Ok((f.display().to_string(), text))
        })
        .collect()
}

fn stdin_lines() -> Result<Vec<String>> {
    let mut out = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line?;
        let word = line.trim();
        if !word.is_empty() {
            out.push(word.to_owned());
        }
    }
    Ok(out)
}
