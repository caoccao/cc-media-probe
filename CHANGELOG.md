# Changelog

All notable changes to the published crates, `cc-media-probe` and `cc-media-probe-cli`, are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). Besides the six kinds of change of that format, a release can list **Changed output**: every change in what a probe reports for the same input, such as a field that is now filled or a value that is now read differently. Each entry is added in the commit that makes the change. An entry for a breaking change, which [the stability guide](docs/guide/stability.md) allows only in a minor release before 1.0 and in a major release from 1.0, begins with **Breaking**, under its kind of change.

## [Unreleased]

Nothing is published yet. The first release will be 0.1.0, once Matroska and WebM, MP4, QuickTime and AVI are complete.

### Added

- The data model of the public API (development plan, Phase 2, task 11): the report, its streams with their codec, video, audio and subtitle information, chapters, tags, attachments, programs and external references, the detection, completeness with its fixed-size record, diagnostics, the source table with portable paths, the typed details with their catch-all, and the error with its serializable record. The serialized form follows.
- The entry points of the public API (development plan, Phase 2, task 11): the ten blocking functions that probe or detect a file, a seekable reader, a forward-only reader, bytes and the caller's own positional source; `Prober` with its options and `Limits` with the proposed defaults; the async source trait with its two functions; and the features `tokio` and `futures-io`, each with a module for the reader traits of its runtime. No format is detected yet, so detection gives the outcome "unknown", or "hint only" for a format hint, and a probe the error "unknown format".

[Unreleased]: https://github.com/caoccao/cc-media-probe/commits/main
