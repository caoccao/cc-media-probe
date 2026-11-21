# CLAUDE.md

Instructions for AI coding agents working in this repository. [AGENTS.md](AGENTS.md) points here; this file is the only copy of the rules.

## The project

`cc-media-probe` is a pure Rust library that inspects multimedia files and reports what they contain: container format, streams, codec parameters, chapters, tags and attachments. It never decodes audio or video. One universal API serves every supported format. The companion crate `cc-media-probe-cli` is the command-line tool. License: Apache-2.0.

**Where things stand:** the development plan, [docs/plans/plan-init-project.md](docs/plans/plan-init-project.md), is approved, and its status line names the current phase. Every gate decision and the progress of each task are in the development log, [docs/plans/plan-init-project-log.md](docs/plans/plan-init-project-log.md). Read both before starting work.

The plan is the authority, and this file summarizes the rules that apply to every change. Section numbers (§), requirement IDs (R…), decision IDs (D…) and gate IDs (G…) below refer to the plan. The plan is long (about 3,000 lines), so read the sections a task touches rather than the whole file. If this file and the plan disagree, stop and ask.

## Working rules

```mermaid
flowchart TD
  T["Task"] --> R["Read the plan sections it touches"]
  R --> D{"Does it need a decision<br/>that is not confirmed yet?"}
  D -->|yes| A["Stop at the gate: present the proposal<br/>and its alternatives, wait for the answer"]
  A --> R
  D -->|no| W["Make the change with its tests and docs"]
  W --> C["node scripts/check.ts passes"]
  C --> L["Leave the change uncommitted and report"]
  L --> Q{"Does the maintainer ask<br/>for a commit?"}
  Q -->|yes| CM["Commit to main"]
  Q -->|no| DN["Done"]
```

- **No commits, pushes, branches or sub-agents unless the maintainer asks for them in the current session** (R10). Leave changes uncommitted for review. A request in one session does not carry over to the next.
- **Gates** (§1.1; all of them are listed in §14):
  - **[CONFIRM]**: present the proposal and its alternatives, then wait for an explicit answer.
  - **[MANUAL]**: give exact instructions, then wait until the maintainer reports the step done.
  - **[OPTIONAL]**: never blocks; the next phase-gate summary records the outcome.
- Nothing that depends on an unconfirmed decision is added to the workspace, the library or CI. Evidence for a decision comes from a disposable spike in `spikes/<name>/`, never from adopted code (§1.2).
- **Standing gates** can arise at any time: a crate or tool outside the approved allow-list (GS.1); a specification source that changes, disappears or is newly proposed (GS.2); the crate name at risk (GS.3); a confirmed requirement of §3 that cannot be met as written (GS.4).
- **Ask, don't guess.** Ask the maintainer about any decision that belongs to a gate or that the plan leaves open. Search online before proposing a technical decision.
- Record gate decisions, with the date and the reasoning, and the progress of each task in the development log, [docs/plans/plan-init-project-log.md](docs/plans/plan-init-project-log.md). The plan holds what is to be done; the log holds what was decided and done; this file holds neither.
- When the plan turns out wrong or incomplete, amend it in the same change and call the amendment out at the phase gate (§1.2). A change to a requirement of §3 goes through GS.4 first.
- When a plan revision changes a rule summarized here, update this file in the same change, and `CONTRIBUTING.md` where it states the rule for contributors.
- **American English (R35).** Write everything in American English: documentation, plans, comments, rustdoc, messages, names in code and scripts, and commit messages. Write `color`, `behavior`, `license`, `initialize`, `analyze`, `catalog`, `signaling` and `labeled`, not `colour`, `behaviour`, `licence`, `initialise`, `analyse`, `catalogue`, `signalling` and `labelled`.
  - Titles and quoted text keep their own spelling: an RFC titled "Additional Colour Sampling Modes" stays so, and a traceability comment keeps a specification's field name such as `colour_primaries`.
  - The identifier for such a field is spelled the American way: `color_primaries`.
  - Verbatim third-party texts, such as the RFCs in `docs/specs/rfc/`, are never edited.

## Clean room and sources

