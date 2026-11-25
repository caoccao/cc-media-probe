# cc-media-probe

`cc-media-probe` is a pure Rust library that inspects multimedia files and reports what they contain: container format, streams, codec parameters, chapters, tags and attachments. It never decodes audio or video. One universal API serves every supported format, and the companion crate `cc-media-probe-cli` is its command-line tool.

**Status:** in development; nothing is published yet. The [development plan](https://github.com/caoccao/cc-media-probe/blob/main/docs/plans/plan-init-project.md) describes the scope, the design and every phase up to 1.0, and the [development log](https://github.com/caoccao/cc-media-probe/blob/main/docs/plans/plan-init-project-log.md) records the decisions and the progress.

## Stability

**Stability.** Before 1.0, a minor release (0.2.0, 0.3.0, …) may change the Rust API, and the changelog lists every such change; a patch release only fixes. From 1.0, both crates follow Semantic Versioning. The JSON output keeps schema version 1 through every 0.x release: a newer release only adds to it, and older readers keep reading it. What a probe reports can improve in a minor release and be fixed in any release, and the changelog lists every such change under "Changed output". The command-line tool's text output is for people; programs read `--json`.

**Minimum supported Rust version.** The stable Rust current on the day a minor line's release candidate is built, 1.100 today. Patch releases keep it. See the [stability guide](https://github.com/caoccao/cc-media-probe/blob/main/docs/guide/stability.md).

## Contributing

Bug reports and proposals are welcome as issues. [CONTRIBUTING.md](https://github.com/caoccao/cc-media-probe/blob/main/CONTRIBUTING.md) says how to write a useful one, how to report a vulnerability privately, and which rules every change follows.

## License

Apache-2.0; see [LICENSE](LICENSE).
