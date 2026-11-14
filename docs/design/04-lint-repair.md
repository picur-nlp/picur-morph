# Repair, lint and quality

Three crates read whole texts. They differ in intent: `repair` fixes damage done to a text
(encoding, OCR, layout, keyboard, pre-reform spelling), `lint` reports typography the author
wrote, and `rank` scores how much of a document the checker can vouch for. A tool that silently
"fixes" a grammatical choice is useless as a source of truth, so none of them touches grammar.

## Repair

```rust
pub enum Mode { Conservative, Modernise }
pub struct Fix { pub at: usize, pub from: Box<str>, pub to: Box<str>, pub rule: &'static str }

pub fn repair(text: &str, mode: Mode) -> Vec<Fix>;   // in text order, never overlapping
pub fn apply(text: &str, fixes: &[Fix]) -> String;
```

Every rule of chapter 10 is licensed the same way: a rewrite is applied only when the original is
rejected and the result is a listed word or a form of one. The checker is the oracle, so a repair
can never invent a word, and a result accepted only as a possible name or a coined compound does
not count. An all-capital token is judged by the word it spells, because the checker accepts any
capital string as an acronym. `Conservative` repairs damage nobody chose: encoding, in every
reading of a code page as another and of UTF-8 as a page (`WS-ENC`, `REP-ENC`), glyphs that stand
for a letter without being one, from ligatures and dead keys to the dotless-i font shift and
mixed-script homoglyphs (`WS-CHAR`, `REP-FONT`, `REP-KEY-006`), keyboard layout and case slips
(`REP-KEY`), OCR confusions (`REP-OCR`), split and fused words (`REP-SPACE`, `REP-OCR-002`). Every
substitution table these mechanisms consult (archaic spellings, assimilation errors, confusions,
font shifts, code pages, homoglyphs, the seven-bit digits) is a list in [`crates/picur-morph-repair/data/glyphs.ron`](../../crates/picur-morph-repair/data/glyphs.ron), each
citing its rule; the crate holds the mechanisms only.
`Modernise` adds the choices a text made under an older norm: pre-reform spelling (`REP-ARCH`),
hyphens on names (`REP-SPELL-012`), length and assimilation spellings (`REP-SPELL`), and accent
restoration (`REP-DIAC`) where the text as a whole is accent-stripped.

Three halves of rules are deliberately absent, each measured on non-Hungarian and fragmentary text
before it was left out: the transposition of `REP-KEY-007` (one edit from most short tokens), the
telegram doubling of `REP-ENC-014` (`been` reads as `bén`), and a fusion cut with the function
word on either side (`node` as `no de`); the fusion that is in requires the function word on the
left and an unattested fused string. A name the lexicon does not list is never resuffixed
(`REP-SPELL-006`), because nothing can regenerate its cell.

Evidence is decided at the right level. Whether a text carries its accents is a property of the
text, decided once; whether a token is a name is the checker's `Unknown` on a capitalised word, and
a name is never half of a split word nor a layout slip. Tokens that look like code are guarded
(`REP-DIAC-009`), and an e-mail or web address is not a token at all.

## Lint

```rust
pub enum Severity { Error, Tendency }
pub struct Diagnostic { span, rule, message, severity, fix: Option<String> }

pub fn check(text: &str) -> Vec<Diagnostic>;
pub fn apply(text: &str, diagnostics: &[Diagnostic]) -> String;
pub fn normalise(text: &str) -> String;              // apply every Error with a fix
```

Covers the families of chapter 08 decidable from the character stream alone: spacing around
marks (`PUN-SPACE`), dashes (`PUN-DASH`), quotation marks (`PUN-QUOT`), the ellipsis (`PUN-ELL`),
apostrophes (`PUN-APOS`), number and date formatting (`PUN-NUMF`, `PUN-DATEF`) and the article
before a numeral or an acronym (`PND-ART`). Severity mirrors the rule's `Confidence`: a form AkH
states is an `Error` and safe to apply unattended; a measured preference is a `Tendency` and is
reported only. Spans inside code, URLs and quoted material are protected. Findings never overlap.

## Quality

```rust
pub struct Quality { tokens, accepted, rejected, unknown, accent_stripped, mojibake, ligatures }
pub fn quality(text: &str) -> Quality;
impl Quality { pub fn score(self) -> f64 }           // 0.0..=1.0
```

The score is the share of tokens the checker accepts, discounted for the damage the detectors see
(mojibake, ligatures). Accent stripping is reported, not charged: every unaccented word is already
rejected or unknown. Unknown words count in full, because a discount that carried a page of names
would also carry damage; a caller that wants to keep name-dense text reads `unknown` and decides.
The score is not zeroed for short texts; `MIN_TOKENS` is there for a caller that wants a floor.
