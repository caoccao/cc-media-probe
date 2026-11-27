# Examples

| | |
|---|---|
| **Status** | Written on 2026-11-27 for plan Phase 2, task 12, from the usage scenarios of [`api.md`](api.md) section 15, which the maintainer approved at G2.2. Each later phase that adds a format or an entry point extends it (plan section 10). |
| **Where** | [`crates/cc-media-probe/examples/`](../../crates/cc-media-probe/examples/), one program per common task. The examples ship in the library's package. |
| **Running** | Compile-only in Phase 2. Until the detection inventory of Phase 3, the bytes of every input match nothing, so every probe ends with an error, `UnknownFormat` for an input that exists. What a program then does is its own: eleven of them return the error from `main`, which exits unsuccessfully, while `export_json` writes each error as an entry of its document and `scan_directory` reports each file's error and goes on, and both of those exit successfully. From Phase 5 they run against the tiny synthetic sample that ships in the package (plan section 9.7). |

Cargo skips an example whose required features are off, and says nothing (plan section 9.7). So this list names, for each example, the features it needs and the development dependencies it uses. Every development dependency is a registry crate, so it stays in the published manifest and an example of the unpacked package still builds.

## Contents

1. [The examples](#1-the-examples)
2. [Building and running them](#2-building-and-running-them)
3. [The quick start](#3-the-quick-start)
4. [The tags and their targets](#4-the-tags-and-their-targets)
5. [What later phases add](#5-what-later-phases-add)

## 1. The examples

| Example | Scenario (`api.md`) | Required features | Development dependencies |
|---|---|---|---|
| `list_streams` | 15.1, the quick start: list the streams | None; its tests need `serde` | `serde_json`, for its tests |
| `video_summary` | 15.2, video size and frame rate, with absent values | None | None |
| `default_audio_language` | 15.3, the default audio language | None | None |
| `dump_tags` | 15.4, tags with their targets | None; its tests need `serde` | `serde_json`, for its tests |
| `list_chapters` | 15.5, chapters and their two timelines | None | None |
| `export_json` | 15.6, JSON on its own and in the document form | `serde` | `serde_json` |
| `probe_with_tokio` | 15.7, async probing with Tokio | `tokio` | `tokio`, with its runtime and `macros` |
| `probe_with_futures` | 15.7, async probing with futures-io | `futures-io` | `futures-lite` |
| `remote_source_async` | 15.7, a custom async source | None | `futures-lite` |
| `other_inputs` | 15.8, bytes, standard input and a source that serves ranges | None | None |
| `scan_directory` | 15.9, a directory scan with per-file errors | None | None |
| `format_details` | 15.10, format-specific and codec-specific details | None | None |
| `estimated_duration` | 15.11, an estimated duration | None | None |

The library's manifest declares the three required features with each example's `required-features`, and `list_streams` and `dump_tags` with `test = true`, so `cargo test` runs their tests. The development dependencies are those of the dependency dossier, section 1.3.

## 2. Building and running them

- **Every example, with every feature:** `cargo build --examples --all-features -p cc-media-probe`. `node scripts/check.ts` lints them that way and runs the tests of the quick start and of `dump_tags`.
- **Every combination of features:** `cargo hack build --all-targets --feature-powerset -p cc-media-probe` builds each example wherever its required features are on, so an example that uses a feature it does not declare fails in the combination that lacks it. CI's feature-combination job runs that build for every crate, the tests included, and documents every combination (plan section 9.8); the tarball job of plan section 9.7 runs each example with exactly its required features.
- **One example:** `cargo run -p cc-media-probe --example list_streams`, adding `--features serde`, `--features tokio` or `--features futures-io` for the three that need one.
- **The tests of the quick start and of `dump_tags`:** `cargo test -p cc-media-probe --features serde --example list_streams --example dump_tags`.

An example reads the file that its scenario names, such as `movie.mkv`, from the working directory.

## 3. The quick start

The quick start has two parts (plan section 9.7). Its probing call is `main` of `list_streams`, compiled in Phase 2 and run from Phase 5. Its rendering half is `print_report`, which writes to any `io::Write`. The example's tests read two reports from JSON, a complete one and one in which the media type, the duration, the display size, the sample rate and the channel count are all absent, and compare the text that `print_report` writes for each, byte for byte, as the maintainer chose on 2026-11-27. A third test gives a stream a title with quotes, a line break and a tab, which `print_report` writes as a quoted string with each of them escaped, so the title stays on its line. The fixtures hold every language and title in the lists from which the primary language and the convenience title come (field guide section 3.2). The crate's front page shows the same program.

## 4. The tags and their targets

`dump_tags` is tested the same way, as the maintainer chose at the second review of tasks 12 and 13, since its targets were misreported a second time (plan section 9.7). Its rendering half is `print_tag_sets`, which writes to any `io::Write`. The example's four tests read a report of one stream, an edition of three chapters, one of them a child, and an attachment from JSON, each with a tag set of its own, and compare the text written byte for byte:

- a level that has a number and no name, as a Matroska level can be, beside a tree of tags whose value has quotes and a line break and a tag that only groups its children;
- every kind of target: a resolved stream, chapter and attachment with their names, a wildcard edition, an unmatched attachment, and two chapters told apart by their paths, one without a title;
- an incomplete scope with the target that was kept, and a level that has a name and no number;
- a target scope and a reference kind of a newer version, each named as written.

## 5. What later phases add

- **The typed details** (plan Phases 4 and 5). In Phase 2, `FormatDetails` and `CodecDetails` have only their catch-all, so `format_details` shows the stable name and the three lists that every kind shares. The H.264 part of the approved sketch in `api.md` section 15.10 joins it with Phase 4, and the Matroska part with Phase 5, as the maintainer chose at the approval review of 2026-11-27.
- **Running.** From Phase 5, each example runs against the shipped sample, and the tarball job of plan section 9.7 runs every example with exactly its required features and fails on one that is skipped.
- **New examples.** An entry point or a feature that a later phase adds gets its example, and a row here, in the same change.
