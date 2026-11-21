# Development log

The development log of the [development plan](plan-init-project.md). It records two things, each entry with its date:

- **Decisions** taken at gates, with the reasoning, so that later sessions do not reopen them. A decision changes only through its own gate or through a standing gate (plan section 1.1).
- **Progress**: what each task produced and how it was verified, and what is still open.

The plan says what is to be done; this log says what was decided and done. `CLAUDE.md` holds neither.

## 2026-11-21 — The plan is approved

The maintainer approved draft 6 of the plan by asking for Phase 1 to be implemented. Amendments made since then are listed in the plan's Appendix E.

## 2026-11-21 — G1.1 Toolchain [MANUAL]

At the maintainer's request, the agent performed the step.

- `rustup update stable` moved `stable` from 1.96.0 to `rustc 1.100.0 (0ef1bc386 2026-11-09)`.
- `rustfmt`, `clippy` and `llvm-tools` were already present for that toolchain.
- `rustup toolchain install nightly` installed `rustc 1.102.0-nightly (c7d5ba59d 2026-11-20)`, which is used only for fuzzing and for the docs.rs reproduction.

## 2026-11-21 — G1.2 Workspace and policy decisions [CONFIRM]

| Topic | Decision | Reasoning |
|---|---|---|
| D1 toolchain | `rust-toolchain.toml` pins `1.100.0` with `rustfmt`, `clippy` and `llvm-tools`. Every manifest uses edition 2024 and `rust-version = "1.100"`. CI runs the pinned release, current stable and the declared `rust-version`; on 2026-11-21 all three are 1.100.0. Nightly is used only by the fuzzing and docs.rs jobs, so it does not appear in Phase 1. | An exact release identifies the Cargo that packages a release. |
| D3 layout | The layout of plan section 7.1 with a sixth member crate, `crates/cc-media-probe-checks/` (`publish = false`). It holds the repository checks, which use only the standard library: structure, coverage, target ownership, the pure-Rust inventory and build-output inspection, and later the table comparisons. | The checks share the workspace lints, toolchain and lock file, and their own code is held to the same rules. No second workspace needs to be kept in step. |
| Repository marker | Option A: `--cfg cc_media_probe_repository` in `[build] rustflags` of the root `.cargo/config.toml`. The name is declared with `fuzzing` under `unexpected_cfgs`. A guard test in the library is compiled whenever the marker is absent and fails if the test kit is present beside the crate. CI never sets `RUSTFLAGS`. | It needs no build script in the published crate. Reproduced on 1.100.0: `cargo test` and `cargo llvm-cov` keep the flag, while any `RUSTFLAGS` variable drops it, which the guard test then reports. |
| D4 unsafe code and pure Rust | `forbid(unsafe_code)` in every crate, through the workspace lint table. The four layers of verification and the build-script rule are as written in plan section 6. The permitted operating-system interface crates are `libc`, `windows-sys` and `windows-link`. `windows-targets` and its per-target import-library crates are added only if the graph approved at G1.3 needs them. The allow-list of operating-system libraries is measured from what the standard library links on each CI platform; on macOS (measured 2026-11-21) that is `/usr/lib/libSystem.B.dylib` only. The full list is presented at G1.3. | Current `windows-sys` releases link through `windows-link` and need no import-library crate. |
| Lints | Rust: `warnings` denied, so that zero warnings is enforced without `RUSTFLAGS`; `unsafe_code` forbidden; `missing_docs` denied; `unexpected_cfgs` with the two declared names. Clippy: `all` and `pedantic` denied; the four lints of D23 (`self_named_module_files`, `definition_in_module_root`, `min_ident_chars` with an empty list of allowed names, `single_char_lifetime_names`); the panic lints of plan section 7.6 (`unwrap_used`, `expect_used`, `panic`, `indexing_slicing`, `arithmetic_side_effects`). `clippy.toml` allows `unwrap`, `expect`, `panic` and indexing in tests. Rustdoc: broken and private intra-doc links denied. | Plan section 7.6 says that its rules are enforced by lints. The set was tested on Clippy for 1.100.0. |
| Where the checks live | `crates/cc-media-probe-checks/`, a library with a binary of the same name, run by `scripts/check.sh` and CI. | As for D3. |
| Coverage command and exclusions | `cargo llvm-cov --workspace --all-features --lcov --output-path target/coverage/lcov.info`. The coverage check reads the LCOV line records, drops each file's lines from its embedded test block onward (the boundary of R31), and keeps the tool's default exclusions (`tests/`, `examples/` and `benches/` directories and `tests.rs` files). Doctests are not measured, because measuring them needs nightly. The support crates are measured but not held to the bar of R28. | The tool counts the lines of embedded test blocks as covered code, which inflates the figure; this was reproduced on 1.100.0, where `#[coverage(off)]` is still unstable. The LCOV records equal the JSON summary line for line. Reading LCOV needs only the standard library, so the check needs no `serde_json` (this amends D7 and D23). |
| Constants modules and data tables | A file named `constants.rs` or lying under a `constants/` directory is a constants module and must hold only constants. Any other file whose items are all `const` and `static` items, with imports and documentation, also counts as one. Data tables therefore need no marker, and code would disqualify a file. | Recognition is automatic, and the content rule gives it no loophole. |
| First list of abbreviations | The base list together with the borderline words, as presented, without `dts`. The list lives in `crates/cc-media-probe-checks/src/names/abbreviations.rs`. | DTS is the name of a codec in scope, and R32 exempts proper names. |
| Name check | Words are split at underscores and at changes from a lower-case letter or a digit to an upper-case letter, and before the last capital of an acronym followed by lower case. `Id3v2` and `H264` therefore stay one word each. A word of a defined name that is on the list fails unless the whole name is one that Rust or a dependency fixes. A single-letter identifier or lifetime fails anywhere. | It follows R32 and the tests of Phase 1. |
| License headers | The license is stated in `LICENSE` and in each manifest's `license = "Apache-2.0"`. Every `.rs` file starts with the maintainer's copyright header: `Copyright (c) 2026. caoccao.com Sam Cao`, `All rights reserved.`, then the Apache-2.0 notice verbatim. The structure check verifies the year or year range and the rest of the text. | It is the maintainer's choice, in the style of a sibling project of the maintainer. |
| `.gitignore` | The root `.gitignore` adds `docs/specs/files/` and the fuzz workspace's `corpus/`, `artifacts/` and `coverage/` directories. `.serena/` keeps its own ignore file with `project.yml` tracked. IDE folders are left to each developer's global ignore file. | Only what this repository creates is ignored here. |
| Bootstrap check | `scripts/check.sh` starts with file hygiene and Markdown links. File hygiene means UTF-8, LF line endings, a final newline, no trailing whitespace, no tabs, and scripts that are executable; verbatim third-party texts are exempt. The link check requires every relative Markdown link and heading anchor to resolve. | These are the checks that apply before any crate exists. |

Chosen by the agent within the plan and open to change at G1.3: the dependency cooldown is 3 days (`registry.global-min-publish-age = "3 days"`), within the plan's "a few days".

## 2026-11-21 — Progress: Phase 1, tasks 3 and 4

**Task 3: directories and the check script.** `crates/`, `scripts/`, `spikes/` and `readers/` exist; the `docs/` directories of plan section 5.2 are created by the tasks that first write into them. `scripts/check.sh` runs these steps in order:

- `scripts/check-hygiene.sh`;
- `scripts/check-markdown-links.sh`, which uses `scripts/markdown-links.awk`;
- formatting, lints, the structure check, tests and documentation;
- target ownership;
- `scripts/check-packages.sh`;
- `scripts/verify-pure-rust.sh`.

`.gitattributes` keeps every text file at LF line endings and leaves the RFC texts byte for byte.

**Task 4: the workspace.**

- **Root files:**
  - The root manifest has six members, excludes `fuzz`, `spikes` and `readers`, and holds the shared package metadata (version 0.1.0, edition 2024, `rust-version = "1.100"`, Apache-2.0) and the lint tables of G1.2, including the declared configuration names.
  - `rust-toolchain.toml` pins `1.100.0`, and `rustfmt.toml` and `clippy.toml` set the formatter and the lints.
  - `.cargo/config.toml` holds the repository marker and a cooldown of 3 days.
- **Crates:**
  - The library has no API and holds the marker's guard test in `src/tests.rs`.
  - The CLI is a placeholder that prints `cc-media-probe 0.1.0`, and an integration test runs the binary.
  - The test kit, doctest and compatibility crates are empty libraries.
  - Both published crates carry `README.md` and `LICENSE`; the library's are copies of the root files.
- **The checks crate** has six commands: `structure`, `coverage`, `targets`, `inventory`, `build-output` and `dynamic-libraries`. It uses only the standard library: a tokenizer, an item outline and a small JSON reader for `cargo metadata`, which runs without recursion. Its committed lists are in `crates/cc-media-probe-checks/data/`: the expected targets, the expected package contents, the reviewed build scripts (none yet) and the allowed operating-system libraries.
- No crate has a dependency.

**Verification, on macOS (aarch64) with Rust 1.100.0, on 2026-11-21:**

- `scripts/check.sh` passes, with zero warnings.
- All 148 tests pass.
- The structure check reports 0 findings over 101 Rust files, its own included. The longest file has 317 counted lines.
- With `RUSTFLAGS` set, the guard test fails and names the cause.
- **Coverage:**
  - `cc-media-probe-cli`: 100.00% (10/10).
  - `cc-media-probe`: no executable lines.
  - `cc-media-probe-checks` (not held to the bar): 4,683 of 4,765 lines, about 98%, before test blocks are dropped.
- **The test list of Phase 1**, each item covered by a test:
  - The structure check fails on a definition in a `mod.rs`, a function in a `lib.rs`, an inline module body in a module root, `name.rs` beside `name/`, and a file with 501 counted lines. A file of 500 counted lines followed by a long test block passes.
  - The name check fails on a single-letter generic parameter and on `header_buf`, and passes on `Id3v2` and on `fmt` in `Display`.
  - The literal check fails on a number in ordinary code, on a word in a `format!` template, and on a function in a constants module. It passes on 0 and 1, an attribute argument, a test block and a constants module.
  - The coverage check fails at exactly 90.0% for a crate and for an area, and passes just above it.
  - `cargo metadata` succeeds at the root with a spike and a reader package present, and inside each of them.
  - A test runs the CLI binary.

**Choices made without asking**, each open to change at the gate named:

- The cooldown is 3 days (G1.3).
- The `warnings` lint group is denied in the lint table, the plan's way of failing on warnings without `RUSTFLAGS` (G1.9).
- The README stub of task 18 was written now, because the package manifests name it. Its links to the plan and the log are absolute GitHub links, because the library ships a byte-identical copy (G1.9).
- The CLI package ships its `tests/` directory, and the expected package list records it (G2.3).
- The coverage check is built but stays out of `scripts/check.sh` until G1.3 approves `cargo-llvm-cov`.

**Open:**

- CI has not run, because nothing has been pushed (task 19 and G1.8).
- The Linux and Windows halves of the fourth pure-Rust layer, and their entries in the allow-list of operating-system libraries, are measured by the first CI run. The Windows step uses `objdump` when Git Bash provides it and `dumpbin` otherwise.
- Task 5, the dependency dossier for G1.3, is next.

## 2026-11-21 — Formatter settings

The maintainer asked for `max_width = 120` and `tab_spaces = 2` in `rustfmt.toml`. Every crate was reformatted. The structure check still reports 0 findings, and the longest file shrank.

## 2026-11-21 — Scripts are TypeScript (R34, D24)

The maintainer's instruction: no shell scripts; every script is TypeScript run by Node.js LTS and uses only Node.js built-in modules; every `.ts` file carries the same copyright header as the Rust files. The answers to the follow-up questions:

| Question | Decision | Reasoning |
|---|---|---|
| Node.js version | Float with the newest long-term-support release: `.nvmrc` names `lts/*`, and the check runner fails on a release that is not long-term support. On 2026-11-21 that is 26.13.1 ("Lithium"). | The maintainer's choice over an exact pin or a major line. |
| Type checking | `tsc --noEmit` with `strict` and `erasableSyntaxOnly`, from the `typescript` package, as a development tool. It needs `@types/node` too, for the types of the built-in modules. Both are pinned in a `package.json` with its lock file, and are added only after G1.3 approves them. The scripts still import only `node:` modules. | Node.js strips types without checking them. |
| Rules for scripts | The Rust code rules where they fit: whole-word names, one concern per file, at most 500 lines, literals as named constants in constants modules, the copyright header, and tests with `node:test`. A TypeScript structure check enforces the header, the line limit and the literals; review covers the rest. | The maintainer's choice. |
| Local Node.js | At the maintainer's request, the agent ran `nvm alias default 26.13.1`, which changes the default for every new shell on this machine. | The maintainer's choice. |

This replaces the G1.2 bootstrap-check entry's "scripts that are executable": no script needs an executable bit, because every one is run with `node`.

## 2026-11-21 — Progress: scripts rewritten in TypeScript

**Removed:** `scripts/check.sh`, `check-hygiene.sh`, `check-markdown-links.sh`, `markdown-links.awk`, `check-packages.sh` and `verify-pure-rust.sh`. They stay in the staged index until the maintainer updates it.

**Added:**

