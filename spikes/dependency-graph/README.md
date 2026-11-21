# Spike: the dependency graph for G1.3

This disposable spike holds the evidence for gate G1.3 of the [development plan](../../docs/plans/plan-init-project.md): the graph of the two published crates, with exactly the dependencies and features of the [dependency dossier](../../docs/design/dependency-dossier.md), which G1.3 approved on 2026-11-21. The pure-Rust verification of D4 runs against it with the repository's own checks. It is a workspace of its own, excluded from the root workspace, with its own lock file (plan section 7.1). Nothing in the root workspace depends on it.

## Layout

- `crates/cc-media-probe/`: the library's proposed features and dependencies, with the development dependencies. Its `lib.rs` has no code of its own and only re-exports each dependency behind its feature.
- `crates/cc-media-probe-cli/`: the command-line tool's dependencies. Its binary uses each normal dependency once, so that the fourth layer inspects a binary that links them all.
- `crates/fuzzing-dependencies/`: the fuzz workspace's crates. They are resolved in the lock file and never built.
- `crates/cc-media-probe-checks/data/`: the lists that the checks read under `--root`. These are the reviewed build scripts, and a copy of the allowed operating-system libraries. On approval, the list of build scripts was copied to the repository.

## Running the layers

From the repository root, with `CC`, `CXX` and `AR` naming a file that does not exist (here `$MISSING`):

| Layer | Command |
|---|---|
| 1 | `cargo run -q --locked -p cc-media-probe-checks -- inventory --root spikes/dependency-graph` |
| 3, build | In `spikes/dependency-graph/`: `cargo build --locked -p cc-media-probe -p cc-media-probe-cli --target-dir target/pure-rust`, once with `--no-default-features`, once with no feature option, and once with `--all-features`. For another target, use `cargo check --target <triple>` with its own target directory. |
| 3, inspection | `cargo run -q --locked -p cc-media-probe-checks -- build-output spikes/dependency-graph/target/pure-rust --root spikes/dependency-graph` |
| 4 | `otool -L` (or `readelf --dynamic`, or `objdump -p`) on the binary that Cargo reports, then `cargo run -q --locked -p cc-media-probe-checks -- dynamic-libraries <platform> <libraries>... --root spikes/dependency-graph` |

The results of 2026-11-21 are in section 3 of the dossier. Every layer reported 0 findings: macOS for all four layers, and Linux and Windows on x86_64 for the third.
