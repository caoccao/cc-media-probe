# Spike: the usage scenarios of the API specification

This disposable spike type-checks the usage scenarios of Phase 2, task 3 against the surface that the [API specification](../../docs/design/api.md) defines. Section 15 of the specification shows the scenarios; the code there is the code of `examples/` here, without the file headers. The spike is a workspace of its own, excluded from the root workspace, with its own lock file (plan section 7.1). Nothing in the root workspace depends on it, and it is deleted once the skeleton of task 11 exists.

## What it holds

- **A stub of the library** in `src/`, named `cc_media_probe` so that the scenarios read exactly as a caller writes them. Its types, accessors and entry points have the signatures of `api.md` sections 3 to 13, as far as the scenarios use them. Enums carry a subset of their variants. Nothing is parsed: every probe opens or touches its input and then returns an error.
- **The scenarios** in `examples/`, one program each. Three need a feature, as their shipped versions will: `export_json` needs `serde`, `probe_with_tokio` needs `tokio` and `probe_with_futures` needs `futures-io`.
- **Six test files** in `tests/`:
  - `archive.rs`, with the `serde` feature, holds the archive cases of the sixth review. A known shortfall rules out `ok` beside a status of a newer version, and a report never allows `error`. A path's display string, flag and raw data describe one path. Chapters and tags are flat arrays with parent positions, and a chain of 200,000 tags reads back with serde_json's default recursion limit. A tag value is a map of its kind and the member named after it, and a kind of a newer version keeps its name. A bare report and a bare error record check their versions. A rational reads back only in its reduced form, and a twin only in its own range.
  - `bounds.rs` checks at compile time that the model, the error and the prober are `Send`, `Sync` and `'static`, and that the probe futures are `Send` for Tokio and for a boxed `Send` source. It also checks that a pattern matches an enum that an accessor returns by reference, and that every status getter of the completeness record returns a reference.
  - `async_source.rs` checks the repeated-request contract of section 9.3 against the stub. A source that answers `Pending` is asked again with the same position and buffer length. A probe dropped while its request is pending never calls again, and a source that keeps the range of its request answers the next request, for another range, with that range's bytes.
  - `document.rs`, with the `serde` feature, checks that a document has the wire form of `api.md` section 12 for every member the stub writes. It reads a document back and checks each entry through its getters. The stub builds no report, so its entries are error records, and a detection entry is written by hand. It also reads a usage-error document back. Reading refuses a document whose usage error stands beside results, an entry of a known outcome that lacks its record or holds one that does not warrant the outcome, a nested record of another version in any record of an entry or in the partial report of an error record, and a document of another major version. It also refuses raw path data of a known encoding that is not whole units of lowercase hexadecimal digits, and keeps a foreign path and the data of a newer encoding. An outcome from a newer version, and a detection outcome from a newer version, read back as written. The entries of plan section 8.9 read back as written there, a lossy input keeps its flag and raw form, a 32-bit identifier in an error's observations is a string, a document rebuilt from entries of version 1.1 gives them its version down to their partial reports, and an entry whose outcome this version cannot derive is refused and takes the outcome given.
  - `report.rs`, with the `serde` feature, reads three cases of a report from JSON. In the first, a limit stopped one candidate's check while another validated, so detection is incomplete, the record names detection, and the entry is `partial`. In the second, a first sample's description is known but was not kept: the codec is absent, there is no candidate, one is missing, and the ISO details keep the index. In the third, a full budget left `reasons()` empty, and the record still keeps the first reason of the part and counts the reasons not kept.
  - `writer.rs`, with the `serde` feature, checks the document written as it goes: it writes what `Document::from_results` writes, for no entries and for mixed ones, down to the version of a partial report, takes an entry only after the previous one was written, and takes none after an output error.

The dependencies are the approved ones of the dependency dossier: Serde, Tokio with `fs`, futures-io, and as development dependencies `futures-lite`, `serde_json` and Tokio's runtime.

## Running it

In `spikes/api-scenarios/`:

- `cargo clippy --offline --all-targets -- -D warnings`, once with no feature, once with each feature alone, and once with `--all-features`;
- `cargo test --offline --all-features`;
- `cargo run --offline --all-features --example <name>` for any scenario. With no media file and no parser, each one ends with an ordinary error, never a panic.

From the repository root, `cargo run -q --locked -p cc-media-probe-checks -- structure --root spikes/api-scenarios` applies the structure rules of R30 to R33 and the header.

The results of 2026-11-22 on macOS: every Clippy run passes, the five tests pass, all 13 programs end with an error and none panics, and the structure check reports 0 findings. The first run of the structure check found the abbreviations `Io`, `io` and `Id` in names that `api.md` had specified; section 15.12 of the specification lists the names that replace them.