- `.nvmrc` with `lts/*`.
- The entry scripts `scripts/check.ts` and `scripts/verify-pure-rust.ts`, which only hand over.
- Their modules in `scripts/checks/`, each with a constants module beside it:
  - the step runner and the command and file helpers;
  - the Node.js version check;
  - file hygiene, which now also rejects scripts in other languages;
  - the Markdown links check, with Unicode-aware anchors;
  - package contents and copies;
  - the pure-Rust layers, where every native-compiler variable points at a missing program instead of at `false`, so the step also works without a Unix shell;
  - the script structure check: a small tokenizer, top-level statements, the literal rule, the header and the line limit.
- `node --test` runs the scripts' 19 tests, which live in `*.test.ts` files.

**Verification, on macOS with Node.js 26.13.1 and Rust 1.100.0:**

- `node scripts/check.ts` passes every step.
- The script structure check reports 0 findings over 30 TypeScript files. The first run found nine violations in the new scripts themselves, and they were fixed.
- All 19 script tests and all 148 Rust tests pass.

**Open:** `tsc` type checking waits for G1.3. CI still has not run.

## 2026-11-21 — The scripts run on every platform

The maintainer's instruction: make sure the scripts run well on Windows, with no hard-coded path separator and nothing else that is specific to one operating system. The audit changed these places:

- **Paths from components.** The expected package lists, the copied files, the pure-Rust target directory and the `vswhere` location are component lists joined with `path.join`. The Rust checks crate does the same for its data files and for every repository file it reads, through `repository_path` and `repository_file`.
- **No assumed binary path.** The fourth pure-Rust layer reads the binary's path from the `executable` field of Cargo's JSON build messages, instead of assuming `debug/` and an `.exe` suffix.
- **No glob.** The script tests are passed to `node --test` as explicit file-system paths, taken from `git ls-files`.
- **No shell.** Every program is started directly. The native-compiler variables of the third layer name a missing file, so that layer needs no `false` command either.

What stays on purpose:

- Names from Git and Markdown, such as `src/lib.rs`, keep `/`, which is their format on every platform. They become file-system paths only through `join`.
- The fourth layer chooses its tool by platform, as D4 asks: `otool` on macOS, `readelf` on Linux, `objdump` or `dumpbin` on Windows.

**Verification:** 20 script tests and 146 checks-crate tests pass, and `node scripts/verify-pure-rust.ts` finds the binary through Cargo's messages. Windows itself is first exercised by CI.

## 2026-11-21 — American English (R35)

The maintainer's instruction: always use American English, and fix the existing documents and code. The plan gained R35 (revision 9), and `CLAUDE.md` states the rule.

**Corrected:**

- **Words:** "licence" became `license`. Likewise `behavior`, `color`, `signaling`, `labeled`, `analyzed`, `catalog`, `organized`, `initialized`, `honored`, `canceled`, `flavor`, `flavored`, `centimeters`, `equaled` and `math`.
- **Files:**
  - the plan, this log and `CLAUDE.md`;
  - the three READMEs, whose heading is now "License" and which stay byte-identical;
  - four doc comments in the Rust crates;
  - a comment in `.serena/project.yml`.

**Kept on purpose:** the title of RFC 4421, "Additional Colour Sampling Modes", which is how the RFC spells it.

**Enforcement:** review only, by the maintainer's choice. No check of the checks crate or of the scripts looks for British spellings.

**Verification:** a search of every tracked file for British spellings (the -our, -ise, -isation, -yse, -re, -ogue and -ence forms, doubled consonants such as -lled, and words such as whilst and learnt) finds only that title.

## 2026-11-21 — G1.3 choices, before the dossier

The plan left these choices open, and the maintainer answered them before the dossier was written. G1.3 itself, the approval of the dossier, is still open.

| Topic | Decision | Reasoning |
|---|---|---|
| Inflater (`zlib`) | `miniz_oxide` 0.9.1, without default features, with `with-alloc`. | 2 crates, no build script and no unsafe code. Its low-level inflater writes into a buffer the caller owns, which fits charge-before-reserve (plan section 7.5). `zlib-rs` uses unsafe code; the `flate2` wrappers allocate internally. |
| Serialization | `serde` 1.0.229 with `derive`. | Attributes cover the contract of plan section 8.8, with hand-written implementations where needed. The proc-macro crates are in the inventory either way. |
| Arguments | `clap` 4.6.8 with the builder API, without default features, with `std`, `help`, `usage` and `error-context`. | 4 crates, no build scripts, and no color, terminal detection or suggestions, so the byte-compared output of plan section 8.9 does not depend on the terminal. |
| futures-io executor | `futures-lite` 2.6.1. | `block_on`, plus adapters that implement futures-io's traits over a file or bytes, which a shipped example needs. |
| Allocation tracker | `peak_alloc` 0.3.0. | No dependencies. It reports current and peak usage, its peak can be reset, and a reallocation counts both blocks. |
| Benchmarks | `criterion` 0.8.2, without default features. | Maintained and named by the plan. `cargo bench` needs a C compiler through `alloca`, which R15 permits for development tools. |
| Rehearsal registry | An in-house script, `scripts/rehearsal-registry.ts`, written in Phase 8, instead of a tool. | It records publication times, so the rehearsal exercises the cooldown. The one existing tool is GPL-3.0, serves a git index and records no times. |
| TypeScript | `typescript` 7.0.2 with `@types/node` 26.6.6. | The current release. It type-checks the scripts six times faster than 6.0.3, with the same result. |
| Installing the tools | `cargo install --locked` at exact versions, locally and in CI. | Each tool is built from crates.io with the lock file it ships, and the cooldown applies to it. |
| Cooldown | 3 days, as set in task 4. | Confirmed. |

## 2026-11-21 — Progress: Phase 1, task 5

**The dossier** is [docs/design/dependency-dossier.md](../design/dependency-dossier.md). For every crate, tool and npm package it gives the exact version, settings, license, release history and dependency list. It also holds the pure-Rust evidence and what approval sets in motion.

**The evidence** comes from the spike [spikes/dependency-graph/](../../spikes/dependency-graph/README.md), a workspace of its own. It holds copies of the two published crates with exactly the proposed dependencies and features, and the development and fuzzing dependencies. The repository's checks ran against it with `--root`:

- **Layer 1:** 24 packages and 0 findings, with a draft list of the six reviewed build scripts.
- **Layer 2:** the build scripts of `proc-macro2`, `quote`, `serde`, `serde_core`, `serde_json` and `zmij` pass. Each was read in full from the packaged source.
- **Layer 3:** 0 findings. On macOS, `cargo build` ran with every native-compiler variable naming a missing file, for no default features, the defaults and all features: 6 build-script runs. On x86_64 Linux and Windows, `cargo check --target` made 10 runs.
- **Layer 4, on macOS:** the binary loads only `/usr/lib/libSystem.B.dylib`, and the check reports 0 findings.
- **Advisories:** none in the OSV database for the 78 crate versions and the 3 npm packages.

**Also changed:**

- The stdio constants in `scripts/checks/constants.ts` are now typed `StdioOptions`. Both TypeScript versions reported them as the only type error in the scripts.
- The plan is at revision 10, with D5 to D7, task 5, Phase 8 task 6, Appendix C.2.3, the layout and the documentation table updated.

**Open:**

- G1.3 approval of the dossier.
- After approval: the reviewed list, `deny.toml`, the coverage step (which needs `cargo-llvm-cov` 0.9.1 locally; 0.8.7 is installed), and the npm files with `tsc`.
- From CI: the 32-bit build inspection, and the fourth layer on Linux and Windows.
- In Phase 8: whether `cargo-semver-checks` 0.51.0 reads the rustdoc JSON of the pinned toolchain.

## 2026-11-21 — G1.3 Dependency allow-list [CONFIRM]

The maintainer approved the [dependency dossier](../design/dependency-dossier.md) as proposed, and allowed the agent to install `cargo-llvm-cov` 0.9.1 and the npm packages on the development machine.

## 2026-11-21 — Progress: Phase 1, task 6

**What the approval set in motion:**

- **Reviewed build scripts.** `crates/cc-media-probe-checks/data/build-scripts.tsv` holds the six reviews of the dossier.
- **`deny.toml`:**
  - licenses Apache-2.0, MIT and Unicode-3.0;
  - crates.io as the only source;
  - yanked versions denied;
  - an allow-list of the 73 crate names of the dossier's sections 2 and 4, with the workspace's own crates allowed;
  - duplicate versions as warnings.

  `cargo-deny` is not installed locally, so CI runs it first (task 19).
- **Type check.** `package.json` (`private`, `type: module`) pins `typescript` 7.0.2 and `@types/node` 26.6.6 exactly, with `package-lock.json`. `tsconfig.json` holds the settings of the dossier, and `node_modules/` is ignored. The check runner's new step "script types (D24)" starts the type checker with Node.js itself, so it needs no shell, and names `npm ci` when the type checker is missing.
- **Coverage.** `cargo-llvm-cov` 0.9.1 replaced 0.8.7 through `cargo install --locked`. The check runner's new step "coverage (R28)" writes `target/coverage/lcov.info` and runs the coverage check of the checks crate.

**Added by the agent, open to change at G1.9:** `.npmrc` sets `min-release-age=3`, npm's equivalent of the Cargo cooldown, and `save-exact=true`.

**Verification:** `node scripts/check.ts` passes every step, the two new ones included, on 2026-11-21:

- the type check reports no error;
- `cc-media-probe-cli` is at 100.00% line coverage (10/10), and `cc-media-probe` has no executable lines;
- the guard test passes under `cargo llvm-cov`, so the repository marker survives it.

**Open:** tasks 7 to 11, the specification set.

## 2026-11-21 — The npm files live in `scripts/`

**The npm files.** The maintainer's instruction: `tsconfig.json` and `package.json` stay in the `scripts` folder. `package-lock.json` and `.npmrc` belong beside `package.json`, so all four moved there. That `package.json` is now the nearest one to every script, so its `"type": "module"` applies to them.

- **Lock file:** regenerated with `npm install` in `scripts/`, with the same exact versions.
- **The type-check step:** it starts `scripts/node_modules/typescript/bin/tsc` with `--project scripts/tsconfig.json`, both paths built from components. When the type checker is missing, it says to run `npm ci` in `scripts/`.
- **Documents:** the plan (D24 and the layout of section 7.1), the dossier and `CLAUDE.md` name the new place.
- **Width:** three lines of the check runner were wider than 120 characters, and are now wrapped. Two of them predate this change.

**Serena.** At the maintainer's request, `.serena/project.yml` lists `typescript` after `rust` as a language server.

## 2026-11-21 — Choices for the specification set (tasks 7 and 8)

The maintainer answered these before tasks 7 and 8 were written. Gate G1.4 reviews the result.

| Topic | Decision | Reasoning |
|---|---|---|
| Where the table checks live | The offline checks of the specification tables are TypeScript in `scripts/checks/specifications/`, run by `node scripts/check.ts`: the RFC manifest against Appendix A and the stored files with their SHA-256, the errata table, the non-RFC manifest, and deliverables against coverage. This amends the note of G1.2 that put the table comparisons in `cc-media-probe-checks`. | The fetch scripts and the checks share one reader of the tables, and `node:crypto` provides SHA-256, so nothing is hand-written twice. |
| What the errata table holds | Every verified erratum of every listed RFC, each classified by relevance (`parsing`, `output` or `none`) with the components it affects. An erratum held for document update is reviewed too and listed only when it affects parsing, marked as held. | The fresh query found 70 verified errata in 25 of the 74 RFCs, and 13 of those RFCs have errata that Appendix A.8 does not list. A complete table turns the refresh at each gate into a comparison, and the classification shows that every erratum was read. |
| Refreshing RFC metadata and errata | `node scripts/fetch-rfcs.ts` also reads the RFC Editor's metadata for each listed RFC and its errata dump, neither of which is stored. It reports an RFC that was obsoleted or whose title or status changed, an updating RFC that Appendix A does not list, and a verified erratum missing from the table. It needs the network, so it never runs in `check.ts` or CI. | Plan section 5.3 refreshes the table at every phase gate; a script makes that repeatable. |
| Immutable references of living pages | An existing Internet Archive capture, fetched as raw bytes at a pinned capture time. Where no suitable capture exists, the row is marked as not reproducible upstream and the local copy is the approved one. Texts hosted on GitHub are pinned to a commit, and W3C documents to their dated versions. No new captures are requested. | It needs no request to an outside service beyond reading, and a pinned capture keeps returning the same bytes. |

## 2026-11-21 — Progress: Phase 1, task 7

**The RFC set.** `node scripts/fetch-rfcs.ts` writes `docs/rfc/manifest.tsv` from Appendix A, stores each RFC as `docs/rfc/rfcNNNN.txt` from the RFC Editor's plain-text copy, and generates `docs/rfc/README.md` with the index and the license notice. On 2026-11-21 it stored all 74 RFCs, 6.7 MB, each checked for the media type `text/plain` and for its own number in the `Request for Comments:` line before it was written. A second run downloads no text and verifies each against its recorded SHA-256.

**The comparison with the RFC Editor.** The same run reads the metadata of each RFC and the RFC Editor's list of every erratum:

- No RFC of Appendix A is obsoleted, and every title and status matches.
- Four RFCs are updated by RFCs that Appendix A does not list: RFC 2046 by 2646, 3798, 5147, 6657 and 8098 (mail and text formats); RFC 3551 by 5761, 7007 and 8860 (RTP sessions); RFC 3558 by 4788 (EVRC-B in RTP); RFC 3986 by 7320 and 8820 (URI design). None changes what this project reads, so none was added; G1.4 reviews this.
- The run prints these as notes on every refresh, together with the 36 held errata that are not in the table: 4609 and 6776 (RFC 2046), 2969 and 5206 (2119), 782 (2250), 5624, 5783 and 6533 (3339), 5135 (3625), 2033, 2624, 2933, 4547, 4789, 4942 and 5428 (3986), 121 (4337), 96 (4425), 8253 (4647), 1659 to 1667 (5215), 8711 and 8712 (5646), 8717 (6386), 4392 and 4829 (6716), 5022 (8174), 7650 and 8438 (8259). Each was read on 2026-11-21 and found not to affect parsing. A later refresh reports a new held erratum as a note that is not in this list.

**The errata table.** `docs/rfc/errata.tsv` has 73 rows: the 70 verified errata of 25 RFCs, and three held errata that affect parsing (RFC 2361 errata 1255 and 5125, RFC 8794 erratum 7191). Eleven rows are relevant, nine to parsing and two to output; Appendix A.8 names them. One needs a decision later: RFC 8259 erratum 5318, verified on 2026-05-28, removes the solidus from the characters that a JSON string may hold unescaped, while `serde_json` writes it unescaped. Phase 2 settles it for the serialization contract (plan section 8.8).

**The offline check.** The new step "specification tables (section 5.2)" of `node scripts/check.ts` needs no network. It compares the manifest with Appendix A, the stored texts with the manifest and their checksums, the index with the one generated from the manifest, and checks the errata table's vocabulary and order. The modules are in `scripts/checks/specifications/`, with 31 tests that replace the RFC Editor with a table of responses.

**Plan.** Revision 11 amends section 5.3, Appendix A.8, the layout of section 7.1, and the roles of RFC 1951 and RFC 2119 in Appendix A, which referred to neighboring rows and read wrongly in the generated index.

**Verification:** `node scripts/check.ts` passes every step on 2026-11-21.

**Open:** tasks 8 to 11. G1.4 reviews the RFC list, the errata table and the four unlisted updating RFCs.

## 2026-11-21 — Choices for the living pages of task 8

| Topic | Decision | Reasoning |
|---|---|---|
| Pages built from a public repository | Where a living page is built from files in a public Git repository, the manifest stores the documentation file at a pinned commit, and the rendered page becomes the landing page: MP4RA's registry files, the Markdown sources of Microsoft Learn, the Matroska site's sources, Xiph's documentation. Only that file is opened, never code in the repository (plan section 1.3). | The same bytes come back on every fetch, the files are small and structured, and no page chrome is stored. |
| Apple's QuickTime documentation | The fetch script walks the documentation tree of Apple's QuickTime File Format, more than 800 JSON pages (the count stopped at 800), and stores them as one combined file, marked as not reproducible upstream. The 2001 PDF is not stored. | The pages are what Apple maintains; the PDF lacks what was added later. Every section that Phase 6 cites is then in hand for task 17. |

## 2026-11-21 — Requests to other sites

The maintainer's instructions while task 8 was written: send requests slowly, so that no site blocks this machine, and back off when a site shows signs of blocking, with a longer interval for that site. The maintainer also approved downloading the specifications without a further confirmation.

- **Pacing.** The fetch scripts send one request at a time, at least 2 seconds apart for each host, and 6 seconds for the Internet Archive. The archive refused connections after 16 index queries 4 seconds apart, during the research for task 8.
- **Back-off.** A response of 403, 429 or 503, a passing server failure (500, 502, 504) or a refused connection doubles that host's interval, or sets it to what the host asks for in `Retry-After` when that is longer, up to 5 minutes. The request is retried up to 4 times, and the longer interval stays for the rest of the run.
- **Sites left alone.** The Hydrogenaudio wiki answered with a challenge page and was not asked again; its APEv2 page is stored from an archive capture.

## 2026-11-21 — Choices for tasks 10 and 11

| Topic | Decision | Reasoning |
|---|---|---|
| Deliverable keys | Kind-prefixed slugs built from the plan's own words: `detect.<format>`, `probe.<format>`, `pair.<codec>.<representation>.<carriage>`, `identify.<codec>.<carriage>`, `tags.<family>`, `registry.<table>`, `rule.<name>`. | Readable in code and in reviews. From Phase 3 on, code registers against these keys. |
| Comparing with section 4 | The check also compares `deliverables.tsv` with the tables of plan sections 4.1, 4.3 and 4.4 mechanically. Each deliverable keeps the plan's wording, and a row of those tables without a deliverable, or a deliverable without a row, fails. | Transcription errors cannot slip through; the review at G1.4 then concerns the content, not the copying. |
| Depth of the source dossier | Each candidate description of Appendix B.6 is read, prose only and slowly paced, to state what it covers and leaves open and whether it is hosted in an implementation's repository, and better sources are searched for. | G1.5 then decides on facts. |

## 2026-11-21 — Where the specifications are stored

The maintainer's instructions: the specifications go to `docs/specs/`, never to a scratch directory, and are organized in folders instead of one directory. Asked how, the maintainer chose topic, then publisher.

- **Layout.** Each stored document is `docs/specs/files/<topic>/<publisher>/<identifier>.<extension>`. The topics follow Appendix B: `containers` (with the pinned Matroska drafts), `video`, `audio`, `subtitles` and `tags-and-registries`. The manifest's new `folder` column gives the topic and publisher of each row, and the community descriptions proposed for G1.5 sit under their topics, classed `descriptive`.
- **Scratch copies.** The research copies of RFCs and candidate descriptions that had been kept in a scratch directory were deleted; every candidate is now a manifest row, stored by the fetch script like any other document.

## 2026-11-21 — Progress: Phase 1, task 8

**The manifest.** `docs/specs/manifest.tsv` has 152 rows, one per stored document or link:

- the 3 pinned drafts of Appendix A.7 and 126 documents of Appendix B.1 to B.5, split where a row of Appendix B bundles several documents;
- 23 candidate descriptions for G1.5, with their `approved` cells empty.

**Retrieval.** `node scripts/fetch-specs.ts` stored 127 documents, 103 MB, each checked for its media type before it was written:

- 39 `fetch` rows, whose addresses name their editions: ETSI, ITU, 3GPP, EBU, SMPTE, W3C and vendor PDFs.
- 87 `snapshot` rows, of four kinds:
  - 37 files at pinned commits of public repositories;
  - 41 Internet Archive captures;
  - 5 wiki pages at a pinned revision of their source;
  - 4 dated W3C versions.
- 1 `tree` row: Apple's QuickTime documentation, 1,152 pages in one 12 MB JSON file, walked in about 40 minutes at one page every 2 seconds.

**Manual rows.** 24 rows wait for a file placed by hand:

- 15 paid ISO/IEC standards and 1 IEC standard, which G1.6 decides on;
- the 5 ITU documents whose download asks an automated client to log in: H.222.0 (04/2025), H.262, T.81, BT.2100-3 and BS.2088-2;
- 3 Dolby documents.

One link row points to ISO's list of free standards.

**Resolved from Appendix B.** Plan Appendix B now records these:

- SMPTE's whole catalog is free since 2026-06-17.
- The 3GPP2 original of C.S0050-B no longer answers and is stored from a capture beside ARIB's transposition.
- The ID3v2.4 structure document is not in the ID3 repository and is a capture.
- The full ASF specification downloads as a Word document.
- ISO's free 2015 edition of ISO/IEC 14496-12 needs a browser.

**Code.** The modules are in `scripts/checks/specifications/documents/`, with the shared pacing in `throttle.ts` and the downloads in `downloads.ts`. The offline check verifies the rules of each retrieval, the pinned drafts of Appendix A.7, and the checksum of every document stored locally, in any folder. The hygiene check now accepts tabs that close the empty last cells of a tab-separated row.

**Open:** the 24 manual documents; G1.4 reviews the manifest.

## 2026-11-21 — Progress: Phase 1, tasks 9 and 10

**Task 9.** `docs/design/scope-baseline.md` copies the cross-check of plan section 4.5 with the baseline's revision `a8d5c34d4` of 2026-11-18, and a table of the comparisons made before each release.

**Task 10.** `docs/design/source-dossier.md` covers every † entry. Each candidate of Appendix B.6 was read, prose only, and better sources were searched for. Found beyond Appendix B.6:

- the vendor's own RealMedia file format reference, from its 2003 SDK documentation;
- the TTA authors' own format description;
- the official mapping of Dirac into Ogg;
- the SSA/ASS v4.00+ script format document;
- the USF specification as now hosted;
- the Library of Congress description of SubRip;
- the patents that the PGS description is based on, and published patents that describe HDMV TextST.

The dossier proposes a disposition for each entry. Where none is approved, it changes plan section 4.3 for RealVideo and the Blu-ray part of Phase 15.

## 2026-11-21 — Progress: Phase 1, task 11

**The tables.** `docs/specs/deliverables.tsv` has 339 keys:

| Kind | Keys |
|---|---|
| Detector entries | 65 |
| Container probers | 56 |
| Codec-by-carriage pairs | 183 |
| Codecs identified only | 11 |
| Tag families | 9 |
| Registry tables | 11 |
| Rules | 4 |

`docs/specs/coverage.tsv` gives each key a status and its sources:

- 309 active rows, each with at least one source of the two manifests;
- 16 deferred to G1.5, where no source was located;
- 14 deferred to G17.n, the detection and probing of the 7 post-1.0 candidates.

Every cited section is provisional until task 17 opens the stored document.

**The check.** The modules are in `scripts/checks/specifications/deliverables/`. The check compares the deliverables with the tables of plan sections 4.1 and 4.3 in the plan's own words, in both directions. It also compares the keys of the two tables and checks that every source names a document of a manifest. Two cells of section 4.1, Flash and ASF, were reworded so that each comma separates one format.

**Open:**

- Approval of sources. Nothing is approved before G1.4, G1.5 and G1.6, so the check does not yet require an approved source for an active row. The exit criteria of G1.9 require it, and the check gains that rule once the gates have filled the `approved` column.
- Fixtures are named in the phases that write them.

**Verification:** `node scripts/check.ts` passes every step on 2026-11-21, with 85 script tests.

## 2026-11-21 — Documents placed by the maintainer

| Document | What was done | Reasoning |
|---|---|---|
| Dolby Vision streams within ISO base media | The maintainer downloaded the PDF; it is stored as `dolby-vision-isobmff` and its checksum recorded. Its version could not be read from the file and is to be confirmed from its title page. | The vendor serves it only to a browser. |
| ITU-T H.222.0 | The maintainer chose an older edition: `itu-t-h-222-0` is now the 06/2021 edition, and a new row, `itu-t-h-222-0-amd-1`, holds its Amendment 1 (12/2022), the file the maintainer downloaded. Both download directly, so both are `fetch` rows. The 04/2025 edition is not stored. | The current edition needs a login. The amendment alone holds only the changes (20 pages), so the 2021 text it amends is stored as well. |
| ITU-T H.262 | `itu-t-h-262` is now the 02/2012 text published as Amendment 4 (2012-02-29), the file the maintainer downloaded, 238 pages. It downloads directly and is a `fetch` row. | The text in force needs a login; this one is the complete recommendation. |
| Dolby Vision in MPEG-2 transport streams | The maintainer supplied "Dolby Vision Streams Within the MPEG-2 Transport Stream Format", version 1.2, as a file for `dolby-vision-isobmff`. It is a different document, the carriage in transport streams, so it got a row of its own, `dolby-vision-mpeg-2-ts`, and a row in Appendix B.1; G1.4 reviews it with the rest. The ISO base media row keeps the document placed earlier. | Phase 10 needs it for Dolby Vision in transport streams; the MP4 document does not cover them. |
| Dolby TrueHD bitstream description | The maintainer could not find the document on the vendor's site and pointed to a copy in a third-party implementation's repository. That copy was not used: the row became a `snapshot` of the Internet Archive's capture of the vendor's own address (2025-09-08), the PDF of 2018-02-07, 36 pages. | The vendor's own bytes, without opening an implementation's repository or naming it here (plan section 1.3). |
| ITU-T T.81 | The maintainer downloaded the 09/1992 edition, 186 pages; it is stored as `itu-t-t-81` and its checksum recorded. | ITU serves it only to a browser. |
| ITU-R BT.2100-3 | The maintainer downloaded the 02/2025 edition, 15 pages; it is stored as `itu-r-bt-2100-3` and its checksum recorded. | ITU serves it only to a browser. |
| ITU-R BS.2088-2 | The maintainer downloaded the 11/2025 edition, 30 pages; it is stored as `itu-r-bs-2088-2` and its checksum recorded. | ITU serves it only to a browser. |
| Dolby TrueHD in ISO base media | The maintainer downloaded "MLP (Dolby TrueHD) streams within the ISO base media file format", the edition of 2019 (its PDF metadata calls it "Dolby TrueHD (MLP) bitstreams within the ISO base media file format"). It is the MP4-carriage document, so it is stored as `dolby-truehd-isobmff`, with the printed title and the 2019 edition, and not as the bitstream description. | The vendor serves it only to a browser. |

Still manual: the 16 paid ISO/IEC and IEC standards, for G1.6.

## 2026-11-21 — Paid standards: no purchases

The maintainer does not want to buy the 16 paid standards, and none of them is in hand. Asked how they are to be covered, the maintainer chose descriptive substitutes.