- Parsing behavior comes only from the specifications listed in §5 and Appendices A and B. Normative prose, tables, syntax tables, syntax pseudocode and worked examples count as specification (§1.3, R16).
- **Never read for implementation** the source code of any third-party multimedia project, nor a reference encoder or decoder attached to a specification. Never copy, translate or paraphrase such code. When an approved document is hosted inside an implementation's source repository, open only that document.
- **Never name** an existing multimedia tool or library in this repository: not in code, comments, documentation, changelog entries or commit messages. Do not choose identifiers that coincide with the name of one; the main result type is called `MediaReport` for this reason.
- No multimedia parsing or demuxing library may be used anywhere (R2). General-purpose crates are allowed, and only those on the allow-list approved at G1.3: [docs/design/dependency-dossier.md](docs/design/dependency-dossier.md), enforced in CI by `deny.toml`. Anything else goes through GS.1. A new version of a crate with a build script is read again before its row in `crates/cc-media-probe-checks/data/build-scripts.tsv` changes (D4). A manifest gains a dependency only in the phase that first uses it. CI uses only GitHub's own actions, each pinned to a full commit SHA (dossier, section 1.7); another action or version also goes through GS.1.
- **Synthetic test media only** (R8): hand-annotated hex fixtures and in-repository builders. No third-party samples, no external tool to generate or validate fixtures, and no real media in the repository.
- **Traceability** (§5.5): every parser module starts with a doc comment naming its specification and section; every field read from a file carries the specification's field name in a comment, in the specification's own spelling (the identifier itself is spelled out, R32); every entry of a signature or registry table carries a source reference; every constant whose value a specification defines names that specification and section in its doc comment.
- RFC text in `docs/specs/rfc/` is committed verbatim (`node scripts/fetch-rfcs.ts`). Other specifications are fetched into the git-ignored `docs/specs/files/`, one folder per topic and publisher, and are never committed (`node scripts/fetch-specs.ts`). Documents are stored only there, never in a scratch directory.
- **Ask other sites slowly.** Send one request at a time, a few seconds apart for each site (the fetch scripts wait 2 seconds, 6 for the Internet Archive). When a site answers 403, 429 or 503, or refuses the connection, back off: wait longer for that site, honor `Retry-After`, and keep the longer interval. A site that answers with a challenge page is not asked again; use an archive capture instead.

## Code rules

These rules hold in every crate of the repository (§7.2). Clippy lints and the structure check of D23 enforce them.

- **Highly modular (R29).** One concern per file. Walking a structure, mapping it to the normalized model and building its typed details are separate files. A format, codec or tag family becomes a directory as soon as it has more than one concern. A structure that several formats read lives in one shared module, never inside one of its consumers.
- **Module roots only declare (R30).** `lib.rs` and every `mod.rs` contain only:
  - module declarations without a body (`mod segment;`, `pub mod tracks;`);
  - `use` imports and `pub use` re-exports;
  - attributes on those items, crate-level attributes, and `//!` documentation.

  Nothing else: no function, type, trait, `impl`, constant, static, type alias, macro definition, macro call, inline module body or inline test module. Tests that belong to a module root go in `tests.rs`, declared with `#[cfg(test)] mod tests;`. A binary's `main.rs` follows the same rule and may in addition hold a `main` function that only hands over to the crate's modules.
- **Directories use `mod.rs` (R30).** A module with submodules is `name/mod.rs`, never `name.rs` beside a `name/` directory.
- **At most 500 lines per Rust file, excluding embedded tests (R31).**
  - Every physical line of the formatted file counts: code, comments, documentation and blank lines.
  - The embedded tests are one `#[cfg(test)] mod tests { … }` block at the end of the file. Lines from its start onward do not count.
  - A file that holds only tests (a `tests.rs`, an integration test, a benchmark) counts in full.
  - There are no exceptions. Static tables are split into files by group. Do not use `#[path]`, and do not pull in Rust source with `include!`.
