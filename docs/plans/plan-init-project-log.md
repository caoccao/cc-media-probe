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

## 2026-11-21 — Progress: Phase 1, task 17, release 0.7.0

**Decided by the maintainer** (GS.2 for the source):

| Topic | Decision | Reasoning |
|---|---|---|
| XML for USF | W3C XML 1.0 (Fifth Edition), the Recommendation of 2008-11-26, is a new `snapshot` row, `w3c-xml-1-0`, class `normative`. The USF rows cite the sections their parser reads, and the contract of G14.1 names it. | The USF specification describes its elements, not the XML syntax they are written in, and no approved document defined that syntax: the declaration and its encoding, comments, references and the predefined entities. The Fifth Edition is still the current one. |
| PGS frame rate | Not reported. Phase 14 task 9 reads canvas size, display-set count and time range. | The approved article says that the byte is "Always 0x10. Can be ignored." and gives no meaning to its values. |
| HDMV TextST totals | The event count is the number of presentation segments; there is no time range. | LG's FIG. 10 names the start and end timestamps of a presentation segment without their size, and Sony's patent does not describe that segment. Sony's figure 11 gives the count, and the walk by segment length confirms it. |

**Confirmed.** All 17 active rows of 0.7.0 have their `confirmed` date, 2026-11-21. The rows of earlier releases that the corrections below changed were checked again against the stored documents and keep their dates.

**Corrected.** A correction that recurs was applied to every row that cites it, in every release:

| Citation | Now | Why |
|---|---|---|
| `w3c-webvtt: 4` (`detect.webvtt` of 0.1.0, `probe.webvtt`) | `4.1`; for the probe also `4.3` and `rfc8216: 3.5` | Section 4 is the whole syntax; 4.1 gives the file structure, 4.3 the region settings, and the probe reports the timestamp map |
| `titlevision-usf: Universal Subtitle Format (USF) Specification v1.1` (four rows, `detect.usf` of 0.1.0 among them) | The numbered sections: `3` for detection; `3`, `3.1`, `3.2` for the headers; `3.4` for the languages of the probe; `3.5` for the prolog of a file | The text is numbered, and its title names the whole document |
| `ietf-draft-cellar-codec: S_VOBSUB` alone (the index pairs of a file, 0.7.0, and of Matroska, 0.1.0) | adds `5.1`; `detect.vobsub` of 0.1.0 cites both instead of `dvdinfo-spu: Sub-Pictures` | Section 5.1 lays out the index file and its first line, which detection reads; the sub-picture page does not describe the index |
| A codec subsection of the Matroska draft that defers to section 5 (six rows of 0.1.0 and 0.7.0) | adds `5.3` (SSA/ASS), `5.4.1` (WebVTT), `5.5.2` (PGS), `5.6.1` (TextST), `5.7.1` and `5.7.2` (DVB subtitles) | The private data or block layout is given there; for DVB, `5.7.1` holds the five bytes of page identifiers, whose meaning is `etsi-en-300468: 6.2.41` |
| `pair.webvtt.container-header.*` (two rows of 0.1.0) | `w3c-webvtt: 4.1` for the header text; Matroska keeps the codec draft, MP4 keeps `iso-iec-14496-30.md: 2` and `iso-iec-14496-12.md: 3` | The Matroska row cited the MP4 description and the MP4 row the Matroska draft |
| TextST in Matroska and in transport streams | `jpo-jp-5339002: Figures 12 to 17`, `uspto-us-8326118: FIGS. 9A to 9D`; for transport streams also Sony's `Figure 10` and LG's `FIG. 6` | The private data and a PES packet hold a segment, not the file of figure 11; figures 10 and 6 say that the first PES packet is the style segment |

**Completed** (sections that the rows needed and did not cite):

| Row | Adds | For |
|---|---|---|
| `pair.ssa.file-header.file`, `pair.ssa.private-data-header.matroska-private-data` (0.1.0), `probe.ssa-ass` | `General information`, `Style lines, [v4 Styles] section`; for the probe also `Dialogue event lines, [Events] section` and `Appendix B: Embedded font/picture encoding` | The line syntax, the styles that the header holds, the event times, and the encoding of embedded fonts and graphics |
| `pair.subrip.cue-scan.file`, `probe.subrip` | `loc-subrip: Notes` | The three line terminators that separate cues |
| The USF rows | The XML sections: the prolog (`2.8`), comments, processing instructions, tags, references, the predefined entities and end-of-line handling; for a file also the document (`2.1`) and its encoding (`4.3.3`, `F.1`) | The decision above |
| `probe.vobsub` | `dvdinfo-dvdmpeg: Extensions`, `itu-t-h-222-0: 2.4.3.6`, `2.4.3.7`, `2.5.3.3`, `2.5.3.4` | The first sub-picture packet of each stream, which the probe checks in the data file at the index's file position: a pack, a PES packet and its sub-stream number |

No change was needed for the TextST and PGS file pairs, `probe.hdmv-textst`, `probe.pgs` or `pair.webvtt.file-header.file`.

**Fetching.** The XML specification was read from w3.org to standard output before the decision, then stored from its dated address; three requests went to w3.org, several seconds apart.

**Plan and dossier.** Revision 18 of the plan changes the specifications of Phase 14 and its tasks 6, 9 and 10, Appendix B.4 (XML 1.0) and B.6 (the PGS frame rate and the TextST timestamps). The source dossier adds task-17 notes for USF, VobSub, PGS and TextST.

**Open:** task 17 for release 0.8.0; then tasks 18 and 19, G1.8 and G1.9.

## 2026-11-21 — Progress: Phase 1, task 17, release 0.8.0

**Found.** The one row, `probe.dvd-video`, cited only `dvdinfo-ifo: IFO file`, the title of the page. Phase 15 also reads the title-set tables and the program chains, and with lookup enabled the object files. The stored pages were compared with the pages on the author's current site, read to standard output:

| Page | Difference | Outcome |
|---|---|---|
| `ifo.html` | The audio sample rate is "0 = 48Ksps, 1 = 96Ksps" where the stored copy read "0 = 48Kbps". The offset of a reserved field is 0x0244, not 0x0144. The aspect value 1 is "not specified", and the reserved resolutions are marked. The title and menu attribute tables are separate. The quantization field reads "0 = 16bps, 1 = 20bps, 2 = 24bps, 3 = DRC" for every coding mode. | Replaced (decision below) |
| `dvdmpeg.html` | Sub-streams 0x80 to 0x87 are AC-3 and 0x88 to 0x8F DTS, where the stored copy gave 0x80 to 0x87 for "AC3, DTS, SDDS" | Replaced |
| `pgc.html`, `ifo_vts.html` | Navigation rules, the karaoke cell types, the meaning of the time map, and the playback mode and still time of the chain header, which trade offsets | Kept: Phase 15 reads none of these |
| `spu.html`, `pci_pkt.html` | Constraints on color and contrast changes, the labels of the button group types | Kept: no row reads them |
| `lpcm.html` | Not on the author's site (404) | Kept |

**Decided by the maintainer** (GS.2 for the source):

| Topic | Decision | Reasoning |
|---|---|---|
| DVD source | `dvdinfo-ifo` and `dvdinfo-dvdmpeg` keep their identifiers and now point to the author's current pages, stored from archive captures of 2026-08-07 and 2026-08-06. | The older copies have a wrong unit, a wrong offset and no DTS range. The rows of earlier releases keep their headings, and an audio sub-stream's codec is still identified by its sync word (G1.5). |
| Matching attribute entries to object-file streams | Through the stream control tables of the first program chain that marks the stream available. An audio entry gives sub-stream 0x80 + *n* (AC-3), 0x88 + *n* (DTS) or 0xA0 + *n* (LPCM), or MPEG audio stream *n*. A sub-picture entry gives 0x20 + *n*, from its number for 4:3 or for wide, according to the video's aspect. The video stream is matched directly. A codec that disagrees, or a chain that maps the stream to another number, raises a diagnostic and leaves the stream unmatched. | Phase 15 makes the attribute tables the report's streams but did not say how they find their streams. The control entry holds the number (3 bits for audio, 5 for sub-pictures); the ranges come from `dvdinfo-dvdmpeg`, and the MPEG audio stream numbers from H.222.0 Table 2-22. Logical and physical order can differ, so the entry's position is not used. |
| Angle blocks in chapter times | Only the first cell of an angle block counts. If the sum over all cells differs from the chain's playback time, a diagnostic is raised, and the chain keeps its own time. | The cells of a block are alternative angles of the same stretch of time; adding them all would overcount. |

Read from the pages without a further decision: the program chains are the title chains of `VTS_PGCI`, not the menu chains; the frames of a playback time count at the 30 or 25 frames per second that the frame byte gives, and its two illegal values give no time; a language is read when the language type is 1 and becomes a BCP 47 tag through `rule.language-tags`.

**Confirmed.** `probe.dvd-video` has its `confirmed` date, 2026-11-21. With it, all 307 active rows of releases 0.1.0 to 0.8.0 are confirmed, and task 17 is done.

**Corrected:**

| Row | Was | Now | Why |
|---|---|---|---|
| `probe.dvd-video` | `dvdinfo-ifo: IFO file` | `dvdinfo-ifo: Video Manager and Video Title Set IFO file headers`, `Video Attributes`, `Title Audio Attributes`, `Title Subpicture Attributes`; `dvdinfo-ifo-vts: VTS_PGCI`; `dvdinfo-pgc: PGC header`, `program map entry (one per program, pad to word boundary with zero)`, `cell playback information table entry (one per cell)`; `dvdinfo-dvdmpeg: Extensions`; `itu-t-h-222-0: 2.4.3.6`, `2.4.3.7`, `2.5.3` | The header table, with the sector pointers, the stream counts and the file structure; the three title attribute tables; the program chain table; the chain header with its playback time and stream control; the programs; the cell times and angle-block marks; and the object files, read as program streams with their sub-streams |
| `detect.dvd-video` (0.1.0; keeps its date) | `dvdinfo-ifo: IFO file` | `dvdinfo-ifo: Video Manager and Video Title Set IFO file headers` | The identifiers that detection reads are in that table; the page's title names the whole page |

Every row that cites `dvdinfo-dvdmpeg: Extensions` was checked again against the new page: it still holds the sub-picture, AC-3 and LPCM ranges, and gives DTS its own range, which the sync-word rule does not need. No other change was needed.

**Fetching.** Before the decision, seven requests went to mpucoder.com, at least six seconds apart, one for each page compared; `lpcm.html` answered 404 and the others 200. Two went to the Internet Archive's availability service, eight seconds apart. After the decision, the fetch script stored the two captures from the Internet Archive.

**Plan and dossier.** Revision 19 of the plan gives Phase 15 its specifications and says, in tasks 1 and 2, what is read. Phase 10 and Appendix B.6 name the new pages. The source dossier adds task-17 notes on the information files and the sub-stream numbers.

**Open:** tasks 18 and 19, G1.8 and G1.9.

## 2026-11-21 — Choices for tasks 18 and 19

The maintainer answered these questions before the project documents and the CI workflow were written.

| Topic | Decision | Reasoning |
|---|---|---|
| Vulnerability reports | GitHub's private vulnerability reporting: the **Report a vulnerability** button on the Security tab. G1.8 turns it on. | No address goes into the repository. |
| `SECURITY.md` (GS.4) | Not in this repository. R14 is amended accordingly. How to report a vulnerability, what counts as one and the containment condition of R18 form a section of `CONTRIBUTING.md`. | The maintainer's instruction. R14 named the file, so the change goes through GS.4, decided by that instruction. |
| Supported versions | Not stated. The question was answered by dropping `SECURITY.md`, so no policy on which releases get security fixes is written down. | — |
| Outside contributions | Issues only. `CONTRIBUTING.md` says how to write a useful issue and sets out every rule for the maintainer's own changes. There are still no pull requests (plan section 1.2). | It leaves the plan's workflow as it is. |
| 32-bit targets | Not supported: a non-goal beside `no_std` and WebAssembly, never built or tested. The code stays portable only because the robustness rules require checked conversions; nothing is promised. Section 9.8 loses its 32-bit build, and the dossier's 32-bit row says "not supported". | The maintainer's decision, chosen over a compile-time error. |
| Actions and runners (GS.1) | GitHub's checkout 7.0.1, setup-node 7.1.0 and cache 6.1.0, each pinned to its full commit SHA; the runner images `ubuntu-26.04`, `macos-26` and `windows-2025`. G1.8 allows only actions created by GitHub and requires SHA pinning. | Nothing in a run changes until the workflow file does. |
| CI scope in Phase 1 | Beside `node scripts/check.ts` on the three runners, the tests on current stable and on the declared `rust-version`, and the audit: joint packaging and the feature combinations. The reproducibility check of Appendix C.2.1 step 5 and the tarball job follow in Phase 8. | Both apply to an empty workspace and already pass. |
| Local tools | `cargo-deny` 0.21.0 and `cargo-hack` 0.6.46 are installed on the development machine with `cargo install --locked --version`, with the cooldown given through Cargo's environment variables. | So that `deny.toml` and the feature build are checked here before the first CI run. |

## 2026-11-21 — Progress: Phase 1, tasks 18 and 19

**Task 18: the project documents.**

- `CHANGELOG.md` follows Keep a Changelog 1.1.0. Beside the six kinds of change of that format, a release can list "Changed output". Its Unreleased section says that nothing is published yet.
- `CONTRIBUTING.md` covers:
  - issues as the way to contribute;
  - the section "Reporting a vulnerability", with GitHub's private reporting, what counts as a vulnerability and the containment condition of R18;
  - the rules: clean room, sources and traceability, dependencies, fixtures, code, scripts, language, coverage and documentation;
  - how a format is added, after section 11;
  - the checks, with what they need and what CI adds.

  It has four Mermaid diagrams: issues, a vulnerability report, adding a format, and the checks with CI.
- `README.md` gains a short "Contributing" section with an absolute link, because the library ships a byte-identical copy, which was updated.

**Task 19: the CI workflow.** `.github/workflows/ci.yml` runs on every push to `main` and to every `release/*` branch, with read-only contents permission. It has five jobs:

- **Checks:** `node scripts/check.ts` on the three runners, after Node.js from `.nvmrc`, `npm ci` in `scripts/`, `rustup toolchain install` and `cargo-llvm-cov`.
- **Tests on current stable**, on the three runners.
- **Tests on the declared `rust-version`**, on Linux, through the new `node scripts/test-declared-rust-version.ts`. Its module, `scripts/checks/rust-version/`, reads the release that both published packages declare from `cargo metadata`, installs it with rustup and runs the tests with it. Three tests cover the reading.
- **Feature combinations**, with `cargo-hack`.
- **The audit**, with `cargo-deny`.

The Cargo tools are cached per tool, version and runner. Checkout keeps no credentials.

**Also changed:**

- `node scripts/check.ts` gained the step "joint packaging (Appendix C.2.1)": `cargo package --locked --allow-dirty` of both published crates in one command, with the verification builds. `--allow-dirty` lets the step run before a commit.
- **Plan, revision 20:**
  - section 2: 32-bit targets are a non-goal;
  - R14: SECURITY is dropped, and CONTRIBUTING says how to report a vulnerability;
  - D7: how the cooldown reaches `cargo install`;
  - section 7.1: the layout;
  - section 7.7: where the containment condition is stated;
  - section 9.8: no 32-bit build; how the workflow is built and when the other jobs join;
  - section 10: the CONTRIBUTING and dossier rows; the SECURITY row is removed;
  - Phase 1: the progress line, tasks 5, 18, 19 and G1.8, and the Docs line;
  - the gate summary;
  - Appendix D: a group of research;
  - Appendix E.
- **Dependency dossier:**
  - section 1.5: the cooldown for `cargo install`;
  - new section 1.7 with the three actions, their SHAs and licenses (MIT);
  - the 32-bit row of section 3.3;
  - the first `cargo-deny` run in section 6.
- **`CLAUDE.md`:**
  - `CONTRIBUTING.md` changes with a plan revision that changes a rule it states;
  - CI uses only GitHub's own actions pinned to SHAs, and another action goes through GS.1;
  - the jobs CI adds to the check script.

**Verification, on macOS with Node.js 26.13.1 and Rust 1.100.0, on 2026-11-21:**

- `node scripts/check.ts` passes every step, the new joint packaging included.
- The 90 script tests and 149 Rust tests pass.
- Coverage: `cc-media-probe-cli` is at 100.00% (10/10), and `cc-media-probe` has no executable lines.
- `cargo deny --locked check` reports advisories, bans, licenses and sources ok.
- `cargo hack build --locked --workspace --feature-powerset` builds all six crates.
- The new module reads `1.100` from the workspace's real `cargo metadata`. The full run of `node scripts/test-declared-rust-version.ts` was not made locally, because it would install the toolchain `1.100` beside the pinned `1.100.0`; CI makes it first.
- The four diagrams of `CONTRIBUTING.md` render without errors.
- Each action's tag resolves to the pinned SHA with `git ls-remote`, and each action is MIT-licensed.

**Choices made without asking**, each open to change at G1.9:

- Joint packaging is a step of the check script rather than a separate job, because Appendix C.2.1 puts it in the check suite and CI runs that suite on fresh runners.
- A script reads the declared `rust-version` instead of repeating it in the workflow, so a raise of `rust-version` needs no edit of `ci.yml`.
- The cooldown variables are set on each tool-install step only, so they never override `.cargo/config.toml` for the workspace's own commands.
- Runs are not canceled when a newer push arrives, so every commit of `main` gets its own result.
- The disclosure wording of `CONTRIBUTING.md`: the advisory is published with the release that contains the fix.