| Topic | Decision | Reasoning |
|---|---|---|
| Sourcing what the paid standards define | For each paid standard still missing, a free public description is researched (wiki pages and published articles, never an implementation's code), stored as a manifest row of class `descriptive`, and proposed at G1.6 in the way G1.5 approves descriptive sources. Behavior taken from it is labeled descriptive in code comments and in `formats.md`. Free official texts are used instead wherever they cover the same structures: ITU-T H.262 for MPEG-1 video, ITU-T H.222.0 for MPEG-1 systems, ISO/IEC 14496-12 for most of 14496-14. | No free official text covers the H.264, HEVC and VVC configuration records, AAC's AudioSpecificConfig, MP4's elementary stream descriptors or the MPEG-4 Visual headers. Deferring them would remove profile, level and audio parameters from 0.1.0. |

G1.6 itself, the decision per document, is still open; this entry records the direction its proposal takes.

## 2026-11-21 — Paid standards: descriptions in the repository

The maintainer's instructions: describe each of the 16 paid standards in a Markdown file in the repository, with content from online research; put the files in `docs/specs/iso-iec/`; move `docs/rfc/` to `docs/specs/rfc/`. Asked about a PDF reader, the maintainer approved installing poppler.

| Topic | Decision | Reasoning |
|---|---|---|
| Descriptions | One file per paid standard, `docs/specs/iso-iec/<identifier>.md`. Each one is in the project's own words, names the source of every statement (a manifest row or an RFC), and ends with its gaps and with what the project does until a source is approved. Field names keep the standards' spelling, because code comments cite them. | Nothing is copied from a standard the project does not hold. Every statement can be checked against a stored document. The gaps go to G1.6 openly. |
| Citations | A coverage row cites a description as `<identifier>.md: <section>`. A row that needs a structure no free source covers cites `<identifier>.md: Gaps`. The coverage check accepts the descriptions as documents (the folder's `README.md` excepted). | The rows show which deliverables rest on a gap, so G1.6 decides each one. |
| RFC folder | The RFC texts, `manifest.tsv`, `errata.tsv` and the generated `README.md` are in `docs/specs/rfc/`. The scripts, the hygiene exemption, `.gitattributes`, the plan and `CLAUDE.md` name the new path. | All specifications in one place. |
| Reading PDFs | poppler 26.11.0 is installed with Homebrew. Its `pdftotext` reads the stored PDFs, and `unzip` reads Word and zipped documents. Extracted text goes to standard output and is not stored. | Tools on the maintainer's machine only; no project dependency (GS.1 does not apply). |

## 2026-11-21 — Progress: descriptions of the paid standards

**The descriptions.** `docs/specs/iso-iec/` has 16 descriptions and an index, `README.md`. Their main free sources:

| Description | Main sources | Gaps |
|---|---|---|
| `iso-iec-14496-12.md` | Apple QuickTime File Format; 3GPP TS 26.244; MP4RA | Movie fragments other than `sidx` and `tfdt` |
| `iso-iec-14496-15.md` | the yohhoy notes; the CELLAR codec mappings | The VVC configuration record |
| `iso-iec-14496-1.md` | ETSI TS 102 428; MP4RA | The long form of the descriptor size rests on community sources only |
| `iso-iec-14496-14.md` | 3GPP TS 26.244; MP4RA | None the deliverables need |
| `iso-iec-14496-30.md` | W3C Breakouts Day 2025 talk; MP4RA; WebVTT | The box layout of `vttC` and `vlab` |
| `iso-iec-23003-5.md` | the iTeh preview; MP4RA | The channel layout box |
| `iso-iec-14496-3.md` | MultimediaWiki; the yohhoy notes; ETSI TS 102 005; RFC 6416 | SBR and PS signaling; LATM and LOAS |
| `iso-iec-13818-7.md` | MultimediaWiki | ADIF |
| `iso-iec-11172-3.md` | mp3-tech; RFC 2250 | The frame length formula |
| `iso-iec-13818-3.md` | mp3-tech | Samples per MPEG-2 layer III frame; the multichannel extension |
| `iso-iec-14496-2.md` | MP4RA, the CELLAR codec mappings, the FourCC lists and RFC 2361, for identification only | The visual object sequence and video object layer headers |
| `iso-iec-11172-2.md` | ITU-T H.262 (section 6.3.1, annex D.9) | The MPEG-1 aspect ratio table |
| `iso-iec-11172-1.md` | ITU-T H.222.0 and its Amendment 1 | The MPEG-1 pack header |
| `iec-61834-2.md` | SMPTE ST 314 (its annex A lists the differences from IEC 61834); RFC 6469; DV data in AVI | The consumer codes of the auxiliary packs |
| `iso-iec-14496-15-amd-1.md`, `iso-iec-14496-30-amd-1.md` | none needed | The deliverables use nothing that the amendments add |

Statements written from memory that no stored source supported were removed or turned into gaps: the ADIF signature, the MPEG audio frame length formula, the 576 samples of an MPEG-2 layer III frame, and the MPEG-1 pack header bits. The section types of DV were checked against SMPTE ST 314: its table 2 prints "Audio" for 100, and section 4.7.1 gives 100 to the video section.

**The coverage table.** 83 rows now cite descriptions instead of the paid standards, and 18 of them cite a `Gaps` section: ADIF (2 rows), LATM and LOAS (3), MPEG-4 Visual (7), the VVC configuration record and VVC's length-prefixed samples (4), and fragmented MP4 (2). Also corrected: the DV rows cited section 3 of RFC 6469, its media type parameters; the DIF structure is in section 2.1. The program-stream rows now also cite the MPEG-1 systems description, and the three MP4 rows for decoder-specific information also cite the descriptions of ISO/IEC 14496-1 and 14496-14.

**For G1.6** (proposals, not decisions):

- Approve each description as the substitute for its standard.
- For each gap: defer the dependent features, or find another source through GS.2.
  - Candidate sources found but not read: ISO's free 2015 edition of ISO/IEC 14496-12 (movie fragments), which only a browser can download, and US patents 5,907,444 and 6,101,070 (the consumer DV packs).
  - VVC's length-prefixed samples need the length size that the configuration record gives, so they depend on that gap too.

**Verification:** `node scripts/check.ts` passes every step on 2026-11-21.

## 2026-11-21 — G1.4 Specification set [CONFIRM]

The agent presented the review of the specification set: the RFC list and the RFC Editor's metadata, the errata table, the pinned drafts rechecked on the IETF datatracker on 2026-11-21 (codec -21 in IESG evaluation, tags -25 in the RFC Editor queue, chapter codecs -05 expired, all unchanged), the non-RFC manifest, the deliverables against plan section 4, and the coverage table's mapping. The maintainer approved the set and decided the open points.

| Topic | Decision | Reasoning |
|---|---|---|
| The specification set | Approved. The 114 normative and secondary rows of `docs/specs/manifest.tsv` that form the set have `G1.4` in their `approved` column. Still open: the 23 candidates of the source dossier (G1.5), and the 16 paid standards with the 9 rows that only serve their descriptions (G1.6): `etsi-ts-102428`, `iteh-iso-iec-23003-5-preview`, `mp3-tech-frame-header`, `multimedia-wiki-adts`, `multimedia-wiki-mpeg-4-audio`, `smpte-st-314`, `w3c-breakouts-2025-mse-text-tracks`, `yohhoy-isobmff-notes` and the link `iso-publicly-available-standards`. The RFC manifest has no `approved` column; this entry approves its rows. | Each row is approved by the gate that decides its kind of source (plan section 4.4). |
| RFCs that update listed ones | Added: the 8 RFCs in force that update a listed RFC (5147, 6657 and 8098 update RFC 2046; 5761, 7007 and 8860 update RFC 3551; 4788 updates RFC 3558; 8820 updates RFC 3986) and the 3 that update those in turn (8035 and 8858 update RFC 5761; 5188 updates RFC 4788). Nothing updates these 3. Left out: the obsoleted RFCs 2646 (by RFC 3676), 3798 (by RFC 8098) and 7320 (by RFC 8820), which the refresh keeps reporting as notes. | The maintainer chose to list the updates rather than only review them. With the second level, no listed RFC is updated by an unlisted RFC in force; an obsoleted RFC is never listed (Appendix A). |
| VC-1 in MPEG-2 streams | SMPTE RP 227, "VC-1 Bitstream Transport Encodings", the publication of 2010-08-12, is a new `fetch` row, `smpte-rp-227`, approved through GS.2 (`approved` is `GS.2`), and a row of Appendix B.2. The coverage row `pair.vc-1.advanced-sequence-header.ts-ps` cites its sections 5.1 (transport streams) and 6.1 (program streams). | The row cited only H.222.0 section 2.4.4, which does not define VC-1. RP 227 assigns the stream type 0xEA and the registration descriptor, and ETSI TS 101 154 refers to it. SMPTE's catalog is free. |
| Carriage sections of H.222.0 | The transport-stream rows of H.264, HEVC, VVC and MPEG-4 Visual also cite the carriage sections of the stored H.222.0 (06/2021): 2.14, 2.17, 2.23 and 2.11.2. | They cited only section 2.4.4, the program-specific information. |
| Committing the RFC texts | Confirmed. The texts stay committed verbatim, with the IETF Trust's license notice in the generated index. | The Trust Legal Provisions, section 3.c, allow copying and distributing IETF documents in full and without modification. |

## 2026-11-21 — Progress: Phase 1, task 12

**RFCs.** Appendix A lists the 11 new RFCs in sections A.2, A.3 and A.4, each with the phase of the RFC it updates. `node scripts/fetch-rfcs.ts` stored their texts (348 KB; the 85 texts are 7.0 MB), rewrote `docs/specs/rfc/manifest.tsv` and the index, and compared the set with the RFC Editor:

- No listed RFC is obsoleted, and every title and status matches.
- The notes name only the three obsoleted RFCs above as unlisted updates, and the same 36 held errata as on the first run; the new RFCs have none held.
- One verified erratum was new: RFC 5761 erratum 3380 corrects, in section 4, the RTP payload type that conflicts with receiver summary information packets from 80 to 81. It concerns RTP sessions and is classified `none`. The errata table has 74 rows.

**The other specifications.** `smpte-rp-227` is stored from SMPTE, with its checksum recorded; its sections were read for the citations above. The manifest has 163 rows: 114 approved at G1.4, 1 through GS.2, 48 open for G1.5 and G1.6.

**Plan.** Revision 12: Appendix A's introduction, A.8, Appendix B.2, the specifications of Phase 10, the progress of Phase 1 and the research of Appendix D record the outcome.

**Verification:** `node scripts/check.ts` passes every step on 2026-11-21.

**Open:** G1.5, the sources for formats without a normative specification, then G1.6.

## 2026-11-21 — G1.5 Sources for formats without a normative specification [CONFIRM]

The agent presented the proposals of the [source dossier](../design/source-dossier.md). The maintainer accepted them and asked for more research on the two entries whose sources had not been read; the results were presented and approved the same day.

| Topic | Decision | Reasoning |
|---|---|---|
| Descriptions approved | The 18 entries that the dossier proposed to approve: IVF, the RealMedia container, RealAudio headers, Wave64, TTA, the ALAC configuration, SubRip, SSA/ASS, USF, VobSub sub-pictures, DVD LPCM, the DVD information files, PGS, XSUB, Vorbis chapter comments, Dirac in Ogg, the Xing/Info header and ID3v1. Their 20 manifest rows, and the 3 that serve the reductions below (`dvdinfo-pci`, `multimedia-wiki-microdvd`, `xiph-wiki-kate`), have `G1.5` in `approved`, each for what the dossier says it covers. | Each was read and covers what its deliverables need; behavior taken from it is labeled descriptive (plan section 5.4). |
| Reductions | As proposed. Detection only: OGM, read as Ogg; MicroDVD, kept instead of dropped. Identification only: RealVideo and AAC in RealMedia, the VobSub palette in MP4, Kate. Standard structure only, extra data as raw bytes: Windows Media Audio, Windows Media Video 7/8. Container-declared values only: Blu-ray LPCM. VobButtons: Matroska identification, `btn` files a hint only (R22). Dropped from the inventory: RIFF CD-XA, HD-DVD subtitles, Windows Television, Internet Video Recording, RealAudio metafiles. Deferred: the Blu-ray part of Phase 15. | No public description of these was found, or none that covers more. |
| HDMV TextST | Approved: Sony's patent JP 5339002 B2 (`jpo-jp-5339002`), whose figures 11 to 17 give every field with its size, and LG's US 8,326,118 B2 (`uspto-us-8326118`), the same fields by name in English, as corroboration. Both are new rows approved through GS.2. TextST keeps its parser in Phase 14; the parser checks the fields against `segment_length`, and a segment whose fields do not add up gives a partial result. | Published patents are public documents, not implementations. They describe the applicants' proposals, and the two differ in detail; Sony shades its outline fields as its own invention. The length check keeps a difference from the disc format from producing wrong values. |
| VBRI header | Approved for the identifier, version, byte count and frame count: the 2012 capture of the CodeProject article "MPEG Audio Frame Header" (`codeproject-mpeg-audio-frame-header`), a new row through GS.2. Its prose and tables only; the article's code is not opened. | The byte and frame counts give the declared duration and are described consistently; two other rows of its table contradict themselves and are not read. |
| DVD private streams | Approved through GS.2: the dvdinfo page on how DVD differs from MPEG (`dvdinfo-dvdmpeg`), which gives the sub-stream ranges 0x20 to 0x3F (sub-pictures), 0x80 to 0x87 (AC-3, DTS, SDDS) and 0xA0 to 0xA7 (LPCM). An audio sub-stream's codec is identified by its sync word, from the approved ETSI specifications of AC-3 and DTS, whatever its number. | The dossier's open question. Other pages put DTS at 0x88 to 0x8F, outside the range the page gives; the sync word decides regardless. |
| Companion lookup | Unchanged: VobSub is still a consumer, so lookup ships in 0.7.0 (R18). | VobSub was approved. |

## 2026-11-21 — Progress: Phase 1, task 13

**The manifest** has 167 rows. `approved` holds G1.4 for 114, G1.5 for 23 and GS.2 for 5 (SMPTE RP 227 and the four sources found at G1.5); the 16 paid standards and the 9 rows that serve their descriptions wait for G1.6. The three new candidates and the DVD page were stored by `node scripts/fetch-specs.ts`, with their checksums.

**Reading the patents.** Both patents are PDFs with a text layer, read with `pdftotext`. The syntax figures of the Sony patent and figure 9B of the LG patent were rendered as images to be read and deleted afterwards; nothing but the PDFs is stored.

**Documents.** The source dossier records the decisions and what task 16 changes. Plan revision 12: Appendix B.6 names the sources found at G1.5, and Phase 1 reports the progress.

**Open:** G1.6, the paid standards. Task 16 then applies the outcomes of G1.5 and G1.6 to section 4, the detection candidates, the deliverables and coverage tables and the affected phases.

## 2026-11-21 — G1.6 Paywalled standards [CONFIRM]

The agent presented the descriptions of `docs/specs/iso-iec/` with their current gaps and the sources they cite. The maintainer decided:

| Topic | Decision | Reasoning |
|---|---|---|
| The 16 paid standards | Substitute, each by its description. The 16 rows are now `link` rows, listed by their catalog page only, with `G1.6` in `approved` and the description named in `applies_to`. The 9 rows that serve the descriptions (`etsi-ts-102428`, `iteh-iso-iec-23003-5-preview`, `mp3-tech-frame-header`, `multimedia-wiki-adts`, `multimedia-wiki-mpeg-4-audio`, `smpte-st-314`, `w3c-breakouts-2025-mse-text-tracks`, `yohhoy-isobmff-notes` and the link `iso-publicly-available-standards`) are approved at G1.6. | The maintainer buys none of them (log, 2026-11-21). A `link` row is not stored, so the fetch script no longer reports them as missing. |
| ISO/IEC 14496-12:2015 | The free 5th edition, on ISO's list of publicly available standards, is a new `manual` row, `iso-iec-14496-12-2015`, approved through GS.2. ISO serves it only after a license page in a browser, so the maintainer places it (G1.7). | It is the official text, older than the 2026 edition, and covers the movie fragments and compact sample sizes that the description leaves as gaps. |
| Documents the descriptions cite | The 14 documents that the descriptions cited by address only are new rows, approved through GS.2 and stored: iTeh samples of ISO/IEC 11172-1 Cor 2, 14496-3 Amd 3 (2005) and Amd 1 (2009), 14496-12:2008 FDAmd 3, 14496-14:2020, 14496-15 Amd 1, 14496-30 Amd 1 and 23000-19 Amd 2; the NTHU course slides with the MPEG-1 aspect ratio table; Microsoft's AAC media types; the Library of Congress page on ADIF; SMPTE ST 381-4, from SMPTE's own site rather than the mirror the description cited; MPEG input document m29240 from Télécom ParisTech; and Unified Streaming's timed-text notes. GOST R 54713-2011 is not added. Other addresses in the descriptions are other forms of existing rows: H.222.0, the MP4RA registries, the codec draft, Apple's QuickTime pages, the AV1 binding, ETSI's transposition of 3GPP TS 26.244, and SMPTE ST 314. | Task 17 confirms sections only in stored documents. The GOST page answered with a challenge page, and the rights of that copy are unclear. |
| Rows that rest on a gap | The 18 rows (ADIF 2, LATM/LOAS 3, MPEG-4 Visual 7, the VVC configuration record and length-prefixed samples 4, movie fragments 2) are reduced to what the free sources cover: signatures, identification, container-declared values, and a partial result where a structure is not read. The full parsers are deferred to G17.n and come back through GS.2 when a source is approved. The movie-fragment rows are reviewed again when the 2015 edition is in hand. | Deferring them entirely would remove the identification and container values that the free sources do support. |
| DV consumer packs | Sony's patent US 5,907,444 (`uspto-us-5907444`) is approved through GS.2 for the consumer pack headers and positions: the audio source pack 0x50 with its frame size, channel mode, 50/60 flag, signal type, sampling frequency (`SMP`: 0 = 48 kHz, 1 = 44.1 kHz, 2 = 32 kHz) and quantization (`QU`: 0 = 16-bit linear, 1 = 12-bit nonlinear), the audio source control 0x51, the video source 0x60 and the video source control 0x61. It gives the position of the display field but not its values, so the consumer aspect ratio stays absent and is deferred to G17.n. US 6,101,070, of the same family, adds nothing and was removed. | The patent's prose and its figures 31, 37 and 38 give the layout; it is a 1993 proposal, earlier than the 1998 standard, which the code comments state. |

Searches for a free source of the VVC configuration record found only implementations' code, which was not used (plan section 1.3), and a patent application that reproduces a draft of the record; nothing was proposed.

## 2026-11-21 — Progress: Phase 1, task 14

**The manifest** has 183 rows, every one approved: 114 at G1.4, 23 at G1.5, 25 at G1.6 and 21 through GS.2. `node scripts/fetch-specs.ts` stored the 14 new documents and the DV patent with their checksums; the 2015 edition of ISO/IEC 14496-12 is noted as missing until it is placed. The rendered figure pages of the patents were read and deleted; only the PDFs are stored.

**Documents.** Plan revision 12: section 5.2, Appendix B and task 15 record the outcome. The specification READMEs describe `link` rows for the paid standards and a manual file named by its media type.

**Verification:** `node scripts/check.ts` passes every step on 2026-11-21.

**Open:** G1.7, placing the 2015 edition by hand; then task 16, which applies the outcomes of G1.5 and G1.6 to plan section 4, the detection candidates, both tables and the affected phases, and updates the descriptions of IEC 61834-2 and ISO/IEC 14496-12 with their new sources; then task 17.

## 2026-11-21 — G1.7 Purchases [MANUAL]

Nothing was bought. The maintainer downloaded ISO/IEC 14496-12:2015, the 5th edition of 2015-12-15 from ISO's list of publicly available standards, 248 pages, as a PDF. It is stored as `iso-iec-14496-12-2015` with its checksum recorded. The row names no artifact address, because the PDF is the document inside ISO's free ZIP and has no address of its own; its edition cell names the ZIP. It holds the compact sample size box (8.7.3.3) and the movie fragment boxes (8.8.1 to 8.8.17), the two gaps it was chosen for.

## 2026-11-21 — Progress: Phase 1, task 16

**Section 4.** The outcomes of G1.5 and G1.6 are applied, in the plan's words that the deliverables check compares:

- *Formats (4.1).* OGM, raw LOAS streams, raw MPEG-4 Visual streams and the Blu-ray structures joined the post-1.0 candidates; an OGM file is read as Ogg, and `ogm` became an Ogg extension. RIFF CD-XA, HD-DVD subtitles, Windows Television, Internet Video Recording and RealAudio metafiles stay in the detect-only row, marked "dropped at G1.5". Disc structures hold DVD-Video only.
- *Pairs (4.3).* A carriage without an approved source is marked "deferred to G17.n": the VVC configuration record and length-prefixed NAL units, every MPEG-4 Visual carriage, RealVideo, AAC in RealMedia, LATM/LOAS, Blu-ray LPCM in transport streams, the VobSub palette in MP4 and Kate. The identification table gained their codecs, each naming its gate: VVC in Matroska and MP4, MPEG-4 Visual in four carriages, RealVideo, AAC in LATM and in RealMedia, VobSub in MP4, and Kate. The ADIF pair reads its header up to the program configuration elements; the VBRI pair reads the byte and frame counts; the Windows Media extra data is kept as raw bytes.
- *Sections 4.2, 4.4, 4.5, 7.7 and 8.6* and the scope baseline say the same; lookup ships in 0.7.0 with VobSub.
- *Phases.* Phase 4 (the VVC record, MPEG-4 Visual, the DV aspect ratio, the AAC program config element), 6 (sample entries), 9 (Kate and OGM), 10 (LATM, disc LPCM, DVD sub-streams), 11 (VBR headers, raw AAC), 12 (MPEG-4 Visual, the DV and Dirac sources), 13 (RealVideo, Windows Media, metafiles), 14 (lookup, TextST), 15 (DVD-Video only, now "DVD-Video and multi-file inputs", size M) and 17 (the deferred items).

**The tables.** 353 keys, 14 more than before: 13 identification rows and the probe entry of raw LOAS streams. Coverage: 308 active, 40 deferred to G17.n, 5 dropped at G1.5, none waiting for G1.5 or G1.6, and none citing a `Gaps` section. TextST is active from its patents, ADIF from the Library of Congress description, the fragment rows from the 2015 edition (clause 8.8), and the DV rows add the patent's pack figures. The VBRI row cites the article, and the program-stream rows the DVD sub-stream ranges.

**The descriptions.** `iec-61834-2.md` gained a section on the consumer packs from US 5,907,444, and its gaps now name only the display codes, the pack positions and two unread codes. `iso-iec-14496-12.md` cites the 2015 edition for the fragment boxes and `stz2`. Five descriptions that still called G1.6 pending now state its outcome.

**The check.** With every manifest row approved, the deliverables check gained the rule of plan section 5.2 that task 11 left for later: an active row must name at least one approved source, which is an RFC, a manifest row with a gate in `approved`, or a description of a paid standard. A test covers it.

**Verification:** `node scripts/check.ts` passes every step on 2026-11-21.

**Open:** task 17, the verification of every cited section against the stored documents; then tasks 18 and 19, G1.8 and G1.9.

## 2026-11-21 — Progress: Phase 1, task 17, release 0.1.0

The maintainer chose to verify the cited sections release by release, with a report after each, and to have wrong or vague citations corrected to the exact section of the same document, every correction listed here; only a row that no approved document supports goes back to the maintainer.

**Confirmed.** 160 of the 162 active rows of 0.1.0 have their `confirmed` date, 2026-11-21: each cited section was opened in the stored document and holds what the row needs. Two rows stay provisional; see below.

**Corrected** (157 changes; a correction of a citation that recurs was applied to every row that cites it, in every release):

| Citation | Now | Why |
|---|---|---|
| `rfc9559: 22` (WebM rows), `rfc9559: 5` (Matroska detection) | `rfc9559: 4.3`, and `webm-container-guidelines: Demuxer Guidelines` | Section 22 is Cues; the DocType rules are in 4.3 and in the WebM guidelines' demuxer section |
| `ietf-draft-cellar-codec: Codec Mappings` (45 rows) | The codec's own subsection, such as `V_MPEG4/ISO/AVC`, `A_AAC` or `S_HDMV/PGS`; `Codec Mapping` for the registry | The draft has no heading "Codec Mappings"; its section 3 is "Codec Mapping", with a subsection per codec ID |
| `itu-t-h-262: 6.2.2.1` | adds `6.3.3` | The sequence header's semantics |
| `etsi-ts-102366: 4.3`, `E.1.2` | add `4.4`, `E.1.3` | The semantics of the AC-3 and E-AC-3 elements |
| `etsi-ts-102114: 5.3`, `7.4`, `Annex E` | add `5.4`, `7.5.2`; `E.2.2` | The core frame header, the extension substream header, and the sample entry |
| `apple-quicktime-file-format: Sample description atoms`, `Video sample description`, `Closed captioning` | `Sample description atom ('stsd')`, `Video sample description ('stsd')`, `Closed captioning sample description` | The pages' exact titles |
| `apple-caf-specification: CAF File Overview` | `CAF File Header` | The overview is a separate document |
| `dolby-truehd-bitstream: Major sync`, `dolby-truehd-isobmff: MLPSpecificBox` | `4.2`, `2.5` | Section numbers |
| `iso-iec-11172-3.md: 2`, `iso-iec-14496-3.md: 2`, `iso-iec-14496-1.md: 4` | add sections 3 to 5, 3 and 4, and 3 | Rates, sample counts, sampling index, channel configuration, and the ES descriptor are in those sections |
| DV rows | add `iec-61834-2.md: 7` | The consumer packs |
| `smpte-rdd-36: Frame header`, `smpte-st-2042-1: 9`, `opendml-avi-1-02: 1` | `5.1.1` and `6.1.1`; `10.5.1`; `2.0` and `3.0` | Syntax and semantics of the ProRes frame header; the Dirac parse info header; the OpenDML extensions rather than the introduction |
| `ietf-draft-cellar-tags: 5` | `3` and `4` | Section 5 is Security Considerations |
| `3gpp-ts-26244: 5`, `3gpp2-c-s0050-b: 8` (detection and the speech codecs) | `5.3.4` and `8.1.1`; `8.4.3` to `8.4.7` | The brands, the file identification, and each speech codec |
| Page-level citations of descriptive sources | The pages' own headings, such as `IFO file`, `Sub-Pictures`, `Presentation Control Information (PCI) Packet Layout`, `RA Format`, `Universal Subtitle Format (USF) Specification v1.1`, `2.0` (WavPack), and the SSA document's section headings; `Introduction` for the lead text of a wiki page or article that has no heading before the layout (IVF, MicroDVD, Wave64, XSUB, PGS) | Every citation names a heading that exists |
| `microsoft-avi-riff-reference: AVI RIFF File Reference`, `adobe-flv-f4v-10-1: F4V file format` | `AVI RIFF Form`, `1` | The section that holds the form and its signature; the F4V section number |

**Not supported** (raised with the maintainer):

- `identify.vobsub.mp4`: the MP4 registration authority lists no VobSub object type; 0xE0 lies in the user-private range 0xC0 to 0xE0. The source dossier said the type was registered, which is corrected.
- `detect.rf64-bw64`: the stored EBU Tech 3306 is version 2 (2018), a three-page notice that refers to ITU-R BS.2088; it has no section 3, and neither document gives the RF64 chunk identifier. BS.2088-2 covers BW64. The earlier version 1.1 of Tech 3306 is still served by EBU as `tech3306v1_1.pdf`. EBU's publications page answered 403 to the agent, so that site is not asked again in this run.

**Decided by the maintainer** for the two rows:

| Row | Decision | Reasoning |
|---|---|---|
| `identify.vobsub.mp4` | Deferred to G17.n. Section 4.3 marks its carriage "deferred to G17.n", and Phase 6 no longer identifies VobSub in MP4. Such a track is reported as a private object type with its container values. | No registered object type and no public description identify it. |
| `detect.rf64-bw64` | EBU Tech 3306 version 1.1, the RF64 specification, is a new `fetch` row, `ebu-tech-3306-v1-1`, approved through GS.2 and added to Appendix B.3. It is downloaded in a later run, because EBU's site answered 403; the row stays provisional until then. | Version 2 refers to BS.2088, which covers BW64 only. |

**Open:** downloading `ebu-tech-3306-v1-1` and confirming the RF64 rows; task 17 for releases 0.2.0 to 0.8.0.

## 2026-11-21 — VobSub in MP4: a community source (GS.2)

The maintainer changed the decision above and asked for online research on VobSub in MP4.

**What exists.** No specification: neither the vendor that introduced the track nor ISO nor the MP4 registration authority describes it. The authority lists stream type 0x38 and object type 0xE0 only within user-private ranges, does not register the handler `subp`, and registers `mp4s` only as generic MPEG-4 Systems. The Library of Congress and MultimediaWiki pages on VobSub only say that it can be embedded in MP4. A forum thread of 2006 on Doom9 reports, from files its posters examined, the handler `subp`, the sample entry `mp4s`, stream type 0x38, object type 0xE0, a 90 kHz timescale and samples that are DVD sub-picture bitmaps. A layout of the palette in the decoder-specific information appears only on an implementation project's issue tracker, with two accounts that disagree on the byte order; it is not a source here (plan section 1.3), and implementation code that the searches pointed to was not opened.

| Topic | Decision | Reasoning |
|---|---|---|
| Source | Page 4 of the thread (posts 61 to 80), stored from an archive capture of 2026-01-13 as `doom9-vobsub-in-mp4`, a `descriptive` row approved through GS.2. | It holds posts 71, 77 and 80, which give the codes and the sample contents. Page 1 of the capture lacked them and was replaced. |
| Scope | Identification only: a track is VobSub when its handler, sample entry, stream type and object type all match those codes. It is reported with its timescale and the dimensions of its track header; the palette stays raw bytes, and the palette pair stays deferred to G17.n. | The codes are private, so only their combination identifies the track; nothing public describes the palette. |

`identify.vobsub.mp4` is active again for 0.1.0, confirmed against the stored page. Section 4.3, Phase 6, Appendix B.6 and the source dossier say so.

## 2026-11-21 — Progress: Phase 1, task 17, RF64 and release 0.2.0

**RF64.** `ebu-tech-3306-v1-1` was downloaded on the first request; EBU did not refuse it this time. Version 1.1 (July 2009) defines RF64 in section 3.4: the file starts with `RF64` instead of `RIFF`, and a mandatory `ds64` chunk follows, whose 64-bit sizes replace the 32-bit fields that hold 0xFFFFFFFF. Annex A.2 gives the `RF64` chunk with its form type `WAVE` and the `ds64` structure. BS.2088-2 gives the same for BW64 in sections 2.4, 3 and 4. Version 2 of Tech 3306 has no section 3, so its citation is replaced:

| Row | Was | Now |
|---|---|---|
| `detect.rf64-bw64` (0.1.0) | `ebu-tech-3306: 3; itu-r-bs-2088-2: 3` | `ebu-tech-3306-v1-1: 3.4`, `A.2`; `itu-r-bs-2088-2: 2.4`, `3` |
| `probe.rf64-bw64` (0.4.0) | the same | the same, and `itu-r-bs-2088-2: 4` for the `ds64` chunk |

`detect.rf64-bw64` is confirmed, so all 162 active rows of 0.1.0 are. Version 2 of Tech 3306 stays in the manifest as the notice that refers to BS.2088; no row cites it. One inconsistency was found: the field table of BS.2088-2 section 3.2 spells the BW64 identifier as the characters 'b', 'w', '6', '4', while its section 2.4 ("the ID 'BW64' is used instead of 'RIFF' in the first four bytes of the file") and the structure of section 3.1 write it in capitals. An online search found no source that reads it in lower case. Phase 11 of the plan now says that the identifier is in capitals.

**Release 0.2.0.** All 12 active rows are confirmed against the stored documents.

| Row | Was | Now | Why |
|---|---|---|---|
| `probe.flac` | `rfc9639: 6` | adds `8` | Section 6 is the layout overview; the metadata blocks that Phase 9 reads are in section 8 |
| `pair.flac.streaminfo.native-file` | `rfc9639: 8.2` | adds `8.1` | The block header that frames STREAMINFO |
| `pair.flac.streaminfo.ogg-mapping-header` | `rfc9639: 8.2; rfc3533: 6` | adds `rfc9639: 10.1` | The Ogg mapping header itself |
| `pair.opus.identification-header.ogg-packets` | `rfc7845: 5.1; rfc3533: 6` | adds `rfc7845: 3` | The placement of the identification header on the first page |
| `pair.speex.header-packet.ogg-packets` | `xiph-speex-manual: Ogg file format` | `7.3` | The section number of the heading |
| Theora pairs | `xiph-theora-specification: 6.2` | adds `6.1`, and `A.2.1` for Ogg | The common header that 6.2 starts with; the placement of the headers in Ogg |
| Vorbis pairs (one of 0.1.0) | `xiph-vorbis-specification: 4.2.2` | adds `4.2.1`, and `A.2` for Ogg | 4.2.2 starts after the common header; the Ogg encapsulation |
| Theora and Vorbis in Matroska | the codec ID subsection | adds `rfc9559: 10.3.2` | The Xiph lacing of the private data, to which the codec draft refers |
| `probe.ogg` | `rfc3533: 6` | adds `rfc3533: 4`, the Skeleton format section, and the granule sections of each mapping: `rfc7845: 4`, `xiph-theora-specification: A.2.3`, `xiph-vorbis-specification: A.2`, `rfc9639: 10.1`, `xiph-speex-manual: 7.3` | Section 4 defines multiplexing and chaining; Skeleton and the granule rules are part of the Ogg probe and its duration (Phase 9, tasks 3 and 5) |
| `tags.vorbis-comment` | `xiph-vorbis-comment: Comment field and header specification` | `Comment encoding`, `Field names` | The cited text is the page title; the structure and the field names are those sections |

Kate in Ogg (the wiki's format specification), the Vorbis chapter comments (the wiki's `Format` section) and FLAC in Matroska needed no change.

**Plan.** Theora's section A.2.3 says that streams before bitstream version 3.2.1, which have VREV 0 in the identification header, mark each frame's start rather than its end, and that one is added to the keyframe part of their granule position. Phase 9's duration rule said only "the granule shift for Theora", so it now names that correction. Appendix B.3 links version 1.1 of Tech 3306. These amendments are revision 13 of the plan.

**Verification:** `node scripts/check.ts` passes on 2026-11-21.

**Open:** task 17 for releases 0.3.0 to 0.8.0; then tasks 18 and 19, G1.8 and G1.9.

## 2026-11-21 — Progress: Phase 1, task 17, release 0.3.0

**Decided by the maintainer** (GS.2 for the sources):

| Topic | Decision | Reasoning |
|---|---|---|
| AV1 in transport streams | `aom-av1-mpeg2-ts` is pinned to `main/index.html` of the same commit, 0dc84bb, the working draft of 2026-10-08, and stored again; its `approved` cell is now `GS.2`. | The stored `index.html` was the site's list of preview builds (4.7 KB), not the specification. The specification moved to `main/` when the front page became that list. |
| 204-byte packets | ETSI EN 300 421 V1.1.2 is a new `fetch` row, `etsi-en-300421`. Clause 4.4.2 applies RS(204,188) to each 188-byte packet, keeping the periodicity of 204 bytes. The 16 parity bytes are skipped and not checked. | H.222.0 defines only 188-byte packets, and Phase 10 task 1 reads 204-byte ones as well. |
| 192-byte packets | The Blu-ray Disc Association's "BD-J Baseline Application and Logical Model Definition for BD-ROM" (March 2005) is a new `fetch` row, `bda-bd-j-baseline-application`, class `descriptive`. Its section 4.1 gives the source packet: a 4-byte `TP_extra_header`, then the 188-byte packet. The header is skipped and not reported. | It is the association's own text, still served by it. The maintainer first approved a BD-ROM patent and chose this document when it was found; it does not divide the header into fields, which only a patent does. |
| Blu-ray stream types | The published patent US 8,208,795 B2 (Panasonic) is a new `fetch` row, `uspto-us-8208795`, class `descriptive`. Its description of FIGS. 9A to 9E gives the stream coding types 0x80 LPCM, 0x81 AC-3, 0x82 DTS, 0x90 presentation graphics, 0x91 interactive graphics and 0x92 text subtitles. They are read as stream types of the program map, in 192-byte BDAV streams only. | H.222.0 leaves 0x80 to 0xFF to private use. The patent gives the values for a playlist's stream table; no source found says that the program map uses the same values, so that reading is this decision. |
| TrueHD in transport streams | `pair.truehd.major-sync.ts` is deferred to G17.n. A TrueHD stream in a transport stream is reported as a user-private stream type. | No approved source gives the Blu-ray types 0x83 (TrueHD), 0x84 (E-AC-3) or 0x85 and 0x86 (DTS-HD). The only page that lists them is an anonymous site that names no sources and gives text subtitles as 0x93, which contradicts the patent; the other lists come from implementations. |
| Opus in transport streams | The pair is split. `pair.opus.transport-stream-control-header.ts` (0.3.0) reads the access unit and its control header (section 6), on a stream identified by stream type 0x06 and the registration descriptor `Opus` (sections 5.1 and 4.3). `pair.opus.transport-stream-descriptor.ts` is deferred to G17.n. | The only source, the unfinished draft 0.1.3, leaves every tag of the descriptor as "0x??", and EN 300 468 gives extension tags 0x80 to 0xFF to users, so the descriptor cannot be found. An online search found no later version. |

**Confirmed.** All 27 active rows of 0.3.0 have their `confirmed` date, 2026-11-21. `detect.transport-stream` (0.1.0) cites the two new packet sources and is confirmed again.

**Corrected.** A correction that recurs was applied to every row that cites it, in every release:

| Citation | Now | Why |
|---|---|---|
| `itu-t-h-222-0: 2.4.4.9` (stream types) | `2.4.4.10` | In the stored edition, 06/2021, 2.4.4.9 is the syntax of the program map; Table 2-34 with the stream types is in 2.4.4.10 |
| `itu-t-h-222-0: 2.4.4` (21 rows) | `2.4.4.10` for transport streams, `2.4.3.7` for program streams, both for TS/PS rows | The clause is all of the program-specific information. A stream is identified by its stream type in the program map (Table 2-34) and, in a program stream, by its `stream_id` (Table 2-22) |
| `itu-t-h-222-0: 2.11.2` | `2.11.2.1` | Table 2-142 and the LATM rule are in that subclause; `identify.mpeg4-visual.ts-ps` also cites 2.5.4.2, since a program stream tells MPEG-4 Visual apart only through its map |
| `etsi-ts-101154: 4` | `4.1.6.1` | Clause 4 is the whole systems layer; the stream types of private audio are in 4.1.6.1 |
| `etsi-ts-103190-1: 4.3` | `4.2.3`, `4.3.3`, and `etsi-ts-103190-2: 6.2.1`, `6.3.2` | The table of contents, in the syntax and semantics of both parts; part 2 covers bitstream versions 1 and later |
| `aom-av1-mpeg2-ts: Start code based format` | `2.1`, `2.2`, `3.1`, `3.2`, `3.4`, and `aom-av1-specification: 5.3` | Section numbers of the specification page; the registration and video descriptors, stream type 0x06, the start-code format, `stream_id` 0xBD, and the OBU syntax that the start codes wrap |
| `smpte-rp-2025: 4` | `8` for Matroska; `4`, `6`, `7` and `8` for MP4 | Section 4 is the track definition; the sample entry, the `dvc1` box and the decoder configuration structure are 6, 7 and 8 |
| `smpte-st-421: 6.1` (advanced-profile sequence header, all four carriages) | adds `Annex E` | The representation is a start-code stream; Annex E gives the start codes and the removal of emulation prevention. The rows of 0.5.0 and 0.6.0 are confirmed in their releases |
| `dvdinfo-lpcm: Audio Pack for LPCM` | `Audio pack for LPCM` | The page's heading |
| `xiph-opus-in-mpeg-ts: 6` | adds `4.3` and `5.1` | The registration descriptor and the stream type that identify the stream |

**Completed** (sections that the rows needed and did not cite):

| Row | Adds | For |
|---|---|---|
| AC-3, DTS and AC-4 in transport streams | `etsi-ts-102366: A.2`; `etsi-ts-102114: F.3`, `F.4`; `etsi-ts-103190-2: D.2`; `etsi-ts-101154: 4.1.6.1`; `etsi-en-300468: 6.1`, `6.3`, `G.1`, `D.7` | Their stream types, `stream_id` and the descriptors that identify them in ATSC and DVB streams |
| AC-3 and DTS in program streams | `dvdinfo-dvdmpeg: Extensions` | The sub-stream numbers of private stream 1 |
| DTS core, PGS and HDMV TextST (0.7.0) in transport streams | `uspto-us-8208795: FIGS. 9A to 9E` | Their Blu-ray stream types |
| DVB subtitles in transport streams | `etsi-en-300743: 6.2`, `6.3` | The PES data field and stream type 0x06 with the subtitling descriptor |
| Subtitling and teletext descriptors | `etsi-en-300468: 6.1` | The descriptor tags 0x59 and 0x56 |
| VC-1 in transport and program streams | `smpte-rp-227: 5.2` | `stream_id` 0xFD and its extension |
| `probe.program-stream` | `itu-t-h-222-0: 2.4.3.6`, `2.4.3.7` | The PES packets, to which 2.5.3.7 refers |
| `probe.transport-stream` | `itu-t-h-222-0: 2.6`, `Annex A`; the two packet sources; `etsi-en-300468: 5.2.3`, `6.1`, `6.2.33`, `6.3`, `Annex A`, `D`, `G`, `H`; `dolby-vision-mpeg-2-ts: 3.1`, `3.3`; the patent | Descriptors, the CRC, 192- and 204-byte packets, service names and their character coding, the codec descriptors, dynamic-range signaling, and the Blu-ray types (Phase 10, tasks 1 and 7) |
| `registry.registration-descriptors` | `itu-t-h-222-0: 2.6.9`, and the sources of `AC-3`, `VC-1`, `AV01`, `Opus` and `DOVI` | The registered identifiers that the registry holds |
| `registry.transport-stream-types` | `itu-t-h-222-0-amd-1: 5`, `etsi-ts-102366: A.2.1`, `etsi-ts-102114: F.3`, `smpte-rp-227: 5.1.1`, the patent | Amendment 1 adds stream type 0x36 to Table 2-34; the private types of AC-3, DTS, VC-1 and Blu-ray |

**Plan and dossier.** Revision 14 of the plan: Appendix B.1 adds EN 300 421 and pins the AV1 row to the specification page. Appendix B.6 and the source dossier add the Blu-ray sources. Phase 10, tasks 1, 7 and 9, and section 4.3 apply the decisions above. Appendix B.1 also named H.222.0 (04/2025), the edition that needs a login; it now names the stored 06/2021 edition with its Amendment 1, which the maintainer chose when the documents were placed.

**Verification:** `node scripts/check.ts` passes on 2026-11-21.

**Open:** task 17 for releases 0.4.0 to 0.8.0; then tasks 18 and 19, G1.8 and G1.9.

## 2026-11-21 — Progress: Phase 1, task 17, release 0.4.0

**Decided by the maintainer** (GS.2 for the sources):

| Topic | Decision | Reasoning |
|---|---|---|
| DTS byte-swapped and 14-bit variants | ETSI TS 102 114 V1.2.1 (2002-12) is a new `fetch` row, `etsi-ts-102114-v1-2-1`. Its section 5.3 gives the sync word as 0xfe7f 0x0180 when the bytes of each 16-bit word are swapped, and a 14-bit format that holds the stream in the low 14 bits of each 16-bit word, with the sync word in three words, 0x1fff, 0xe800 and 0x07f. The frame header still comes from V1.6.1. | The stored V1.6.1 mentions 14-bit streams only to align two sync words; V1.3.1 already dropped the text. |
| TrueHD interleaved with AC-3 | The published patent US 8,086,331 B2 (Panasonic, 2011) is a new `fetch` row, `uspto-us-8086331`, class `descriptive`. Its description of FIG. 7 calls the stream "DD/MLP": AC-3 and MLP audio frames are multiplexed in one primary audio stream at very different rates, and the AC-3 frames are the base data. Each frame is read by its own sync word and length. | The Dolby description does not mention the variant. The patent gives no stream type and no exact order of the frames. |
| WavPack sample rate | A native file reports its rate from the WAVE header that its sub-block `ID_RIFF_HEADER` stores. Without it the rate is not available, and the index is kept raw; the table of standard rates is deferred to G17.n. | The format document gives a 4-bit index into "15 standard rates" and does not list them; its text of 2009 and the MultimediaWiki page do not either, and only implementations do. Matroska stores the rate itself. |
| ID3v2 tags in WAVE | A chunk whose data starts with a valid ID3v2 header (ID3v2.4 structure, section 3.1), and whose tag size fits the chunk, is read as an ID3v2 tag; its identifier is reported as found. | No document assigns the chunk. Community reports name three identifiers, and the Microsoft page that an encyclopedia cites for ID3 in WAVE does not mention WAVE. |
| APEv2 | Five new `snapshot` rows, stored from archive captures: `hydrogenaudio-ape-tags-header` (header and footer), `-ape-tag-item`, `-ape-tags-flags`, `-ape-key` and `-ape-item-value`. | The stored specification page is an overview of the order of header, items and footer; the fields are on the pages it links. |
| Where the Xing/Info header starts | `codeproject-mpeg-audio-frame-header`, approved at G1.5 for the VBRI fields only, is also approved for its sections 2.2 and 2.3.1: the Xing/Info header follows the 4-byte frame header and the Layer III side information, 32, 17 or 9 bytes by version and channel mode. | The Info tag page places the `Xing` string only "mostly at 0x24", and the ISO/IEC 11172-3 description does not cover side information. |

**Confirmed.** All 32 active rows of 0.4.0 have their `confirmed` date, 2026-11-21. The rows of 0.1.0 and 0.3.0 that the corrections below changed were checked again against the stored documents and keep their dates; `pair.pcm.wave-format.asf-stream-format` (0.6.0) is confirmed in its release.

**Corrected.** A correction that recurs was applied to every row that cites it, in every release:

| Citation | Now | Why |
|---|---|---|
| `microsoft-riff-mci-1-0: 3` and `2` | `Waveform Audio File Format (WAVE)`; `INFO List Chunk` and `CSET (Character Set) Chunk` | The document has no numbered sections. Chapters 3 and 2 hold every multimedia format and all of RIFF |
| `microsoft-waveformatex: Remarks` (`detect.wave`, `probe.wave`) | `Members` | The remarks give only an example of extra data for MS-ADPCM |
| `microsoft-waveformatextensible: Members` (four rows) | adds `Remarks` | The speaker positions of `dwChannelMask` and the subformats are in the remarks |
| `loc-aac-adif: Description` | `Self-documentation` | The description says only that a brief header precedes the data; the header's fields are under "Self-documentation" |
| `apple-alac-magic-cookie: ALACSpecificConfig` | `ALAC Specific Info (24 bytes) (mandatory)`, `Channel Layout Info (24 bytes) (optional)`, `Channel Layout Tags`, and `CAF File` or `MP4/M4A File` by carriage | The name of a structure, not a heading; the cookie includes the optional channel layout |
| `dolby-truehd-bitstream: 4.2` | adds `3.3` for the major sync pairs; `4.1` and `5` for raw files | The syntax of `major_sync_info`; the access unit length, and the two file types, with or without the SMPTE timestamp header |
| `mp3-tech-info-tag: Info tag`, `hydrogenaudio-apev2: Specification` | `Introduction` | Neither page has a heading; the convention of 0.1.0 for pages without one. The new APEv2 pages are cited the same way |
| `id3-v1: ID3v1` | `What is ID3 (v1)?`, `What is ID3v1.1?` | The page's headings |
| `id3-v2-4-frames: 4` | `4.2`, `4.14` | Section 4 holds every declared frame; Appendix B.5 names text frames and attached pictures |

**Completed** (sections that the rows needed and did not cite):

| Row | Adds | For |
|---|---|---|
| ADTS pairs (0.1.0, 0.3.0, 0.4.0) | `iso-iec-14496-3.md: 3`, `4` | The sampling-frequency index and channel configuration to which the ADTS description refers |
| `probe.aac` | `iso-iec-13818-7.md: 3`, `loc-aac-adif: Self-documentation` | ADIF files |
| AC-3 raw file and `probe.ac3` | `etsi-ts-102366: E.2.8` | Channel and programme extensions: how dependent substreams replace or add channels (Phase 11 task 7) |
| `probe.caf` | `CAF Chunk Header`, `Audio Description Chunk`, `Audio Data Chunk`, `Packet Table Chunk`, `Channel Layout Chunk`, `Magic Cookie Chunk`, `Information Chunk` | Phase 11 task 4 |
| `probe.dts` | `etsi-ts-102114: 7.4`, `7.5.2`, and V1.2.1 `5.3` | DTS-HD streams and the variants |
| `pair.mpeg-audio.vbr-headers.raw-file` | `codeproject-mpeg-audio-frame-header: 2.2`, `2.3.1` | Where the Xing/Info header starts |
| `probe.tta` | `TTA Frame Structure` | The frame length and the trailing tags |
| WavPack | `wavpack-5-file-format: 3.0` for the native pair; `1.0` to `4.0` for the probe; `ietf-draft-cellar-codec: 4.1` and `wavpack-5-file-format: 2.0` for Matroska | Metadata sub-blocks, multichannel block sequences and trailing tags; the reduced block header, to which `A_WAVPACK4` refers |
| `probe.wave` | `microsoft-riff-mci-1-0: Chunks`, `LIST Chunk`; `ebu-tech-3285: 2.3`; `id3-v2-4-structure: 3.1` | The chunk rules, the INFO list, the broadcast extension and embedded ID3v2 tags (Phase 11 task 3). Tech 3285 was in the manifest, and no row cited it |
| `probe.wave64` | `Waveform Audio File Format (WAVE)`, `microsoft-waveformatex: Members` | The chunks that Wave64 carries in 64-bit chunks |
| `detect.truehd` | `dolby-truehd-bitstream: 5`, the new patent | The two file types, and files of the interleaved variant |
| ID3v2 tags | `4.2`, `4.15` and `5` of versions 2.2 and 2.3; `4`, `5` and `6` of the 2.4 structure | Text frames, attached pictures, unsynchronization; the text encodings of version 2.4 and its tag location with the footer |
| `tags.id3v1` | `id3-v2-3: 8.1` | The genre list, which the ID3v1 page does not give |

`probe.rf64-bw64`, MPEG audio frame headers, the DTS core and extension substream pairs, and TTA headers needed no change.

**Fetching.** The Internet Archive slowed down while the APEv2 pages were stored: the fetch script backed off to 96 seconds, and one page failed. It was stored on a second run a few minutes later, which backed off the same way.

**Plan and dossier.** Revision 15 of the plan: Appendix B.3 adds TS 102 114 V1.2.1, Appendix B.5 the five APEv2 pages, and Appendix B.6 the patent and the two topics without a source. Phase 11, tasks 3, 5, 8, 9 and 10, applies the decisions above. The source dossier adds the same topics and the widened approval of the VBRI article.

**Verification:** `node scripts/check.ts` passes on 2026-11-21.

**Open:** task 17 for releases 0.5.0 to 0.8.0; then tasks 18 and 19, G1.8 and G1.9.

## 2026-11-21 — Progress: Phase 1, task 17, release 0.5.0

**Decided by the maintainer** (GS.2 for the source):

| Topic | Decision | Reasoning |
|---|---|---|
| Dolby Vision metadata in raw byte streams | The published patent US 10,462,491 B2 (Dolby, 2019) is a new `fetch` row, `uspto-us-10462491`, class `descriptive`. Its Table 1 marks the metadata units of an HEVC enhancement-layer stream by the two bytes 0x7C01 after the start code (HEVC type 62). Such a unit in any HEVC byte stream reports Dolby Vision metadata as present. H.264, H.266, and enhancement-layer units carried in the same stream are deferred to G17.n. | Phase 12 task 1 asks for these units. The stored Dolby Vision documents say only that the metadata unit is "a special NAL unit", and Dolby's profiles document, read in an archive capture, says "unspecified NAL units". The patent's table describes the enhancement-layer stream of a dual-layer stream. |
| IVF codec codes | `VP80` is VP8, as the IVF description gives it. Any other code is reported as found, and the codec is identified only when the first frame matches exactly one of these: a VP8 key frame (RFC 6386, 9.1); a VP9 key frame with its frame marker and sync code (VP9 6.2, 7.2); or an AV1 temporal unit in the low-overhead format that starts with a temporal delimiter (AV1 5.2, 5.3, 7.5). | The description names only `VP80`, as an example. No document other than implementation code gives the codes for VP9 and AV1, or says that AV1 frames in IVF use the low-overhead format. |

**Confirmed.** All 23 active rows of 0.5.0 have their `confirmed` date, 2026-11-21. The rows of 0.1.0 and 0.3.0 that the corrections below changed were checked again against the stored documents and keep their dates. `pair.av1.low-overhead.flv-packets` (0.6.0) will be confirmed in its own release.

**Corrected.** A correction that recurs was applied to every row that cites it, in every release:

| Citation | Now | Why |
|---|---|---|
| `multimedia-wiki-ivf: Frame header` (three IVF pairs) | `Introduction` | The page has no such heading. The frame header is in its lead text, so it follows the 0.1.0 convention |
| `smpte-st-2042-1: 10` (Dirac pairs), `10.5.1` (`probe.dirac`) | `10`, `11`, `Annex A`, `Annex B` | Clause 10 covers the stream and its parse info headers. The sequence header is clause 11, its values are coded as Annex A gives, and the base video formats that it overrides are in Annex B |
| `itu-t-h-262: 6.2.2.1`, `6.3.3` (five MPEG video pairs, 0.1.0 to 0.5.0) | adds `6.2.2.3`, `6.2.2.4`, `6.3.5`, `6.3.6` | The representation is "Sequence header and extensions", which includes the sequence extension and the sequence display extension, with their semantics |
| `aom-av1-specification: 5.3` for the low-overhead format (five pairs, `detect.av1-obu`, `probe.av1-obu`) | adds `5.2` | Section 5.2 defines the low-overhead format; section 5.3 defines only the OBU |

**Completed** (sections that the rows needed and did not cite):

| Row | Adds | For |
|---|---|---|
| `pair.dirac.sequence-header.ogg-packets` | `dirac-ogg-mapping: 4`, `6.1` | The mapping approved at G1.5, which no row cited: the first packet starts with a sequence header, and section 6.1 says what a packet holds |
| `pair.av1.length-delimited.raw-file` | `aom-av1-specification: 5.3` | Annex B wraps the OBUs of section 5.3 |
| `detect.h264`, `detect.hevc`, `detect.vvc` (0.1.0) and the matching probes | H.264 `7.3.1`, `7.4.1`; H.265 and H.266 `7.3.1`, `7.4.2` | The NAL unit header, which tells parameter sets apart and which multi-unit validation checks (Phase 12 tasks 1 and 6) |
| `probe.h264`, `probe.hevc`, `probe.vvc`, `probe.av1-obu` | H.264 `E.1.1`, `E.2.1`; H.265 `E.2.1`, `E.3.1`; H.266 `7.3.5.1`, `7.3.5.2`, `7.4.6.1`, `7.4.6.2`; AV1 `5.5.3`, `6.4.3`; the new patent for HEVC | The timing information that gives a raw stream its frame rate (Phase 12 task 1), and Dolby Vision metadata units |
| `probe.ivf` | `rfc6386: 9.1`; `webm-vp9-bitstream-0-7: 6.2`, `7.2`; `aom-av1-specification: 5.2`, `5.3`, `7.5` | The identification by content decided above. The VP9 and AV1 IVF pairs add the same sections |
| `probe.dv`, `probe.mpeg-video`, `probe.vc-1`, `probe.dirac` | The sections of their raw-file pairs | A raw stream's probe reports what its pair reads, as the audio probes of 0.4.0 do |

No change was needed for the Annex B pairs of H.264, HEVC and VVC, the VP8 and VP9 frame headers, or the DV and VC-1 pairs.

**Found for a later release.** The stored Dolby Vision document for ISO base media files is version 2.8 (9 July 2026), but its manifest row and Appendix B.1 name v2.5. No row of 0.5.0 cites it; the verification of 0.6.0 takes it up.

**Fetching.** The patent was stored in one request. The stored copy is a scan without a text layer, so its front page and Table 1 were read as page images.

**Plan and dossier.** Revision 16 of the plan adds the patent to the Phase 12 specifications, and Phase 12 tasks 1 and 3 apply the decisions above. Appendix B.6 adds the two topics and now links the Dirac mapping itself. The source dossier adds a Video section with the Dolby Vision entry, and adds the IVF codes to the IVF entry.

**Verification:** `node scripts/check.ts` passes on 2026-11-21.

**Open:** task 17 for releases 0.6.0 to 0.8.0; then tasks 18 and 19, G1.8 and G1.9.

## 2026-11-21 — Progress: Phase 1, task 17, release 0.6.0

**Decided by the maintainer** (GS.2 for the sources):

| Topic | Decision | Reasoning |
|---|---|---|
| H.263 in FLV | Adobe's SWF File Format Specification, version 19 (2012), is a new `snapshot` row, `adobe-swf-file-format-19`, class `normative`, stored from the Internet Archive capture of Adobe's address of 2013-02-02. Its chapter 14, "Sorenson H.263 Bitstream Format", gives the picture header that `pair.h-263.flash-picture-header.flv-packets` reads. | The plan said that the FLV specification defines this header, but FLV 10.1 says "For all but AVCVIDEOPACKET, see the SWF File Format Specification". The SWF table refers to the H.263 clauses 5.1.x for the meaning of its fields. adobe.com answered 403, so it was not asked again. |
| Windows Media codes | Microsoft's pages "Video Subtype GUIDs" and "Audio Subtype GUIDs" are new `snapshot` rows, pinned to the documentation commit of the other Learn pages. The video page gives `WMV1`, `WMV2`, `WMV3` and `WVC1`; the audio page gives the format tags 0x000A and 0x0161 to 0x0163. The wiki's list of format tags, `multimedia-wiki-twocc` (revision 16072), class `descriptive`, is approved for 0x0160 alone. | No stored document gave these codes; the ASF specification lists only 0x0161 to 0x0163 (11.1). No Microsoft document gives 0x0160, for version 1. |
| ASF tags | Microsoft's "Attribute List" of ASF attributes is a new `snapshot` row, `microsoft-asf-attribute-list`, for normalized keys and value types. `WM/Picture` is kept as a binary tag, and no picture fields are decoded. | The ASF specification names only the content description fields and the aspect-ratio attributes. Microsoft documents `WM_PICTURE` only as an in-memory structure with pointers, so no document gives the layout a file stores. |
| RealVideo in RealMedia | Reported at container level: its kind from the top-level type of the MIME type, with the declared bit rates and duration, and no codec name. | Neither approved description says where the code of a video stream sits in its type-specific data, or names the video MIME type; the wiki's RealVideo page gives no layout either. |
| Windows Media Audio extra data | For 0x0161, `dwSamplesPerBlock`, `wEncodeOptions` (as a raw number) and `dwSuperBlockAlign` are read as typed details, in every carriage. Other tags keep raw bytes. | ASF section 11.1.1 defines them, while section 4.3 said that no public description existed. The extra data of a wave format belongs to its format tag. |
| Dolby Vision in ISO base media files | The stored version 2.8 (9 July 2026) is adopted. The manifest and Appendix B.1 record it, and `probe.mp4` (0.1.0) cites its section 3.2. | The plan named v2.5. Version 2.7 assigned reserved bits of the configuration record to feature flags, and 2.8 added CMAF profiles. |

**Confirmed.** All 33 active rows of 0.6.0 have their `confirmed` date, 2026-11-21, including `pair.av1.low-overhead.flv-packets`, `pair.pcm.wave-format.asf-stream-format` and the VC-1 ASF pair, which earlier releases had changed. The six rows of 0.1.0 that the corrections below changed were checked again against the stored documents and keep their dates.

**Corrected.** A correction that recurs was applied to every row that cites it, in every release:

| Citation | Now | Why |
|---|---|---|
| `adobe-flv-f4v-10-1: Annex E` (seven rows, `detect.flv` of 0.1.0 among them) | The subsections that each row reads: `E.2` for detection; `E.4.2.1` and `E.4.2.2` for AAC; `E.4.2.1` for MPEG audio; `E.4.3.1` and `E.4.3.2` for H.264; `E.2` to `E.5` for `probe.flv`; `E.4.2.1` and `E.4.3.1` for the codec registry | Annex E is the whole FLV format |
| `adobe-flv-f4v-10-1: Annex E` for AV1 (two rows) | `veovera-enhanced-rtmp-v2: Enhanced Video` | FLV 10.1 has no AV1; Enhanced RTMP carries it under the code `av01` |
| `veovera-enhanced-rtmp-v2: ExVideoTagBody (SequenceStart)`, `ExVideoTagBody (CodedFrames and CodedFramesX)`, `Enhanced codecs` | `Enhanced Video`; `Enhanced Audio` and `Enhanced Video` for the registry | These are parts of a table, or no heading at all; the page's headings are "Enhanced Audio" and "Enhanced Video" |
| `microsoft-asf-specification: 3` (`detect.asf`, `detect.wma`, `detect.wmv` of 0.1.0) | `3.1`, `10.1` | Detection reads the Header Object, whose GUID is in 10.1 |
| `realnetworks-rmff: Header Section` (`detect.realmedia` of 0.1.0) | `RealMedia File Header` | Detection reads the `.RMF` object |
| `pair.h-263.flash-picture-header.flv-packets`: `Annex E`, twice | `adobe-swf-file-format-19: Sorenson H.263 Bitstream Format`, `itu-t-h-263: 5.1`, `adobe-flv-f4v-10-1: E.4.3.1` | The decision above |
| `identify.realvideo.realmedia`: `multimedia-wiki-realmedia: Codecs` | `rfc6838: 4.2` | The decision above: the kind comes from the top-level type of the MIME type, and the codec is not named |

**Completed** (sections that the rows needed and did not cite):

| Row | Adds | For |
|---|---|---|
| The ASF pairs of PCM, WMA, WMV and VC-1 | `microsoft-asf-specification: 9.1` (audio) or `9.2` (video); `5.2.3` for the VC-1 payloads | The media types that fill the type-specific data of section 3.3 |
| The three WMA pairs | `microsoft-asf-specification: 11.1`, `11.1.1`; the audio subtype page; the list of format tags | The format tags, and the extra data of 0x0161 |
| The two WMV pairs and the four VC-1 pairs in ASF, AVI and Matroska | The video subtype page (`Encoded Video Types`); for the simple- and main-profile structure, `rfc4425: 1` and, in AVI and Matroska, `microsoft-bitmapinfoheader: Members` | The codes; Windows Media Video 9 as VC-1; the bitmap header that carries the structure |
| The FLV pairs of AAC, MPEG audio and H.264 | `veovera-enhanced-rtmp-v2: Enhanced Audio` or `Enhanced Video` | The enhanced headers carry them too, under `mp4a`, `.mp3` and `avc1` |
| `pair.av1.low-overhead.flv-packets` | `aom-av1-specification: 7.5` | A body of coded frames holds one temporal unit, as an IVF frame does |
| `probe.asf`, `probe.wma`, `probe.wmv` | `2`, `3`, `4.1`, `4.6`, `4.7`, `4.8`, `4.13`, `5`, `7.4`, `7.5`, `9`, `10`, and the attribute list | What Phase 13 task 2 reads: the objects, the header extension objects, the first payloads, the rules for metadata, the pixel aspect ratio, the media types and the GUIDs. The three probes share one reader |
| `probe.flv` | Enhanced RTMP's `Enhanced Audio`, `Enhanced Video`, `Enhancing onMetaData` and `Multitrack Streaming via Enhanced RTMP` | The extended and multitrack headers, and the metadata (Phase 13 task 1) |
| `probe.realmedia` | The RMFF headers it reads, the RealAudio type-specific data, the wiki's codec list, `rfc6838: 4.2` | Phase 13 task 4 |
| `probe.realaudio`, `pair.realaudio.file-headers.standalone-file` | `audio/x-pn-realaudio and audio/x-pn-multirate-realaudio`, `Codecs` | Version 5 is laid out only in the section on type-specific data, and the code names the codec |
| `identify.aac.realmedia`, `pair.realaudio.type-specific-data.realmedia-stream-properties` | `Codecs`, and for AAC the RMFF media properties header | The codes `raac` and `racp`, and those of RealAudio |
| `probe.mp4` (0.1.0) | `dolby-vision-isobmff: 3.2` | The decision above |

No change was needed for `registry.asf-identifiers`, `registry.realmedia-fourcc` or the RealAudio pair in Matroska.

**Fetching.** The SWF specification was read from the archive capture to standard output before the decision, then stored; adobe.com answered 403 and was not asked again. The Microsoft pages were stored from the documentation repository at the pinned commit, after one request each to find their last change, and the wiki page at its revision. Requests went one at a time, a few seconds apart for each site.

**Plan and dossier.** Revision 17 of the plan changes section 4.3 (the WMA row and the identification of RealVideo), section 4.4 (RealMedia), Phase 6 task 6 (Dolby Vision 2.8), Phase 13 (its specifications and tasks 1 to 4), and Appendices B.1 (SWF, the subtype pages, Dolby Vision 2.8), B.5 (the attribute list) and B.6 (the WMA extra data, 0x0160, the RealVideo codes, ASF pictures). The deliverables table follows the WMA row. The source dossier adds rows and task-17 notes for RealMedia, WMA, 0x0160 and ASF pictures.

**Verification:** `node scripts/check.ts` passes on 2026-11-21.

**Open:** task 17 for releases 0.7.0 and 0.8.0; then tasks 18 and 19, G1.8 and G1.9.
