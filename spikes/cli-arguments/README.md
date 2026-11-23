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
- **A program** in `src/main.rs` and `src/run.rs`, which writes one line for the invocation and one for each input on standard output, and a usage error on standard error with exit status 2. In JSON mode a usage error also writes one line on standard output, which stands for its document. A failure to write standard error is ignored, as `cli.md` section 9 says.
- **Three test files** in `tests/`:
  - `arguments.rs` checks what each command line of `cli.md` sections 2 and 3 gives: standard input before and after `--`, the exact argument of JSON mode, unknown and abbreviated options, an unknown option with a value, named as written, a value given to a flag, the first unknown option from the left, help before everything, the version without an input, and arguments that are not valid Unicode.
  - `output.rs` drives `write_inputs` with a writer that fails at a chosen byte and inputs that count what was taken, so a failure inside an entry and one at the final line feed are tested where a closed pipe cannot place them (twelfth review).
  - `process.rs` runs the program and checks that each output holds only what the program writes, so `clap` writes nothing itself, that an output whose read end is closed before the program starts gives the exit status 1, that a usage error keeps the status 2 when standard error or, in JSON mode, standard output is closed, and that standard error escapes a line feed, an escape and a bidirectional control in a usage error's message, while `arguments.rs` checks that the message itself holds them.

The dossier approves `clap` with the features `std`, `help`, `usage` and `error-context`. The spike builds with `std` and `error-context` only. Its feature `generated-help` adds the other two, and every check passes both ways.

## Running it

In `spikes/cli-arguments/`:

- `cargo clippy --offline --all-targets -- -D warnings -W clippy::pedantic`, once without features and once with `--features generated-help`;
- `cargo test --offline`, the same two ways.

From the repository root, `cargo run -q --locked -p cc-media-probe-checks -- structure --root spikes/cli-arguments` applies the structure rules of R30 to R33 and the header.

The results of 2026-11-23 on macOS: both Clippy runs pass, the 8 tests pass both ways, and the structure check reports 0 findings. What the tests found is in `cli.md` section 12.

After the eighth review, on the same day: both Clippy runs pass, the 10 tests pass both ways, and the structure check reports 0 findings. A test written before the fix failed as the review said: `--bogus=value` was named `--bogus`, since `clap`'s context names only the option, and `--=x` would have been named `--`. The message now takes the long option as written from the command line. `parse` takes the arguments as a slice, which the message reads too.

After the ninth review, on the same day: both Clippy runs pass, the 12 tests pass both ways, and the structure check reports 0 findings. A test written before the fix failed as the review said: the program wrote a line feed and an escape of an unknown option to standard error as they were. The message is still built from the decoded argument, as a JSON document holds it, and the program now escapes it only where it writes standard error.

After the tenth review, on the same day: both Clippy runs pass, the 14 tests pass both ways, and the structure check reports 0 findings. A test written before the fix failed as the review said: with standard error closed, `--jsno` exited with 1 instead of 2, because the program passed on the error of its write to standard error. The program now ignores that result, and two new tests check a usage error with a closed standard error, in both modes, and a JSON usage error whose document cannot be written.

After the eleventh review, on the same day: both Clippy runs pass, the 14 tests pass both ways, and the structure check reports 0 findings. The program did not change. Its test of a JSON usage error whose document cannot be written already expected the output failure's line before the usage error's, the order that `cli.md` section 4 now states. Run into a pipe whose reader has gone, it writes `standard output failed: Broken pipe (os error 32)` first and exits with 2.

After the twelfth review, on the same day: both Clippy runs pass, the 17 tests pass both ways, and the structure check reports 0 findings. A pipe closed before the program starts fails its first write, so it cannot place a failure inside an entry or at the final line feed. The program now writes its inputs through `write_inputs`, which takes any writer and any sequence of inputs, and the three tests of `tests/output.rs` drive it with a writer that fails at a chosen byte and inputs that count what was taken: a failure inside the second line leaves the third input untaken, a failure at the final line feed comes after all three, and a flush that fails fails as a write does. Writing them found that a run whose standard output failed exited with 1 and wrote nothing to standard error, while `cli.md` section 9 has the failure's line there. The run now writes it through `report_output_failure`, which the usage error uses too, and the binary test with a closed pipe checks the line.