**Expected on the first CI run:** the fourth pure-Rust layer fails on Linux and Windows, because `crates/cc-media-probe-checks/data/system-libraries.tsv` has only the macOS entry. That run measures the libraries, which are then added and reported at G1.9, as task 5 planned.

**Open:**

- **G1.8 [MANUAL]**, before the first push:
  - In **Settings → Actions → General**, under **Actions permissions**:
    - choose "Allow caoccao, and select non-caoccao, actions and reusable workflows";
    - tick "Allow actions created by GitHub" and "Require actions to be pinned to a full-length commit SHA";
    - click **Save**.
  - Under **Workflow permissions**, choose read access to contents and packages, and leave "Allow GitHub Actions to create and approve pull requests" off. Click **Save**.
  - In **Settings → Advanced Security**, enable "Private vulnerability reporting".
- After G1.8 and a push on the maintainer's request:
  - the first CI run;
  - the allow-list entries of Linux and Windows;
  - G1.9.

## 2026-11-21 — G1.8 Repository settings [MANUAL]

The maintainer reported the settings done:

- GitHub Actions allows only actions created by GitHub, pinned to a full-length commit SHA.
- The default workflow token has read access only.
- Private vulnerability reporting is enabled.

The commit with tasks 18 and 19 starts the first CI run.

**Open:**

- The results of that run, with the allow-list entries of Linux and Windows that it measures.
- G1.9.

## 2026-11-22 — Progress: the first CI run

The first run, on commit `dc07ac1`, failed.

**Passed:**

- the audit: advisories, bans, licenses and sources ok;
- the tests on the declared `rust-version`, on Linux;
- the feature combinations;
- the tests on current stable, on the three runners.

**Failed:** the check script, on all three runners, at the coverage step. Every earlier step passed. The coverage tool writes the file given with `--output-path` but does not create its directory, and a fresh checkout has no `target/coverage`. Locally the directory was left over from earlier runs, so the local check passed. The tool's README and other projects that hit the same error confirm it: only `--output-dir` creates its directory, and the LCOV report takes `--output-path`.

**Fix:** the coverage step of `scripts/checks/all-checks.ts` creates the report's directory before it runs the tool.

**Verification, on macOS on 2026-11-22:** with `target/coverage` removed, `node scripts/check.ts` passes every step.

**Not measured yet:** the steps after coverage did not run in CI: documentation, targets, packaging, joint packaging and the three pure-Rust layers. The next run measures the fourth layer on Linux and Windows. Each library outside the allow-list is reported as a finding of its own, so one run gives both lists.

**Open:**

- A commit and push, on the maintainer's request, so that CI runs again.
- The allow-list entries of Linux and Windows from that run.
- G1.9, which needs CI green on the three runners (the exit criteria of Phase 1).

## 2026-11-22 — Progress: the second CI run

On the maintainer's request, the fix was committed as `63a9e7d` and pushed.

**Passed:** every job except the check script on Linux and Windows. That includes the check script on macOS, which is green end to end for the first time in CI.

**Failed:** the fourth pure-Rust layer on Linux and Windows, as expected. Every other step of the check script passed there, the joint packaging included. The layer reported each library of the CLI binary as outside the allow-list:

- **Linux** (`ubuntu-26.04`, `readelf --dynamic`): `libc.so.6`, `ld-linux-x86-64.so.2` and `libgcc_s.so.1`.
- **Windows** (`windows-2025`): 10 libraries:
  - `KERNEL32.dll`, `ntdll.dll` and `api-ms-win-core-synch-l1-2-0.dll`;
  - six Universal C runtime sets: `api-ms-win-crt-heap-l1-1-0.dll`, `-locale-`, `-math-`, `-runtime-`, `-stdio-` and `-string-l1-1-0.dll`;
  - `VCRUNTIME140.dll`.

The Universal C runtime is part of Windows from Windows 10 on. `VCRUNTIME140.dll`, the Visual C++ runtime, is not: the `x86_64-pc-windows-msvc` target links the C runtime dynamically by default. `crt-static` would link the same runtime into the binary instead. R15 allowed only operating-system libraries, so this went to GS.4.

## 2026-11-22 — GS.4 The C runtime under R15 [CONFIRM]

The agent presented the Windows result and two choices: allow the Visual C++ runtime and amend R15, or set `crt-static` for Windows in the repository's Cargo configuration. The maintainer decided:

| Topic | Decision | Reasoning |
|---|---|---|
| The Visual C++ runtime | Allowed. R15's exception becomes the system libraries that the Rust standard library itself links: operating-system libraries and the C runtime. D4, its third layer and the layout use the same term. | The standard library links a C runtime on every platform, so the exception was never limited to the operating system: glibc and GCC's unwinder on Linux, the C runtime on Windows. `crt-static` would hide the runtime inside the binary rather than remove it, and it would change only the repository's builds, not those of users. Users who build with MSVC already have the runtime, and the project ships no prebuilt binaries. |
| The allow-list entries | The 13 measured libraries by exact name: 3 for Linux, 10 for Windows. No prefix entry for the Universal C runtime. | If the standard library starts using another library, CI fails and the list is updated on purpose. |

## 2026-11-22 — Progress: the allow-list of Linux and Windows

**Changed:**

- `crates/cc-media-probe-checks/data/system-libraries.tsv` gains the 13 libraries, with where and how each platform was measured.
- The check messages name the allow-list as one of system libraries, in the checks crate and in `scripts/checks/pure-rust/constants.ts`.
- **Plan, revision 21:**
  - R15, through GS.4;
  - D4 and its third layer;
  - the layout of section 7.1;
  - Phase 1: the progress line and task 5;
  - Appendix D: a group of research;
  - Appendix E.
- **Dependency dossier:** the status line, the Linux and Windows hosts of section 3.3 and their libraries in section 3.4. They were measured on the workspace's CLI binary, which has no dependencies yet, not on the spike binary.
- `CLAUDE.md` and `CONTRIBUTING.md` state the exception of R15.

**Open:**

- A commit and push, on the maintainer's request, so that the third run checks the new entries.
- G1.9, once CI is green on the three runners.

## 2026-11-22 — Progress: the third CI run

On the maintainer's request, the allow-list was committed as `2b1b9fd` and pushed. Every job of the run passed:

- the check script on `ubuntu-26.04`, `macos-26` and `windows-2025`, with 0 findings in the fourth pure-Rust layer on each;
- the tests on current stable on the three runners;
- the tests on the declared `rust-version`;
- the feature combinations;
- the audit.

Phase 1's last exit criterion, CI green, is met.

**Open:** G1.9, the phase gate.

## 2026-11-22 — G1.9 Phase gate [CONFIRM]

The agent presented the summary of Phase 1, which covered:

- what was built: the workspace, the checks, the dossier, the specification set and the project documents with CI;
- CI green on `2b1b9fd` on the three runners, with 90 script tests and 149 Rust tests;
- coverage: 100.00% for the CLI (10/10); the library and its areas have no executable lines;
- no resource, fuzz or output results yet, because no parser exists;
- no optional steps in Phase 1;
- every exit criterion met;
- the deviations of revisions 7 to 21;
- eight choices made without asking.

| Topic | Decision | Reasoning |
|---|---|---|
| The phase gate | Approved. Phase 1 is complete, and Phase 2 is next. | The exit criteria are met, and CI is green. |
| The choices made without asking | All eight kept: warnings denied in the lint table; absolute links in the README; the npm cooldown and exact versions in `scripts/.npmrc`; joint packaging as a step of the check script; the declared `rust-version` read by a script; the cooldown variables on the install steps only; runs not canceled by a newer push; the advisory published with the fixing release. | No change was asked for. |
| Unconfirmed active rows | The deliverables check fails on an active coverage row without the date of its confirmation. | All 307 active rows have one, but no check required it, and later phases add rows. |

## 2026-11-22 — GS.1 Duplicate-code checks [CONFIRM]

The maintainer asked for `cargo-dupes` to enforce a duplicate-code rule, with its threshold in `CLAUDE.md` and CI. The agent installed it, measured the repository, and presented the tool, the thresholds and where the check runs. It then researched and proposed `jscpd` for the scripts, which `cargo-dupes` does not read. The maintainer decided:

| Topic | Decision | Reasoning |
|---|---|---|
| `cargo-dupes` | Approved: 0.2.1 of 2026-02-17, MIT, 93 crates in its lock file. | It finds exact and near copies per function, method and closure after normalizing names and literal values. |
| Rust thresholds | Functions, methods and closures of 5 lines or more. No exact copy, and no near copy at 80% similarity (the tool's default) or more. Test code is left out, and so are `spikes/` and `readers/`. In `dupes.toml`. | With the tool's default of 10 syntax nodes, the first run found 19 exact and 5 near groups. Most were one-line functions that differ only in a constant, and tests that differ only in their values. At 5 lines without tests, only real copies remain. The other choices measured 7 exact and 2 near groups with tests included, and 10 and 2 at the default size. |
| Where it runs | A step of `scripts/check.ts`, so locally on every change and in CI's check job on the three runners. | A duplicate is found before a commit, not after a push. |
| `jscpd` | Approved: 5.4.3 of 2026-11-08, MIT, no dependencies and no install script. Its native engine comes as one optional npm package per platform. | It reads TypeScript and compares functions by normalized structure, as `cargo-dupes` does for Rust. |
| Script thresholds | Exact clones of 5 lines and 50 tokens or more (its defaults), and functions at 80% similarity of structure or more. Any clone fails. `*.test.ts` and `node_modules` are left out. In `scripts/jscpd.json`. | Adding renamed matching token by token found 20 groups, all runs of constant declarations, message lists and import blocks, none copied logic. |

## 2026-11-22 — Progress: no duplicate code (R36)

**Added:**

- `dupes.toml`, at the repository root. `cargo dupes --path . check` reads it, because exclusions match the walked path as a substring: from `.` that path is relative, while from an absolute root a parent directory's name would match. Every exclusion is written for both path separators. `tests.rs`, `tests/` and `benches/` are excluded by path, because the tool's `exclude_tests` covers only `#[test]` functions and `#[cfg(test)]` modules.
- `scripts/jscpd.json`, and `jscpd` 5.4.3 in `scripts/package.json`. The lock file records all eight platform packages, and `npm ci` installs the host's.
- Two steps of `node scripts/check.ts`: "script duplicate code (R36)" after the type check, and "duplicate code (R36)" after the Rust structure check. The npm tools now start through one function that reports a missing `npm ci`.
- CI's check job installs `cargo-dupes` 0.2.1 with the cooldown and caches it, as it does `cargo-llvm-cov`.

**Duplicates removed**, in `cc-media-probe-checks`:

- The JSON reader's `Scanner` repeated the lexer's `Cursor`: the same fields, constructor and `advance`. `Cursor` moved to the crate's shared module `src/cursor.rs`, and the scanner now wraps it. The lexer keeps its character classes in `lexer/characters.rs`. The line feed is one shared constant instead of one in each module.
- A token compared with a list of words or punctuation was written out 13 times. Four of those were the lists of opening and closing brackets. `Token` gained `is_any_word`, `is_any_punctuation`, `opens_group` and `closes_group`, with the bracket lists and the keywords that make `const` a function qualifier as shared constants.
- The crate's documentation named `scripts/check.sh` and left out the command `dynamic-libraries`; both are corrected.

**Verification, on macOS on 2026-11-22:**

- `node scripts/check.ts` passes every step, the two new ones included.
- `cargo dupes --path . check` reports 0 exact and 0 near groups among 171 code units, and `jscpd` reports 0 clones in the 57 script files above its token minimum.
- The 150 Rust tests pass, including a new one for the token methods, and so do the 91 script tests, including a new one for the confirmation rule.
- A synthetic pair of renamed functions in a scratch directory makes `jscpd` fail with `scripts/jscpd.json`. During the work, `cargo-dupes` failed the check on a near pair of closures, which was then removed.

**Documents:**

- Plan, revision 22:
  - R36;
  - D7, D23 and D24;
  - section 5.2;
  - the layout of section 7.1;
  - section 9.8;
  - the status line and the Phase 1 progress line;
  - Appendix D;
  - Appendix E.
- The dossier: section 1.5, section 1.6 (now the scripts' npm tools) and the status line.
- `CLAUDE.md` and `CONTRIBUTING.md`: the rule, its thresholds, the checks and the tools.

**Open:**

- A commit and push, on the maintainer's request, so that CI installs `cargo-dupes` and runs both checks on the three runners.
- Phase 2, from its first task.

## 2026-11-22 — Progress: the fourth CI run

On the maintainer's request, the work of R36 was committed as `1a8a723` and pushed. Every job of run 45013181966 passed. Both duplicate-code steps ran in the check script on `ubuntu-26.04`, `macos-26` and `windows-2025`:

- `jscpd` found 0 clones;
- `cargo-dupes` found 0 exact and 0 near groups among 171 code units on each runner, so the exclusions hold for both path separators.

**Open:** Phase 2, from its first task.

## 2026-11-22 — Choices for Phase 2

Before the first document, the agent asked three questions. The Cargo SemVer reference, read on 2026-11-22, confirms that adding a variant to a `#[non_exhaustive]` enum is a compatible change, while adding a field to an existing variant is not unless that variant is non-exhaustive as well. The maintainer decided:

| Topic | Decision | Reasoning |
|---|---|---|
| Depth of the field inventory (task 1) | Exact sources (element, box or field, with section, unit, precedence and provenance) for Matroska and WebM, the ISO base media family with QuickTime, AVI and the codecs of Phase 4. For the formats of `0.2.0` to `0.8.0`, a matrix of the normalized fields each one can fill, with the section that states it. Each format phase replaces its entry. | The model has to fit every format before it is approved, and the matrix shows that it does. The exact sources of a later format are read in its own phase. |
| Typed details (tasks 2 and 11) | `api.md` fixes the conventions: the two open enums, the catch-all variant that keeps the kind name, naming, the serialized form with `kind`, raw values, observed bounds and values beyond the exact range. Matroska and H.264 are worked examples. The skeleton ships both enums with the catch-all variant only. Each format phase designs its own detail type within the conventions and has it reviewed at its gate. | A detail type designed before its parser exists would likely change once the parser is written. A new variant is a compatible change. |
| Review | One document at a time: each is presented for review before the next is started. The gates G2.1 to G2.4 follow as planned. | A design choice that changes early does not spread into later documents. |

**Plan, revision 23:** the status line, sections 10 and 11, and Phase 2 (its goal, a progress line, and tasks 1, 2 and 11).

**Open:** Task 1, the field inventory.

## 2026-11-22 — Progress: Phase 2, task 1

**Written:** the first version of `docs/guide/fields.md`, the field inventory.

- Section 2 gives the general rules and a diagram of how one value is chosen.
- Section 3 is the catalog of normalized fields: report, stream, codec, video, audio, subtitle, chapters, tags, attachments and programs.
- Section 4 gives the exact sources of Matroska and WebM, the ISO base media family with QuickTime, and AVI.
- Section 5 gives the codec headers that `0.1.0` parses, in nine families.
- Section 6 is the matrix of the formats of `0.2.0` to `0.8.0`.
- Section 7 lists 18 findings.

The guide includes the codec parsers that the plan assigns to Phases 5 to 7: H.263, AMR, QuickTime text, SSA/ASS, DVB page identifiers and XSUB. They ship in `0.1.0` too.

**Sources read.** Every section number was checked in the stored document, or taken from a coverage row that task 17 of Phase 1 confirmed. The documents read: RFC 9559 and RFC 8794, the 2015 edition of ISO/IEC 14496-12, the QuickTime documentation, the F4V specification, OpenDML 1.02, the DV in AVI page, the ITU-T video recommendations, the AV1 and VP9 specifications, ETSI TS 102 366, 102 114 and 103 190-2, the TrueHD documents, RFC 9639, RFC 7845, RFC 6381, RFC 4867, RFC 2361, the Matroska tags draft, the WebM guidelines, the IANA media types, SMPTE RDD 36 and 3GPP TS 26.244 and 26.245. Microsoft's page for `AVISTREAMHEADER` was read online. No implementation was opened; the archive of RDD 36 also holds a reference decoder, which was left unopened.

**Findings that need a decision**, from section 7 of the guide:

- For the API specification (G2.2): F1 default values, F2 floating point, F3 media types without an IANA entry, F4 report facts and tag keys, F5 codec strings, F6 dynamic range, F7 the direction of rotation, F8 cover art, F14 embedded streams, F18 room for chapter-like structures.
- Plan amendments: F9 the Matroska statistics tags of Phase 5, F12 a deliverable row for the chapter text track of the ISO family, F15 the AV1 frame size in Phase 4.
- Through GS.2: F10 the HDR boxes `mdcv` and `clli`, F11 the defined channel layouts of `chnl`, F13 Microsoft's three AVI structure pages, F16 object audio in E-AC-3 (optional), F17 ITU-T H.274 for VVC.

**Verification:** `node scripts/check.ts` passes. The guide's diagram renders without error.

**Open:** the maintainer's review of `fields.md` and the decisions on its findings; then task 2, the API specification.

## 2026-11-22 — Progress: Phase 2, task 1, first review

The maintainer reviewed the staged `docs/guide/fields.md` and found 20 rows that would report wrong values for valid files or contradict the plan. Every comment was checked against the stored source or the plan and accepted. The revision is in the working tree; the staged version was left as it was.

**Duration, timing and stream discovery:**

- *Fragmented MP4.* A segment index establishes only its reference track's end. The report's duration is the latest end of all tracks after their edits, set only when every track's end is established (plan Phase 6, task 11).
- *AVI.* The report's duration is the latest stream end minus the earliest start, not the longest stream.
- *Transport stream.* A report duration only for a file of one program. A program's end needs the last access unit's duration (plan Phase 10, task 6).
- *Ogg.* Both endpoints of every timed stream of every link, summed over a complete chain inventory (plan Phase 9, tasks 4 and 5).
- *FLV.* Streams come from observed tags and multitrack identifiers, never from the header's flags (plan section 8.3; Phase 13, task 1).
- *Subtitle files.* The time range runs from the earliest start to the latest end, because cues can overlap (WebVTT 4.1). A VobSub index gives no end for its last subtitle, and the approved PGS description gives none either: new finding F20.
- *Program streams and DVD.* Aligned with Phase 10, task 8 and Phase 15, task 2.

**Frame rates and scan types:**

- *H.264.* A clock tick is not a frame interval. Constant only with `fixed_frame_rate_flag` 1 and `pic_struct_present_flag` 0 (E.2.1, Table E-6).
- *HEVC.* Constant only from the HRD parameters' `fixed_pic_rate_within_cvs_flag` and `elemental_duration_in_tc_minus1`, with `frame_field_info_present_flag` 0 (E.3.2, Table E.7).
- *VVC.* As HEVC, when no HRD parameters let picture timing messages repeat pictures (7.4.6.2; D.4).
- *MPEG-2.* Its mode is unknown, because `repeat_first_field` changes the cadence (6.3.10). Its `progressive_sequence` 0 does not prove interlace (6.3.5).
- *ISO family.* The mode compares the `stts` deltas instead of counting entries.
- *IVF.* The header's time base is not a frame rate.
- *AV1.* A rate only with `equal_picture_interval`.
- *Matroska.* `DefaultDuration` counts a block element, which RFC 9559 calls a frame.
- *HEVC scan type.* Read from both source flags (7.4.4).

**Codec and container mappings:**

- *H.263.* 12:11 pixels for the standard source formats, and progressive 4:2:0 (4.1).
- *WebM.* Its WebVTT tracks `D_WEBVTT/` are UTF-8, without private data. The stored guidelines do not name AV1. New finding F19.
- *PCM in the ISO family.* The channel count comes from `chnl` for multichannel `ipcm` and `fpcm` (`iso-iec-23003-5.md` 2).
- *OpenDML.* `dmlh` counts the file's frames, not a stream's.
- *Timed text.* The canvas is the track size (TS 26.245 5.7). The default text box goes to the details.
- *CAF.* `mNumberPackets` and `mNumberValidFrames` are kept apart.
- *Creation times.* A creation time needs UTC. That excludes the broadcast extension of WAVE and QuickTime's `mvhd`, which older movies set to local time.

**API contract:**

- *VP9.* Its render size goes to the details, as plan section 8.4 allows no other source of display size.
- *Stream estimates.* Each stream now has its own estimate accessors.

**The findings now number 20:**

- *Questions for the API specification:* F1 to F8, F14 and F18.
- *Plan amendments:* F9, F12, F15, F19 and F20.
- *Sources for GS.2:* F10, F11, F13, F16 and F17.

F4, F8 and F18 were brought in line with plan Phases 9, 13 and 15.

**Verification:** `node scripts/check.ts` passes. The phase tasks of Phases 4 to 15 already list the codec fields the corrected rows need: the VUI with its HRD parameters, and the timing and HRD parameters of the VVC sequence parameter set.

**Open:** the maintainer's decisions on the findings; then task 2, the API specification.

## 2026-11-22 — Progress: Phase 2, task 1, second review

The maintainer reviewed the staged `docs/guide/fields.md` again and raised 16 issues. Each was checked against the stored source and accepted. The revision is in the working tree; the index was left as it was.

**Timing and duration:**

- *Matroska.* `CodecDelay` is in nanoseconds: samples are `CodecDelay` × rate ÷ 10^9.
- *ISO family, unknown durations.* A duration of all ones in `mvhd`, `tkhd` and `mdhd` means unknown (ISO 8.2.2.3, 8.3.2.3, 8.4.2.3). It is never converted.
- *Fragmented MP4.* A track's duration subtracts its first presentation time.
- *ISO family, frame rate.* The cadence is judged on the composition timeline, with `ctts` or the `trun` offsets, not on decode deltas.
- *Encoder delay and padding.* An edit fills them only where an approved source says that it marks priming. That is `xiph-opus-in-isobmff` 4.4 for Opus, under which the edit is mandatory and `PreSkip` informative. For AAC, it holds only with roll sample groups (QuickTime, "Using track structures to represent encoder delay explicitly"). Without them, QuickTime assumes an implicit delay instead. This amends plan section 8.4 and Phase 6, task 4 (revision 24).
- *Sample counts.* The sample count leaves out only an established delay and padding.
- *WAVE.* The `fact` sample length becomes a duration by dividing by `nSamplesPerSec` (BS.2088-2 Annex 2, 1.3).

**Geometry and codec identification:**

- *OpenDML.* The visible size comes from both field descriptors.
- *MPEG-2.* The coded height is 32-line aligned when `progressive_sequence` is 0 (H.262 6.3.3).
- *AVC.* The codec string starts with the track's own sample-entry code, such as `avc3`.
- *VP8 in the ISO family.* VP8 reads the mandatory `vpcC` record; its color code points are those of H.273, whose twin text is ISO/IEC 23001-8.
- *AAC.* The codec string follows the `esds` object type, as in `mp4a.67` for MPEG-2 AAC.
- *VC-1.* The coded size comes from the first entry-point header (SMPTE ST 421 6.2.13).

**Chapters and audio metadata:**

- *Matroska.* `ChapLanguageBCP47` makes `ChapLanguage` and `ChapCountry` ignored.
- *WebM.* Every element that the WebM guidelines mark unsupported is now absent there, without its default: the edition flags and identifiers, `ChapterFlagHidden`, the tag targets other than tracks, and the unsupported segment and track elements.
- *FLAC.* `WAVEFORMATEXTENSIBLE_CHANNEL_MASK` (RFC 9639 8.6.2) takes precedence over the default layout.
- *BW64.* `chna` does not establish immersive audio without ITU-R BS.2076 and BS.2094, which the manifest lacks: new finding F21.
- *Immersive audio.* It is now tied to the fields that state it: AC-4's `immersive_audio_indicator` and TrueHD's 16-channel presentation. The catalog allows declared provenance for it.

**Found while applying them:**

- *VC-1.* Its display size is a codec header's, so it goes to the details, as VP9's render size does. Its frame rate is constant only without `PULLDOWN`.
- *Codec strings.* The proposal of F5 changes: outside the ISO family and WebM, no codec string.

**Verification:** `node scripts/check.ts` passes.

**Open:** the maintainer's decisions on the 21 findings; then task 2, the API specification.

## 2026-11-22 — Progress: Phase 2, task 1, third review

The maintainer reviewed the staged guide and plan again and raised 15 issues. Each was checked against the stored source and accepted. The revision is in the working tree; the index was left as it was.

**ISO family timing and audio edits:**

- *AAC encoder delay.* It needs QuickTime's complete convention: exactly one `sgpd` and one `sbgp` of grouping type `roll`, a single roll-distance entry, and one table entry that covers every sample ("Sample group description atom ('sgpd')", "Sample-to-group atom ('sbgp')"). It applies only in a QuickTime movie, because no approved source extends it to other files of the ISO family. A roll group of another shape is decoder recovery.
- *Opus scope.* The Opus binding excludes QuickTime (its section 1), so the MOV column no longer inherits it.
- *Opus padding.* The binding already gives the last sample only its valid samples (4.3.4). The padding therefore comes from the last packet's TOC byte (`rfc6716` 3.1, 3.2.5) minus that duration, when those bytes are read. The sample count subtracts the delay but not the padding again. `dOps` `PreSkip` stays informative in the ISO family.
- *Track duration.* Plain and fragmented MP4 now give the same interval: the end of the last media edit minus `start_time`. `tkhd` `duration`, which includes leading empty edits, goes to the details when an edit list exists.
- *Derived bit rate.* All sample bytes are divided by the whole media's duration, not by an edited selection.

**Video fields:**

- *DV.* 4:1:1 joins the chroma formats, for 525-line consumer DV (`iec-61834-2.md` 6).
- *Alpha.* `alpha` has its own row, with bitstream provenance first: ProRes and FFV1.
- *VP8 profile.* The frame tag's `version` is no profile. It goes to the details, and the profile comes only from `vpcC`.
- *VP8 bit depth and chroma.* Both are 8-bit 4:2:0 in every container (`rfc6386` 2).
- *Motion JPEG.* A JPEG image can be one field (OpenDML 7.0). It gives no coded size; the container does.

**Audio and container metadata:**

- *WAVE.* The `Samples` union is read as `wValidBitsPerSample` only for PCM and floating point; for compressed formats it is `wSamplesPerBlock`. `wBitsPerSample` is a bit depth only for PCM and floating point.
- *TrueHD.* A 16-channel presentation is immersive only when it shows objects, an intermediate spatial format, or a top pair (Tables 17 to 19).
- *AC-4.* Its codec string is `ac-4.` with the bitstream version, presentation version and `md_compat` (ETSI TS 103 190-2 E.13), and F5 now lists it.
- *FLV.* The audio tag fields are read per codec, as E.4.2.1 directs. For AAC they are ignored, and `SoundSize` counts only for uncompressed audio. The nominal `SoundRate` gives no exact rate.
- *ASF.* The duration and creation time need the Broadcast Flag at 0. Play Duration is in 100 ns units and Preroll in milliseconds. Average Time Per Frame is an average, so its mode is unknown.

**Plan:** revision 24 is narrowed to these rules. Section 8.4 states the AAC and Opus conditions and gains tests for a partial roll group and for Opus padding.

**Verification:** `node scripts/check.ts` passes.

**Open:** the maintainer's decisions on the 21 findings; then task 2, the API specification.

## 2026-11-22 — Progress: Phase 2, task 1, fourth review

The maintainer reviewed the staged guide and plan a fourth time and raised 16 issues. Each was checked against the stored source and accepted. The revision is in the working tree; the index was left as it was.

**ISO family and QuickTime:**

- *AAC encoder delay.* Apple defines the convention for "QuickTime movie files (.mov) and related MPEG-4 files", so it applies to conforming tracks of the ISO family as well. The single `sbgp` run must have `group_description_index` 1, because 0 places no sample in the group (ISO 8.9.2).
- *AAC padding.* It needs an edit end that is exact to the sample. The edit's duration is in the movie timescale, which Apple warns "may not be sample accurate". So the movie timescale must be a whole multiple of the media timescale, and the end must convert to whole samples.
- *Initialization segment.* On its own it keeps every count absent, not zero (plan Phase 6, task 10).
- *Bit rate.* Throughout the guide, bytes are divided by the time of the same units. In fragmented files that time includes the `trun` durations and the defaults.
- *Opus channels.* In an Opus sample entry, `channelcount` counts the decoded channels (binding 4.3.1). The output count is `dOps` `OutputChannelCount`, and the two are never compared.
- *Subtitle canvas.* `tkhd` sizes count only when they are pixels: not with `track_size_is_aspect_ratio` set, and not at 0 × 0 (ISO 8.3.2.3). The same conditions now guard the video display size.
- *Display aspect ratio.* It follows the same presentation size as the display size, because ISO scales every image to the `tkhd` size.

**Matroska and video codecs:**

- *Alpha.* Embedded and supplemental alpha are two carriages. `AlphaMode` describes only the block additions, so it never conflicts with ProRes's or FFV1's alpha.
- *Chroma.* The Cb-specific factors of Matroska enter the chroma format, which distinguishes 4:2:1 from 4:2:2 (RFC 9559 5.1.4.1.28.21).
- *Overall bit rate.* Matroska and WebM keep the derived report bit rate, `size` × 8 ÷ `duration`.
- *VP8 profile.* Its profile is the frame tag's version, which RFC 6386 9.1 calls a profile. `vpcC`'s profile, a different definition, goes to the details.
- *Motion JPEG.* A JPEG image gives the coded size when it is shown to be a whole frame: the OpenDML `AVI1` field value 0, or a progressive declaration of the container.

**Other containers:**

- *DVD editions.* They may carry the declared playback time of their program chain.
- *WavPack.* The total samples count only from a first audio block with `block_index` 0, and not when the low word is all ones (2.0).
- *Enhanced FLV.* Its multichannel configuration supplies the channel count and layout.
- *ASF.* The average bit rate comes from the stream bitrate properties (3.12). The Data Bitrate of the extended stream properties is a leaky-bucket parameter, so it goes to the details.

**Plan:** revision 24 now covers AAC in related MPEG-4 files, the index of the run, and the sample-exact padding. Section 8.4 gains the tests for them.

**Verification:** `node scripts/check.ts` passes.

**Open:** the maintainer's decisions on the 21 findings; then task 2, the API specification.

## 2026-11-22 — Progress: Phase 2, task 1, fifth review

The maintainer reviewed the staged guide and plan a fifth time and raised 12 issues. Each was checked against the stored source and accepted. The revision is in the working tree; the index was left as it was.

**AAC under the QuickTime convention:**

- *Sample count.* `stts` includes both priming and padding, so the valid count is reported only when both are established. With an unknown padding it is absent, and the total goes to the details.
- *Coarse clock.* A coarse movie clock keeps the encoder delay, which the media timescale gives exactly. Only the padding and the count are lost. The test in plan section 8.4 now says so.
- *Precision of the end.* The edit's end counts when it converts exactly to whole samples and a movie tick is no longer than one sample. A 90,000 Hz movie clock over 48 kHz audio qualifies, although it is no whole multiple.

**QuickTime:**

- *Track header flag.* `0x0008` means "Track in poster" in QuickTime. The aspect-ratio condition of ISO therefore does not apply to MOV's display size, display aspect ratio or subtitle canvas, and only the zero check remains.
- *Version 1 bit depth.* A version 1 sound description stores 16 in `sampleSize` and 2 in `bytesPerSample` for formats of more than 16 bits. The depth comes from the data format, such as `in24`.
- *Unspecified language.* The language code `0x7FFF` marks an unspecified language and leaves the field absent.

**AVI:**

- *Frame count.* Palette-change chunks, `##pc`, belong to the video stream but carry no image, so the frame count leaves them out. The packet count keeps them, and both rules require an index that accounts for every chunk.

**Matroska:**

- *Pixel aspect ratio.* It is the display aspect ratio divided by the visible aspect ratio, for display units 0 to 3: a 720 × 576 frame with a declared 16:9 has pixels of 64:45.
- *Forced flag.* `FlagForced` applies only to subtitles, so other tracks leave `forced` absent.
- *Chapter reference.* A chapter's segment reference keeps the edition identifier beside the segment identifier.

**General rules:**

- *Rationals.* The range rule checks a reduced rational's numerator and denominator, as plan section 8.8 does.
- *Derived bit rates.* They need a duration greater than zero.

**Verification:** `node scripts/check.ts` passes.

**Open:** the maintainer's decisions on the 21 findings; then task 2, the API specification.

## 2026-11-22 — Progress: Phase 2, task 1, sixth review

The maintainer reviewed the staged guide a sixth time and raised 12 issues. Each was checked against the stored source and accepted. The revision is in the working tree; the index was left as it was.

**WebM:**

- *Forced tracks.* The WebM guidelines define `FlagForced` for audio, video and subtitles ("Set if that track MUST be used during playback"). An explicit flag is reported on every kind there, and the default only on subtitles.

**Matroska:**

- *Rotation.* Rotation comes from the roll only when the pose is planar: projection type 0, and yaw and pitch both 0. RFC 9559 5.1.4.1.28.46 gives yaw 180 with roll 180 as a vertical flip, a reflection.

**ISO family and QuickTime:**

- *Opus conversions.* Media ticks are converted exactly to 48 kHz samples, because the binding allows other media timescales: 624 ticks at 96 kHz is 312 samples. The same holds for the padding and the sample count.
- *Cadence.* The comparison includes the last presented frame's own duration. Durations of 40, 40 and 80 ms are variable.
- *Version 2 bit depth.* `constBitsPerChannel` is set "only for uncompressed audio" and is 0 otherwise. It never gives a compressed entry a depth.
- *Rounded `tkhd` sizes.* A fractional size is rounded and becomes derived, with its exact value in the details.

**Video codecs:**

- *HEVC and VVC.* With `field_seq_flag` or `sps_field_seq_flag` set, each picture is a field. The coded and visible sizes then come from the container, and the picture size goes to the details.
- *MPEG-2.* The pixel aspect ratio uses `display_horizontal_size` and `display_vertical_size` when a sequence display extension is present (H.262 6.3.3). H.262's ratios are inverted from height over width first.
- *DV.* The scan type comes from `IL` of the source control pack. The field order needs `FF` = 1 (`iec-61834-2.md` 4).
- *Motion JPEG.* A frame header with `Y` 0 defers the height to the define-number-of-lines marker (T.81 B.2.5). The header-only probe takes the container's height then, or none.
- *FFV1.* `bits_per_raw_sample` is inferred to be 8 when absent, as in version 0, and a stored 0 is read as 8 (RFC 9043 4.2.7).

**CAF:**

- *No packet table.* A file of constant packet size may omit it. Its counts and duration then come from the Audio Data chunk's payload, less the 4-byte `mEditCount`, with `mBytesPerPacket` and `mFramesPerPacket`.

**Verification:** `node scripts/check.ts` passes.

**Open:** the maintainer's decisions on the 21 findings; then task 2, the API specification.

## 2026-11-22 — Progress: Phase 2, task 1, seventh review

The maintainer reviewed the staged guide a seventh time and raised 12 issues. Each was checked against the stored source and accepted. The revision is in the working tree; the index was left as it was.

**ISO family:**

- *Color.* A `colr` box "takes precedence, and over-rides the information in the bitstream" (ISO 12.1.5.1), so in the ISO family the container comes first for color.
- *Clean aperture.* A `clap` box overrides the codec's cropping in the same way (ISO 12.1.4.1).
- *Coded size.* A sample entry's width and height are "the maximum visual width and height" (ISO 12.1.3.3). They go to the details, and the coded size comes from the codec header.
- *Cadence.* Non-output samples, which carry the most negative `ctts` offset, are left out first (ISO 8.6.1.1). The last interval runs to `cslg` `compositionEndTime` when it is known (ISO 8.6.1.4), and to the last `stts` delta without offsets.
- *Sample counts.* The guide now matches plan Phase 6, task 10. For Opus, the plan accepts any media timescale, which its binding defines in 48 kHz samples. AAC needs the sampling timescale.

**Matroska:**

- *Ordered editions.* Playback comes from the leaf chapters. A parent's end is ignored for playback and should not be set (RFC 9559 20.2.4). This amends plan section 8.5 and Phase 5, task 9 (revision 25).
- *Opus.* For `A_OPUS`, `SamplingFrequency` holds the input rate (codec mapping 3.4.32). It goes to the details, and `CodecDelay` converts at 48 kHz.
- *Legacy languages.* A legacy code with a country, such as `fre-ca`, becomes `fr-CA` (RFC 9559 12).

**Other containers:**

- *BW64.* `ds64` holds `dummyLow` and `dummyHigh` where RF64 has its sample count, so they are ignored (BS.2088-2 4.2), and BW64 derives the count from the data size.
- *CAF.* A constant-packet file derives its packet count from the payload, even with a packet table, which may carry `mNumberPackets` 0 ("Constant Bit Rate Format").
- *ASF.* A Start Time of 0 is ignored (4.1).
- *AV1.* A still picture with a reduced header and no super-resolution has the maximum frame size, so its coded size is exact (AV1 5.9.2, 5.9.5, 5.9.8). F15 notes it.

**Verification:** `node scripts/check.ts` passes.

**Open:** the maintainer's decisions on the 21 findings; then task 2, the API specification.

## 2026-11-22 — Progress: Phase 2, task 1, eighth review

The maintainer reviewed the staged documents an eighth time and raised 19 issues. Three would give wrong answers for `0.1.0` files, nine name features of the plan that had no field, and seven define fields too narrowly. Each was checked against the stored source and accepted. The revision is in the working tree; the index was left as it was.

**Wrong answers:**

- *Matroska bit rate.* The derived rate uses the first segment's own byte size, the `Segment` element's data size. The input's size counts only when the segment is the whole input.
- *QuickTime coded size.* A QuickTime video sample description gives "the width of the source image in pixels". MOV therefore keeps it as the declared coded size; only the ISO family treats the entry as a maximum.
- *Opus output rate.* For `A_OPUS`, the default of `OutputSamplingFrequency` would copy the input rate, so it does not apply.
- *AV1 coded size.* F15 now agrees with the MP4 table. The coded size comes from the still-picture case or a container that states the actual frame size, as Matroska does. Where both sources give maxima, it is absent.

**Fields for planned features:**

- `protection` (F22). Payload encryption is reported independently of the header status, from Matroska's content encodings and ISO 8.12.
- `start_timecode` (F23). The label and drop-frame flag of a QuickTime timecode track.
- External references (section 3.11), which receive Matroska's linked segments and QuickTime's data references and reference movies.
- The codec's human-readable `name`, from the registry.
- An edition's `has_linked_chapters`, which is yes, no or unknown.
- Picture `width` and `height`, and the native picture type.
- Matroska's `Emphasis`, a version 5 element of the pinned schema, in the details.

**Fields defined too narrowly:**

- `languages` (F24). Several language and audio-type pairs per stream, with 0x01 to 0x03 of Table 2-63 kept as roles.
- `channel_layout` now carries Ambisonic components (`rfc8486` 3).
- `channel_count` is the decoder's output. AAC's core under parametric stereo goes to the details.
- Tag values keep their native type (F25): ASF's integers, Booleans and GUIDs.
- A FLAC picture that holds a URI is an external reference, never fetched, and no attachment.
- `PCR_PID` 0x1FFF leaves the clock reference absent (H.222.0 2.4.4.9).
- A chapter title keeps all its languages.
- `raw_identifiers` lists every identifier present, and is empty for a format such as SubRip that names none.

One claim was left out: that QuickTime names a timecode track through a `tmcd` track reference. The stored documentation does not say so.

**Verification:** `node scripts/check.ts` passes.

**Open:** the maintainer's decisions on the 25 findings; then task 2, the API specification.

## 2026-11-22 — Progress: Phase 2, task 1, ninth review

The maintainer reviewed the staged documents a ninth time and raised 16 issues: six wrong results, six missing mappings and four definitions too narrow. Each was checked against the stored source and accepted. The revision is in the working tree; the index was left as it was.

**Wrong results:**

- *Matroska protection.* An encryption encoding protects the blocks only when `ContentEncAlgo` is not 0, because its default 0 means "The data are not encrypted" (RFC 9559 5.1.4.1.31.9).
- *Matroska Opus output.* Without `OutputSamplingFrequency`, the output rate is absent. Opus counts at 48 kHz, but a decoder may output 8 to 48 kHz (RFC 7845 5.1). The Opus sample-rate cell now says so as well.
- *Transport-stream language.* `language` is the pair marked primary, 0x80, or the only pair unless it is clean effects. With several pairs and no primary, it is absent. The pairs keep their order and every role of Table 2-63.
- *WAVE.* A `wavl` list interleaves `data` and `slnt` chunks, so its count and duration come from the `fact` chunk, which that form requires ("Storage of WAVE Data", "FACT Chunk").
- *DVB canvas.* A complete display set without a display definition segment gives 720 × 576, since the segment is "transmitted once per display set" and its absence means that size (EN 300 743 5.1.3). A read cut short leaves the canvas absent.
- *Linked chapters.* Positive evidence comes first: a link seen gives yes even when the edition was not read in full.

**Missing mappings:**

- *MP4 data references.* External `dref` entries map to external references, with their track and sample description. The flag 0x000001 marks a self-contained entry, which gives none (ISO 8.7.2).
- *Linked pictures.* A linked picture stays an attachment with its description, picture type and pixel size. Its URI goes into a new `link` field and is also listed as an external reference, never fetched.
- *Media types for later releases:*
  - `audio/flac` (RFC 9639 12.1);
  - `audio/mpeg` (RFC 3003);
  - Ogg by RFC 5334 sections 5 and 10, under the evidence rule;
  - `audio/vnd.wave` (RFC 2361);
  - `text/vtt` (WebVTT 10.1);
  - `application/vnd.ms-asf` (IANA registry).

  Each phase settles the rest.
- *Xing/Info extension.* Its bytes B1 to B3 give the encoder delay and padding, in 12 bits each. The trimmed sample count and the duration subtract them.
- *Opus output gain.* It goes to the details: Q7.8 dB (RFC 7845 5.1), or `dOps` `OutputGain`, apart from any gain tag.
- *WAVE cue points.* They become chapters without an end, with `dwName`, `dwPosition` and the label from the associated data list. The catalog now allows a chapter without an end where the format marks a point.

**Definitions too narrow:**

- *Timecode.* `start_timecode` is a signed frame number, which allows negative times (flag 0x0004). A QuickTime counter (flag 0x0008) is no timecode and stays in the details (QT "Timecode sample data").
- *Color range.* Matroska's `Range` is normalized before comparison: 1 is limited and 2 is full, as H.273's flag 0 and 1. A 0 or 3 leaves the range absent.
- *Floating-point values.* Tag values may be floating point, QuickTime's types 23 and 24 (QT "Well-known types").
- *Tag locales.* Tags carry sets of languages and countries, with QuickTime's default locale kept apart (QT "Localization list sets").

**Verification:** `node scripts/check.ts` passes.

**Open:** the maintainer's decisions on the 25 findings; then task 2, the API specification.

## 2026-11-22 — Progress: Phase 2, task 1, tenth review

The maintainer reviewed the staged documents a tenth time and raised 19 issues in the guide, plus a wrong count in this log. Each was checked against the stored source and accepted. The revision is in the working tree; the index was left as it was.

**Wrong values:**

- *MP4 rotation.* Rotation composes the track's matrix with the movie's, in that order, because ISO 6.2.2 composes each track by its matrix and then transforms the result by the movie's.
- *Report bit rate.* The derived rate needs the input to hold all the media. A reference movie with external data has none.
- *Ogg media type.* The type follows the dominant content. `audio/ogg` covers audio with lyrics, metadata or cover art. `application/ogg` requires Skeleton and is never inferred (RFC 5334 4, 5).
- *WAVE cues.* They start at their place in the data: `dwSampleOffset` for PCM in one `data` chunk, and an address resolved through the chunk and block fields in a `wavl` list. `dwPosition` follows the playlist's play order and never gives the start.
- *Enhanced FLV.* Its metadata is scoped by track: track 0 uses the top level, and every further track uses its entry in `audioTrackIdInfoMap` or `videoTrackIdInfoMap`.
- *MP3 trimming.* The delay and padding are read at offset 21 from the recognized extension, because the header's place and the fields before it vary. The trimmed sample count is derived.
- *DTS rates.* `ddts` `DTSSamplingFrequency` is the stream's maximum rate. `SFREQ` gives the core's rate and is never compared with it.

**Missing mappings:**

- *ISO protection.* It covers `enct` and `encs` as well. The IPMP descriptors stand in when `schm` is absent (ISO 8.12).
- *External references.* They name their `owner`: an attachment, or a stream with its sample description.
- *Ogg codec strings.* They come from RFC 5334 4 and RFC 7845 9.
- *MP4 metadata scope.* File-level `meta` (ISO 8.11) and media-level metadata are mapped, and each tag keeps its scope.
- *Declared averages.* DTS's `avgBitrate` and ALAC's `avgBitRate` give `bit_rate`. Peaks and nominal rates stay in the details.
- *ASF duration.* A stream's duration is End Time minus Start Time when both are set (4.1).
- *FLAC tags.* From `0.4.0` they include a leading ID3v2 tag (plan Phase 11, task 2).

**Definitions that lost information:**

- *Bit depth.* Luma and chroma depths are kept apart: H.264 allows 8-bit luma with 9-bit chroma.
- *QuickTime keys.* Metadata keys keep their namespace and key bytes.
- *Timecode wrap.* The timecode keeps its 24-hour wrap flag, 0x0002.
- *ALAC layout.* Above two channels without a layout, ALAC has "no specific channel designation or role", so no speaker positions are guessed.
- *Localized names.* ASF's localized stream names fill `titles`, and `title` is chosen from them.

**Log:** the entry of the ninth review now says 16 issues, the sum of its own subtotals.

**Verification:** `node scripts/check.ts` passes.

**Open:** the maintainer's decisions on the 25 findings; then task 2, the API specification.

## 2026-11-22 — Phase 2, task 1, consolidated field review

The maintainer requested corrections in the working tree and a second review of the corrected documents. Revision 26 aligns the guide with the phase tasks and the pinned specifications.

- **Presentation timing.** ISO media distinguishes decoding order, composition intervals and edit-list playback. A later fragment alone cannot identify the first sample's description. QuickTime requires roll distance −1 for its AAC priming convention. The Opus binding's worked example has 34,560 decoded samples, 312 priming samples and 33,600 valid samples, leaving 648 samples of padding.
- **Streams and metadata.** USF subtitle groups retain separate languages, roles and cue counts. DVD title inventory includes the video stream. Ogg Skeleton timing, FLV sound formats and presentation intervals, ASF frame timing and RealMedia timestamp adjustments now map to the corresponding fields.
- **Field scope.** The tables distinguish original and protected sample entries, core and output audio properties, component precision, declared and derived values, metadata owners and localized names. Matroska chapter flags and DV audio pack mappings use the approved supplemental sources. Unresolved standardized speaker codes stay in the details under F11.

The 25 proposals in section 7 of the guide retain their existing decision points. Task 2 awaits those decisions.

## 2026-11-22 — Progress: Phase 2, task 1, review of the maintainer's revision

The maintainer revised `docs/guide/fields.md` directly and asked for a review. The review raised 13 issues and six slips in citations and wording. Each was checked against the stored source, and the maintainer asked for all of them to be applied. The revision is in the working tree; the index was left as it was.

**Wrong values:**

- *Matroska cropping.* The guide put the codec header's cropping first. RFC 9559 section 15.1 applies `PixelCrop*` to the decoded frame and the display size after it, so with its own pillarbox example the visible size was 1920 × 1080 instead of 1440 × 1080, and the derived pixel aspect ratio 3:4 instead of 1:1. Matroska now joins the ISO family, where the container's crop comes first.
- *Chapters that link a whole edition.* A chapter with `ChapterSegmentEditionUID` plays the whole linked edition, and its end is optional (RFC 9559 section 17.2.2), so its stated length is not what plays. The edition's playback duration and the positions after that chapter are now absent. A chapter that plays a stretch of another segment keeps its length, on that segment's timeline (17.2.1). This amends section 8.5 and Phase 5, task 9 of the plan (revision 27).
- *ProRes codes.* RDD 36 uses 0 for "Unknown/unspecified" in its primaries, transfer and matrix tables (6.1.1, Tables 5 and 6), where H.273's matrix 0 is the identity matrix of RGB. A 0 now supplies no code point. `aspect_ratio_information` 1, "Square pixels", gives the pixel aspect ratio 1:1, not a display ratio (Table 3), and `frame_rate_code` 0 gives no rate (Table 4).
- *H.264 scan type.* `frame_mbs_only_flag` 1 says only that every coded picture is a frame (7.4.2.1.1). Such a frame can be shown as two fields (`pic_struct` 3 or 4, Table D-1), and `ct_type` can mark an interlaced source (D.2.3). The scan type is now absent without the picture timing messages.
- *Matroska defaults and versions.* An element's `minver` is "the first version of the docType to support the EBML Element" (RFC 8794 section 11.1.6.13). The guide applied this only to `Emphasis`. A default now applies only when the file's `DocTypeVersion` reaches the element's version: from 2 for `FlagEnabled` and `FlagInterlaced`, from 3 for `StereoMode` and `AlphaMode`, from 4 for `FieldOrder` and `CodecDelay`. Before, a file of version 2 would have reported a codec delay of 0 for an Opus track whose pre-skip is 312. Phase 5, task 2 now records the minimum version in the schema table (revision 27). F1 states the consequence for `CodecDelay`.

**Gaps and ambiguity:**

- *Audio precedence.* `encoder_delay` and `output_sample_rate` had no order. The header comes first where the codec's mapping makes it normative, as Opus's pre-skip in Ogg and Matroska; the container comes first where the mapping makes the header informative, as the Opus binding to the ISO family does. The Matroska Opus mapping requires `CodecDelay` and sets it from the pre-skip, so the two are compared.
- *Pixel aspect ratio.* Matroska's ratio was the presented pixel shape and the ISO family's the coded `pasp`, even under a track size of another shape. The guide's own example reported 1:1 beside 16:9 for a 640 × 480 aperture. New finding F26 proposes the presented shape for every format. Section 3.4 and the MP4 table follow the proposal until G2.2.
- *Fallback wording.* Section 1 now says that a lower source is used only when no higher one establishes a value, and never where the format forbids the fallback.
- *QuickTime chapter lists.* They are entry points: the end is absent. The `chap` reference counts only from an enabled track, and the first list is taken.
- *`btrt` average.* A 0 supplies no value, as for `esds`.
- *DTS with an extension.* The asset's rate and the `ddts` rate are maxima, so the rate comes from the container. The asset's channels are the output only for one presentation of one asset (7.5.2). Their citations move from 7.5.2 to 7.5.3.2, where the asset's fields are defined.
- *`application/ogg`.* RFC 5334 leaves "complex" content to the writer, so Phase 9 defines the evidence, and until then the type is not reported.
- *Opus priming in the ISO family.* It is the edit's `media_time`, never `PreSkip`.

**Citations and wording:** the Matroska bit rate cites `rfc8794` 6.2 for an unknown size, where the bare "6.2" pointed at RFC 9559's CRC-32 section; an MP4 cell names ISO for 8.6.6; `tags.itunes` is called a coverage row; the AV1, VP9 and DVB size products have parentheses; the DTS and TrueHD codec strings say "the sample-entry code alone" instead of "likewise"; QuickTime's track names fill `titles`, one per language code. The review's remark that `tnam` belongs to the movie's user data was wrong: QuickTime's "User data atoms" page places it in the track's.

**Verification:** `node scripts/check.ts` passes.

**Open:** the maintainer's decisions on the 26 findings; then task 2, the API specification.

## 2026-11-22 — Phase 2, task 1, review against the format bindings

The follow-up review corrected the field inventory and amended the plan in revision 28.

- **Matroska.** RFC 8794 section 15.1 distinguishes the state assumed by older versions from the version in which an element can first be stored. A version 1 track can be effectively enabled with no stored flag. RFC 9559 section 18.10 gives the older stereo codes. A whole-edition chapter link can have a known playback position even when its length and all following positions are unknown.
- **ISO media.** An empty track fragment can extend the preceding sample through `tfdt`. A composition end in `stbl` covers the movie tables, and one in `trep` covers the fragments. Per-codec bindings settle which audio entry fields supply rates, channel counts and precision. Examples include the integer rate in `mlpa`, the reserved channel word in 3GP AMR, and the compatibility rates of DTS and high-rate FLAC.
- **Other mappings.** The guide distinguishes the active OpenDML frame from the digitized area, different DTS and Dolby presentations, coded and presented pixel shape, and FLAC channel assignments from channels without assigned speakers. It covers Ogg Opus pre-skip, end trimming and sample counts, and the zero-rate non-audio case of FLAC.
- **Source decisions.** F27 records the missing major-sync description for DVD-Audio MLP. F28 records the reserved-word arithmetic missing from the WavPack description for extended totals. Both await GS.2. The guide now lists 28 open findings.

F2 now proposes rounding exact half-nanosecond ties away from zero. That choice remains with G2.2.

**Verification.** The field tables and finding references were checked again. `node scripts/check.ts` and both Git whitespace checks pass.

## 2026-11-22 — Progress: Phase 2, task 1, review of revision 28

The maintainer revised `docs/guide/fields.md`, the plan (revision 28) and this log directly and asked for a review until no issue remains. Every changed cell was checked against the stored sources.

The main correction of revision 28 holds. RFC 8794 section 15.1 says that the default of a new mandatory element "MUST represent the state that was assumed for previous versions", so revision 27's rule that a default applies only from the element's `minver` was wrong.

These cells were also confirmed against their sources:

- the `tfdt` extension of the preceding sample, an empty fragment included (ISO 8.8.12.1), and the scope of `cslg` in `stbl` and in `trep` (8.6.1.4.1);
- the sample-entry fields that the AC-3, E-AC-3, AC-4 and TrueHD bindings tell readers to ignore;
- the 32-bit `SampleRate` of `mlpa`, the reserved 2 and 16 of the 3GP AMR entry, the 48,000 of high-rate FLAC and DTS, and the FLAC `samplesize` rule;
- the DTS presentation masks;
- the OpenDML active frame;
- the WAVE PCM byte order and signedness;
- the WavPack 40-bit total;
- the Ogg granule of −1;
- the RFC 9639 zero rate and channel masks;
- the Vorbis channel order above eight channels;
- ASF's average bit rate with its overhead;
- the `mp4s` and `mp4v.20.9` forms of RFC 6381.

The review corrected the following in the guide:

- **AC-4 sample rate.** The cell gave the base frequency of `fs_index` alone. A substream's `dsi_sf_multiplier` raises a 48 kHz base to 96 or 192 kHz (TS 103 190-1 4.3.3.7.3, table 89; TS 103 190-2 E.11), and G.2.2 derives the presentation's rate from both.
- **Matroska version checks.** "Check a stored element" said nothing about the outcome. A stored element above the file's version breaks RFC 9559 section 7, and is now read with a diagnostic. One past its `maxver`, such as `OldStereoMode` in a file of version 3, "SHOULD be discarded" (RFC 8794 section 15.2): it supplies no field. Revision 29 carries this into Phase 5, task 2, with two tests. The `stereo_mode` cell now limits `OldStereoMode` to its `maxver` 2.
- **Opus codec delay in Matroska.** The cell gave no numbered precedence. It also said that the mapping requires "an explicit nonzero delay", but the mapping requires `CodecDelay` at any value (codec draft 3.4.32). The header's pre-skip now comes first. A stored value less than one nanosecond from the exact duration agrees with it; a value farther away is a conflict. An omitted element beside a nonzero pre-skip is a missing declaration.
- **Pixel aspect ratio.** Section 3.4 paired every container ratio with the visible size, although OpenDML's ratio describes the active frame. The rule now names the region that each ratio describes, and an unestablished region leaves the field absent, in section 3.4, the MP4 table and F26 alike.
- **OpenDML.** The display aspect ratio cell gains a numbered precedence, and the pixel aspect ratio cell its provenance. The bitmap can digitize more or less than the active frame, not only part of it. The table of format tokens lists the reciprocal ratio, 2160:1940 for NTSC CCIR 601.
- **`FlagDefault`.** It is limited to audio, video and subtitle tracks by its definition, in the same form as `FlagForced`.
- **WavPack.** The cell took "the 40-bit total samples" and then excluded a nonzero upper byte. It now takes the low word when the upper byte is 0.
- **F27.** It now cites section 1.2 of the Dolby description, which says that the FBB syntax of DVD-Audio is not described, and names its documents by key, as F28 does too.
- **Wording.** The MP4 display size's second source, the output sample rate, the quotation marks and citation form of the AVI sample format, the 3GP citation by its key, a capital after a semicolon in the Ogg row, and the forms "finding F11", "finding F27", "finding F28" and "Finding F2".

In the plan:

- Appendix E's row 28 and its themes now name the Matroska defaults.
- The Phase 2 progress line mentions revision 28 once.
- Phase 5, task 2 names the IANA registry again.

**Verification:** `node scripts/check.ts` passes.

**Open:** the maintainer's decisions on the 28 findings; then task 2, the API specification.

## 2026-11-22 — Phase 2, task 1, review of revision 29

Revision 30 corrects field sources that change the report for valid files. The review covered the working guide, the plan and their source references.

- Matroska `V_AV1` track dimensions repeat the sequence maxima. A reduced still-picture header can establish the output raster even when the coded width depends on super-resolution. A stereo track's display dimensions describe one eye. RFC 9559's 3840 × 1080 side-by-side example consequently has square pixels. The guide also distinguishes each encryption scope and records an omitted Opus `CodecDelay` even for a zero pre-skip.
- The AV1 ISO binding requires movie timing for presentation and defines which color values go into its codec string. AV1, AVC and HEVC configuration records can supply fields without an in-band header. Matroska's VP9 feature list declares profile, level, bit depth and chroma subsampling. F29 proposes its missing deliverable and fixture. VP9 color-space values now follow the VP9 table, including the two possible BT.2020 matrices.
- A 3GP H.263 `bitr` box declares an average bit rate. The H.263 picture header can mark a field. The standard's 704 × 240 field with a 5:11 pixel ratio represents a 704 × 480 frame with a 10:11 pixel ratio. The guide now covers the codec's 8-bit precision and limited range. The VC-1 matrix distinguishes the target scan type and converts stored size codes to pixels.
- ETSI TS 102 366 V1.4.1 reserves sample-rate code 3. Its E-AC-3 program layout includes only the selected program's substreams and counts replacement channels once. The RealAudio matrix now identifies which header versions actually declare audio properties and tags. The Ogg rules require evidence of what a final granule denotes as well as its clock.

The follow-up review checked these mappings across the general rules, individual cells, findings and phase tasks. The guide has 50 tables and 29 findings. The repository checks and Git whitespace checks pass. The API proposals and deliverable decisions remain open for maintainer review.

## 2026-11-22 — Phase 2: the field guide's review closed, task 2 drafted

**Decisions of the maintainer, with the reasons given in the proposals:**

- **The fifteen API proposals of the field guide** (F1 to F8, F14, F18, F22 to F26) are the baseline of the API specification, as written. G2.2 confirms or changes them when the maintainer walks through `api.md`.
- **The six plan amendments** (F9, F12, F15, F19, F20, F29) are accepted and applied in revision 31. Each follows from a pinned source: the tags draft defines only `BPS` and `FPS` as statistics; no source describes a chapter list box; the AV1 sequence header states only maxima; the WebM guidelines define four WebVTT identifiers; neither the VobSub index nor the PGS description gives the end of the last event; and the `V_VP9` feature list is a representation of its own.
- **Sources (GS.2).** F13's three pages are added now, because Phase 7 of `0.1.0` reads the members they define: `microsoft-avimainheader`, `microsoft-avistreamheader` and `microsoft-avioldindex`, snapshots of the MicrosoftDocs repository at commit 656e9656bfae, the commit of the stored bitmap pages, with their checksums recorded. The other source questions (F10, F11, F16, F17, F21, F27, F28) wait for their phases. The stored stream header still defines `wLanguage` only as "Language tag", so the AVI stream language stays absent.
- **E-AC-3 programs.** Program 0, independent substream 0 with its dependent substreams, fills the stream's normalized fields, because every decoder decodes it and it carries the main audio program (ETSI TS 102 366 E.2.8.1, E.2.10.1). The other programs go to the details, as AC-4 presentations do. The field guide's E-AC-3 cells say so.
- **The async source trait** is poll-based, like `AsyncRead`. Return type notation is still unstable, so a public `async fn` could not let a caller require a `Send` future; the poll-based trait is object-safe, and the probe future is `Send` exactly when the source is.
- **Thoroughness.** No section defined a level, so the option is left out of `0.1.0`; a release can add it. Plan section 8.2 is amended in revision 31.
- **The usage summary** is proposed as public, so callers can tune limits from real numbers. G2.2 confirms it.
- **A path-based async convenience** is proposed for Tokio: `asynchronous::tokio::probe_file` and `detect_file`. It needs Tokio's `fs` feature, which the dossier does not approve, so it goes through GS.1, together with G2.2.

**GS.1 evidence for Tokio's `fs` feature.** The disposable spike `spikes/tokio-file-access/` builds Tokio 1.54.1 with `default-features = false` and `fs` alone. The graph keeps the two approved crates, `tokio` and `pin-project-lite`; the feature enables no other feature and no optional dependency, and no runtime is pulled in. A file opened with `tokio::fs` satisfies the probe's bounds, and the future is `Send`. Outside a Tokio runtime, Tokio panics on the first poll, so the rustdoc of the convenience states that it must run inside one. The GS.1 decision is open.

**Task 2.** The first draft of `docs/design/api.md` specifies:

- the principles, naming conventions, features and public paths;
- the ten blocking entry points, the `Prober` builder with its options, and the limits;
- the positional source traits, blocking and async;
- the ordered outcome procedure, the stream inventory classes, completeness with its fixed-size record, and the header statuses;
- every type of the report model, with an accessor for each of the 100 fields of the field guide's section 3;
- detection, the async surface, the live error and its record;
- the conventions of the typed details, with Matroska and H.264 as worked examples;
- the serialization points that task 5 expands, the companion lookup design and the compatibility mechanisms.

The normalized tag keys of F4 come with their source names in Matroska, QuickTime and RIFF `INFO`. Each name was read in the stored tags draft, the QuickTime documentation and the RIFF specification. A key is given by a tag's name alone, because a Matroska level means different things in audio and video (tags draft section 3.3).

**Open points the draft raises:**

- No approved source defines the item-list codes that carry track and disc numbers (plan Phase 6, task 12), so those keys have no `0.1.0` source.
- The tags and chapters options are new: plan section 8.3 allows a section to be "not requested", which only an option can cause.

**Verification.** `node scripts/check.ts` passes, and the spike's tests pass.

**Open:** the review of `api.md`; GS.1 for Tokio's `fs` feature; then task 3, the usage scenarios.

## 2026-11-22 — GS.4: generated test files (revision 32)

**The request.** The maintainer asked for test files generated with ffmpeg, mkvtoolnix and similar tools, by a TypeScript script in `scripts/`. Each file is to be generated only once and kept, unused files are to be removable, and the files are to go into `./target/test-files/` in the folder structure of the specifications. CI is to follow the same rules, every implementation phase is to be covered by tests above 90%, and the rules are to be written into the documents. The request changed R8, which forbade external tools to generate fixtures, and the rule of `CLAUDE.md` that no multimedia tool is named in the repository. The maintainer pointed out that only the tools' binaries are run to make files: no source code of theirs is read or built, so the clean-room rule (R16) is not involved.

**Decisions of the maintainer:**

- **R8, through GS.4.** Generated files replace hex fixtures wherever a tool can produce the case. Hex fixtures and builders remain for what no tool writes. No test compares the library's results with what a tool reports; expected values come from the recipes and the specifications.
- **Naming.** The tools may be named in the material of generation and testing; the library and its rustdoc stay free of them. Writing the rules showed that the plan's requirement table and revision history, the dependency dossier and this log must name them too, because they record the decision and the approved tools. The exception therefore lists the generator, its recipes, the CI workflow, the plan, the dossier, this log, `docs/guide/test-files.md`, `CLAUDE.md` and `CONTRIBUTING.md`. The library, its rustdoc, the other guides, the changelog and commit messages never name them. This wider list awaits the maintainer's confirmation.
- **Layout.** By topic, then format: recipes in `crates/cc-media-probe-testkit/test-files/<topic>/<format>.json`, files in `target/test-files/<topic>/<format>/`.
- **Tools, through GS.1.** On a development machine the generator assumes the tools are installed. It looks on the system path, then in their usual install locations, and reports an error when it finds neither. In CI each platform installs them with its package manager and generates its own files. The requested versions are the current releases, ffmpeg 9.1 and mkvtoolnix 104.0. The CI steps start with the first recipes, as plan section 9.8 starts every job.

**What was done:**

- **The generator.** `scripts/generate-test-files.ts` hands over to `scripts/checks/test-files/`, which has one concern per file: recipes, identity and order, state, tools, pruning and the run.
  - A file's identity is the SHA-256 of its path, tool, tool version line, arguments and input identities. A file with an unchanged identity is never generated again.
  - A tool writes to `target/test-files/.partial/`, and the file moves into place only on success. A failed changed recipe also removes its outdated file.
  - Every run removes the files that no recipe describes. Recipe problems and missing tools stop the run before any file is touched.
  - The usual locations are one list for every platform: folders below the file-system root, and folders below `ProgramFiles`, `ProgramData`, `LOCALAPPDATA` and `USERPROFILE`, with and without `.exe`. The script therefore adds no platform branch to the scripts.
  - 16 tests cover the generator with a stand-in for the tools.
- **Trials.** Two end-to-end trials ran on macOS, with ffmpeg 8.1.2 from Homebrew and mkvmerge 101.0 from `/Applications/MKVToolNix.app/Contents/MacOS`. The generator found the latter without any change to `PATH`.
  - The first trial was an FFV1 file.
  - The second was an H.264 stream and an Opus stream, muxed into Matroska with `--deterministic`.
  - In both, the second run generated nothing, and every file generated again from scratch was byte for byte identical.
- **The documents.**
  - `docs/guide/test-files.md` documents the generator in full.
  - The plan states the rules in R8 and sections 9.1, 9.2, 9.3, 9.8, 9.9, 11 and 12, the Phase 3 test kit and the risk table (revision 32).
  - `CLAUDE.md`, `CONTRIBUTING.md` and the dependency dossier (section 1.8) carry the rules and the approved tools.

**Verification.** `node scripts/check.ts` passes.

**Open:** the maintainer's confirmation of the wider naming list; the CI steps with the first recipes; the review of `api.md`, and GS.1 for Tokio's `fs` feature.

## 2026-11-22 — GS.1 Tokio's `fs` feature, the naming list and Linux CI [CONFIRM]

The agent asked four questions on the points the previous entries left open. The maintainer took every recommended option.

| Topic | Decision | Reasoning |
|---|---|---|
| Tokio's `fs` feature (GS.1) | Approved for `asynchronous::tokio::probe_file` and `detect_file`. G2.2 still confirms the functions with the rest of the API. | The spike `spikes/tokio-file-access/` showed that the feature adds no crate and enables no other feature, and that the opened file meets the bounds of `probe_reader`. The functions must run inside a Tokio runtime, as every operation of `tokio::fs` must. |
| Naming list of R8 | The wider list is confirmed: the plan, the dependency dossier and this log may name ffmpeg and mkvmerge. | They record the decision and the approved tool versions. The library, its rustdoc, the other guides, the changelog and commit messages still never name them. |
| Test-file tools in Linux CI | Ubuntu's own packages. On `ubuntu-26.04` these are ffmpeg 8.0.1 (`7:8.0.1-3ubuntu2`) and mkvtoolnix 97.0 (`97.0-1build1`), both in `universe`, as Launchpad, UbuntuUpdates and Repology listed them on 2026-11-22. | Each platform records the version it ran. A test asserts only what its recipe fixes, so an older version changes no expected value. MKVToolNix's own repository has 104.0 for amd64 only, and ffmpeg has no upstream package repository, so taking a third-party source would bring the Linux versions into line for one tool at most. |
| Order of the Phase 2 documents | Task 3 starts now, and `api.md` is reviewed together with its scenarios. | Awkward scenarios can still revise the draft before the review, rather than after it. |

MKVToolNix 104.0 was released on 2026-11-22, the day of the request, so the requested versions are still the current releases.

**Plan, revision 33:** D5, section 8.2's table of entry points, section 8.7 (the path input of `asynchronous::tokio`), section 9.8 (the Linux versions), the status line, the Phase 2 progress line, and Appendix E. **Dossier:** the `tokio` row of section 1.1 now has `fs`, and section 1.8 gives the Linux versions. **Graph spike:** `spikes/dependency-graph/` asks for `fs` too, so it still copies the approved settings; `cargo tree -e normal` still lists only `tokio` and `pin-project-lite`, and the lock file is unchanged. **Testing guide:** section 8 gives the Linux versions.

## 2026-11-22 — Progress: Phase 2, task 3

**Written:** section 15 of `docs/design/api.md`, the usage scenarios. It has eleven subsections that follow the list of task 3 and contain 13 programs:

- listing streams, which is the rendering half of the quick start;
- video size and frame rate, with absent values handled;
- the default audio language;
- tags with their targets;
- chapters on both timelines;
- JSON, on its own and as a document;
- async probing with Tokio, with futures-io and with a custom async source;
- bytes, standard input and a custom blocking source;
- a directory scan on several threads, with the partial data of errors;
- format and codec details;
- an estimated duration.

**Checked by a spike.** `spikes/api-scenarios/` is a stub of the specified surface, named `cc_media_probe`, with the scenarios as its examples. The code in `api.md` is generated from those files, so the two cannot differ.

- Clippy passes with warnings denied, with no feature, with each feature alone and with all features.
- Five tests pass. They check at compile time that the model, the error and the prober are `Send`, `Sync` and `'static`. They check that the Tokio futures and the probe of a boxed `Send` source are `Send`, and that patterns match enums returned by reference. They also check that a source answering `Pending` is asked again with the same request.
- All 13 programs end with an ordinary error, and none panics. The Tokio program's path convenience ran inside its runtime.
- The structure check of D23 reports 0 findings for the spike.
- The two diagrams of `api.md` that changed render without error in mermaid.js.

**What the scenarios changed in the specification** (its section 15.12, to be confirmed at G2.1 and G2.2):

- **Enums by reference.** The catch-all of an open enum holds text, so no such enum is `Copy`. The draft returned `Option<Provenance>`, `FrameRateMode`, `InventoryClass`, `Option<Rotation>`, `FloatingPointWidth`, `StoredValue<…>` and `Option<Estimate<…>>` by value. Each would have needed a clone, so all are now references (rule 7).
- **Stable names.** Every enum with a catch-all has `as_str()`, its serialized name. A program can then print a kind without relying on `Debug`.
- **A codec's display.** `CodecInformation` displays the codec's name, or else its first raw identifier. `Codec::name()` returns `Option<&str>`.
- **Tags.** `TagSet::targets_whole_file()` is new. `EntityLocation` has `index()` and `chapter_path()`, and `ChapterEdition::chapter_at()` turns a path into a chapter. `BinaryValue` has `size()` and `data()`, `IntegerValue` displays itself, and `TagValue` has a catch-all.
- **What a catch-all implies.** `Limits::get()` returns `Option<u64>`. `Rotation::degrees()` returns `Option<u16>`, and `Format::support()` returns `Option<SupportLevel>`. Limits are passed as `&Limit`.
- **Repeated requests.** After `Pending`, the library repeats the same request to an async positional source, so the source can keep one request in flight. The Phase 3 I/O spike confirms that the block cache keeps this promise (G3.1).
- **Building a document.** `Generator::new(name, version)`, `PortablePath::from_path(&Path)` and `DocumentEntry::error(index, input, ErrorRecord)`; `push` takes `&mut self`, and the index is a `usize`.
- **Whole words (R32).** The structure check rejected `Io`, `io` and `Id` in names that the draft specified. `ErrorKind::Io` becomes `ErrorKind::SourceFailed`, and `io_error()` becomes `source_failure()`. `io_error_kind()` becomes `source_failure_kind()`, and `os_error_code()` becomes `operating_system_error_code()`. The namespace `MatroskaCodecId` becomes `MatroskaCodecIdentifier`. `Feature` keeps only `Zlib`, the one feature that can make information unavailable, so `FuturesIo` is gone. The same scan of every identifier in `api.md` found no other abbreviation of the project's own: the remaining hits are names that the specifications or the standard library fix, such as `DocType`, `MuxingApp` and `PathBuf`.

**Choices made without asking,** each open at the gate named:

- The scenarios use `futures-lite`'s `AssertAsync` over a blocking file for futures-io, as the dossier planned for the shipped example (G2.2).
- `Error`'s `Debug` stays derived. A `main` that returns the error prints that form (G2.2).

**Open:**

- the maintainer's review of `api.md` with its scenarios;
- the G2.3 question whether `document` offers an entry-at-a-time form for long scans;
- task 4, the CLI contract, after the review.

## 2026-11-22 — Progress: Phase 2, the first review of the API specification

The maintainer reviewed `docs/design/api.md` with its scenarios and held back G2.2 on 21 findings: two of priority 1, eighteen of priority 2 and one of priority 3. The review also noted that the spike's async test never drops a probe and reuses its source. Each finding was checked against the plan, the field guide, the stored RFCs and the QuickTime documentation, and each one holds. The revision is in the working tree.

**Decisions of the maintainer.** The agent asked three questions. The maintainer took the recommended option for the first two and the alternative for the third.

| Topic | Decision | Alternatives offered | Reasoning |
|---|---|---|---|
| Attachments and cover art | The options split the attachments by role. The attachments option covers every attachment that is not cover art; the cover-art option covers cover art in any carriage, even inside tags that are not read. The section is "not requested" only when both are `Skip`; otherwise it is measured against the requested roles. | Pictures inside tags examined only when tags are read; the cover-art option as a refinement of the attachments option | With attachments skipped and cover art at its default, one row promised "not requested" and the other asked for cover metadata. Split by role, each option has one meaning, and a skipped role never makes `is_complete()` false. |
| Companion lookup | An option for each kind of entry point: `with_companion_lookup` for the blocking ones and `with_async_companion_lookup` for the async ones. The async resolver returns a boxed `Send` future of an `AsyncResolution`, whose found source is an `AsyncPositionalSource`. | One option, with a resolver of the wrong kind not called; one option, with async probes calling a blocking resolver inline | A mismatch cannot be configured, and a blocking resolver never blocks an async task. A poll method on a resolver shared through `Arc<dyn …>` has nowhere to keep each request's state when probes run concurrently. An `async fn` in a trait still cannot be called through `dyn` in stable Rust, as the Rust project's 2026 goals state, so the future is boxed. |
| 64-bit values in JSON | Wide fields are always strings. A detail integer of a 64-bit type and the fixed-size record's two omitted counts are decimal strings whatever their value. A floating-point value is written by its bits in hexadecimal. Offsets and usage counters keep the twins of plan section 8.8. | The twin rule of section 8.8 for every integer | A field's JSON type never depends on its value. RFC 7493 section 2.2 recommends strings for integers beyond 2^53 − 1, and the bit pattern of double-precision 1.0 is already beyond it. |

**The findings and their changes** (the table in section 16 of `api.md` lists them with their sections):

- **Async sources and lookup.**
  - *A dropped probe and a reused source (priority 1).* The repeated-request promise now holds within one probe. A probe that borrowed `&mut source` and was dropped left its request with the source. The scenario's source returned that request's bytes to the next call, which could ask for another offset. The source now keeps the range of its request in flight and starts a new request when the range differs. A new spike test drops a probe while its request is pending, asks for another range, and runs a second probe on the same source.
  - *Tokio's `detect_file`.* It goes through `detect_reader` and keeps the file-name hint.
  - *Resolvers.* As decided above. `limit_reached(limit)` names the limit, as plan section 7.7 requires.
- **Outcomes.**
  - *Candidate checks.* A rejected candidate beside one that cannot be checked is `FormatNotDetermined`, with the candidates that were not rejected (plan section 8.3).
  - *An empty FLV file.* An observed end of input at a unit boundary reaches the inventory where the format's entry says so, as FLV's does; a limit that stops at the same byte does not.
  - *End of input before a header.* `NotFoundWithinBudget` takes truncated input as its reason (plan section 7.4).
  - *Attachments and cover art.* As decided above.
- **Report fields.**
  - *Protection (priority 1).* `Protection` lists declarations, each with its scheme and scope. A sample-description scope names the description's index, so a clear description beside an encrypted one is told apart (field guide F22).
  - *Mastering display.* Each primary and each coordinate is optional, because RFC 9559 stores each as an element of its own (5.1.4.1.28.31 to 5.1.4.1.28.40).
  - *Color.* Each component has its provenance, because an override supplies only the components it states (field guide section 2): a QuickTime `nclc` atom has no range.
  - *Metadata keys.* `TagName::bytes()` keeps the key bytes where a format defines them. The QuickTime keys atom stores a key as `Key_value[Key_size-8]`, whose meaning depends on its namespace.
  - *H.264.* Every field of a parameter set is optional, and `primary_sequence_parameter_set()` names the set that supplied the normalized fields. Section 11.2 makes the rule general: a field that the input can cut off is optional.
  - *EBML versions.* `DocTypeVersion` and `DocTypeReadVersion` have a default of 1 (RFC 8794 11.2.7, 11.2.8), so they are `StoredValue`s, as is `TimestampScale`.
  - *Instants.* `UtcTime` holds years 0 to 9999, which RFC 3339 can write (`date-fullyear = 4DIGIT`, 5.6).
- **Serialization and accessors.**
  - *64-bit values.* As decided above.
  - *Error offsets.* `Error` has `offset()` and `logical_offset()`, so `ErrorRecord` has them too.
  - *Paths.* A path that is not lossy has no raw form (plan section 8.8), so `to_native()` rebuilds it from the display string.
  - *Documents.* `Document` and `DocumentEntry` have getters, with `EntryOutcome`. A nested root record carries the document's version, and reading checks it.
- **Scenarios.** The directory scan runs a fixed number of workers instead of one thread per file. The async scenario returns the file's error instead of probing an empty object.

**Found by the spike.** The getters `report()` and `error()` share an `impl` with the constructors `DocumentEntry::report(…)` and `DocumentEntry::error(…)`, which Rust does not allow. The constructors are now `from_report`, `from_detection` and `from_error` (G2.1).

**Choices made without asking,** each open at the gate named:

- An instant outside years 0 to 9999 is implausible, as a quantity beyond 2^53 − 1 is. The normalized time is absent, with a diagnostic, and the raw timestamp stays in the details. The alternatives were ISO 8601's expanded years, which RFC 3339 readers reject, and a numeric form (G2.2).
- A protection declaration has one scope. A Matroska `ContentEncoding` that covers both frames and private data gives two declarations with the same scheme (G2.2).
- `TagName::bytes()` is present only where a format defines its keys as bytes; Matroska stores names as text (G2.2).
- `primary_sequence_parameter_set()` returns the set itself rather than its index (G2.2).
- The EBML versions and `TimestampScale` are `Option<&StoredValue<u64>>`, because the observations of an error can hold a header cut short (G2.2).
- A tag's integer value keeps its `_exact` twin. The maintainer's choice named detail fields and the two counts, and a tag's value is part of the model (G2.2; task 5 can align it).
- The directory scenario uses one worker per available processor (task 12 turns the scenarios into the shipped examples).

**Plan, revision 34:** section 7.7 and Phase 14, task 1 (the options in the plural), section 8.2 (the attachment options by role, the two lookup options), section 8.7 (repeated requests within one probe, and a test that reuses a source), section 8.8 (raw 64-bit values as strings, paths rebuilt from their display string), the status line, the Phase 2 progress paragraph and Appendix E.

**Verification.**

- **Spike.** In `spikes/api-scenarios/`, Clippy passes with warnings denied, with no feature, with each feature alone and with all features. Seven tests pass: the five of task 3, the dropped-and-reused source, and a document read back through its getters. The stub builds no report, so that document holds error records only; Phase 3's version tests cover a document with mixed outcomes and a nested record of another version (plan section 8.8).
- **Programs.** All 13 programs end without a panic. The JSON scenario reads its document back and exits normally; the others end with an ordinary error. With stand-in files, the directory scan probed twelve files on its fixed pool of workers, and the custom async source reached the stub's own error.
- **Structure.** The structure check of D23 reports 0 findings for the spike. The 13 programs in section 15 of `api.md` match the spike's examples.
- **Diagrams.** The two diagrams of `api.md` render without error in mermaid.js.
- **Checks.** `node scripts/check.ts` passes.

**Open:**

- the maintainer's review of the revision, for G2.1 and G2.2;
- the G2.3 question whether `document` offers an entry-at-a-time form for long scans;
- task 4, the CLI contract, after the review.

## 2026-11-22 — Progress: Phase 2, the second review of the API specification

The maintainer reviewed the revision of `docs/design/api.md` and again held back G2.2, on 14 findings: two of priority 1 and twelve of priority 2. Each finding was checked, and each one holds:

- against the plan, the field guide and RFC 9559 (Tables 21 and 23);
- against ITU-T H.264 (7.4.1.2.1);
- against Tokio's documentation of `tokio::fs`, whose cancellation section says that a dropped future "will stop waiting for the result" while "the underlying blocking operation will continue to run on the thread pool";
- by running the JSON scenario through `jq empty`, which failed at the first summary line.

The revision is in the working tree.

**Decisions of the maintainer.** The agent asked three questions. The maintainer took the recommended option each time.

| Topic | Decision | Alternatives offered | Reasoning |
|---|---|---|---|
| Lookup off | An expected companion is listed only as an external reference, with its validated identifier and the name a request would carry. No request is made, so it gets no source identifier and no source-table entry. Plan section 8.3 is amended to match section 7.7. | A source-table entry with a new outcome "not enabled" | Section 7.7 numbers sources by request, and the source table lists requested sources. The external reference already carries what a caller needs. Section 8.3 was the one passage that disagreed. |
| Protection | Absence is proof. `Stream::protection()` is absent only when every structure that can declare protection was examined and none does. Otherwise `Protection` is present, even with no declaration, says whether everything was examined, and gives each sample description as `Clear`, `Declared` or `Unknown`. | `Protection` only with a declaration, with an unexamined stream visible only through its completeness reasons | A caller can trust a missing `Protection`, and an unread description is never labeled clear. |
| An unresolved codec | `CodecInformation::codec()` returns `Option<&Codec>`, absent while the candidate descriptions of an unproven first sample name different codecs. `candidate_codecs()` lists them. | A variant `Codec::Unresolved` | Rule 2 of `api.md`: a missing value is `None`. `Codec::Unknown` keeps its one meaning, a codec that this version does not know. |

**The findings and their changes** (the table in section 16 of `api.md` lists them with their sections):

- **Protection (priority 1).** As decided above. A declaration establishes a mechanism for its scope, not that every byte is encrypted, because a scheme can leave samples or parts of them clear (field guide, `protection`). The Matroska scope `0x4` applies to the next encoding's settings (RFC 9559 Table 21). It is now `ProtectionScope::EncodingSettings`, and `encoding_order()` names the affected encoding. A scope bit that this version does not know gives `Other`.
- **Attachments (priority 1).** With `WithData`, the section is complete only when every embedded payload was kept. A payload that the retained-payload or read limit stopped leaves `data()` absent and the section incomplete, with the limit named. Data that is not kept is still read and discarded on a forward-only source, and those bytes count toward `BytesRead` (plan section 7.5).
- **Outcomes.**
  - *FLV end of input.* A known length establishes the end: a slice, a file, or a positional source that states its length. Only a source of unknown length needs the read that returns no bytes (plan section 7.5).
  - *Unsupported compression.* A Matroska header under `bzlib` or `lzo1x`, which RFC 9559 lets a reader leave unsupported, is `Identified` with `UnsupportedCompression`.
- **Report fields.**
  - *Codecs.* As decided above.
  - *H.264.* The primary set is the one that the first picture activates: through the picture parameter set that its first slice names, or through a buffering-period SEI message of an IDR access unit (ITU-T H.264 7.4.1.2.1). Without that evidence, and with sets that disagree, no set is primary. A normalized field then comes only from values on which every set agrees.
  - *Owners of detail values.* Conflicts, observed lower bounds and exact values have `owner()`, an `EntityLocation`. `EntityLocation` now has `kind()`: stream, edition, chapter, attachment or program.
- **Lookup and async.**
  - *Shared resolvers.* `CompanionRequest::primary_path()` gives the path that the caller passed to `probe_file`. A caller who needs other context gives each probe its own resolver, through a cloned `Prober`. The built-in resolver declines when the input has no folder.
  - *Lookup off.* As decided above.
  - *Cancellation.* Dropping the future stops the probe. A file operation already on Tokio's blocking pool runs to its end, and its result is discarded.
- **Documents.**
  - *Usage errors.* A document is built by `Document::from_results` or `Document::from_usage_error`, and has no `push`. Reading refuses a usage error beside results, and an entry of a known outcome without exactly one record.
  - *The scenario.* Standard output carries only the document, and the summaries go to standard error. A detection entry has its own arm, and only an outcome from a newer version prints as such.

**Found while revising.** Plan section 8.9 writes the usage error as an object with a `message` member, while the specification returned `Option<&str>`. It is now `Option<&UsageError>` with `message()`. The spike had no detection type, so its document could not hold the detection entries that section 12 specifies. It now has a minimal `Detection`.

**Choices made without asking,** each open at the gate named:

- Compression that this version cannot undo is a final state, as a pending parser is, rather than a cut-short state. Under `--strict`, the report therefore stays complete (G2.2).
- A document is built from all its entries at once. An entry-at-a-time form stays the G2.3 question.
- `UsageError` follows the object of plan section 8.9 (G2.2).
- A detection entry's outcome follows the `--detect` rows of plan section 8.9. A source that fails during detection becomes an error record (G2.3).
- `ProtectionScope::Other` holds a scope that this version does not know, rather than leaving the declaration out (G2.2).
- With no primary parameter set, the fields on which every set agrees are still reported (G2.2; the H.264 phase states, for each binding, when one set is the only candidate).
- Context for a shared resolver comes from the primary path and from a resolver per probe, not from a context parameter on every entry point (G2.2, and the release that ships lookup).
- The built-in resolver declines a request on an input without a path, so the source table shows the request, rather than reporting lookup as "not enabled" (G2.2).
- The three detail entries keep `stream()` beside `owner()` (G2.2).
- An unresolved codec displays its candidates, separated by " or " (G2.2).

**Plan, revision 35:**

- section 6.2 and Phase 5, task 8: unsupported compression, with two new Phase 5 tests;
- section 7.7: no request when lookup is off, the primary path in a request, and the built-in resolver without a folder;
- section 8.3: the split row on companions, the header states, and attachment payloads;
- section 8.7: cancellation;
- Phase 6, tasks 7 and 9: the unresolved codec and the protection states;
- the status line, the Phase 2 progress paragraph and Appendix E.

**Verification.**

- **Spike.**
  - In `spikes/api-scenarios/`, Clippy passes with warnings denied: with no feature, with each feature alone and with all features.
  - Ten tests pass: the seven earlier ones and three new tests of documents. A usage-error document reads back. Reading refuses a usage error beside results, and an entry whose record was removed.
- **Programs.** All 13 programs end without a panic. The JSON scenario exits normally; `jq empty` accepts its standard output, and its summaries are on standard error.
- **Structure.** The structure check of D23 reports 0 findings for the spike. The 13 programs in section 15 of `api.md` match the spike's examples.
- **Diagrams.** The two diagrams of `api.md` render without error in mermaid.js.
- **Checks.** `node scripts/check.ts` passes.

**Open:**

- the maintainer's review of the revision, for G2.1 and G2.2;
- the G2.3 question whether `document` offers an entry-at-a-time form for long scans;
- task 4, the CLI contract, after the review.

## 2026-11-22 — Progress: Phase 2, the third review of the API specification

The maintainer reviewed revision `ed2f78a` of `docs/design/api.md` and held back G2.2 again, on 12 findings: two of priority 1 and ten of priority 2. Each finding was checked, and each one holds:

- against the plan (section 7.7, Phase 5, task 8, Phase 6, task 7), the field guide (sections 3.9 and 4.1) and RFC 9559 (5.1.4.1.31.4, 5.1.4.1.31.9 and 5.1.8.1.1);
- against ITU-T H.264 7.4.1.2.1: a parameter set "shall be available to the decoding process prior to its activation", and a later set with the same identifier replaces the earlier one's content;
- against the committed description of ISO/IEC 14496-15 and the W3C ISO BMFF byte stream format, by which `avc1` keeps parameter sets in the sample entry while `avc3` also lets samples carry them. A web search found the same in the published amendment that added `avc3` and in the DVB DASH profile;
- by reading the spike, whose display of an unresolved codec paired raw identifiers with candidates by position.

The review also noted that the spike's document was not in the wire form of section 12: a version object, capitalized names and nested errors without a version. The revision is in the working tree.

**Decisions of the maintainer.** The agent asked three questions. The maintainer took the recommended option each time.

| Topic | Decision | Alternatives offered | Reasoning |
|---|---|---|---|
| The stem of a VobSub index | From the final component of the probed path, or else from the file-name hint. With neither, no request is made: the data file is an external reference without an identifier or a location, and with lookup on the companions are cut short with the new reason `CompanionNameUnavailable`. | The same stems, with a nameless request that only a per-probe resolver can answer; the path only, never the hint | Every request keeps a name that the library built. The hint is the caller's own statement of the name, and the plan's test of a reader with a supplied resolver needs it. |
| Image headers of attachments | Read within the per-stream header limit, charged to `BytesRead` and working memory, also when the data is not kept. The rest of the payload is passed over, or read and discarded on a forward-only source. | A new limit of its own; the read allowance alone | The image sniffers of Phase 4 are header parsers, and one malformed picture must not use up the whole read allowance. No new counter is needed. |
| The scope of a tag set | `TagSet::target_scope() -> &TargetScope`, with `WholeFile`, `Entities` and `Incomplete`. It replaces `targets_whole_file()`, and the whole-file lookups take only `WholeFile`. | `targets_whole_file() -> Option<bool>` with a flag for cut lists; `Option<bool>` alone | One value states what the lists mean, including lists that were cut short after some targets. |

**The findings and their changes** (the table in section 16 of `api.md` lists them with their sections):

- **Protection.**
  - *A bounded table (priority 1).* `sample_descriptions()` lists only the descriptions found clear or declared, within the retained-data limit. `sample_description_count()` gives the declared count. `sample_description_state(index)` gives `Unknown` within it and `None` outside it.
  - *What declares it.* Only a Matroska encryption with a nonzero `ContentEncAlgo` declares protection, as field guide section 4.1 says; compression and header stripping declare none.
  - *Header status.* A clear configuration beside protected packets has its parser's state; `Protected` means that the bytes holding the header are encrypted.
- **H.264 (priority 1).** The candidates of the first picture are the record's sets and those before its first slice. Agreement counts only once the probe has every candidate: the record of a binding that keeps sets out of the samples, read in full, or the first access unit up to its first slice. Agreeing sets give their common fields without naming a primary set. With candidates missing, no normalized field comes from the sets.
- **Codecs.**
  - *The fallback.* The three cases of plan Phase 6, task 7 are stated. An initialization segment uses the first declared description and lists no candidates.
  - *Candidates.* `CodecCandidate` keeps each description's index, codec and identifiers, and replaces `candidate_codecs()`. Two codecs that this version does not know name one codec only when their identifiers match.
- **Partial reads.**
  - *Tag scope.* As decided above.
  - *Matroska defaults.* Every defaulted detail element is `Option<&StoredValue<…>>`, absent until its omission is established. `FlagDefault`, `FlagEnabled`, `CodecDelay` and `SeekPreRoll` are among them; RFC 9559 gives them the defaults 1, 1, 0 and 0.
  - *Image headers.* As decided above.
- **Documents.** Reading derives the outcome that the record warrants and fails on a mismatch. A report with a status from a newer version, a detection outcome from a newer version, and an outcome from a newer version keep what was written.
- **Lookup.**
  - *A VobSub index without a path.* As decided above.
  - *Reasons per source.* `SourceEntry::reason()` keeps the reason of each request, with the resolver's text as the detail of `CompanionDeclined`. `PartReason::source_identifier()` names the entry. `declined()` takes `impl Into<String>`.

**Found while revising.**

- RFC 9559 sets no order for the children of a `Tag`, and a `Tag` whose `Targets` is empty or omitted describes the whole segment (5.1.8.1.1). So only a tag read in full shows that its targets are absent.
- The spike's document module had grown past one concern. It is now a directory of four files: the document, its entries, its version and its error.
- The spike's documents now have the wire form of section 12 for every member they carry:
  - the version `"1.0"`;
  - `snake_case` outcomes, error kinds and detection outcomes, with their catch-alls;
  - a version on every nested record;
  - the stable I/O error kind;
  - no member for an absent value.

  Its tests read that form back.
- `git mv`, used once to move `document.rs`, staged the rename. The index was restored at once with `git restore --staged`, so nothing stays staged.

**Choices made without asking,** each open at the gate named:

- The table of sample descriptions is sparse, with a declared count and a lookup, rather than a prefix or a list of ranges (G2.2).
- `CodecInformation::raw_identifiers()` holds every candidate's identifiers while the first sample is unproven, so it keeps the field guide's meaning, and each candidate holds its own (G2.2).
- A failed source entry has no reason, since the error keeps its cause (G2.2).
- `PartReason::source_identifier()` exists beside `SourceEntry::reason()`, so both the completeness record and the source table link a reason to its request (G2.2).
- An image header cut short by the limit makes the attachments section incomplete (G2.2, G3.3).
- With candidates missing, no H.264 normalized field comes from the sets; the record's profile and level still do (G2.2, G4.1).
- Reading accepts the written outcome wherever the record holds a value of a newer version (G2.2, G2.3).
- The hint gives the stem only for the name of a request, and the stem is validated like any identifier (G2.2, Phase 14).

**Plan, revision 36:**

- section 7.5: the per-stream header limit covers an attachment's image header;
- section 7.7: the stem of a VobSub index, no request without a name, and the reason in each source entry, with the diagram;
- section 8.3: the header status beside protected packets;
- section 8.5: the scope of a tag set;
- section 8.9: a reader checks the outcome table;
- Phase 4: the primary H.264 set and the image inspection;
- Phase 5, task 8 and its tests: what declares protection, the header status beside it, a track entry cut short and a tag cut short before its targets;
- Phase 6, tasks 7 and 9 and their tests: the fallback, the candidates, the bounded table, `avc3` and an embedded picture;
- Phase 14, task 8 and its tests: the stem and the reasons per source;
- the status line, the Phase 2 progress paragraph and Appendix E.

The field guide follows in sections 3.2 (`protection`), 3.3 (`raw_identifiers`), 3.8 (targets) and 3.9 (`width`, `height`).

**Verification.**

- **Spike.**
  - In `spikes/api-scenarios/`, Clippy passes with warnings denied: with no feature, with each feature alone and with all features. `rustfmt` reports no difference.
  - 16 tests pass: the ten earlier ones and six new tests of documents. The new tests cover the wire form, an outcome that its record does not warrant, detection entries, a nested record of another version, another major version and an outcome of a newer version.
- **Programs.** All 13 programs end without a panic. The JSON scenario and the directory scan exit normally.
- **JSON.** `jq empty` accepts the JSON scenario's standard output. In it `jq` reads:
  - the version `"1.0"`;
  - the outcomes `error` and the kind `source_failed`;
  - the nested version `"1.0"`;
  - the I/O kind `not_found`;
  - no `null` member.
- **Structure.** The structure check of D23 reports 0 findings for the spike. The 13 programs in section 15 of `api.md` match the spike's examples; scenarios 4 and 10 were synced from it.
- **Diagrams.** All 15 diagrams of `api.md` and the plan render in mermaid.js, with no syntax error and no console error.
- **Checks.** `node scripts/check.ts` passes.

**Open:**

- the maintainer's review of the revision, for G2.1 and G2.2;
- the G2.3 question whether `document` offers an entry-at-a-time form for long scans;
- task 4, the CLI contract, after the review.

## 2026-11-22 — G2.1 Naming [CONFIRM]

The maintainer's fourth review of `docs/design/api.md` recommended approving G2.1 and found no issue with the names. The agent asked whether G2.1 was approved, and the maintainer took the recommended option.

| Topic | Decision | Alternatives offered | Reasoning |
|---|---|---|---|
| G2.1, naming | Approved for the names of revision 36 of the plan: the binary `cc-media-probe`, the feature names of `api.md` section 3.1, the async modules `asynchronous::tokio` and `asynchronous::futures`, `MediaReport` and `Stream`, and the other names that section 16 of `api.md` lists. The names that the fourth review adds are confirmed at G2.2. | Hold G2.1 until the names of the fourth review were reviewed too; approve those names as well | The review found no issue with the names. The few names that the revision adds are reviewed together with the API they belong to. |

**Plan, revision 37:** section 8 says that the names are approved, the Phase 2 progress paragraph and Appendix E record it. `api.md` records it in its status, its introduction and section 16, and the field guide in section 3.

## 2026-11-22 — Progress: Phase 2, the fourth review of the API specification

The maintainer reviewed revision `dbeec40` of `docs/design/api.md`. The review recommended approving G2.1, which the entry above records. It held back G2.2 on six findings, five of priority 2 and one of priority 3, and answered yes to G2.3's open question with three more findings of priority 2 on the JSON contract. All 16 tests of the spike had passed, and the review's further checks reproduced four problems of the document that they missed. Each finding was checked, and each one holds:

- by running a scratch program against the stub, which reproduced all four document problems:
  - a document rebuilt from the entries of a 1.1 document was written as `"1.0"` around a `"1.1"` record, and its own reader refused it;
  - an entry of the outcome `skipped` with a 1.0 detection and a 2.0 error was accepted;
  - the plan's `"input": "notes.txt"` was refused as "invalid type: string";
  - a `partial` entry rebuilt with `from_detection` became `error`;
- against the plan (sections 7.4 to 7.7, 8.3, 8.8 and 8.9, Phase 6, task 7) and the text of `api.md`;
- against Serde's documentation of `Serializer::serialize_seq` and `collect_seq`. The length of a sequence "may or may not be computable before the sequence is iterated", and "some serializers only support sequences whose length is known up front". The call is followed by zero or more calls to `serialize_element`, then a call to `end`.

**Decisions of the maintainer.** The agent asked four questions; the first is the G2.1 entry above. The maintainer took the recommended option on the other three. To the question on the writer the maintainer answered "Yes, means yes I found issues. Fix those issues.": the review's points are the maintainer's own, and every one is fixed.

| Topic | Decision | Alternatives offered | Reasoning |
|---|---|---|---|
| A document written as it goes (G2.3's open question) | Yes. `Document::serialize_results(generator, entries, serializer)`, behind `serde`, takes a lazy sequence of entries and any Serde serializer. It writes what `from_results` writes, releases each entry before it takes the next, and stops at the first output error. The CLI uses it, and `from_results` stays. | No writer: callers and the CLI write the framing themselves | Library callers get the CLI's framing, and a scan of any length holds one report at a time. |
| An outcome that this version cannot derive | `from_report` and `from_detection` return `Result<DocumentEntry, UnderivableOutcome>`. The refusal keeps the parts, and `with_outcome(outcome)` builds the entry with a recorded outcome. | Refuse only, and reuse the entry as it was read | A derived guess for a record of a newer version can make a newer reader reject the document. The way back keeps the outcome that a document recorded. |
| A limit that stops detection | A part of its own, `Part::Detection`, which only an error names and which has no status in a report | Record it as discovery | Discovery is stream discovery, which never began. |

**The findings and their changes** (the table in section 16 of `api.md` lists them with their sections):

- **Codecs.** The fallback to the first declared description needs a track proven to have no sample, as an initialization segment examined to its end. A probe that stopped before the first sample's selection leaves it unresolved, with every candidate that it read and kept. `CodecInformation::has_every_candidate()` says whether one is missing, and the candidates name a codec only when none is.
- **Header status.** `NotFoundWithinBudget` and `PartiallyParsedForLackOfBytes` take `RangeUnavailable`, for a header that left the window of a forward-only source before its stream was discovered.
- **Detection.** As decided above. `api.md` shows the error of a read limit of zero, and the directory scan prints the limit and the part.
- **Identity tokens.** `SourceIdentityToken::new(bytes)` and `as_bytes()`. Equal bytes name one source, so the later source becomes an alias of the first.
- **Entry construction.** As decided above.
- **Tags.** The scenario prints all four target lists. An unmatched identifier is "unresolved in this report", and `Unmatched` says that the entity can still be in the file.
- **Nested versions.** A document gives every record it holds its own version: `from_results` and the writer set it. Reading checks every record of every entry, including an entry of a newer outcome that holds several.
- **Paths.** An input is its display string, as plan section 8.9 shows. A lossy path adds `input_lossy` and `input_raw` beside it, and a source-table path adds `path_lossy` and `path_raw`.
- **Identifiers.** Identifiers are strings at every coded width, and the rule comes before the rules for integers of the typed details.

**Found while revising.**

- `from_report` shared the flaw of `from_detection`: a report whose statuses include one from a newer version got `partial`, which a newer reader that knows the status may reject. The refusal covers both constructors.
- The plan wrote the encodings of a raw path `unix-bytes` and `windows-wide`, against the `snake_case` rule of `api.md` section 2. They are now `unix_bytes` and `windows_wide` (plan section 8.8).
- The stub serialized `SourceIdentifier` as a number; it is a string now.
- The stub's detail enums keep the tagging of Serde's derive, so the fixture of a 32-bit identifier checks the identifier member's form, not the enum's `kind` form, which task 5 fixes.
- In Rust 2024 a return-position `impl Iterator` captures the borrow of `&self`. The iterator of nested versions therefore says `use<>`, so that the document can move its entries after the check.
- A read limit inside the movie box would lose the stream inventory (plan Phase 6, task 14), so the plan's test of a missing candidate uses the retained-data limit.
- The scenario of section 15.6 writes its document to standard output as it goes, so it can no longer read the same text back. It reads an earlier `catalog.json` instead and exits normally when there is none.

**Choices made without asking,** each open at the gate named:

- The writer is an associated function of `Document` that takes `&Generator`, any `IntoIterator<Item = DocumentEntry>` and the serializer by value, and it passes no length for the results (G2.2, G2.3).
- A document read and written again as a whole keeps the version it was read with; its records already carry that version (G2.3).
- A path's flag is written only when it is true, and a flag or raw form without the other is malformed input (G2.3, task 5).
- `has_every_candidate()` is a `bool`, not a count of candidates (G2.2).
- An identity token is bytes, charged to working memory for the rest of the probe; a token that cannot be charged closes its source with `LimitReached`, and no token is ever written to the report (G2.2, Phase 14).
- `RangeUnavailable` applies to both cut-short header states (G2.2, G3.1).
- `Part::Detection` has no status in the record (G2.2, G3.3).
- `UnderivableOutcome` keeps its parts behind one pointer and implements `std::error::Error` (G2.2).
- In the JSON scenario a refused report would take `partial`. That cannot happen for a report of this version's probe, which the comment says (G2.2).

**Plan, revision 37:**

- section 7.4: the reason "range unavailable" for a header out of the forward-only window;
- section 7.5: detection in the fixed-size record;
- section 7.7 and its tests: identity tokens;
- section 8: G2.1 approved;
- section 8.3: detection as an activity;
- section 8.8 and its tests: identifiers at every width, paths with their members and encodings, nested versions on writing, and the version and round-trip cases;
- section 8.9 and its tests: the library's writer;
- Phase 6, task 7 and its tests: the fallback and missing candidates;
- the status line, the Phase 2 progress paragraph and Appendix E.

The field guide follows in sections 3 (the names), 3.3 (`raw_identifiers`) and 4.2 (the first sample's description).

**Verification.**

- **Spike.**
  - In `spikes/api-scenarios/`, Clippy passes with warnings denied: with no feature, with each feature alone and with all features. `rustfmt` reports no difference.
  - 24 tests pass: the 16 earlier ones, six new tests of documents and two of the writer. The new tests cover the plan's entries, a lossy input, an identifier of 32 bits, a rebuilt document, every nested record, a refused outcome, the writer's output, and an output failure that leaves the next entry untaken.
- **Programs.** All 13 programs end without a panic. The JSON scenario and the directory scan exit normally.
- **JSON.** `jq empty` accepts the JSON scenario's standard output. In it `jq` reads:
  - the version `"1.0"`;
  - each input as a plain string;
  - the outcomes `error`, with the kinds `unsupported_format` and `source_failed`;
  - the nested version `"1.0"`;
  - the I/O kind `not_found`;
  - no `null` member.
- **Structure.** The structure check of D23 reports 0 findings for the spike. The 13 programs in section 15 of `api.md` match the spike's examples; scenarios 4, 6 and 9 were synced from it.
- **Diagrams.** No diagram changed.
- **Checks.** `node scripts/check.ts` passes.

**Open:**

- the maintainer's review of the revision, for G2.2;
- G2.3, with task 4, the CLI contract, and task 5, the JSON contract;
- the open points of `api.md` section 16, among them the item-list track and disc numbers.

## 2026-11-22 — Progress: Phase 2, the fifth review of the API specification

The maintainer reviewed revision 37 (`69f83bb`) of `docs/design/api.md`, ran the spike's 24 tests, and held back G2.2 on three findings of priority 2. The review confirmed the two caveats of the fourth revision, the 32-bit identifier and the refused report. It also raised three findings of priority 2 for the joint review of G2.3, and a smaller one in the JSON scenario. No gate is approved. Each finding was checked, and each one holds:

- against the plan (sections 7.5, 8.3, 8.8 and 8.9, Phase 6, task 7), `api.md` section 1, rule 7, and the stub, whose completeness getters return references;
- against ISO/IEC 14496-12:2015, whose contents give 8.7.4, 8.8.3 and 8.8.7 for the sample-to-chunk, track-extends and track-fragment-header boxes. Each box holds `sample_description_index` as `unsigned int(32)`;
- by tests written before the fixes, which failed against the stub:
  - a 1.0 document accepted an error record whose partial report was of version 1.1; the review's case was 2.0, which the same test now covers;
  - a document rebuilt from a 1.1 document kept its partial report at `"1.1"`, with `from_results` and with the writer, which agreed with each other;
  - reading took the `unix_bytes` data `"gg"`;
- by running the JSON scenario in a scratch folder with a `catalog.json` that is not UTF-8. It printed the diagnostic and exited 0.

The last-byte panic was not timed locally: a pipe takes the whole document at once, so no reader can close it between the last line and the line break. The scenario wrote that break with `println!`, which panics when its write to standard output fails, as the standard library documents. That matches the review's exit status of 101.

**Decision of the maintainer.** The agent asked one question, and the maintainer took the recommended option.

| Topic | Decision | Alternatives offered | Reasoning |
|---|---|---|---|
| A limit stops one candidate's validation check while another validates and its probe reaches the inventory | The report is incomplete. `Completeness::detection()` returns `Incomplete`, `reasons()` holds the limit against `Part::Detection`, the record names detection as the part that the limit stopped first, and `is_complete()` is false, so `--strict` fails. | Incomplete only when the unchecked candidate's confidence is above the chosen one's; or a diagnostic only, with the report complete | The unchecked candidate might have validated and been chosen, so the report's format is not established in full. Every shortfall still changes `is_complete()`. |

**The findings and their changes** (the table in section 16 of `api.md` lists them with their sections):

- **Getter types.** The completeness table returns `&DetectionStatus`, `&DiscoveryStatus`, `&SectionStatus`, `&TimingStatus` and `&CompanionStatus`, as rule 7 requires for a stored enum. A spike test fixes the types.
- **Detection in a report.** As decided above. `Completeness` gains `detection()`, so the fixed-size record keeps the status under any limits. Section 5.1 says that a stopped check beside a validated candidate leaves the report incomplete.
- **A known description that was lost.** The first sample's description is established, but the probe could not read it or keep it. `codec()` and `name()` are absent, `candidates()` is empty, since no other description can hold the first sample, and `has_every_candidate()` is false. `IsoBaseMediaDetails::track(index)` gives `IsoBaseMediaTrackDetails`, whose `first_sample_description_index() -> Option<u32>` keeps the index. The display is "a description that was not read".
- **The partial report's version.** It is a nested root record too. `ErrorRecord` gives its partial report its own version when a document sets it, and yields both versions for the reader's check.
- **The last byte.** The JSON scenario keeps the lock of standard output, writes the line break with `writeln!` and flushes, and returns both errors through `?`. The writer's description says that it writes nothing after the closing.
- **Raw path data.** A reader takes the data of `unix_bytes` and `windows_wide` only as whole units of lowercase hexadecimal digits, two a byte or four a unit, and refuses anything else as malformed. A foreign path that passes is kept, and so is the data of an encoding of a newer version.
- **The catalog.** Only `NotFound` skips the earlier catalog; any other read error is returned.

**Found while revising.**

- `HeaderStatus::identification()` returned the stored enum `Identification` by value. It returns `Option<&Identification>` now.
- The stub's hexadecimal reading used `u16::from_str_radix`, which takes a leading `+` and uppercase digits, so `"+f"` rebuilt as the byte 0x0f. The check of whole units now comes first.
- The plan's test "the same file under a retained-data limit" left open whether the read limit still applied. It now names both limits, and a new test covers the established index.
- The stub's `Completeness` had no fixed-size record. It has one now, so a test can read `first_stopped()` from a report.
- `RawPath` moved to `source/raw.rs`, one concern per file, with its encoding and the check.

**Choices made without asking,** each open at the gate named:

- `DetectionStatus` has the two values `Complete` and `Incomplete` and its catch-all, like `DiscoveryStatus` (G2.2).
- The digits of raw path data must be lowercase, as the writer writes them, so that one path has one form (G2.3, task 5).
- Empty raw path data of a known encoding counts as whole units and is accepted (G2.3, task 5).
- `first_sample_description_index()` keeps the coded width, `u32`, and the ISO track details are reached by stream index, as Matroska's are (G2.2, Phase 6).
- The JSON scenario prints "catalog.json: none yet" to standard error when there is no catalog (G2.2).

**Plan, revision 38:**

- section 7.5: a report's record names detection too;
- section 8.3: the detection row of the parts, the validated-candidate row, the activity paragraph and `is_complete()`;
- section 8.8 and its tests: the partial report's version, and the check of raw path data;
- section 8.9 and its tests: an output failure at the last byte;
- Phase 2, task 13: raw path data and the partial report's version;
- Phase 3, tests: the report of two ambiguous candidates, one of them unchecked;
- Phase 6, task 7 and its tests: the established index of a lost description;
- the status line, the Phase 2 progress paragraph and Appendix E.

The field guide follows in section 3.3 (`raw_identifiers`) and section 4.2 (the first sample's description).

**Verification.**

- **Spike.**
  - In `spikes/api-scenarios/`, Clippy passes with warnings denied: with no feature, with each feature alone and with all features. `rustfmt` reports no difference.
  - 29 tests pass: the 24 earlier ones, two of them extended, and five new. The new tests cover the reference types of the statuses, the partial report inside an error record, raw path data, a report with a stopped check, and a lost description.
- **Programs.** All 13 programs end without a panic. With stand-in files, the directory scan exits normally. The JSON scenario exits normally without a catalog and with a valid one, and with an error for a catalog that is not UTF-8. Its document ends with a line break.
- **JSON.** `jq` reads the JSON scenario's standard output: the version `"1.0"`, plain-string inputs, the outcomes `error` with the kind `source_failed`, the nested version `"1.0"`, the I/O kind `not_found`, and no `null` member.
- **Structure.** The structure check of D23 reports 0 findings for the spike. The 13 programs in section 15 of `api.md` match the spike's examples; scenario 6 was synced from it.
- **Diagrams.** No diagram changed.
- **Checks.** `node scripts/check.ts` passes.

**Open:**

- the maintainer's review of the revision, for G2.2;
- G2.3, with task 4, the CLI contract, and task 5, the JSON contract, which take the last-byte failure, the partial report's version and the raw path data;
- the open points of `api.md` section 16, among them the item-list track and disc numbers.

## 2026-11-23 — G2.2 API approval [CONFIRM]

At the start of the session the maintainer sent the standing brief again, with no review of revision 38. The plan reviews each Phase 2 document before the next one starts, so the agent asked how task 4 should begin. It offered three options: start task 4 while the review of revision 38 continued (its recommendation), approve G2.2 first, or wait for the review. The maintainer approved G2.2.

| Topic | Decision | Alternatives offered | Reasoning |
|---|---|---|---|
| G2.2, API approval | Approved for revision 38 of the plan and `docs/design/api.md` at `198991f`, as section 16 of `api.md` lists. That covers the specification as a whole with D10, D11, D20 and D21, and the fifteen field-guide proposals as specified, with the amendment of plan section 8.2 that F3 implies. It also covers the public usage summary, the tags and chapters options, and Tokio's `probe_file` and `detect_file`. The usage scenarios, the revisions of the five reviews with the maintainer's choices at them, and the names added after G2.1 are approved as well. | Start task 4 while the review of revision 38 continued; wait for that review | Revision 38 fixed the findings that the fifth review held G2.2 back on. The maintainer approved it without a further review. |

**Plan, revision 39:**

- section 7.5: the usage summary is public;
- section 8: the names are approved;
- section 8.2: the media types of F3;
- section 8.7: the Tokio convenience is confirmed;
- the status line, the Phase 2 progress paragraph and Appendix E.

**Other documents:**

- `api.md` records the approval in its status, its introduction, sections 4.3 and 6.2, and section 16.
- The field guide records it in its status, section 3 and section 7, and became normative with the approval.
- The coverage table's row `rule.media-types` names `rfc2361: 3` and `webm-container-guidelines: Naming`. Both sections were confirmed in the stored documents on 2026-11-23. RFC 2361 section 3 writes `video/vnd.avi`, and the WebM guidelines' naming table gives `video/webm` and `audio/webm`.

## 2026-11-23 — Progress: Phase 2, task 4

The draft of [`docs/design/cli.md`](../design/cli.md) is ready for review, before task 5 starts. It fixes what plan section 8.9 leaves to it:

- the order in which the command line is read, and the messages of usage errors;
- the help and version texts;
- the entry points and options of each input;
- the JSON document as the tool writes it;
- the text layout of reports, detection results, errors and warnings;
- the escaping of text that reaches a terminal;
- output failure.

The disposable spike [`spikes/cli-arguments/`](../../spikes/cli-arguments/README.md) checks the command line against the approved `clap`.

**Checks and sources.**

- **`clap` 4.6.8** on docs.rs:
  - the feature flags: `color` styles the help and error texts, `suggestions` proposes options, and `wrap_help` follows the terminal's width;
  - `ArgAction::SetTrue` refuses a repeated flag unless `Command::args_override_self` is set;
  - `try_get_matches_from` neither prints nor exits.
- **A closed pipe.** The Rust runtime ignores the signal of a closed pipe before `main` runs on Unix. A write to a closed pipe then returns `BrokenPipe`, which `println!` turns into a panic (rust-lang/rust issue 46016 and the Rust internals forum). So the exit status 1 of plan section 8.9 needs no signal handling, and changing the signal would need `unsafe` code, which R15 forbids.
- **The GNU coding standards** for `--version`: the output on standard output, a constant program name, and the version after the last space.
- **The POSIX utility syntax guidelines** 9, 10, 11 and 13: options before operands, `--` as the end of the options, the order of options, and `-` as standard input.
- **Terminal escape injection.** Recent advisories and fixes escape or strip the C0 and C1 controls of untrusted text before it reaches a terminal. The Unicode property `Bidi_Control` lists U+061C, U+200E, U+200F, U+202A to U+202E and U+2066 to U+2069, the characters of the Trojan Source attack (CVE-2021-42574).

**Decisions of the maintainer.** Besides the order of the documents (the G2.2 entry above), the agent asked three questions about the text mode and, while writing, one about the API. The maintainer took the recommended option for three of them and the alternative for one.

| Topic | Decision | Alternatives offered | Reasoning |
|---|---|---|---|
| What text mode prints for a report | A summary: the file's values, one line for each stream, the numbers of chapters, tags and attachments, and the completeness with its reasons | Every normalized field with each chapter, tag and attachment; that and the typed details as well | It is small and has the same shape for every format. The JSON document carries the whole report. |
| Whether the text layout is a compatibility contract | No. It is for people, may change in any minor release, and each change is listed under "Changed output" | Stable within a major release, as the JSON document is; stable from 1.0 | Programs read the JSON document, whose contract is plan section 8.8. A text layout can then improve as formats are added. |
| An error that carries a partial report, in text mode | The error goes to standard error. The partial report goes to standard output in the normal layout, with a line naming the error. | The error on standard error only, with the partial report left to the JSON document (the agent's recommendation) | A person sees what was established before the failure, which R19 keeps. The line that names the error keeps the block from reading as a report that succeeded. |
| A partial report's parts that a source failure stopped | `ReasonKind::SourceFailed`, with `PartReason::source_identifier()` naming the source (`api.md` section 5.5) | Decide it in Phase 3; leave those parts without a reason | Every shortfall then names its reason (`api.md` section 5.3), and the CLI can print it. A new variant of an open set is compatible, so G2.2 stands. |

**What the spike found** (`cli.md` section 12). In `spikes/cli-arguments/`, with the approved `clap`:

- `-` is an input before and after `--`. Only the tool can count a second one.
- After `--`, `--json` and `--help` are inputs. Both plain scans stop at the first `--`.
- A long option is never abbreviated. An unknown option is `UnknownArgument`, whose context names the argument; for `-json` it names `-j`.
- `--json=yes` and `--json=` are `TooManyValues`, whose context names the option alone.
- A repeated flag needs `args_override_self`.
- An empty argument is an input. An input that is not valid Unicode is kept, and an unknown option that is not valid Unicode is named with U+FFFD.
- With `clap`'s own help and version flags off, `clap` writes nothing to either output, which a test of the spike's program shows.
- `std` and `error-context` are enough. The dossier also approves `help` and `usage`, which generate texts that the tool never writes. The spike passes with and without them.

**Found while writing.**

- No reason said that a source failed, so a partial report could not say why its remaining parts were incomplete. The maintainer decided it, as above.
- Plan section 8.9 said that the whole command line is validated before anything is probed. That left open whether `--help` beside an unknown option prints help, and whether `--version` needs an input. The contract answers both: a plain scan for help comes first, and the version comes after the usage errors and before the check for an input. Section 8.9 now says so.
- R21 says that `--help` always prints help. Without the plain scan, `clap` would report an unknown option that stands before `--help` and print help for one that stands after it.

**Choices made without asking,** each open at G2.3 (`cli.md` section 13):

- the reading order: help before every other step, and the version before the check for an input;
- no short options, so no `-h`; options accepted after the inputs, and a repeated option accepted;
- the messages, the three lines of a usage error, and the help and version texts;
- the JSON document pretty-printed with two spaces and followed by one line feed;
- the labels, value formats and order of the text layout, with the language as its BCP 47 tag, and the header status shown only through the completeness reasons;
- diagnostics of the severities `error` and `warning` on standard error in text mode, and none of the severity `information`;
- the escaping of control characters and bidirectional controls as `\u{…}`, with the backslash left as it is;
- on a closed pipe, exit status 1 with a message, as plan section 8.9 has it;
- a failure to write standard error ignored;
- whether the dossier's row for `clap` drops `help` and `usage`, which it keeps until G2.3.

**Plan, revision 39** (besides the G2.2 changes above):

- section 8.3: the reason "source failed";
- section 8.9: the text mode with the maintainer's three choices, the plain scan for help, the order of the version, the escaping, and the tests;
- section 8.10: the stability guide says that the text layout is no compatibility contract;
- Phase 2, task 5: the same for the stability guide;
- the Phase 2 progress paragraph and Appendix E.

`api.md` follows in its status, its introduction, sections 5.3 and 5.5, and section 16.

**Verification.**

- **Spike.**
  - In `spikes/cli-arguments/`, Clippy passes with warnings denied and the pedantic lints on, with and without the feature `generated-help`. `rustfmt` reports no difference.
  - The 8 tests pass both ways: 7 for the command lines and 1 that runs the program.
  - The structure check of D23 reports 0 findings.
  - The lock file resolves the four crates of the dossier at its versions: `clap` 4.6.8, `clap_builder` 4.6.7, `clap_lex` 1.1.2 and `anstyle` 1.0.14.
- **Diagrams.** The new diagram of `cli.md` renders with Mermaid 11.4.1 in headless Chromium, with the 16 diagrams of `api.md`, the plan and `CLAUDE.md`, and none has a syntax error.
- **Text.** The new text names no multimedia tool and has no British spelling.
- **Checks.** `node scripts/check.ts` passes.

**Open:**

- the maintainer's review of `cli.md`, before task 5 starts;
- G2.3, with task 5, the JSON contract and the stability guide;
- the features of `clap` in the dossier, at G2.3;
- the open points of `api.md` section 16, among them the item-list track and disc numbers.
