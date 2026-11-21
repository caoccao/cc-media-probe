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