- **Names are whole words; no abbreviations (R32).** Every name in the code is human-readable English made of whole words: items, fields, variants, variables, parameters, generic parameters, lifetimes, modules, crates, and the names in scripts.
  - Spell words out: `identifier`, not `id`; `information`, not `info`; `maximum`, not `max`; `buffer`, not `buf`; `index`, not `idx`.
  - No single-letter names anywhere: not for loop counters or closure arguments, not for generic parameters (`Source`, not `S`), not for lifetimes (`'input`, not `'a`).
  - Exempt are only names that Rust, its standard library, a dependency or an external tool fixes (`std::io`, `fmt`, `str`, `u8`, trait and conventional method names such as `len`, `iter` and `from_str`, Cargo and workflow keys), and proper names of formats, codecs, standards and products as commonly written (`Ebml`, `Mp4`, `H264`, `Id3v2`, `Crc32`, `serde`, `tokio`, the `cc` of the crate names).
  - A specification's spelling of a field (`pic_width_in_mbs_minus1`) goes in the traceability comment; the identifier is spelled out (`picture_width_in_macroblocks_minus_one`), in American English (R35).
  - The name check of D23 knows only the abbreviations on its list, so passing it is not proof. When a review finds an abbreviation it missed, the abbreviation joins the list.
- **Magic numbers and strings are named constants in dedicated modules (R33).** Every numeric, string, character and byte-string literal in Rust code is a named constant.
  - Constants live in a constants module beside the code that uses them: `constants.rs`, or a `constants/` directory split by group once it nears 500 lines. A constant that several modules use lives in the constants module of their nearest common parent. A constants module holds only `const` and `static` items, their imports and their documentation. The data tables (detection signatures, registries, element schemas) count as constants modules.
  - Literals may stay inline only in test code (embedded test blocks, `tests.rs`, integration tests, benchmarks), in documentation, doc examples and the example programs, in attribute arguments and other places where Rust accepts only a literal, and as the numbers 0 and 1.
  - Message text is a constant too. A `format!` or `write!` template keeps only placeholders, spacing and punctuation and names the constant inline: `write!(formatter, "{UNEXPECTED_END}: {element}")`.
- **Copyright header (G1.2).** Every `.rs` and every `.ts` file, tests and constants modules included, starts with the same copyright header: the maintainer's copyright line and the Apache-2.0 notice. Copy it from any existing file; only the year or year range may differ. The structure checks hold the exact text and verify it.
- **Scripts are TypeScript (R34, D24).** Never write shell, awk, Python or JavaScript scripts. Every script is a `.ts` file run by Node.js on a long-term-support release (`.nvmrc` names `lts/*`): `node scripts/check.ts`.
  - Import only Node.js built-in modules, through `node:` specifiers.
  - Node.js strips types and does not check them, so use only erasable syntax: no `enum`, no `namespace`, no parameter properties. Write `.ts` extensions in import paths and `import type` for type-only imports.
  - The Rust rules apply where they fit: whole-word names, one concern per file, at most 500 lines per file, literals as named constants in a `constants.ts` beside their users, and the copyright header. A template literal outside a constants module holds only substitutions, spacing and punctuation.
  - Entry scripts sit in `scripts/` and only hand over to the modules in `scripts/checks/`. Every module with logic of its own has tests in a `*.test.ts` file in its directory, using `node:test` and `node:assert`. Modules that only start programs are exercised by the check run.
  - The scripts run the same on Linux, macOS and Windows. Build every file-system path from components with `path.join`; never write a separator into a path or concatenate one. Names that Git or Markdown give, such as `src/lib.rs` from `git ls-files`, use `/` by their own format on every platform and become file-system paths only through `join`. Start programs without a shell, and take generated paths from the tool that makes them, such as the `executable` of Cargo's JSON build messages, instead of assuming a directory or an `.exe` suffix. The only platform branch is the choice of the tool that lists a binary's libraries (D4, fourth layer). The Rust checks follow the same convention.
  - The script structure check enforces the header, the line limit and the literal rule; review covers names and modularity. `tsc` type-checks the scripts through `scripts/tsconfig.json` (strict, erasable syntax only); run `npm ci` in `scripts/` once to install it. The npm files (`package.json`, `package-lock.json`, `.npmrc`, `tsconfig.json`) stay in `scripts/`, never at the repository root.