After the first review of the specification, on the same day: every Clippy run passes, the seven tests pass, and the structure check reports 0 findings. All 13 programs end without a panic. The JSON scenario now reads its document back and exits normally, and the others end with an error. With stand-in files, the directory scan probes twelve files on a fixed pool of workers, and the custom async source reaches the stub's ordinary error. Writing the document's read side found that the entry constructors would clash with the getters; section 16 of the specification lists the new names.

After the second review, on the same day: every Clippy run passes, the ten tests pass, and the structure check reports 0 findings. All 13 programs end without a panic. The JSON scenario writes only its document to standard output, which `jq empty` accepts, and its summaries to standard error. The stub gained `Detection` for detection entries, an optional codec with its candidates, the kind of an entity location and the owner of a conflict. A document is now built from its results or from a usage error, and its two invariants are checked when it is read.

After the third review, on the same day: every Clippy run passes, the 16 tests pass, and the structure check reports 0 findings. All 13 programs end without a panic, and the JSON scenario and the directory scan exit normally. The document module is a directory of four files now, one concern each. The document that the JSON scenario writes has the form of section 12 for every member it carries, and `jq` confirms it: the version `"1.0"`, `snake_case` outcomes and error kinds, a version on each nested record, the I/O error kind `not_found`, and no `null` member. The rest of the model keeps the default form of Serde's derive, since the stub builds no report. The stub also gained:

- `CodecCandidate`, whose own identifiers fix the display of an unresolved codec, which had paired identifiers with candidates by position;
- `TargetScope`, which the whole-file lookups now use;
- an optional `FlagDefault`;
- the reason of a source entry, and the source of a completeness reason;
- the check that an entry's record warrants its outcome.

After the fourth review, on the same day: every Clippy run passes, the 24 tests pass, and the structure check reports 0 findings. All 13 programs end without a panic, and the JSON scenario and the directory scan exit normally. The JSON scenario writes its document with the writer, and `jq` reads the input as a plain string. The review's four problems of the document reproduced first, in a scratch program against the stub: a rebuilt document refused by its own reader, a record of version 2.0 accepted beside a detection, the plan's `"input"` string refused, and a `partial` entry rebuilt as `error`. The stub also gained:

- `Document::serialize_results` and `UnderivableOutcome`, in a document module of eight files;
- a `source/` directory, with `RawPath` and the path's members in JSON, and `SourceIdentifier` as a string;
- the ISO family's details with its 32-bit track identifiers, and the observations of an error record;
- `Part::Detection` with the fixed-size record, the error's reason, `ReasonKind::RangeUnavailable` and `Limit::as_str()`;
- `CodecInformation::has_every_candidate()`, and the attachments that the tag scenario names.

After the fifth review, on the same day: every Clippy run passes, the 29 tests pass, and the structure check reports 0 findings. All 13 programs end without a panic. With stand-in files, the directory scan exits normally. The JSON scenario exits normally without a catalog or with a valid one, and with an error for a catalog that is not UTF-8. Tests written before the fixes reproduced the review's problems: a partial report of another version, read and rebuilt by either writer, and raw data `"gg"`. The scenario had exited 0 with a catalog that is not UTF-8. The JSON scenario now writes its line break and flushes through the locked output. The stub also gained:

- `Completeness::detection()` with `DetectionStatus`, and the completeness record on the report;
- the partial report in the versions of an error record;
- `source/raw.rs`, where reading checks the data of a raw path, which the hexadecimal reading had taken with a leading `+` or uppercase digits;
- the ISO family's track details with `first_sample_description_index()`, and the display of a codec with no candidate kept.

After the sixth review, on 2026-11-23: every Clippy run passes, the 41 tests pass, and the structure check reports 0 findings. All 13 programs end without a panic, and the JSON scenario exits normally. Six tests of a new `tests/archive.rs`, written before the fixes, failed as the review said: the labels `ok` and `error` beside a known shortfall and a status of a newer version, the path `other`, a tree of 64 tags in a document, a bare report of version 2.0 with a bare error record whose partial report is 2.0, and a zero denominator with an unread twin. The stub also gained:

- `model/tree.rs`, the flat form of chapters and tags, and their `Drop`, `Clone`, `PartialEq`, `Hash` and `Debug`, which walk a tree with an explicit stack;
- `model/tag_value.rs`, with the map form of `TagValue`, and `twin.rs`, which reads a quantity from its number or its `_exact` twin;
- entries without an index of their own, which the document writes from their positions, and `with_outcome()`, which refuses `error` for a report;
- the check that a path's three members describe one path, a `Rational` that reads back only in its reduced form, and the version check of a bare report and of a bare error record's partial report;
- the record's first reason of each part, the optional track number and codec identifier of a Matroska track, and the async source implementations for `Box<Source>` and `Pin<Pointer>`.

The stub decodes a record's members in the order written, so its version error comes first only where the version does. `serde_json::Value` orders a map's members by name, which puts `generator` before `schema_version`; the neutral form of plan section 8.8 that reads the version first wherever it stands is task 5's to show.
