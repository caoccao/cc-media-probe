# Spike: Tokio's `fs` feature for a path-based async convenience

This disposable spike holds the evidence for the GS.1 request that the [API specification](../../docs/design/api.md) makes for `asynchronous::tokio::probe_file` and `asynchronous::tokio::detect_file`. On 2026-11-22 the maintainer chose to add them for Tokio. They need the runtime's file API, and the [dependency dossier](../../docs/design/dependency-dossier.md) approved Tokio with no features at all. The spike is a workspace of its own, excluded from the root workspace, with its own lock file (plan section 7.1). Nothing in the root workspace depends on it.

## What it shows

Tokio 1.54.1, the approved version, built with `default-features = false` and `features = ["fs"]`:

- **No new crate.** `cargo tree -e normal` lists `tokio` and `pin-project-lite`, the two crates that the dossier already approved for the `tokio` feature. The `fs` feature enables no other feature and no optional dependency (the feature list of Tokio 1.54.1 on docs.rs).
- **No runtime is pulled in.** The library code builds with `fs` alone. `tokio::fs::File` implements `AsyncRead` and `AsyncSeek`, so it satisfies the bounds of `asynchronous::tokio::probe_reader`, and the future of the opening function is `Send`.
- **The caller's runtime does the work.** Tokio's file operations run on the blocking pool of the caller's runtime. The test with a current-thread runtime opens, seeks and reads a file. Outside a runtime, the first poll panics: this is Tokio's own contract for `tokio::fs`, so the convenience documents that it must be called from within a Tokio runtime.

## Running it

In `spikes/tokio-file-access/`: `cargo test --offline` and `cargo tree --offline -e normal -f "{p} {f}"`.

The results of 2026-11-22: both tests pass on macOS, and the tree shows `tokio v1.54.1 fs` with `pin-project-lite v0.2.17` only.
