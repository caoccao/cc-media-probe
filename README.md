# cc-media-probe

`cc-media-probe` is a pure Rust library that inspects multimedia files and reports what they contain: container format, streams, codec parameters, chapters, tags and attachments. It never decodes audio or video. One universal API serves every supported format, and the companion crate `cc-media-probe-cli` is its command-line tool.

**Status:** in development; nothing is published yet. The [development plan](https://github.com/caoccao/cc-media-probe/blob/main/docs/plans/plan-init-project.md) describes the scope, the design and every phase up to 1.0, and the [development log](https://github.com/caoccao/cc-media-probe/blob/main/docs/plans/plan-init-project-log.md) records the decisions and the progress.

## Contributing

Bug reports and proposals are welcome as issues. [CONTRIBUTING.md](https://github.com/caoccao/cc-media-probe/blob/main/CONTRIBUTING.md) says how to write a useful one, how to report a vulnerability privately, and which rules every change follows.

## License

Apache-2.0; see [LICENSE](LICENSE).
