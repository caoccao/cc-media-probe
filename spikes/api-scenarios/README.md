# Spike: the usage scenarios of the API specification

This disposable spike type-checks the usage scenarios of Phase 2, task 3 against the surface that the [API specification](../../docs/design/api.md) defines. Section 15 of the specification shows the scenarios; the code there is the code of `examples/` here, without the file headers. The spike is a workspace of its own, excluded from the root workspace, with its own lock file (plan section 7.1). Nothing in the root workspace depends on it, and it is deleted once the skeleton of task 11 exists.

## What it holds

- **A stub of the library** in `src/`, named `cc_media_probe` so that the scenarios read exactly as a caller writes them. Its types, accessors and entry points have the signatures of `api.md` sections 3 to 13, as far as the scenarios use them. Enums carry a subset of their variants. Nothing is parsed: every probe opens or touches its input and then returns an error.
- **The scenarios** in `examples/`, one program each. Three need a feature, as their shipped versions will: `export_json` needs `serde`, `probe_with_tokio` needs `tokio` and `probe_with_futures` needs `futures-io`.
- **Three test files** in `tests/`:
  - `bounds.rs` checks at compile time that the model, the error and the prober are `Send`, `Sync` and `'static`, and that the probe futures are `Send` for Tokio and for a boxed `Send` source. It also checks that a pattern matches an enum that an accessor returns by reference.
  - `async_source.rs` checks the repeated-request contract of section 9.3 against the stub. A source that answers `Pending` is asked again with the same position and buffer length. A probe dropped while its request is pending never calls again, and a source that keeps the range of its request answers the next request, for another range, with that range's bytes.
  - `document.rs`, with the `serde` feature, reads a document back and checks each entry through its getters. The stub builds no report, so its entries are error records.

The dependencies are the approved ones of the dependency dossier: Serde, Tokio with `fs`, futures-io, and as development dependencies `futures-lite`, `serde_json` and Tokio's runtime.

## Running it

In `spikes/api-scenarios/`:

- `cargo clippy --offline --all-targets -- -D warnings`, once with no feature, once with each feature alone, and once with `--all-features`;
- `cargo test --offline --all-features`;
- `cargo run --offline --all-features --example <name>` for any scenario. With no media file and no parser, each one ends with an ordinary error, never a panic.

From the repository root, `cargo run -q --locked -p cc-media-probe-checks -- structure --root spikes/api-scenarios` applies the structure rules of R30 to R33 and the header.

The results of 2026-11-22 on macOS: every Clippy run passes, the five tests pass, all 13 programs end with an error and none panics, and the structure check reports 0 findings. The first run of the structure check found the abbreviations `Io`, `io` and `Id` in names that `api.md` had specified; section 15.12 of the specification lists the names that replace them.

After the first review of the specification, on the same day: every Clippy run passes, the seven tests pass, and the structure check reports 0 findings. All 13 programs end without a panic. The JSON scenario now reads its document back and exits normally, and the others end with an error. With stand-in files, the directory scan probes twelve files on a fixed pool of workers, and the custom async source reaches the stub's ordinary error. Writing the document's read side found that the entry constructors would clash with the getters; section 16 of the specification lists the new names.
