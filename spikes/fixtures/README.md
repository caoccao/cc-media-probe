# Spike: fixtures, expectations, seeds and hidden entry points

This disposable spike holds the evidence for gate G3.2 of the [development plan](../../docs/plans/plan-init-project.md), Phase 3, task 3: D14, the fixture format, and D19, access to internals for fuzzing. The proposal itself is [`docs/design/test-kit.md`](../../docs/design/test-kit.md). The spike runs that proposal end to end on two sample fixtures:
- a hex assembler reads them, with their cases and regions;
- the real library probes them in each case through each blocking adapter, and the expectations are split into their parts;
- the seeds are routed and checked, and the check writes them into a corpus;
- cargo-fuzz builds and runs a target that reaches a private parser through a hidden module, one that runs the real library's full public probe, and one that walks a whole toy file.

The maintainer chose its shape on 2026-11-27:
- the document plus this spike, with a real cargo-fuzz build;
- hex bytes, quoted text and repeats in a fixture's lines;
- expectations in the library's JSON form, split into semantic and adapter parts;
- the hidden fuzz module compiled in repository builds too, so that the seed check is an ordinary test.

The maintainer's review of the first spike, the same day, kept G3.2 open with 25 findings, which `test-kit.md` section 10 lists one by one. The maintainer chose four more things then:
- a fixture's cases on `case:` lines;
- a repository-only counter of the file adapter's reads;
- retained data net of the test's paths;
- the seed check as the corpus tool.

The spike was revised to match.

The maintainer's second review, of revision 75, approved the section named in a heading, the hidden `Format::Toy` and the choices of `test-kit.md` section 8, and held the proposal on four contract issues, which section 10.2 lists. The maintainer chose three more things then:
- a file probe's retained-data limit raised by its path's charge, in folders of a fixed length;
- no whole fixture routed to `probe`, while a region may be;
- `hint=` cases for the names that no file can have, and the fuzz hint as native bytes on Unix.

The spike was revised again to match.

The maintainer's third review, of revision 76, held the proposal on 19 findings, which section 10.3 lists. The maintainer chose two more things then:
- a path that the retained-data limit cannot hold left out of its entry, with the limit exhausted and no part stopped;
- each case's folder of 512 bytes, from components of a fixed width for the run and the case.

The spike was revised a third time to match.

The spike is a workspace of its own, excluded from the root workspace, with its own lock file (plan section 1.2), and `fuzz/` is a cargo-fuzz workspace inside it. The stand-in library depends on the real library by path, for its public probe and detection, and its tests use the real library's JSON form. Nothing depends on the spike. It stays as the reference for the test kit's first part (task 4) and the fuzz integration (task 20), and the commit that completes task 20 deletes it, as `test-kit.md` section 9 proposes.

## Contents