- **Pure Rust, no `unsafe`** (R15, D4): `forbid(unsafe_code)` in every crate, and nothing in the published crates' dependency graph compiles or links C or C++.
- **No panics on input** (§7.6): no `unwrap`, `expect`, unchecked indexing or unchecked arithmetic on values derived from input. Every loop over file structure is bounded by the probe budget (§7.5). Walkers are iterative, with explicit stacks; there is no input-driven recursion. A failing source is an error that keeps its original cause (R19), never "unsupported format".
- **Stable API** (§8.1, §8.10): private fields with accessors; `#[non_exhaustive]` on public enums and options types; builders instead of struct literals; a missing value is `None`, never a zero or an empty string; estimates only through the separate estimate accessors (R25); Cargo features are additive.

## Quality bars

- **Coverage (R28):** line coverage **above 90%**, where exactly 90.0% fails. It applies to each published crate as a whole and to each library area: `containers/`, `codecs/`, `tags/`, `text/`, `disc/`, `detect/`, `registry/`, `input/` and `utilities/`. A crate or area is held to it from the first change that gives it executable code, so write the tests together with the code. It is measured from the LCOV report of `cargo llvm-cov --workspace --all-features --lcov`, without the lines of embedded test blocks, which the tool would count as covered.
- Zero warnings from the compiler, Clippy and rustdoc. Every public item has rustdoc with an example; `missing_docs` is denied.
- A bug found after a format ships gets a minimized synthetic fixture before it is fixed.
- A change to reported output updates its expectation files and is listed in `CHANGELOG.md` under "Changed output" and in the phase-gate summary (§9.6).
- **Diagrams (R14, §10):** in Markdown documents (the plan, this file, guides, design documents), draw workflows and processes as Mermaid diagrams in fenced `mermaid` blocks. Directory trees and byte layouts stay as text. A diagram restates the prose beside it and adds no rule of its own; when the prose changes, update the diagram in the same change. Rustdoc stays prose, because it renders no Mermaid without an extra crate (GS.1).
- `node scripts/check.ts` must pass on every change; CI repeats it on Linux, macOS and Windows. It runs the Node.js version, hygiene, link, specification-table, script-structure, script-type and script-test checks, the Rust format, lint, test, coverage, documentation and packaging checks, and the checks of `cc-media-probe-checks`: `structure` (R30 to R33 and the header), `coverage` (R28), `targets`, and the pure-Rust layers (`inventory`, `build-output`, `dynamic-libraries`). CI also runs the tests on current stable and, through `node scripts/test-declared-rust-version.ts`, on the declared `rust-version`; every feature combination with `cargo-hack`; and the `cargo-deny` audit of `deny.toml` (§9.8).

## Layout and toolchain

The planned workspace is in §7.1, and the library's module tree is in §7.2.

- `crates/cc-media-probe/`: the library (published).
- `crates/cc-media-probe-cli/`: the command-line tool (published; binary `cc-media-probe`).
- `crates/cc-media-probe-testkit/`, `crates/cc-media-probe-doctest/`, `crates/cc-media-probe-compatibility/`: unpublished support crates. Fixtures live in the test kit under `fixtures/<format>/`.
- `crates/cc-media-probe-checks/`: the unpublished repository checks, standard library only. `data/` holds their committed lists. The name check's abbreviations are in `src/names/abbreviations.rs`; an abbreviation that a review finds is added there.
- `fuzz/`, `spikes/<name>/`, `readers/<version>/`: workspaces of their own, excluded from the root workspace.
- `docs/`: `plans/` (the plan and its development log), `rfc/`, `specs/`, `guide/`, `design/` and `releases/`.

Toolchain: Rust 1.100, edition 2024, pinned to the exact release `1.100.0` in `rust-toolchain.toml` (D1). Nightly is used only for fuzzing and the docs.rs job. `rustfmt.toml` sets `max_width = 120` and `tab_spaces = 2`; the scripts use the same width and indentation. The scripts need Node.js on a long-term-support release (`lts/*`, D24), and `npm ci` in `scripts/` for the type checker; `scripts/.npmrc` gives npm the same 3-day cooldown as Cargo. The Cargo tools (`cargo-llvm-cov`, `cargo-deny`, `cargo-semver-checks`, `cargo-hack`, `cargo-fuzz`) are installed with `cargo install --locked --version <version>` at the versions of the dossier.

Repository builds are marked by `--cfg cc_media_probe_repository` in the root `.cargo/config.toml` (option A of §7.1). **Never set `RUSTFLAGS`**: it replaces that flag, and the library's guard test then fails. Warnings are denied by the workspace lint table instead.
