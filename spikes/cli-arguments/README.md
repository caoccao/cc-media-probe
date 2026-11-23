# Spike: the command line of the CLI contract

This disposable spike checks how the approved argument parser reads the command line that the [CLI contract](../../docs/design/cli.md) specifies (Phase 2, task 4). It uses `clap` 4.6.8 with the builder API, as the [dependency dossier](../../docs/design/dependency-dossier.md) approves it. The spike is a workspace of its own, excluded from the root workspace, with its own lock file (plan section 7.1). Nothing in the root workspace depends on it, and it is deleted once the CLI skeleton of Phase 3, task 21 exists.

## What it holds

- **The reading order** of `cli.md` section 3, in `src/`, one concern per file:
  - `scan.rs` holds the two plain scans, for `--json` and `--help` before any `--`;
  - `command.rs` describes the command line to `clap`, with `clap`'s own help and version flags turned off;
  - `usage.rs` turns `clap`'s errors into the contract's messages;
  - `invocation.rs` decides between help, the version, a run and a usage error;
  - `escape.rs` escapes a message where it reaches standard error, by the rule of `cli.md` section 8.1.

  Nothing is probed.
- **A program** in `src/main.rs` and `src/run.rs`, which writes one line for the invocation and one for each input on standard output, and a usage error on standard error with exit status 2.
- **Two test files** in `tests/`:
  - `arguments.rs` checks what each command line of `cli.md` sections 2 and 3 gives: standard input before and after `--`, the exact argument of JSON mode, unknown and abbreviated options, an unknown option with a value, named as written, a value given to a flag, the first unknown option from the left, help before everything, the version without an input, and arguments that are not valid Unicode.
  - `process.rs` runs the program and checks that each output holds only what the program writes, so `clap` writes nothing itself, that an output whose read end is closed before the program starts gives the exit status 1, and that standard error escapes a line feed, an escape and a bidirectional control in a usage error's message, while `arguments.rs` checks that the message itself holds them.

The dossier approves `clap` with the features `std`, `help`, `usage` and `error-context`. The spike builds with `std` and `error-context` only. Its feature `generated-help` adds the other two, and every check passes both ways.

## Running it

In `spikes/cli-arguments/`:

- `cargo clippy --offline --all-targets -- -D warnings -W clippy::pedantic`, once without features and once with `--features generated-help`;
- `cargo test --offline`, the same two ways.

From the repository root, `cargo run -q --locked -p cc-media-probe-checks -- structure --root spikes/cli-arguments` applies the structure rules of R30 to R33 and the header.

The results of 2026-11-23 on macOS: both Clippy runs pass, the 8 tests pass both ways, and the structure check reports 0 findings. What the tests found is in `cli.md` section 12.

After the eighth review, on the same day: both Clippy runs pass, the 10 tests pass both ways, and the structure check reports 0 findings. A test written before the fix failed as the review said: `--bogus=value` was named `--bogus`, since `clap`'s context names only the option, and `--=x` would have been named `--`. The message now takes the long option as written from the command line. `parse` takes the arguments as a slice, which the message reads too.

After the ninth review, on the same day: both Clippy runs pass, the 12 tests pass both ways, and the structure check reports 0 findings. A test written before the fix failed as the review said: the program wrote a line feed and an escape of an unknown option to standard error as they were. The message is still built from the decoded argument, as a JSON document holds it, and the program now escapes it only where it writes standard error.