1. [What it holds](#1-what-it-holds)
2. [Running it](#2-running-it)
3. [What it shows for G3.2](#3-what-it-shows-for-g32)
4. [What it leaves out](#4-what-it-leaves-out)

## 1. What it holds

```
spikes/fixtures/
├── Cargo.toml          the workspace of kit/ and library/, with the root workspace's lint table
├── fixtures/           the stand-in for crates/cc-media-probe-testkit/fixtures/
│   ├── toy/setup-and-parameters.hex                     the first sample: a toy file whose packet is a codec
│   │                                                    header, with two cases
│   ├── toy/setup-and-parameters.expected.json           its semantic part, and adapters.json its adapter part
│   ├── toy/setup-and-parameters.short-reads.*.json      the parts of each case
│   ├── toy/header-across-packets.hex                    a stream whose header comes in two packets
│   └── flac/decoding-example-1.hex                      the second sample: RFC 9639, Appendix D.1, with its parts
├── kit/                the stand-in for the test kit: the standard library only, no dependency on the library
│   └── src/            syntax/ (the text format, its cases and its assembler), fixture/ (the layout, loading, file
│                       names, seeds, the places of expectations), readers/ (a counting reader, and a faulting
│                       reader that cuts at fixed bytes), builder/ (a byte writer and a builder of toy files)
├── library/            the stand-in for the library
│   ├── src/            toy/ (the accepted codec parser, detection and a walker), fuzzing_entry_points/,
│   │                   repository_entry_points.rs
│   └── tests/cases/    the seed check that writes the corpus, the configuration, the expectations, the
│                       conditions of a case, the split, the recipes, and the toy outcomes
└── fuzz/               a cargo-fuzz workspace: the targets toy, toy_codec and probe
```

- **`kit/` stands in for the test kit.** It reads the text format of `test-kit.md` section 2, cases included, and checks the fixtures' folder against its layout. It gives each fixture's seeds with their routes, by name only, since the test kit knows no target, and finds fixtures and their expectations from its own manifest folder. Its tests check the format's refusals with their lines, the layout's refusals, and each sample against a second transcription of its bytes.
- **`library/` stands in for the library.** Its toy codec parser is the I/O spike's, ported whole: the parser that G3.1 accepted. Its detection follows the I/O spike's rule. Its walker is reduced from the I/O spike's: it enters lists, declares streams with their codec and framing checked, and assembles each stream's header across packets. They are compiled only under `cfg(any(fuzzing, cc_media_probe_repository))`, as the permanent `toy/` is (plan section 7.2). `fuzzing_entry_points` is compiled under the same condition. It holds the four targets `toy`, `toy_codec`, `detection` and `probe`, their input encodings, and what an input reached. `repository_entry_points` is compiled under the marker alone.
- **`library/tests/cases/` stands in for the library's tests.** `common/` holds what `tests/common/` would:
  - the conditions of a case;
  - the places of source entries, with their paths and their retained data;
  - the split of a record into its parts;
  - the comparison of an expectation with its file;
  - the I/O error kinds that a failed companion names;
  - the match of a recipe's values;
  - JSON pointers;
  - the corpus that the seed check writes.
- **`fuzz/` is the cargo-fuzz workspace.** It keeps the root workspace's lint table. Each target's root declares only, as R30 asks of a binary's root, and its macro sits in `target.rs`.

## 2. Running it

```sh
cd spikes/fixtures
cargo test                                            # the kit's 22 cases and the library's 46, 2 of each embedded
CC_MEDIA_PROBE_WRITE_EXPECTATIONS=1 cargo test        # writes the expectation files instead of comparing them
CC_MEDIA_PROBE_WRITE_CORPUS=1 cargo test -p library seeds   # the seed check, which then writes fuzz/corpus/
cd fuzz
cargo +nightly fuzz build
cargo +nightly fuzz run toy corpus/toy -- -max_total_time=30 -timeout=5 -rss_limit_mb=1024 -max_len=1048576
cargo +nightly fuzz run toy_codec corpus/toy_codec -- -max_total_time=30 -timeout=5 -rss_limit_mb=1024 -max_len=1048576
cargo +nightly fuzz run probe corpus/probe -- -max_total_time=30 -timeout=5 -rss_limit_mb=1024 -max_len=1048576
```

cargo-fuzz 0.13.2 is installed with `cargo install --locked --version 0.13.2 cargo-fuzz`, the version that the dependency dossier approves. The runs used nightly 1.102.0 of 2026-11-20 on macOS.

## 3. What it shows for G3.2

- **The format reads both samples, and a second transcription agrees with each.**
  - The FLAC fixture assembles to the bytes of the RFC's own dump in Appendix D.1.1, which its test transcribes separately as the RFC prints it. Its streaminfo block, bytes 4 to 41, is a region routed to `flac_codec`, the codec target of Phase 4.
  - The toy fixture assembles to what the builder writes from the elements' values, with the lengths computed.
  - A builder writes a wrong length on purpose, through an override of the computed field.
- **The RFC's text and its dump disagree on one offset.** Appendix D.1.4 says the frame CRC starts at 0x38. Its dump in D.1.1, and its table 31, whose second subframe ends with byte 0x36, put it at 0x37. The fixture follows the bytes and says so in a comment. The RFC Editor lists no erratum for it (errata 8256, 8810 and 9096, the last rejected, on 2026-11-27).
- **The format refuses each malformed form at its line.** Among them:
  - a line of bytes without a comment that names its field, or above every heading;
  - hex digits of an odd count;
  - quoted text beyond printable ASCII;
  - a repeat without its item or count, or a count of 0;
  - a body beyond 16 MiB;
  - a region left open, nested, empty, or routed to `none`, and an `@end` with a word after it;
  - a header member given twice, unknown or missing, and a stated length that the body does not assemble to;
  - a case without a condition, a malformed or repeated condition, a case given twice, and an outcome beside the target `none`;
  - a file name with a separator, and one that not every platform can write, such as `NUL.idx`, `con.backup` or `movie.`, which revision 75 accepted;
  - a fixture of the target `none` that marks no region, which revision 76 accepted with no seed at all.
- **The layout is checked whole.** A test builds a folder of each wrong shape below Cargo's folder for integration tests, and each is an error at its path:
  - a file beside the format folders, or a folder inside one;
  - a misnamed format folder or fixture, and, by its name, a fixture or format named as a device that Windows reserves, such as `toy/nul`;
  - another extension or part;
  - an expectation without its fixture;
  - on Unix, a file name that is not Unicode.

  On macOS's file system, which ignores case, `Toy/` and `toy/` are one folder, so the test misnames a folder and a fixture with names that differ by more than case.
- **A case runs the same bytes under its conditions.** The toy fixture's `short-reads` case ends its reads at byte 40, 48 and so on, whatever sizes they ask for, through the faulting reader under the seekable and forward-only adapters and at the custom source's own positions. Two kit tests show the cut at fixed bytes, across seeks too; revision 76 cut by read calls, so reads of 8 and then 64 bytes delivered 8 and 8. The case was `split-header` until the third review, which found that the library may join short reads, so it established no split at the codec. The `small-header-limit` case sets the header limit by its serialized name. Each case has its own expectation files. The Phase 2 library reads no input, so every case gives the same unknown-format record. The spike shows the cases' files and their conditions, not results that differ. A condition that no rule reads, and a misspelled limit, fail the test. Lookup is off by default. The lookups `custom` and `built_in` and the companions' answers are read and checked, and conditions that no adapter can run together are refused, such as a built-in companion named `NUL.sub` or the file's own name in another case, but no case uses them, since the library has no lookup before Phase 14. A hint-only case's `hint=` is percent-decoded, and a sign among its hexadecimal digits refused, as a token's is: revision 75 read the token `+a` as the byte 10. `hint=none` makes a nameless case, and built-in lookup with a hint or none is accepted, without an answer and without the file, for the two outcomes of lookup without a path. A requested name is percent-decoded too, each decoded name once, and each response takes the arguments of the resolver's result: revision 76 accepted `limit_reached:streams`, `failed:whatever` and `found:`, refused `declined:` with a reason and an empty token, and kept `Movie%20%282026%29.sub` encoded. Pieces beside built-in lookup with the file are refused.
- **A header across packets reaches the codec in two deliveries.** `toy/header-across-packets` is a stream whose setup and parameters are two packets' payloads. The walk parses its header, and without the first packet the parameters are malformed, as they would be with a lost continuation.
- **The expectations work on the library's real JSON form.**
  - Each case's prober starts from `fuzzing_entry_points::prober`, the one that the `probe` target runs, so a default case's expectation and the seed check's probe outcome come from one setup.
  - Every adapter is given the fixture's file name, the file adapter as its file's name and the others as the hint. A hint-only case skips the file adapter.
  - The file adapter's retained-data limit is the case's raised by its path's charge, which a test checks, with a saturating sum at the largest limit.
  - The semantic part is the same through bytes, a file, a seekable reader and a custom positional source, and the forward-only part equals it, so no forward file is kept.
  - The adapter part holds each adapter's bytes read and peak memory, keyed by JSON pointer, and the read calls and bytes delivered of the reader, the stream and the custom source. The file adapter's counts wait for the repository-only counter that the I/O layer of task 6 brings.
  - A file that no case needs is refused, or removed when the variable is set: a test file `setup-and-parameters.old-case.expected.json` failed the run, and the writing run removed it.
- **The split takes members only at the places that the form defines.** A typed detail's own `usage`, `sources` and `size` stay semantic, which the first spike's split got wrong. An error's partial report and observations split as a report and as a source table do. An error's code and cause messages become the placeholder `{native}`, with their presence and number kept.
- **A path below the test's folder is written as `{folder}/` and the rest.** The lossy flag and the raw form collapse into that one Unicode path. The record's retained data is taken net of the path's charge: the strings that the report keeps for it. On Unix a lossy path of 5 bytes by `OsStr::len` is charged 17, its display string's 7 and its raw form's 10, where revision 75 took 5. The folder is below `CARGO_TARGET_TMPDIR`, one for each case: `expectations`, the process's identifier and the case's number in the process, 8 hexadecimal digits each, then `_` up to 512 bytes in components of at most 255, removed when the case is done, which a test checks. So every path's charge is the same on every machine, no name counts, two runs never share a folder, and no expectation holds the process or the run. Revision 76 padded to 200 bytes after the names, so a fixture name of 155 characters panicked, and so did the two samples in a scratch copy whose path is long. A path is below the folder by its components, and must be a handed path written exactly as its form gives it: revision 76 rewrote `/tmp/fixture2/file` as `{folder}2/file` and let a Unicode path marked lossy collapse. A partial report's path comes out of the error's usage too, where revision 76 left 24 beside the partial report's 2.
- **An expectation file compares as a value.** The comparison parses the stored file and compares the values directly, so member order and white space never count. The library's tests also pass with `cargo test -p library --features serde_json/preserve_order`, which failed the first spike's FLAC golden test. A written file is sorted by an explicit sort, and a recipe's differences are listed in the order of their pointers, with member names escaped as RFC 6901 escapes them.
- **The toy outcomes follow the accepted grammar.** Eleven tests hold the cases that the reviews found the spike's reduced parser and walker to get wrong:
  - a setup that ends at the packet's end needs a later packet;
  - a stream skips its payload units;
  - the needs-decoding flag ends partially parsed by design;
  - a file of its header alone, or whose first element lies beyond its end, is insufficient data, not a walked report;
  - another codec is unsupported, and framing 2 declares no stream;
  - lists are entered, and a child beyond its list ends the list;
  - a list that the input cuts short keeps the stream that it declared, and a packet that it cuts short offers what arrived, so a whole header parses, in packet mode and in stream mode: revision 76 gave no stream and `not_found`;
  - a codec input whose context states another length than its bytes', or a length in stream mode, is rejected: revision 76 parsed a claim of 100 bytes beside 8;
  - the stream fixture's header parses across its two packets.
- **The seed check runs in an ordinary repository build, and writes the corpus.**
  - Both seeds of the toy fixture reach their parser with their recorded outcome: the file reaches the container walk with `report`, and its packet's codec header the codec parser with `parsed`.
  - Each fixture is also a seed of `probe`, whose outcome, `unknown_format` in Phase 2, is its expectation's. A whole fixture routed to `probe` is refused, and a region routed to it is checked with its own outcome and the fixture's file name as its hint.
  - The FLAC fixture names `flac` and `flac_codec`, which the check accepts only from its list of later targets, each with its phase.
  - A seed routed with the other representation is `malformed`, a representation code that no representation has is rejected before the parser, and an input beyond 1 MiB after encoding is refused, so a wrong route fails the check rather than passing unseen.
  - The check runs the golden pass first, once in the process, so with `CC_MEDIA_PROBE_WRITE_EXPECTATIONS` set it reads the expectations that the pass wrote. Revision 76 ran them apart, and with a new fixture the check failed on its missing `expected.json`.
  - Every seed's bytes are held to the input bound before a later target is let through: revision 76 let a region of 2 MB routed to `flac` pass.
  - With `CC_MEDIA_PROBE_WRITE_CORPUS` set, the check wrote six seeds: `seed.toy.setup-and-parameters` and `seed.toy.header-across-packets` in `toy/` and in `probe/`, `seed.toy.setup-and-parameters.1` in `toy_codec/`, and `seed.flac.decoding-example-1` in `probe/`. A rerun after the fuzz runs below kept every file that the fuzzer had added, 394 in `toy/`, 212 in `toy_codec/` and 13 in `probe/`, and rewrote only the seeds. The writer fails with its path at a folder that it cannot read, where revision 76 kept a stale seed there unseen. A test shows that a seed that no fixture gives any more is removed, that the fuzzer's files are kept, and that two seeds of one file are refused before anything is written.
- **The input encodings are written once, in the library.** The test kit gives routes by name. `fuzzing_entry_points::encode` turns a route, the file name and the bytes into the target's input, and the targets and the seed check all use it, so no table is copied between crates (R36). A codec input is one byte for the representation, then the target's fixed context, then the bytes; a detection or probe input is one byte for the hint's length, then the hint, then the bytes. On Unix the hint's bytes are the native name as they stand, so a name that is not UTF-8 stays one, where revision 75 turned the byte `ff` into U+FFFD; elsewhere such a hint is rejected. The library's two embedded tests check both, under `cfg(unix)` and `cfg(not(unix))`.
- **cargo-fuzz builds without the repository marker.**
  - It passes `--cfg fuzzing` in `RUSTFLAGS`, which Cargo takes in place of the root `.cargo/config.toml`'s `build.rustflags`. Each target checks on every input that the build has `fuzzing` and not the marker.
  - `toy_codec` ran 7,148,755 inputs in 31 seconds without a failure, at a peak RSS of 532 MB under the limit of 1 GiB, and added 7 inputs to its corpus. `probe` ran 6,887,776 inputs without a failure, at a peak RSS of 464 MB; on the Phase 2 library every input ends at detection. The first spike's `toy_codec` ran about five times as many inputs in the same time, before its entry point gave its outcome as an owned string.
  - After the second review, with the hint read as native bytes, both were built again and ran 21 seconds each without a failure: `toy_codec` 4,872,752 inputs and `probe` 4,720,011.
  - After the third review, with the `toy` target added for the whole toy fixtures' corpus, all three ran 21 seconds each without a failure: `toy` 3,390,710 inputs from its 2 seeds, `toy_codec` 4,799,386 and `probe` 4,641,077.
  - Built so, the first spike's stand-in failed to compile: an import that only `repository_entry_points` used was unused there, and warnings are denied. A fuzz build is a third configuration that every cfg-gated item must compile cleanly in, so the toy walk's stream states are read by the fuzz entry point too.
- **The fuzz workspace keeps the lint table.** `forbid(unsafe_code)` holds: `fuzz_target!` writes `no_mangle`, but from another crate's macro, which the lint does not report. Clippy finds nothing in the fuzz crate.
- **A build outside the repository has neither hidden module.**
  - A copy of `library/` outside the repository builds with warnings denied.
  - A consumer that names `fuzzing_entry_points` or `repository_entry_points` fails with E0433 for each. That is the tarball job's check (plan section 9.7).
  - Cargo reports the stand-in's dependency on the real library as unused there, since only gated code uses it. The permanent library has no such dependency: its toy walker uses its own types.
- **The structure check of the repository gives no finding on the spike.** The first spike's check found `hex` in five names, which R32 spells out as `hexadecimal`, and text in the target's message template (R33).

## 4. What it leaves out

- the bit writer, the container builders, and the faulting reader's errors and interruption, whose interface `test-kit.md` section 5 proposes;
- the async adapters, whose adapter parts would join the same file;
- the repository-only `with_toy_format` and `with_source_counter`, which need the I/O layer of task 6, and so the file adapter's read counts;
- the custom resolvers and the built-in lookup of a case's companions, which the library has from Phase 14, task 1, on, and the check that each request has its answer and each answer is asked for;
- the toy format in `fuzzing_entry_points::prober`, which needs task 5's walker inside the library: the spike's prober is the default one, which the golden tests and the targets share all the same;
- names that are not Unicode as a path or a hint of a probe, whose platform-specific tests belong to the library (plan section 7.7);
- a file probe at its own limit one byte below its path's charge and exactly at it, since the Phase 2 library charges no path: task 7 tests both;
- results that differ by case, since the Phase 2 library reads no input;
- a detection fixture: the detection target's encoding and its hint are tested, but the Phase 2 library's file-name hint suggests no format, so no fixture could record `hint_only` yet;
- the paths of generated test files, which the test-files guide already describes;
- a fixture with an erratum: the `erratum` member is read but neither sample depends on one.
