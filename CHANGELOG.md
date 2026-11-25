# Changelog

All notable changes to the published crates, `cc-media-probe` and `cc-media-probe-cli`, are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html). Besides the six kinds of change of that format, a release can list **Changed output**: every change in what a probe reports for the same input, such as a field that is now filled or a value that is now read differently. Each entry is added in the commit that makes the change. An entry for a breaking change, which [the stability guide](docs/guide/stability.md) allows only in a minor release before 1.0 and in a major release from 1.0, begins with **Breaking**, under its kind of change.

## [Unreleased]

Nothing is published yet. The first release will be 0.1.0, once Matroska and WebM, MP4, QuickTime and AVI are complete.

### Added

- The data model of the public API (development plan, Phase 2, task 11): the report, its streams with their codec, video, audio and subtitle information, chapters, tags, attachments, programs and external references, the detection, completeness with its fixed-size record, diagnostics, the source table with portable paths, the typed details with their catch-all, and the error with its serializable record. Probing, the options, the serialized form and the async surface follow.

[Unreleased]: https://github.com/caoccao/cc-media-probe/commits/main
