# Test kit: fixtures, expectations, seeds and hidden entry points

| | |
|---|---|
| **Status** | Draft of 2026-11-27 for gate G3.2 (plan Phase 3, task 3), written at revision 74 of the plan, revised at revision 75 after the maintainer's review the same day, at revision 76 after the maintainer's second review, at revision 77 after the third, at revision 78 after the fourth, at revision 79 after the fifth, at revision 80 after the sixth, and at revision 81 after the seventh (section 10). The second review approved the section named in a heading (section 2.2), the `Toy` variant (section 7) and the choices of section 8. The fourth approved D19, with the hidden module of fuzz targets and the repository-only entry points (sections 6.3 and 7), and the rest of D14, the fixture syntax, the seed routing and the revised rule of path accounting among it, and kept G3.2 open on two findings of the fixture protocol, which revision 78 answered (sections 2.3 and 4.1). The fifth review, of revision 78, kept G3.2 open on eight findings of the fixture protocol, which revision 79 answered (sections 2.1, 2.3, 4.1, 4.2 and 6.4). The sixth review, of revision 79, kept it open on five more, which revision 80 answered (sections 2.3, 4.1, 4.2, 4.3 and 6.4), and the seventh, of revision 80, on three more, which this revision answers (sections 4.2 and 4.3). The proposal as a whole is open at G3.2 until the maintainer confirms it. |
| **Decides** | D14, the fixture format, and D19, access to internals for fuzzing (plan section 6). |
| **Evidence** | The spike [`spikes/fixtures/`](../../spikes/fixtures/README.md), which runs this proposal end to end on the two sample fixtures of section 3. |
| **Plan** | Sections [7.1](../plans/plan-init-project.md#71-workspace-layout-and-target-ownership), [9.1](../plans/plan-init-project.md#91-test-files), [9.2](../plans/plan-init-project.md#92-test-layers), [9.4](../plans/plan-init-project.md#94-resource-gates) and [9.5](../plans/plan-init-project.md#95-fuzzing). Generated test files and their recipes are in the [test-files guide](../guide/test-files.md). |

## Contents

1. [What G3.2 decides](#1-what-g32-decides)
2. [The hex-fixture format](#2-the-hex-fixture-format)
3. [Two sample fixtures](#3-two-sample-fixtures)
4. [Expectations](#4-expectations)
5. [Builders, readers and paths](#5-builders-readers-and-paths)
6. [Fuzz seeds and the hidden access path](#6-fuzz-seeds-and-the-hidden-access-path)
7. [Repository-only entry points](#7-repository-only-entry-points)
8. [Choices made without asking](#8-choices-made-without-asking)
9. [What happens after G3.2](#9-what-happens-after-g32)
10. [The reviews of the proposal](#10-the-reviews-of-the-proposal)

## 1. What G3.2 decides

Task 3 asks for six things before any permanent test-kit or fuzz code:
- the hex-fixture format with two sample fixtures (sections 2 and 3);
- how a recipe states the values its tests expect (section 4.4);
- the builder interface (section 5);
- the routing fields of fuzz seeds (section 6.1);
- the hidden access path of fuzz targets (section 6.3);
- the list of repository-only entry points (section 7).

The maintainer settled four questions on 2026-11-27, before the first draft:

| Question | Answer |
|---|---|
| What evidence comes with the proposal | This document plus a disposable spike, with a real cargo-fuzz build of a target that reaches a hidden module |
| What a line of a fixture may hold beside its bytes in hex and its comment | Quoted text and repeats; nothing is computed, so every byte stays as written, and nested sizes are the builders' job |
| What form expectations take | The library's JSON form ([`json-output.md`](../guide/json-output.md)), split into a semantic part and an adapter part, and written only when an environment variable asks |
| Where the seed check runs | In repository builds too: `fuzzing_entry_points` is compiled under `cfg(fuzzing)` and in repository builds, so the seed check is an ordinary test |

The maintainer's review of the first draft, the same day, kept G3.2 open, and the maintainer settled four more questions (section 10):

| Question | Answer |
|---|---|
| Where a fixture's cases live: the conditions under which its tests probe the same bytes | On `case:` lines of the fixture's header, each a name and `name=value` conditions, with expectations of their own (sections 2.3 and 4.1) |
| How the file adapter's reads are counted | A second repository-only entry point counts the reads of every source that the library opens itself (section 7) |
| How parity treats the retained data of a path | It stays semantic, net of the paths that the test handed the library, which the library must charge at their length (section 4.2) |
| What shape the corpus tool takes | The seed check writes the corpus when an environment variable asks, and removes only its own stale files (section 6.4) |

The maintainer's second review, of revision 75, approved the section named in a heading, the `Toy` variant and the choices of section 8, kept the proposal open on four contract issues, and settled three more questions (section 10.2):

| Question | Answer |
|---|---|
| How a file probe keeps the others' budget at a limit | The library charges a path at the strings that keep it, held at their length, before anything else; the test raises the file adapters' retained-data limit by that charge, in case folders of a fixed length (section 4.2) |
| What a route to `probe` means, beside the seed that every fixture already gives it | A whole fixture's is refused; a region may route to `probe` (section 6.1) |
| How tests reach the names that no file can have | A hint-only case, `hint=` in percent-encoded UTF-8; names that are not Unicode in platform-specific tests of the library; and the fuzz input's hint as native bytes on Unix (sections 2.3, 4.1 and 6.2) |

The maintainer's third review, of revision 76, kept the proposal open on 19 findings and settled two more questions (section 10.3):

| Question | Answer |
|---|---|
| What a file probe gives when its retained-data limit cannot hold the charge of its path | The entry stays without its path, the limit is exhausted without a part that it stopped, and the probe goes on under the same limit (section 4.2, `api.md` section 6.15) |
| How each case's folder is built | 512 bytes, from a component for the run and one for the case, each of a fixed width, then padding, so that no name counts and concurrent runs never share a folder (section 4.2) |

The maintainer's fourth review, of revision 77, approved D19 and the rest of D14, kept G3.2 open on two findings of the fixture protocol, and settled two more questions (section 10.4):

| Question | Answer |
|---|---|
| How a case records outcomes that a limit makes differ by adapter, while each adapter honors the limit | The condition `outcomes=per_adapter`, only beside a `limit.` condition, with each adapter's semantic part in `name.case.outcomes.json`, and refused where every adapter agrees with its class; every other case keeps the shared parts (sections 2.3 and 4.1). The fifth review replaced the `limit.` condition with the evidence of a limit (below) |
| Where the async adapters' outcome under built-in lookup is tested | In a library test of Phase 14, task 1: an async probe with only the blocking built-in option makes no request, and its companions are "not enabled". A fixture case of built-in lookup runs through blocking adapters alone (sections 2.3 and 4.1) |

The maintainer's fifth review, of revision 78, kept G3.2 open on eight findings of the fixture protocol and settled two more questions (section 10.5):

| Question | Answer |
|---|---|
| How the default case records outcomes that a limit makes differ by adapter, as the default limits can | A header line `default: outcomes=per_adapter`, which takes no other condition. No case needs a `limit.` condition beside `outcomes=per_adapter` any more, since the evidence below replaces it, and the seed check reads the outcome of the bytes adapter (sections 2.1, 2.3, 4.1 and 6.4) |
| What shows that a limit caused a difference between adapters | In each class, the adapters that exhausted no limit share one part, so two adapters that differ while neither exhausted a limit fail the case, whatever a third exhausted (section 4.1) |

The maintainer's sixth review, of revision 79, kept G3.2 open on five findings of the fixture protocol and its spike and settled one more question (section 10.6):

| Question | Answer |
|---|---|
| Which features of the library compare and write expectations, since the async adapters are behind `tokio` and `futures-io` and `zlib` changes what a compressed header gives | Every feature. The golden tests and the seed check form one test target whose `required-features` name each feature of the library, so each expectation has one feature set, and a run with fewer features compares and writes none (section 4.3) |

The maintainer's seventh review, of revision 80, kept G3.2 open on three findings of the fixture protocol and its spike and settled one more question (section 10.7):

| Question | Answer |
|---|---|
| What runs without `zlib`, whose behavior Phases 5 and 6 test while the golden target is skipped there | From task 14, a step of `node scripts/check.ts`, which CI runs on Linux, macOS and Windows: Clippy and every test of the library alone, selected with `-p`, under every feature but `zlib`. The tests of the disabled behavior are gated on `not(feature = "zlib")` and assert its status by hand; the golden files stay under every feature (section 4.3) |

How a fixture serves the tests and the fuzzer:

```mermaid
flowchart TD
  F["A fixture: name.hex<br/>in the folder of its format"] --> A["Test kit: assembler"]
  A --> B["Bytes, file name and cases"]
  A --> S["Seeds by name: the whole fixture,<br/>and each region marked @seed"]
  B --> P["Library tests under every feature:<br/>each case through each adapter,<br/>from the probe target's prober,<br/>under the same conditions"]
  P --> X["Usage within each adapter's limits,<br/>the primary input's path as given,<br/>then paths with a placeholder,<br/>then the split of each record"]
  B --> W["From task 14, library tests without zlib:<br/>the disabled behavior's status,<br/>asserted by hand, with no expectation file"]
  X --> E1["name.case.expected.json<br/>semantic part"]
  X --> E2["name.case.forward.json<br/>only where it differs"]
  X --> E4["name.case.outcomes.json<br/>semantic part per adapter,<br/>where a limit explains it"]
  X --> E3["name.case.adapters.json<br/>adapter part per adapter"]
  S --> N["fuzzing_entry_points::encode"]
  B --> N
  N --> C["Seed check: an ordinary test<br/>that runs each seed"]
  E1 -->|"read after the golden pass"| C
  E4 -->|"its bytes member,<br/>where the default case keeps one"| C
  C -->|"when the variable asks"| K["fuzz/corpus/target/<br/>seed files, stale ones removed"]
  K --> Z["cargo-fuzz targets"]
```

The default case has no case name in its files' names: `name.expected.json`.

## 2. The hex-fixture format

A fixture is a text file `crates/cc-media-probe-testkit/fixtures/<format>/<name>.hex`. It is UTF-8, and its lines end with a line feed, with or without a carriage return. It has a header, the line `---`, and a body. Tests name it by `<format>/<name>`, with `/` on every platform.

The fixtures' folder holds format folders only. Each format folder holds files only: the fixtures `<name>.hex` and their expectation files of section 4.1. A format, a fixture and a case are each named by lowercase words and digits joined by `-`, none of them a device name that Windows reserves, such as `nul` or `com1`, since `nul.hex` names that device there. Every listing checks this layout whole, so a misnamed fixture is an error rather than a fixture left out unseen. The checks refuse:
- a file beside the format folders;
- a folder inside a format folder;
- a name that breaks the rule, or that is not valid Unicode;
- any other extension or part;
- an expectation file without its fixture.

### 2.1 The header

The header is `name: value` lines, one member to a line. Blank lines are skipped, and so are comments that start with `#`.

| Member | Required | Meaning |
|---|---|---|
| `description` | yes | What the fixture holds, in one line |
| `source` | yes | The specification and section that the fixture comes from as a whole, such as `RFC 9639, Appendix D.1` |
| `erratum` | no, and may repeat | An erratum that the fixture depends on, such as `RFC 9639 erratum 8810` (plan section 9.1) |
| `target` | yes | The fuzz target of the whole fixture (section 6.1), or `none` for a fixture whose whole bytes are no seed, which then marks at least one region |
| `representation` | for a codec target | The representation of the bytes, by name, such as `one_byte_lengths` |
| `context` | no | For a codec target, the context that the container knows, in hex, in the layout that the target defines |
| `outcome` | yes, except beside `none` | The outcome that the target's entry point gives the whole fixture, by name, such as `report` or `parsed` |
| `file_name` | no | The name of the input as a file, such as `movie.idx`, which every adapter is given (section 4.1), and which every platform can write |
| `case` | no, and may repeat | A case: its name, then its conditions (section 2.3) |
| `default` | no | The default case's conditions, `name=value` words as a case's, of which `outcomes` is the only one (section 2.3) |
| `length` | no | How many bytes the body assembles to; the assembler checks it |

A target, a representation and an outcome are names: lowercase letters and digits, their words joined by `_`. A context needs a representation beside it. The target `none` takes no representation, context or outcome: it serves a fixture longer than a fuzz run's input (section 6.2), whose regions give bounded seeds. So it marks at least one region, or the fixture would give no seed at all, as the third review found.

A file name is written to disk, so it is one that every platform can write, as the maintainer chose at the second review: ASCII letters, digits, `.`, `-` and `_`, at most 255 bytes, ending with no `.`, so neither `.` nor `..`, and whose part before the first `.` is none of the device names that Windows reserves with any extension, `CON`, `PRN`, `AUX`, `NUL`, `COM1` to `COM9` and `LPT1` to `LPT9`, compared without regard to ASCII case ([Microsoft, "Naming Files, Paths, and Namespaces"](https://learn.microsoft.com/en-us/windows/win32/fileio/naming-a-file)). So `movie.idx`, `.hidden` and `icon.backup` are file names, and `NUL.idx`, `Aux.tar.gz`, `con.backup` and `movie.` are none. Its letters keep their case, since a test may ask how a hint in another case is taken. Without the member, a fixture's file name is its own name without an extension, such as `setup-and-parameters`, which suggests no format and is no device name either (section 2). A name that no file can have reaches the library as a hint-only case's hint (section 2.3), and a name that is not Unicode through a platform-specific test of the library (section 4.1). A member other than `erratum` and `case` is given once.

### 2.2 The body

Each line of the body is one of:

- **Blank**, or a **comment**: `#` and any text. Neither assembles to anything.
- **A heading**: `##` and the specification and section of the lines of bytes below it, up to the next heading, such as `## RFC 9639 section 8.2, and Appendix D.1.3, table 28: streaminfo`. A heading needs text.
- **A line of bytes**: items, then `#` and a comment that names the field or fields that the bytes hold. The comment is required, and a line of bytes before the first heading is refused.
- **A directive**: `@seed` and `name=value` words, which open a region, or a bare `@end`, which closes it (section 6.1).

The items of a line of bytes:

| Item | Form | Bytes |
|---|---|---|
| Hex bytes | Pairs of hex digits, in either case, in words separated by white space: `00 0a`, `0A0b`, `664c6143` | One byte per pair |
| Quoted text | `"fLaC"`: printable ASCII from space to `~`, without escapes and without `"` | Its characters' codes |
| Repeat | An item, `*` and a decimal count of at least 1: `00 * 4096`, `"ab" * 2` | The item before it, that many times in all |

A `#` inside quoted text is text. A repeat applies to the one item before it. Nothing is computed: no size, offset or checksum is derived from other bytes, so every byte of a fixture is written where it stands, as the maintainer chose. A body assembles to at least one byte and at most 16 MiB, so that a repeat cannot ask a test for more memory than it should hold.

**Approved at the second review: the section in the heading.** Plan section 9.1 had "each line carries bytes and a comment naming the field and spec section". A fixture names the specification and section once, in the heading above a group of lines, and the field on each line. A line that a different subsection governs, such as `(9.1.6)`, names it in its comment. So both samples stay traceable line by line, with lines about half as long as the alternative, the specification and section on every line, would make them. The maintainer approved the heading form at the second review, and revision 76 of the plan amends section 9.1 to match.

### 2.3 Cases

A fixture's bytes are tested under conditions beyond the bytes themselves, and the same bytes give different results under different conditions. Plan Phase 3, task 6, for one, keeps a header split across deliveries, which a stream's packets give in the fixture's own bytes (section 3), a packet larger than the header limit, which a case gives, and a two-file input with its resolver's results. As the maintainer chose, each such case is a `case:` line of the header: the case's name, then at least one condition, each a `name=value` word, each name once. The default case takes one condition, `outcomes`, on a `default:` line without a name, as the maintainer chose at the fifth review.

```text
case: short-reads pieces=40,8
case: small-header-limit limit.header_bytes_per_stream=8
case: two-files lookup=custom companion.movie.toyc=found:toy/movie-companion
case: reserved-hint hint=NUL%20.backup
case: no-name lookup=custom hint=none
case: no-folder lookup=built_in hint=movie.idx
case: no-memory limit.working_memory=0 outcomes=per_adapter
default: outcomes=per_adapter
```

The test kit reads the words and never interprets them, as it reads a route. The library's tests read each condition by one rule, and a word that no rule reads fails the test, so that a misspelled condition is never ignored:

| Condition | Value | Effect |
|---|---|---|
| `limit.<limit>` | A decimal number | The limit of that serialized name, such as `limit.header_bytes_per_stream=8` |
| `pieces` | Sizes, each at least 1, separated by `,` | The input cut into pieces of these sizes from byte 0, the last size repeating: no read of the seekable and forward-only readers or of the custom source delivers bytes of two pieces, so `pieces=40,8` ends reads at byte 40, 48, 56 and so on, whatever sizes the reads ask for. The library may join short reads before its parsers see them, so pieces test that joining, and a header split across deliveries to a codec is a matter of the fixture's own bytes, such as a stream's packets (section 3) |
| `hint` | A name in percent-encoded UTF-8 ([RFC 3986, section 2.1](https://www.rfc-editor.org/rfc/rfc3986#section-2.1)), at least one character, such as `NUL%20.backup` or `Movie%3A%20Part%202`; or `none` | A hint-only case: every adapter but the file adapters is given this hint in place of the file name. It reaches the names that no file can have: a device name that Windows reserves, a name that ends with a period or a space, and the characters that Windows reserves in a name. `none` makes a nameless case: those adapters get no hint at all, so that no name can be built. A hint spelled `none` is written `%6Eone` |
| `lookup` | `off`, `custom` or `built_in` | The companion lookup. `off`, the default, is the API's: no request is made, and the companions are "not enabled" (`api.md` section 13). `custom` gives every blocking adapter `CompanionLookup::custom` and every async one `AsyncCompanionLookup::custom`, with resolvers that answer alike. `built_in` gives the blocking adapters `CompanionLookup::built_in()` and the async ones nothing, since an async probe reads only the async option, which has no built-in resolver (`api.md` section 13). So the async adapters run no such case. With the fixture's file name, it runs through the blocking `probe_file` alone, the only entry point that gives the built-in resolver a folder; with a hint or none, through every other blocking adapter, where the resolver has no folder and declines each request itself |
| `companion.<requested name>` | With `custom`: `found:<fixture>`, `found_with_identity:<fixture>:<token in hex>`, `not_found`, `declined:<reason>`, `limit_reached:<limit>` or `failed:<error kind>`. With `built_in` and the file: `found:<fixture>` or `not_found` | The answer to the request for that name. The name is percent-encoded as a hint is, such as `companion.Movie%20%282026%29.sub`, and each name, decoded, has one answer. The arguments are those of the resolver's results (`api.md` section 13): a fixture by its name of the layout, such as `toy/movie-companion`; a token of any length, none among them; a reason in percent-encoded UTF-8, which may be empty; one of the limits that a resolver may name, `companion_requests`, `sources_opened` or `bytes_read`; and an I/O error kind by the stable name that a record gives it (`json-output.md` section 4.4), such as `permission_denied`. A custom resolver's found companion is the named fixture's bytes. Built-in lookup writes a found companion beside the file under its requested name, which is then a file name of section 2.1, apart from the file's and from each other companion's by more than case, since Windows and macOS compare names without regard to case by default |
| `outcomes` | `shared` or `per_adapter` | Whether the adapters of each class share one semantic part, the default, or each adapter has its own (section 4.1). `per_adapter` is for a case where a limit stops adapters differently while each honors it: the working memory that a reader's cache needs and bytes in memory do not, or, under the default limits, the read limit that a reader's cache blocks use up while bytes in memory read in place. A limit must explain each difference (section 4.1). The default case takes this condition alone, on a `default:` line |

The default case is the fixture under the default limits, with reads that are not cut and lookup off, as the `probe` target runs it (section 6.3). Its one condition is `outcomes`, which a `default:` line of the header gives; any other condition there is refused, so that the default case stays the probe target's. Conditions that no adapter can run together are refused: an answer without a lookup that asks for it; beside built-in lookup with the file, an answer that only a custom resolver gives, and pieces, since the file adapter's reads are not cut; and beside built-in lookup without the file, any answer, since the resolver declines each request itself. Each request that a probe makes, to the test's custom resolver or to built-in lookup with the file, must have its answer in the case, so a misspelled name fails the test rather than giving `not_found`. A request that built-in lookup without the file declines takes no answer, since the resolver gives its own. Each answer must be asked for by at least one adapter of the case, not by each, since a limit can stop one adapter before a request that another makes. The fifth review found the rule silent on both. Each case has expectations of its own (section 4.1).

**Companions arrive in Phase 14.** The library has no companion lookup before Phase 14, task 1, which ships the resolvers and the builder options, so until then the tests refuse every lookup but `off`. Task 6 keeps the I/O spike's two-file toy input as fixtures of their default case, with the spike's resolver results noted in their comments, and Phase 14, task 1, adds their cases of custom and built-in lookup, with their expectations. The async adapters run no case of built-in lookup, so the same task adds a library test beside the resolvers, as the maintainer chose at the fourth review. A prober with only `CompanionLookup::built_in()` runs every async entry point, seekable and forward-only on both runtimes, and each result's usage counts no companion request and no source opened, which counts companions alone (`api.md` section 6.14). Each `probe_*` method's report also lists the primary input and no companion in its source table, and reports its companions "not enabled" for an input that expects one, such as the two-file toy input. A `Detection` of a `detect_*` method has its usage alone, since it has neither a source table nor a companions status (`api.md` section 8). The fourth revision said "no source-table entry", which the primary input's own entry contradicts, as the fifth review found, and the fifth asked a detection for the source table and the status that only a report has, as the sixth found.

### 2.4 What the assembler refuses

The assembler reports the first problem with its line, counted from 1, or 0 for the fixture as a whole. A test names the fixture too: `flac/decoding-example-1: 12: hex bytes are pairs of digits: 0a0`.

- **The header:**
  - no line `---`;
  - a line that is not `name: value` or a comment;
  - an unknown member, or a member given twice;
  - a required member missing;
  - a context without a representation;
  - a representation, context or outcome beside the target `none`;
  - a name that is not lowercase words;
  - a file name that the rule of section 2.1 does not allow;
  - a case without its name or a condition, a condition that is not `name=value`, a condition given twice in a case, and a case given twice;
  - a length that is not a decimal number, or that the body does not assemble to.
- **A line of bytes:**
  - no comment;
  - no heading above it;
  - a word that is not hex digits;
  - hex digits of an odd count;
  - quoted text without its closing quote, without a character, or with a character beyond printable ASCII;
  - a repeat without its item or count, or with a count that is not a decimal number of at least 1;
  - more than 16 MiB in all.
- **A heading** without text, a body without bytes, and a fixture of the target `none` that marks no region.
- **A directive:**
  - one other than `@seed` and `@end`;
  - an `@end` with a word after it;
  - a region opened inside another;
  - an `@end` without a region;
  - a region without bytes, or still open at the end;
  - a region routed to the target `none`;
  - a `name=value` word that is not a route member.

## 3. Two sample fixtures

**A toy file whose packet is a codec header.** The toy format and its codec are the drafts of the I/O spike's README, sections 2 and 3, which G3.1 accepted as the start of task 5. The region marks the packet's payload as a seed of the codec target, with the context that the container knows (section 6.2). The two cases probe the same bytes with reads cut short, which the library joins, and with a header limit smaller than the packet.

```text
description: one stream in packet mode whose header, a setup and a parameters unit, fills one packet
source: the draft of the toy format and its codec, spikes/input/README.md sections 2 and 3
target: toy
outcome: report
length: 48
# Reads that end at byte 40, inside the codec header that starts at byte 36, and every 8 bytes after give what
# reads that are not cut give, since the library joins short reads before its parsers see them; a header split
# across deliveries to the codec is toy/header-across-packets. A header limit of 8 bytes is smaller than the
# packet's 12.
case: short-reads pieces=40,8
case: small-header-limit limit.header_bytes_per_stream=8
---
## Toy format (spikes/input/README.md section 2): the file header
"CCToy" 00              # signature, "CCToy" and a zero byte
01                      # version 1
00                      # flags: none is defined
00 00 00 0c             # offset of the first element: 12, right after the header

## Toy format (section 2): the stream declaration, element STRM
"STRM"                  # code
00 00 00 07             # length of the body: 7
01                      # stream number 1
00                      # framing 0: packet mode
00                      # representation 0: units with one-byte lengths
"TCOD"                  # codec code: the toy codec

## Toy format (section 2): a packet, element PACK
"PACK"                  # code
00 00 00 0d             # length of the body: 13
01                      # stream number 1
# The packet's payload is the codec header, so it is a seed of the codec target too: in packet mode (framing 0),
# with the packet's known length, 12, as the container knows them.
@seed target=toy_codec representation=one_byte_lengths context=000000000c outcome=parsed

## Toy codec (spikes/input/README.md section 3): the setup, kind 1
01                      # kind 1: setup
06                      # length 6
01 40                   # width 320
00 f0                   # height 240
01                      # parameter count 1
00                      # flags: none, so the parameters can be read

## Toy codec (section 3): the parameters, kind 2
02                      # kind 2: parameters
02                      # length 2: the setup's one 16-bit parameter
00 07                   # parameter 7
@end
```

**A worked example of a specification.** RFC 9639, Appendix D.1, decodes a FLAC file of 57 bytes table by table. The fixture transcribes the tables, field by field. Its test compares the bytes with the RFC's own dump in D.1.1, transcribed separately as the RFC prints it. The two transcriptions agree, which gives expected bytes that do not originate in this project (plan section 9.1). The streaminfo block, with its header, is the codec header that a container carries apart from the native file's signature. So the region marks it as a seed of the FLAC codec target of Phase 4, beside the native file's target of Phase 9 (section 6.1).

```text
description: the FLAC file of two channels of one sample each that RFC 9639 decodes step by step
source: RFC 9639, Appendix D.1, "Decoding Example 1", from its tables 27 to 31 and its text
target: flac
outcome: report
length: 57
---
## RFC 9639 section 6, and Appendix D.1.3, table 27: the signature
"fLaC"                  # signature 0x664C6143

## RFC 9639 section 8.1, and Appendix D.1.3, table 27: the metadata block header
# The metadata block, its header and the streaminfo, is the codec header that a container carries apart from the
# native file's signature, so it is a seed of the FLAC codec target too, of Phase 4, in the representation of
# metadata blocks.
@seed target=flac_codec representation=metadata_blocks outcome=parsed
80                      # last metadata block 1; block type 0, streaminfo
00 00 22                # length of the block: 34

## RFC 9639 section 8.2, and Appendix D.1.3, table 28: streaminfo
10 00                   # minimum block size: 4096
10 00                   # maximum block size: 4096
00 00 0f                # minimum frame size: 15
00 00 0f                # maximum frame size: 15
0a c4                   # sample rate, its first 16 of 20 bits: 0x0ac44 is 44100
42                      # sample rate, its last 4 bits 0b0100; channels - 1, 3 bits 0b001: 2; bit depth - 1, first bit 0b0
f0                      # bit depth - 1, its last 4 bits: 0b01111 is 15, so 16 bits; total samples, first 4 of 36 bits
00 00 00 01             # total samples, the last 32 bits: 1
3e 84 b4 18 07 dc 69 03 # MD5 checksum of the unencoded audio, first 8 of 16 bytes
07 58 6a 3d ad 1a 2e 0f # MD5 checksum, last 8 bytes
@end

## RFC 9639 section 9.1, and Appendix D.1.4, table 29: the frame header
ff f8                   # frame sync, 15 bits; blocking strategy 0: fixed block size
69                      # block size bits 0b0110: 8-bit value below (9.1.1); sample rate bits 0b1001: 44.1 kHz (9.1.2)
18                      # channels bits 0b0001: stereo, no decorrelation (9.1.3); bit depth bits 0b100: 16 (9.1.4); 0
00                      # coded number (9.1.5): frame number 0
00                      # uncommon block size - 1 (9.1.6): 0, a block of 1
bf                      # frame header CRC-8 (9.1.8)

## RFC 9639 section 9.2, and Appendix D.1.4, table 30: the first subframe
03                      # subframe header (9.2.1): 0; type 0b000001, verbatim (9.2.4); wasted bits flag 1 (9.2.2)
58 fd                   # wasted bits unary 0b01: 2; sample 0b011000 11111101: 6397 in 14 bits

## RFC 9639 section 9.2, and Appendix D.1.4, table 31: the second subframe
03                      # subframe header (9.2.1): 0; type 0b000001, verbatim; wasted bits flag 1
12 8b                   # wasted bits unary 0b0001: 4; sample 0b0010 10001011: 651 in 12 bits

## RFC 9639 section 9.3, and Appendix D.1.4: the frame footer
# The appendix's text says that the frame CRC starts at 0x38, but its dump in D.1.1 and its own table 31, whose
# second subframe ends with byte 0x36, put it at 0x37; this fixture follows the bytes. No erratum covers it.
aa 9a                   # frame CRC-16, at 0x37
```

**A header across packets.** The spike holds a third toy fixture, `toy/header-across-packets`: one stream in stream mode whose setup is the first packet's payload and whose parameters are the second's. The split lies in the bytes, so the codec gets the header in two deliveries whatever the reads, and parses the second only with the continuation of the first. Its test also removes the first packet, which leaves the parameters malformed, as a lost continuation would. The first draft's case `split-header pieces=40,8` cut the reads instead, which the library may join before the codec sees them, so it established no split at the codec, as the third review found.

Writing the second sample found that the appendix's text puts the frame CRC one byte later than its own dump and tables do. On 2026-11-27 the RFC Editor lists no erratum for it, only 8256 and 8810, both verified, and 9096, rejected. The fixture follows the bytes, which the formal sections decide, and says so in a comment.

## 4. Expectations

### 4.1 The files beside a fixture

Plan section 9.1 stores the expected results beside each fixture and compares them exactly; section 9.2 splits them into a semantic part and an adapter part. As the maintainer chose, both are in the library's JSON form ([`json-output.md`](../guide/json-output.md)), the form that the CLI writes and the strict reader checks, so the comparison sees every member and whether it is present. Each case of section 2.3 has its own files, named with the case between the fixture's name and the part; the default case's files have no case name:

| File | Holds | Compared with |
|---|---|---|
| `<name>[.<case>].expected.json` | The semantic part of the report, or of the error record, as the seekable adapters give it | Every seekable adapter's semantic part: file, `Read + Seek`, bytes, custom positional source and async seekable |
| `<name>[.<case>].forward.json` | The semantic part through a forward-only adapter, only where it differs from the seekable one | The blocking and async forward-only adapters; when it would equal `expected`, the file must not exist |
| `<name>[.<case>].outcomes.json` | One member per adapter, each the semantic part of that adapter's result, only in a case with `outcomes=per_adapter`, or a default case whose `default:` line says so (section 2.3) | That adapter's semantic part only; such a case has no `expected` and no `forward` file |
| `<name>[.<case>].adapters.json` | One member per adapter, each the adapter part of that adapter's result | That adapter's result only, never across adapters |

A test probes the fixture's bytes in each case through each adapter, checks each adapter's usage and reads against the case's limits (below) and its primary input's path (section 4.2), splits each result (section 4.2), checks that the semantic parts of a class agree, unless the case keeps them per adapter, and compares each part with its file. A file of a fixture that no case produces any more fails the test, or is removed when expectations are written.

**Outcomes at a limit.** Parity holds where every adapter has what the probe needs. A limit can stop adapters differently while each honors it. The fourth review ran the accepted I/O spike with a working memory of 0 on bytes that are no toy file. The probe of bytes in memory, which reads in place, decided `Unknown` with no limit exhausted. The seekable reader, whose cache's first block needs memory, and the forward-only reader gave `InsufficientData` with the working memory exhausted. The fifth review found the same under the default limits. On a toy file of 675,504 bytes whose title references point back across ten cache blocks in turn, bytes in memory gave all 1,269 titles from 31,665 bytes read. The seekable reader read a block of 64 KiB for each reference and stopped at 1,024 titles with the read limit of 64 MiB exhausted. A shared expectation would call either a regression. As the maintainer chose, such a case says `outcomes=per_adapter`, and the default case says it on a `default:` line of its header (section 2.3):
- its file `<name>[.<case>].outcomes.json` keeps each adapter's semantic part, compared with that adapter's alone, and the adapter parts stay in `adapters`;
- a limit explains each difference, as the maintainer chose at the fifth review: in each class, the adapters that exhausted no limit, whose record's `exhausted` is empty, share one part. So two adapters that differ while neither exhausted a limit fail the case, whatever a third exhausted, while adapters that each exhausted a limit may stop at different points. The fourth revision took any `limit.` condition for evidence, and needed one, so it accepted two seekable adapters that reported two formats with no limit exhausted, and could not take the default case;
- a case whose classes all agree fails, since `expected` and `forward` would do, as FileCheck refuses a check prefix that no line uses ([LLVM, FileCheck, `--allow-unused-prefixes`](https://llvm.org/docs/CommandGuide/FileCheck.html)), and a difference between the classes alone is `forward`'s;
- a case that no seekable adapter runs fails by a message of its own, never as a difference; the fourth revision wrote an empty `outcomes` file for it.

Every other case keeps the shared parts, and so does the default case without its line. The seed check reads the outcome of the bytes adapter, which the `probe` target runs: from `expected`, or from the `bytes` member of `outcomes` (section 6.4). In Phase 2 every adapter gives the same error, so the samples have no such case; task 7 adds the first, under a working memory of 0.

**The usage within the limits.** Before anything is compared or written, each adapter's record is checked, so that an outcome of its own never hides a limit that an adapter passed, and writing expectations never accepts one, as the fifth review asked. The record is checked as the library wrote it, before its paths are netted (section 4.2), against the limits that the adapter was given: the case's, and the file adapters' with the retained-data limit raised by their file's path. The fifth revision netted first and then held the file adapter to the case's limit. So a file probe passed the check when it overran its raised limit by less than the charge of a built-in companion's path, which the limit does not raise. Under a case's limit of 128 with two paths of 18 bytes, a probe limited to 146 that reported 147 was netted to 111 and passed, as the sixth review found:
- its usage is read by the library's strict reader, through `Usage`, so each counter has exactly one valid form, its number or beyond 2^53 − 1 its twin, and a missing usage, a missing counter or a malformed twin fails. The fourth revision skipped a counter that it could not read, and took the number where both forms stood;
- its usage, and an error's partial report's, keep every counter within the adapter's limit. An error's usage covers its partial report's bytes read (`api.md` section 6.14), but nothing ties their peaks of working memory, so the fourth revision passed a partial report's peak of 3 under a limit of 0. A partial report without a usage fails, which the fifth revision passed as no partial report at all, as the sixth review found;
- where the test counts an adapter's reads, the bytes delivered stay within the read limit and equal the usage's bytes read, which counts every byte that a source delivers (plan sections 7.5 and 9.4). The counters cover every source of a probe that the test hands the library: the primary input, and from Phase 14 each companion of a custom resolver, while the file adapter's reads join with task 6's counter. The fourth revision copied `bytes_delivered` into the adapter part unchecked, so an overread that the usage under-reported would have been written.

**The same conditions for every adapter.** A file name or a resolver can change what a probe reports. A VobSub index, for one, asks for its data file by its own stem, which a probe from memory knows only from a hint. So each adapter of a run starts from the one prober of section 6.3, `fuzzing_entry_points::prober`, and gets the same conditions:
- the fixture's file name (section 2.1): the file adapter's file has that name, and every other adapter gets it as the file-name hint;
- the case's limits, with the file adapters' retained-data limit raised by their path's charge (section 4.2);
- the case's lookup: off by default, as the API's is, so that no request is made and the companions are "not enabled"; or, for `lookup=custom`, resolvers that answer as the case says, through `CompanionLookup::custom` for the blocking adapters and `AsyncCompanionLookup::custom` for the async ones, so that no adapter's result depends on the folder of a file.

A hint-only case gives its hint to every adapter but the file adapters, whose file has a name that every platform can write, and a nameless case gives them none. A case with `lookup=built_in` and the fixture's file name depends on the file's folder by its nature, and only the blocking `probe_file` gives the built-in resolver a folder (`api.md` section 13). It runs through that adapter alone, with each found companion written beside the file, and its expected part is that adapter's semantic part. With a hint or none, the case runs through every other blocking adapter, none of which has a folder: with a hint, the built-in resolver declines each request itself, and without a name no request is made and the companions are cut short with `CompanionNameUnavailable` (`api.md` section 13). The third review found both outcomes out of reach, since every adapter but the file had a hint and built-in lookup ran only through the file. The fourth review found that the third draft ran such a case through the async adapters too and expected the decline there. An async probe reads only the async option, which has no built-in resolver, so it makes no request, and its companions are "not enabled". So the async adapters run no case of built-in lookup, and a library test of Phase 14, task 1, checks their outcome (section 2.3).

**Names that are not Unicode** are a platform's own: bytes that are not UTF-8 on Unix, which a Mac's file system does not hold as a file name, and 16-bit units with an unpaired surrogate on Windows. Fixture cases run alike on every platform, so these names are not fixture cases, as the maintainer chose at the second review. Platform-specific tests of the library, under `cfg(unix)` and `cfg(windows)`, give such a name as the hint or as a path, as the stem rule's tests of plan section 7.7 do.

**The reads of every adapter.** The adapter part adds the reads that a counter saw: `read_calls` and `bytes_delivered`. The test kit's counting reader counts them under the `Read + Seek` and forward-only adapters, and the test's own custom source counts its own. The file adapter opens its file itself, so the second repository-only entry point of section 7 counts its reads, and those of each companion that the built-in resolver opens. Every adapter but bytes in memory then has both counts, as plan section 9.4 asks, so a change that reads a file more often fails its expectation.

### 4.2 The split

The adapter part holds what legitimately differs between adapters (plan section 9.2), keyed by its JSON pointer in the record (RFC 6901), so that each value keeps its place. The split takes members only at the places that the form defines. A typed detail of a newer version may keep members of any name, among them an object named `usage`, and the split leaves those in the semantic part:

| Where | Members that go to the adapter part |
|---|---|
| A root record (report, error record, detection) and an error's partial report | `size` and `size_exact`, since a source states the size or not; the `usage`'s `bytes_read` and `peak_working_memory`, with their twins `_exact`; and the source entries' members below |
| Each entry of the source table of such a record, and of an error's observations | `bytes_read` with its twin; `path`, `path_lossy` and `path_raw` |

Everything else is semantic. That includes the usage's other counters: `work_units`, `retained_data`, `retained_payloads`, `decompressed_output`, `companion_requests` and `sources_opened`. It includes `attempted_identity`, which every adapter now gives alike, since each gets the same file name. A count that is found to differ between adapters joins the table by a reviewed change of this document.

**Retained data, net of paths.** The plan charges retained data by capacity (plan section 7.5). A file probe keeps its path, which a probe from memory does not have, and the path's length depends on the folder that the test wrote the file into. As the maintainer chose, `retained_data` stays semantic, by one rule for the library and the test that the maintainer chose at the second review:
- **The charge.** A path is charged at the strings that the report keeps for it: its display string and, for a lossy path, its raw form's data, the hexadecimal digits of `api.md` section 6.15. The library holds each at exactly its length, as a boxed string holds it, so that the capacity charged is that length. The primary input's entry, with its path, is the probe's first charge to retained data. So the lossy Unix path of the bytes `/f`, `ff` and `/a`, 5 bytes as `OsStr::len` counts them, is charged 17: 7 for its display string `/f�/a` and 10 for its raw form's data `2f66ff2f61`.
- **A path beyond the limit.** As the maintainer chose at the third review, a path that the retained-data limit cannot hold is left out: its entry stays without the path, the limit is exhausted without a part that it stopped, as for a diagnostic that does not fit, and the probe goes on under the same limit (`api.md` sections 5.3 and 6.15). So a file probe under a limit one byte below its path's charge reports its file without the path, and one exactly at the charge keeps the path and nothing else that it would retain. Task 7 tests both at the caller's own limit, which the golden tests never meet, since they raise it.
- **An equal remaining budget.** The test raises the file adapters' retained-data limit by the charge of the probed file's path. The rest of the probe then has exactly the budget of a probe from memory, so a limit that leaves out a field leaves it out in every adapter alike. Subtracting the charge afterwards, as the first draft did, could not restore a field that the path's charge had crowded out. A companion that built-in lookup opens is charged within the case's limit, as every companion is; its case runs through the file adapter alone (section 4.1), so no other adapter's budget is to match.
- **The path kept.** The raised limit always holds the probed file's path, the probe's first charge, so a file adapter's record lists its primary input with exactly that path, written as its form gives it, wherever the record has a source table: a report's, and an error's in its observations and in its partial report where they stand. Every other adapter is given no path, since a name hint is never one (`api.md` section 4.3), so its primary input lists none. The test checks both after the usage and before the netting (section 4.1). The sixth revision checked the usage alone. So a file report under a raised limit of 146 that left out its path of 18 bytes and retained 146 passed, with the whole raised limit for the rest of the probe, and the comparison of the file adapter's part took the path's absence as written, as the seventh review found.
- **Net of paths.** Before the split, the test takes out of each record's `retained_data` the charge of each path that the test handed the library and the record lists: the probed file, and each companion that the built-in resolver opened. An error's usage covers its partial report's (`api.md` section 6.14), so a path that the partial report lists comes out of both, as the third review found. The figure is read from its number or its twin and written net in its canonical form, a number up to 2^53 − 1 and its twin beyond, as the fifth review found a twin left gross and then refused at its limit. Any other charge leaves the file adapter's figure apart from the others', so the parity check enforces the rule. The netting comes after the check of the record against the adapter's limits (section 4.1), so it serves the comparison alone and never the limit.
- **A fixed length.** As the maintainer chose at the third review, each case of each fixture writes its files into a folder of its own of 512 bytes below Cargo's folder for integration tests, `CARGO_TARGET_TMPDIR`: `expectations`, then a component for the run, the process's identifier, and one for the case, its number in the process, each of 8 hexadecimal digits, then components of `_` of at most 255 bytes each up to the length. No name of a fixture or a case counts, so any valid name fits, and two runs that share a target folder never share a case's folder or remove each other's files. The test requires the folder to be valid Unicode, and a target folder too long to leave room fails with a message that names `CARGO_TARGET_DIR`. A file name of 255 bytes below the folder stays under the 1,024 bytes that macOS allows a path ([Apple's `sys/syslimits.h`, `PATH_MAX`](https://github.com/apple-oss-distributions/xnu/blob/main/bsd/sys/syslimits.h)), and the standard library gives Windows the long form of a path beyond 260 characters by itself ([rust-lang/rust#89174](https://github.com/rust-lang/rust/pull/89174)). Every path below the folder, a built-in companion's among them, therefore has the same charge on every machine. So a built-in case that meets its limit, which has no other adapter to agree with, gives the same result everywhere. The second draft's folder of 200 bytes, named after the fixture and the case, refused a valid fixture name of 155 characters and every case of a long checkout, as the third review found.

**Paths with a placeholder.** Before the split, a path below the test's folder, by its components, must be one that the test handed the library, written exactly as its form gives it: its display string, with the lossy flag and the raw form only for a lossy path. It is then written as `{folder}/` and the rest, with `/` as the separator, as the expectation stores it. The display string, the lossy flag and the raw form all collapse into that one Unicode path, so no expectation holds the machine's folder, whatever its own form. The second draft matched the folder as a string prefix, so `/tmp/fixture2/file` became `{folder}2/file` beside a folder `/tmp/fixture`, and removed the flag and the raw form unchecked, so a serialization that marked a Unicode path as lossy passed, as the third review found. The fixture's part of the path is ASCII by the rule of section 2.1. The folder's run and case components are hidden by the placeholder, so no expectation holds the process or the run.

**Native details.** An error record's `operating_system_error_code` and `cause_messages` come from the operating system, so the same missing companion gives other codes and messages on Linux and on Windows. The semantic part keeps them as the placeholder `{native}`: the code's presence, and the number of messages, are compared, and their text is not. The error's kind, `source_failure_kind` and its source are compared as they are. A test that needs a code checks it where it chose the error itself, through the faulting reader of section 5.

### 4.3 Writing expectations

An expectation is written only by an explicit action, whose diff is then reviewed (plan section 9.1). With the environment variable `CC_MEDIA_PROBE_WRITE_EXPECTATIONS` set, the golden tests write each part instead of comparing it, and remove a file that no case needs any more. Without it, a missing, different or stale file fails the test, and the message names the variable.

**One feature set.** As the maintainer chose at the sixth review, expectations are compared and written under every feature of the library alone. The golden tests and the seed check, which runs their pass first (section 6.4), form one test target of the library whose `required-features` name each of its features: `serde`, `tokio` and `futures-io`, and `zlib` once task 14 declares it. The `serialization` target names `serde` the same way. Cargo builds and runs a target only when its required features are on ([The Cargo Book, "Cargo Targets", `required-features`](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-required-features-field)). Every test run of `node scripts/check.ts` and of CI, the declared `rust-version`'s and the coverage run among them, passes `--all-features`. A run with fewer features skips the target without a word, and `cargo test --test` with its name refuses and names the missing features, as a run on 2026-11-28 showed. A feature that the library gains joins the list in the change that declares it. Without the rule, the maps keyed by adapter, `outcomes` and `adapters`, would lack the async adapters' members in a run without their features and lose them when written there. A run without `zlib` would give a compressed header's semantic part as "compression unavailable" (plan Phase 5), even in `expected`. The sixth review found the protocol silent on which run may write.

**Without `zlib`.** The golden target is skipped without `zlib`, so it never sees what a build without the feature gives: compressed codec private data whose status is "compression unavailable" (plan Phase 5), and a compressed movie box that leaves no stream to enumerate (plan Phase 6). As the maintainer chose at the seventh review, from task 14, which declares the feature, `node scripts/check.ts` has a step that lints and tests the library alone under every feature but `zlib`: `cargo clippy` with `--all-targets` and `cargo test`, each with `-p cc-media-probe --no-default-features --features serde,tokio,futures-io`. CI runs it on Linux, macOS and Windows, as it runs the script. The tests of the disabled behavior are gated on `not(feature = "zlib")`, read a fixture's bytes, and assert the reported status by hand. No expectation file is written or compared there, since Cargo skips the golden target. Every other test of the library runs too, so a test that silently needs `zlib` fails there.
- **The library alone.** Cargo turns off the default features of the selected packages alone ([The Cargo Book, "Features", command-line feature options](https://doc.rust-lang.org/cargo/reference/features.html#command-line-feature-options)), and a selected package that depends on the library with its default features turns `zlib` back on. In a scratch workspace on 2026-11-28, `--workspace --no-default-features` beside a member that depends on the library, as the CLI will, ran the golden target and none of the gated tests, while `-p` with the library ran the gated tests and skipped the golden target. So the step selects the library alone, and no development dependency of the library depends on it with its default features; the test kit depends on no library at all (plan section 7.1).
- **What the other runs miss.** Clippy and coverage run under every feature, so neither sees code that only a build without `zlib` compiles: the step lints and tests it, and coverage leaves it out of its count. CI's feature-combination job builds every combination and runs none.
- The sixth revision left the disabled behavior without a run, as the seventh review found.

A file is parsed and compared with the value as a value, so the order of its members and its white space never count. This holds whichever order `serde_json::Map` keeps: its default sorts members, and its `preserve_order` feature keeps them as inserted. A written file has its members sorted by an explicit sort. The variable is no compiler flag, so it leaves the repository marker alone.

### 4.4 The values that a recipe states

A generated file's bytes depend on the tool's version, so its test asserts only what its recipe fixes and the specifications derive from it (plan section 9.1). The recipe states those values in a member `expected` of the file's entry, which the generator already ignores and leaves out of the file's identity (test-files guide, section 3). `expected` is a partial report in the library's JSON form, matched with the probe's report:

- every member that it names holds that value; a member that it does not name is not compared;
- a member that it names as `null` is absent, since the form never writes null;
- an array has exactly as many entries as `expected` lists, each matched in order;
- a number, string or flag is equal to the report's.

Each difference is reported by its JSON pointer, with each member's name escaped as RFC 6901 escapes it, such as `/streams/0/video/coded_size/width: expected 640, found 320`. The differences are listed in the order of their pointers: an object's members by name, and an array's entries by index, so `/1` comes before `/2` and `/10`. Sorting the messages instead, as the second draft did, compares the text after each pointer too and put `/10` before `/1`, as the third review found. The test-files guide, section 3, shows a recipe with its `expected` member.

## 5. Builders, readers and paths

The test kit produces bytes and standard-library readers only, and never depends on the library (plan section 7.1). Its interface, as proposed for tasks 4 and 19:

- **`ByteWriter`.** Appends in the orders that specifications state, and returns itself so that calls chain:
  - `byte`, `bytes` and `text`;
  - `big_endian_u16`, `big_endian_u24`, `big_endian_u32` and `big_endian_u64`, and their `little_endian_` twins;
  - `four_character_code`;
  - `as_bytes` and `into_bytes`.
- **`BitWriter`.** Writes bits most significant first or least significant first:
  - `bits(value, count)` and `flag`;
  - `unsigned_exponential_golomb` and `signed_exponential_golomb`;
  - `align_with_zeros` and `align_with_ones`;
  - `into_bytes`, which refuses an unaligned end.

  `insert_emulation_prevention` turns a NAL unit's payload into its escaped bytes.
- **Container builders,** one per family, as the formats arrive: EBML elements, boxes, RIFF chunks, Ogg pages and transport packets. Each builds a tree of values, and computes each size, length, offset and checksum at `build()`. Each computed field has a `with_…` override that writes a wrong value on purpose, such as `ToyElement::with_length(9)`. `build()` returns `Result<Vec<u8>, BuildError>`, since a computed field can overflow its width.
- **The toy format's builder,** `ToyFile`, takes over the I/O spike's: streams, packets, titles, lists, references, companions and stray bytes, with the first element's offset and every length computed.
- **Readers** over any standard-library reader:
  - `CountingReader` counts read calls, bytes delivered and seeks, and its `Counts` count the reads of a custom source too;
  - `FaultingReader` gives short reads that end at the fixed bytes of a case's pieces (`Pieces`, which a custom source applies at the positions of its own reads), an error of a chosen kind at a chosen offset, an error that wraps any Rust error, or an interruption once.

  Async pending readers implement the library's traits or a runtime's, so they live in the library's `tests/common/` (plan section 7.1), with the case's custom resolvers, blocking and async, from Phase 14 on.
- **Paths.**
  - `Fixture::load("flac/decoding-example-1")` reads and assembles a fixture from the test kit's own manifest folder, and refuses a name that the layout does not allow. `Fixture::file_name()` gives its file name (section 2.1).
  - `fixture::names()` lists every fixture and `fixture::layout()` every expectation file too, each after checking the layout of section 2. `expectation_path(name, case, part)` gives the path of a part.
  - `generated::path("containers/matroska/h264-opus.mkv")` gives a generated test file below `target/test-files/`. It fails with a message that names `node scripts/generate-test-files.ts` when the file is missing, and never skips (test-files guide, section 6).

## 6. Fuzz seeds and the hidden access path

### 6.1 Routing fields

A seed's route is the fixture header's `target`, `representation`, `context` and `outcome` for the whole fixture. For each region that the fixture marks, the same members are the `name=value` words of its `@seed` directive. A region covers the bytes between its `@seed` and its `@end`, such as the codec header that a container embeds (plan section 9.5).

The test kit gives each fixture's seeds by name: the whole fixture first, unless its target is `none`, then each region with `#` and its number, such as `toy/setup-and-parameters#1`. No fixture's name holds `#`, so no seed's name is another's. The test kit knows no target, so it reads the names and never interprets them.

- **Target names.** A container target is named after its format, such as `toy`, `matroska` or `flac`, and a codec target after its codec, such as `h264`. Where the two would be the same name, the codec target takes `_codec`: `toy_codec`, and `flac_codec` for the FLAC header parser of Phase 4, which takes metadata blocks, the first a streaminfo block, in the representation `metadata_blocks`. Phase 4 confirms that name and representation when it adds the target.
- **Later targets.** A target that a fixture names before the library has it is on the seed check's list of later targets, each with the phase that adds it: `flac` (Phase 9) and `flac_codec` (Phase 4) for the second sample. Every seed's bytes are held to the input bound of section 6.2 before a later target is let through, since no target's encoding is shorter than its bytes. The second draft let a region of a later target past every length check, as the third review found.
- **The full public probe.** Every fixture whose target is not `none` is also a seed of the `probe` target, with its file name, without a routing field of its own. Its outcome is the bytes adapter's in the default case's expectation, since the target probes bytes in memory: `report`, or the error record's kind. So, as the maintainer chose at the second review, the seed check refuses a whole fixture whose target is `probe`, which would give that seed twice with two outcomes. A region may route to `probe`, such as a file that another embeds, with the fixture's file name as its hint and its own outcome.
- **Detection.** A fixture or region routed to `detection` is a seed of detection, with the fixture's file name as its hint. So a fixture of bytes that match nothing, with `file_name: movie.mkv`, records the outcome `hint_only` (plan section 8.6).

### 6.2 Input encodings

Each target's input encoding is written once, in the library's `fuzzing_entry_points`. The targets decode it, and the seed check encodes it with the same function, so no table is copied between crates (R36):

| Target | Input |
|---|---|
| A container target, such as `toy`, `matroska` or `flac` | The file's bytes as they are |
| A codec target, such as `toy_codec` or `h264` | One byte, the representation's code in that target; then the context, of the length and layout that the target defines; then the bytes |
| `detection` and `probe` | One byte, the length of the file-name hint, 0 for none; then the hint's bytes; then the bytes |

- **The toy codec's context** is five bytes: the framing, 0 for packet mode and 1 for stream mode, and the packet's known length in 32 bits, big-endian. In packet mode the length is the bytes' own, which the entry point offers in one call as a whole packet, and in stream mode it is 0, the bytes offered as a stream to its end. Any other length is rejected before the parser, so that a seed cannot claim a packet of another length than it holds. The second draft parsed a claim of 100 bytes beside 8 that held a whole setup, as the third review found.
- **The hint's bytes.** A seed's hint is its fixture's file name in UTF-8. As the maintainer chose at the second review, on Unix, where the fuzz runs, the hint's bytes become the native name as they stand, through `OsStringExt::from_vec`, so a hint that is not UTF-8 reaches the library as the final component of such a path would. Elsewhere a native name is no string of bytes, so such a hint is rejected before any parser. The first draft read it with U+FFFD, which is valid Unicode and so never reached a name that is not.
- **Rejected before any parser:** a representation code or a context that its target does not define, a toy codec context whose length is not its bytes' in packet mode or not 0 in stream mode, a hint whose length runs beyond the input, and a hint that is not UTF-8 off Unix.
- **Names, not codes.** A target names its representations as fixtures do, so a fixture's route never holds a code.
- **The input bound.** An encoded input is at most 1 MiB, the fuzz run's `-max_len` of plan section 9.5. libFuzzer truncates a corpus file beyond it ([LLVM's `FuzzerFlags.def`](https://llvm.googlesource.com/libfuzzer/+/97ca7b08eada830925c9f7ba4fb68fedd9054c20/FuzzerFlags.def)), so a longer seed would never reach its target whole. The encoding refuses one, with its length, and the seed check fails on it. A fixture longer than that, such as a regression of 16 MiB, takes the target `none` and marks bounded regions as its seeds.

### 6.3 `fuzzing_entry_points`

As the maintainer chose, the module is compiled under `cfg(fuzzing)`, which cargo-fuzz passes to every crate that it builds, and in repository builds:

```rust
#[cfg(any(fuzzing, cc_media_probe_repository))]
#[doc(hidden)]
pub mod fuzzing_entry_points;
```

A published build has neither configuration name, so the module is absent there. The tarball job's consumer that names it fails to compile (plan section 9.7), and it is no part of the public API (`stability.md` section 2.1). The module holds:
- `Target`, with `from_name`, `name` and `parser`;
- `encode(target, representation, context, file_name, bytes)`, which refuses an input beyond the bound;
- `run(target, input)` and one function per target;
- `Reached`: the parser that an input reached, or none for an input rejected before any parser, with its outcome by name, and for a container each stream's header state;
- `prober(hint)`, the prober of the default case (below);
- `configuration()`, which tells a fuzz build from a repository build.

The private walkers and parsers that it reaches are compiled under the same condition when they exist only for repository builds and fuzzing, as `toy/` does (plan section 7.2).

**One prober.** `prober(hint)` gives the prober of the default case: `Prober::new()`, with its lookup off and the toy format added, and the hint where one is given. It adds the toy format through one private function, which `with_toy_format` of section 7 calls too, in every build that compiles the module, the repository build and the fuzz build alike. The `probe` and `detection` targets run its `probe_bytes` and `detect_bytes`, so the seed check does too, and the golden tests start each case's prober from it. A default case's expectation and the `probe` target's outcome therefore come from one setup in both builds. The first draft added the toy format in a fuzz build alone, so once the walker lands, the same bytes could have given `unknown_format` to the seed check and a report to the fuzzer, as the second review found.

cargo-fuzz passes `--cfg fuzzing` in `RUSTFLAGS`. Cargo takes `RUSTFLAGS` in place of `build.rustflags`, so a fuzz build has no repository marker. A fuzz build is therefore a third configuration, beside the repository build and the published one, and every gated item must compile in it without a warning. The spike's first fuzz build failed on an import that only `repository_entry_points` used. To avoid a field that a fuzz build never reads, a container's stream states are part of `Reached`.

### 6.4 The seed check, which writes the corpus

As the maintainer chose, the seed check is also the corpus tool, so the corpus holds only seeds that passed it:

- **The check** is a test of the library in `tests/`. For every fixture, it encodes each seed whose target the library has and runs it, and it asserts that the seed reaches the parser it was written for with the outcome that the fixture records. A target that the library does not have must be on the list of later targets. A seed rejected before its parser, or beyond the input bound, fails the check (plan section 9.5). The whole fixture's seed of `probe` takes its outcome from the default case's expectation, the bytes adapter's: `expected`, or the `bytes` member of `outcomes` where the default case keeps outcomes of each adapter, as the fifth review asked. So the check reads the expectations only after the golden tests' pass has run in the same process, through one function that runs the pass once: with `CC_MEDIA_PROBE_WRITE_EXPECTATIONS` set, it reads what the pass wrote, and a pass that fails fails the check too. The second draft ran the two as independent tests, so regenerating with a new fixture failed the check on a file that the writer had not written yet, as the third review found.
- **The corpus.** With the environment variable `CC_MEDIA_PROBE_WRITE_CORPUS` set, the check then writes every seed into `fuzz/corpus/<target>/`, which Git ignores. A whole fixture's file is `seed.<format>.<name>`, and a region's is `seed.<format>.<name>.<number>`. No fixture's name holds `.`, so no two seeds share a file, which the writer checks before it writes anything.
- **Regeneration.** A file that starts with `seed.` is the writer's. One that no seed writes any more, after a fixture is renamed, removed or routed elsewhere, is removed. Every other file is kept, among them the inputs that the fuzzer found, which libFuzzer names by their digest. So a local corpus after a rename starts from the same seeds as a clean checkout's. A corpus that does not exist yet is empty, and any other error of its listing fails the writer with its path, so that an unreadable folder never keeps stale seeds unseen.
- **Where it runs.** The scheduled fuzz workflow runs the seed check with the variable set before it runs the targets, and a developer does the same before a local run. The check is in the golden tests' target, which requires every feature of the library (section 4.3), so each of them runs it with `--all-features`: a run with fewer would skip it without a word and write no corpus, as the sixth review's finding on features implies.

**The seeds in the test kit, the corpus in the library's tests.** Plan section 7.1 placed the corpus tool in the test kit, which cannot call the library. The first draft proposed to split it, since the encodings are in `fuzzing_entry_points`. With the maintainer's choice of the seed check as the corpus tool, the routes and regions stay in the test kit (task 19), and the writing is the library's seed check (task 20). Revision 75 of the plan amends sections 7.1 and 9.5 and tasks 19 and 20 to match. The alternative that the choice set aside kept a tool in the test kit, with a copy of each target's representation codes, which the seed check would have checked.

### 6.5 The fuzz workspace

- A target's crate root only declares, as R30 asks of a binary's root: `#![no_main]` and `mod target;`. The `fuzz_target!` call sits in `target.rs`.
- The workspace keeps the root's lint table, `forbid(unsafe_code)` included. `fuzz_target!` writes a `no_mangle` function, but from another crate's macro, which the lint does not report.
- `fuzz/` has a `rust-toolchain.toml` of its own, naming a nightly of a stated date, raised deliberately as the root's stable release is (D1). The root's file names 1.100.0, which cargo-fuzz cannot use.
- The CI smoke test of plan section 9.5 builds and runs two targets: `toy_codec`, which reaches a private codec parser, and `probe`, which runs the full public probe.
- Every target that the seed check writes a corpus for has a binary. The whole toy fixtures route to `toy`, which walks a toy file, so the workspace has a `toy` target too, and task 20 adds it (plan Phase 3, task 20). The first two drafts wrote `fuzz/corpus/toy/` with no target to run it, as the third review found.
- The run's limits are those of plan section 9.5: `-timeout=5 -rss_limit_mb=1024 -max_len=1048576`.

## 7. Repository-only entry points

`repository_entry_points` is compiled under the repository marker alone, `#[doc(hidden)]`. Integration tests and benchmarks may use what it lists, and nothing else that is private (plan section 7.1):

| Entry point | For |
|---|---|
| `with_toy_format(prober: Prober) -> Prober` | Adds the toy format to the prober's detection, so that every method of `Prober` reaches the toy walker: the blocking ones, and those of the `tokio()`, `futures()` and `asynchronous()` handles. The budget tests, fault injection, the header-assembly cases and the toy benchmark run the public paths, as callers do. `fuzzing_entry_points::prober`, from which the golden tests start, adds the toy format through the same private function (section 6.3) |
| `with_source_counter(prober: Prober, counter: SourceCounter) -> Prober` | Counts the reads of every source that the library opens itself, the file of `probe_file` and `detect_file`, blocking and async, and each companion that the built-in resolver opens. It counts read calls, bytes delivered and seeks for each source identifier, inside the real adapter over the real file. `SourceCounter` is a cheap clone of one shared counter, so a test reads its counts after the probe. The adapter parts of section 4.1 and the budget tests of plan section 9.4 use it, as the maintainer chose |

Two entry points are enough. Every other internal that a test measures is tested inside `src/`, where unit tests compiled under the marker may use the test kit: each charge against the real buffer's capacity, the readers, and the parsers' outcomes. A later phase adds an entry point only by amending this list at its phase gate.

**Approved at the second review: the toy format's value.** A toy report needs a `Format`. `Format` gains a variant `Toy`, with the JSON name `toy`, `#[doc(hidden)]` and compiled only under `cfg(any(fuzzing, cc_media_probe_repository))` ([the Rust reference, "Conditional compilation"](https://doc.rust-lang.org/reference/conditional-compilation.html)). The toy walker can then report the format that it recognized in every build that has it, without `serde` too, since the variant's name needs no constructor of the text form. The ordinary published build has neither configuration name, so it omits the variant, and the API comparison, made on the unpacked package, never sees it (plan section 8.10). `Format::Unrecognized` keeps its meaning, a value that a newer version wrote. The alternative set aside reported the toy format through that catch-all, which needs its crate-private text constructor `from_text`, compiled only with `serde`, in repository and fuzz builds without it. The maintainer approved the variant at the second review, revision 76 of the plan amends section 7.2 to match, and task 5 builds it.

## 8. Choices made without asking

The maintainer approved these choices at the second review, and those of the next two lists at the fourth, with the rest of D14, apart from the one that the fourth review's first finding changed. Where a review's own choices changed one, it is given as changed, and section 10 says how:

- The extension `.hex`, the header's members, `---` as the header's end, `#` and `##`, and `@seed` and a bare `@end` with `name=value` words.
- Hex digits in either case; quoted text without escapes.
- A bound of 16 MiB on a body.
- An optional `length` that the assembler checks, and comments in the header.
- The layout check of section 2, which refuses anything that is not a format folder of fixtures and their expectations.
- `file_name`, its rule, now one that every platform can write, and the default of the fixture's own name without an extension.
- The target `none` for a fixture whose whole bytes are no seed.
- The conditions of a case and their values (section 2.3), and the default case's files without a case name.
- The files `<name>[.<case>].expected.json`, `.forward.json` and `.adapters.json` beside the fixture, a stale file refused or removed, and the adapter part keyed by JSON pointer, with `read_calls`, `bytes_delivered` and the `{folder}` placeholder.
- The members of the split and its places (section 4.2), `attempted_identity` in the semantic part, and the placeholder `{native}`.
- One folder below `CARGO_TARGET_TMPDIR` for each case of each fixture, now padded to a fixed length.
- `null` in a recipe's `expected` for an absent member, and the differences listed in the order of their pointers.
- The variable `CC_MEDIA_PROBE_WRITE_EXPECTATIONS`, and comparison as values with an explicit sort on writing.
- The input encodings of section 6.2, among them the hint's length byte, with the hint now read as native bytes on Unix.
- The names of seeds with `#`, and of corpus files with `seed.`; the variable `CC_MEDIA_PROBE_WRITE_CORPUS`.
- The target names: `_codec` only where a codec's name is its container's; `flac_codec` and `metadata_blocks` until Phase 4 confirms them.
- Every fixture with a target as a seed of `probe`, with the default case's expectation as its outcome, now beside a whole fixture's route to `probe` refused; detection seeded only by the fixtures and regions that route to it. Changed at the fifth review, as it asked: the bytes adapter's outcome in that expectation.
- The names `with_source_counter` and `SourceCounter`.
- The fuzz workspace's own `rust-toolchain.toml` with a dated nightly.
- The spike's deletion with task 20 (section 9).

Choices made since the second review, approved at the fourth:

- Device names that Windows reserves refused as the names of formats, fixtures and cases too, beside the file-name rule (section 2).
- The condition `hint`, percent-encoded, and the lookup names `off`, `custom` and `built_in` (section 2.3).
- A built-in companion's name held to the file-name rule, and apart from the file's and the other companions' by more than case (section 2.3).
- Each request answered by the case, and each answer asked for (section 2.3). Changed at the fifth review, as it asked: a request that built-in lookup without the file declines takes no answer, and each answer is asked for by at least one adapter.
- Conditions that no adapter can run together refused: an answer without a lookup, a custom resolver's answer beside built-in lookup, and a hint beside built-in lookup (section 2.3). Changed at the third review: a hint beside built-in lookup runs through the other adapters, and an answer there is refused.
- Task 6 noting the I/O spike's resolver results in the comments of the two-file fixtures, for Phase 14 (section 2.3).
- The folder's fixed length of 200 bytes, padded with `_`, and a folder for tests that is valid Unicode (section 4.2). Changed at the third review, as the maintainer chose: 512 bytes from components of a fixed width.
- `fuzzing_entry_points::prober` as the one prober of the default case, from which the golden tests start (section 6.3).
- A hint that is not UTF-8 rejected off Unix (section 6.2).

Choices made since the third review, approved at the fourth:

- `pieces` as cuts at fixed bytes from byte 0; the case `short-reads` in place of `split-header`; and the spike's stream fixture `toy/header-across-packets` for a header split across deliveries (sections 2.3 and 3).
- `hint=none` for a nameless case, with `%6Eone` for a hint spelled `none` (section 2.3).
- Built-in lookup without the file run through every other adapter, any answer refused there, and pieces refused beside it with the file (section 2.3). Changed at the fourth review, as the maintainer chose: through every other blocking adapter, with the async adapters' outcome left to a library test of Phase 14.
- A requested name percent-encoded as a hint is, each decoded name once; `declined:<reason>` with a reason that may be empty; tokens that may be empty; only the three limits that a resolver may name; and the I/O error kinds by their stable names (section 2.3).
- A fixture of the target `none` with at least one region (section 2.4).
- The rule for a path that the limit cannot hold for every path that a report keeps, a built-in companion's too (section 4.2).
- A path below the test's folder that is no handed path in its exact form failing the test (section 4.2).
- The run's component as the process identifier and the case's as its number in the process, 8 hexadecimal digits each, below `expectations` (section 4.2).
- The toy codec's context length 0 in stream mode (section 6.2).
- The golden pass run once in a process, through one function that the seed check calls first (section 6.4).
- A `toy` target in the fuzz workspace beside the two smoke targets (section 6.5).

Choices made since the fourth review, open at G3.2:

- Each adapter's usage held within the case's limits in every case, not only in a case of outcomes of each adapter, with each counter read as its number or its twin (section 4.1). Changed at the fifth review, as it asked: read strictly, the partial report's usage too, beside the measured reads.
- `shared` as the explicit default of `outcomes`, and the adapter parts of a case of outcomes of each adapter kept in `adapters` (sections 2.3 and 4.1).
- A class that no adapter of a case runs given no part, so that a case of built-in lookup with the file, through `probe_file` alone, has no `forward` file (section 4.1).
- Task 7 as the task of the first case of outcomes of each adapter (section 9).

Choices made since the fifth review, open at G3.2:

- The header member `default`, which the test kit reads as a case's words without a name, with `outcomes` the only condition that the library's tests take there (sections 2.1 and 2.3).
- A limit counted as exhausted when the record's `exhausted` lists it: a report's in its completeness, an error record's at its root (section 4.1).
- The bytes delivered equal to the usage's bytes read, beside the bound that the review asked for, by plan section 7.5's definition of the bytes read (section 4.1).
- A usage that is not the form's left as it is by the netting of paths, for the check of the limits to refuse with its place (section 4.2). Changed at the sixth review, as it asked: the check comes first, so no such usage reaches the netting.
- The seed check's choice between `expected` and `outcomes` by the default case's condition, not by the files present (section 6.4).
- A toy fixture whose title references use up a reader's read limit under the default limits, with `default: outcomes=per_adapter`, added by task 7 beside the case under a working memory of 0 (section 9).

Choices made since the sixth review, open at G3.2:

- A partial report checked wherever its member stands, so one of `null` fails as one without a usage does (section 4.1).
- The file adapters' retained-data limit raised by the probed file's path alone, a built-in companion's charged within the case's limit, as section 4.2 said; the spike summed every handed path, which held the primary input's alone before Phase 14 (section 4.2).
- A feature that the library gains joining the golden target's required features in the change that declares it, as `zlib` does in task 14 (section 4.3).
- The fuzz workflow running the seed check with `--all-features`, since the check is in the golden tests' target (section 6.4).

Choices made since the seventh review, open at G3.2:

- The check of the primary input's path held beyond the review's successful file report: in an error's observations and partial report too, and for every other adapter, whose primary input lists no path (section 4.2).
- The step's selection, `-p cc-media-probe --no-default-features` with every other feature named, and no development dependency of the library depending on it with its default features (section 4.3).
- The spike's tests of paths built on records that the library's strict reader reads, as its serialization tests build theirs, with a companion of a role of a later version, the twin's and the partial report's tests among them (section 10.7).

## 9. What happens after G3.2

- Task 4 writes the test kit's first part from sections 2, 4 and 5, starting from the spike's `kit/`. It moves the two samples into `crates/cc-media-probe-testkit/fixtures/`, `toy/` and `flac/`, the second for Phases 4 and 9. The golden tests start each case's prober from `fuzzing_entry_points::prober`, so task 4 adds the hidden module with that function alone, as the third review asked. Their test target requires every feature of the library (section 4.3), and task 20 adds the seed check to it. They check each source table's primary input against the path that its adapter was given (section 4.2).
- Task 5 writes the toy format's description, builder and fixtures, and the walker with `with_toy_format` and the hidden `Format::Toy`, and adds the toy format to task 4's `fuzzing_entry_points::prober` through the private function that `with_toy_format` calls. It writes the spike's `toy/header-across-packets` as a fixture of the toy format, for task 16's header split across deliveries. It retargets the toy fixture's `source` and headings, which cite `spikes/input/README.md`, to that description, since task 6 deletes the I/O spike with its README (plan Phase 3, task 5, revision 75).
- Task 6 adds `with_source_counter` with the I/O layer, so that the file adapter's parts gain their read counts. It keeps the I/O spike's two-file toy input as fixtures of their default case, for Phase 14, task 1, which adds their cases of custom and built-in lookup once the resolvers ship, and the library test of the async adapters' outcome under built-in lookup (section 2.3).
- Task 7 charges a path as section 4.2 says: its strings held at their length, the primary input's path first, and a path that the limit cannot hold left out of its entry, tested at a file probe's own limit one byte below its path's charge and exactly at it. It adds the first case of outcomes of each adapter, a toy fixture under a working memory of 0, once the block cache of task 6 and the working-memory accounting can stop adapters differently, and, as proposed, a toy fixture whose title references use up a reader's read limit under the default limits, whose `default:` line keeps outcomes of each adapter, as the fifth review found in the I/O spike (section 4.1).
- Task 14 declares `zlib` and joins it to the golden target's required features. It adds the step of `node scripts/check.ts` that lints and tests the library alone without the feature, with the first tests of the disabled behavior, and `CLAUDE.md` and `CONTRIBUTING.md` name the step (section 4.3).
- Task 19 adds the regions' routes to the test kit.
- Task 20 adds the rest of `fuzzing_entry_points`, the seed check that writes the corpus, and the `fuzz/` workspace with its two smoke targets and the toy-container target.
- The spike stays as the reference of tasks 4 and 20, and is deleted in the commit that completes task 20, as section 1.2 of the plan keeps a spike.
- The plan amendments. Revision 75 made those of the maintainer's choices at the first review: sections 7.1, 9.1, 9.2, 9.4 and 9.5, and Phase 3, tasks 5, 19 and 20. Revision 76 made those of the second review's approvals and choices: section 9.1, the section named in the heading, the file-name rule and the hint-only case; section 7.2, the `Toy` variant behind the two conditions; section 7.5, the charge of a path; sections 9.2 and 9.5, the lookup of a case, the equal remaining budget, the route to the probe target and the one prober; and Phase 3, tasks 3, 5 and 7, and Phase 14, task 1. Revision 77 made those of the third review's choices and direct asks: section 7.5, a path that the limit cannot hold; section 9.2, the folder of a fixed length from components of a fixed width, and built-in lookup without the file; section 9.5, the seed's bound before a later target, a fixture beyond it with a region, and the seed check after the golden pass; and Phase 3, tasks 3, 4, 5, 7 and 20. `api.md` sections 5.3, 6.15 and 16 state the path's rule. Revision 78 made those of the fourth review's approvals and choices: section 9.2, outcomes of each adapter at a limit and built-in lookup through blocking adapters alone; and Phase 3, tasks 3 and 7, and Phase 14, task 1. Revision 79 made those of the fifth review's choices and direct asks: section 9.2, outcomes of each adapter where a limit explains each difference, the default case's on a `default:` line, and the usage and reads checked before any expectation; section 9.5, the seed's outcome from the bytes adapter; and Phase 3, task 3, and Phase 14, task 1, the async test's outcome beside the primary input's entry. Revision 80 made those of the sixth review's choice and direct asks: section 9.2, the usage checked against the adapter's own limits before the paths are netted, a partial report's usage required, and the golden tests under every feature of the library; section 9.5, the seed check in their target; and Phase 3, tasks 3, 4 and 20, and Phase 14, task 1, the async test's assertions by the result's type. Revision 81 made those of the seventh review's choice and direct asks: section 9.2, the primary input's path and the run without `zlib`; section 9.8, that run among CI's; Phase 3, tasks 3, 4 and 14; and Phases 5 and 6, their tests without `zlib` in that run. No amendment waits on G3.2.

## 10. The reviews of the proposal

### 10.1 The first review

The maintainer reviewed the first draft and its spike on 2026-11-27 and kept G3.2 open: 2 findings of priority 1, 16 of priority 2 and 7 of priority 3. The review found no reason to reopen G3.1. "Reproduced" means that a test or a scratch run failed before the fix and passes after it. Each finding has a test in the spike, apart from the plan and document fixes.

| # | Finding | What changed | Where | Reproduced |
|---|---|---|---|---|
| 1 | P1: the file adapter had a file name and a folder that the other seekable adapters lacked, so a companion's name could differ by adapter | Every adapter gets the fixture's file name, as its file's name or as the hint, and the same custom resolver, with built-in lookup off; a built-in lookup case runs through the file adapters alone | 2.1, 4.1 | By reading |
| 2 | P1: a fixture had no way to select limits, a delivery schedule, companions or resolver responses, nor an expected result for each | `case:` lines with their conditions, each case with its own expectation files, as the maintainer chose | 2.3, 4.1 | By reading |
| 3 | P2: a file probe retains a path that a bytes probe lacks, and its charge depends on the folder | `retained_data` semantic, net of the paths that the test handed the library, as the maintainer chose | 4.2 | By reading |
| 4 | P2: an error record's native code and cause messages stayed in the common expectation | The placeholder `{native}`, with presence and count compared; the kind and source compared as they are | 4.2 | By reading |
| 5 | P2: the comparison serialized both values, so an order that `preserve_order` keeps failed it | Parsed values compared directly, an explicit sort on writing, and the recipe matcher's differences sorted by pointer | 4.3, 4.4 | Yes: the FLAC golden test with `preserve_order` |
| 6 | P2: the split walked every object, so a typed detail's own `usage` lost members | The split takes members only at the places that the form defines | 4.2 | Yes: a detail's `usage/bytes_read` moved |
| 7 | P2: only the display path got the placeholder, not `path_lossy` and `path_raw` | All three collapse into one Unicode path with the placeholder, and the folder no longer holds the process | 4.2 | By reading |
| 8 | P2: the file adapter's reads were never counted, as plan section 9.4 asks | `with_source_counter`, as the maintainer chose; `read_calls` and `bytes_delivered` for every adapter but bytes | 4.1, 7 | By reading |
| 9 | P2: the toy codec's reduced parser gave three outcomes that the accepted grammar does not | The spike's parser is the I/O spike's, ported whole: the packet-boundary rule, payload skipping in stream mode, and the needs-decoding flag | 3, spike | Yes: `malformed`, `no_header` and `parsed` for `needs_more_bytes`, `parsed` and `partially_parsed_by_design` |
| 10 | P2: a file of its header alone, or whose first element lies beyond it, was a walked `report` | The accepted detection rule: a known first element's code, and insufficient data while the bytes can still begin a toy file | 6.2, spike | Yes: both gave `report` |
| 11 | P2: the walker ignored the codec code and took every nonzero framing for stream mode | Only `TCOD` gets an assembler, another codec is unsupported, and only framings 0 and 1 declare a stream | spike | Yes: `XCOD` parsed, and framing 2 declared a stream |
| 12 | P2: the FLAC example's streaminfo block had no region for the codec target of Phase 4 | A region routed to `flac_codec`, and the target names of section 6.1 | 3, 6.1 | By reading |
| 13 | P2: the full public-probe target of the CI smoke test had no entry point, encoding or seeds | The `probe` target, every fixture its seed, and a second fuzz target in the spike | 6.1 to 6.5 | By reading |
| 14 | P2: a seed could exceed the fuzz run's 1 MiB input length, which libFuzzer truncates | The bound applied after encoding, and the target `none` with bounded regions for a longer fixture | 2.1, 6.2 | By reading |
| 15 | P2: a bytes-only detection target could not reach the outcome that only a hint gives | The detection and probe inputs carry an optional hint | 6.1, 6.2 | By reading |
| 16 | P2: a region's seed and a fixture named like it shared a corpus file, and hyphens flattened names ambiguously | Seeds named with `#`, files with `.`, and the writer refuses two seeds of one file before it writes | 6.1, 6.4 | By reading |
| 17 | P2: the corpus writer skipped unknown targets and checked less than the seed check | The seed check writes the corpus, as the maintainer chose | 6.4 | By reading |
| 18 | P2: a renamed fixture left its old seeds in the corpus | The writer removes its own `seed.` files that no seed writes, and keeps every other | 6.4 | By reading |
| 19 | P3: `@end stray words` closed a region | A bare `@end`, and a word after it refused at its line | 2.2, 2.4 | Yes |
| 20 | P3: fixture discovery took files and folders of any shape, and a lossy name | The layout check of section 2, also in `Fixture::load` | 2, 5 | By reading |
| 21 | P3: the inventory tests required exactly the two samples | They require the samples among the fixtures and seeds | spike | By reading |
| 22 | P3: the recipe matcher wrote member names into pointers unescaped | Names escaped as RFC 6901 escapes them, by the helper that the split uses | 4.4 | Yes: `a/b` and `a`'s `b` gave one pointer |
| 23 | P3: the API checklist's C-HIDDEN row gave the old compilation condition | The row says `cfg(fuzzing)` and repository builds | `api-guidelines-checklist.md` | By reading |
| 24 | P3: the catch-all alternative cannot be built in a repository build with default features | The alternative says what it would compile | 7 | By reading |
| 25 | P3: the toy fixture would cite the I/O spike's README after task 6 deletes it, and the spike's code cited section 6 for section 7's list | Task 5 retargets the citations, and the spike's comment cites section 7 | 9 | By reading |

### 10.2 The second review

The maintainer reviewed revision 75 the same day. The review approved the specification and section named in a heading (section 2.2), the hidden `Format::Toy` (section 7) and the choices of section 8, and held the proposal as a whole on four contract issues, each fixed here. The maintainer settled three questions that the fixes raised (section 1). The review ran the fixture workspace's 44 tests and `git diff --cached --check`, both passing. "Reproduced" means as in section 10.1; the throwaway tests ran against a copy of the staged revision 75, outside the index.

| # | Finding | What changed | Where | Reproduced |
|---|---|---|---|---|
| 1 | Companion cases did not match the approved API: every default run had a custom resolver, whose unanswered request gave `NotFound` where the API's default makes no request and reports the companions "not enabled", and `lookup=built_in` ran through every file adapter, though only the blocking API offers built-in lookup | Lookup off by default; `lookup=custom` explicit, with blocking and async custom resolvers; `lookup=built_in` through the blocking `probe_file` alone; each request answered and each answer asked for; companion cases from Phase 14, with task 6 keeping the two-file input for it | 2.3, 4.1, 9 | By reading: the spike's golden test built no resolver at all, while the draft gave one to every run |
| 2 | Subtracting `OsStr::len` after the probe matched neither a lossy path's charge, whose display string and raw data exceed it, nor a file probe whose path had used up the budget that left a field out | A path charged at the strings that keep it, held at their length, first; the file adapters' limit raised by that charge; folders of a fixed length, as the maintainer chose | 4.2, plan 7.5 | Yes: a lossy Unix path of 5 bytes by `OsStr::len` keeps 17 in the report |
| 3 | The seed check ran the probe without the toy format, which the draft added in a fuzz build alone, and an explicit route to `probe` beside the automatic seed was undefined | One prober, `fuzzing_entry_points::prober`, for both targets that run the public API, the seed check and the golden tests, adding the toy format in both builds; a whole fixture's route to `probe` refused, a region's allowed, as the maintainer chose | 6.1, 6.3, 7 | By reading: the spike's probe target and golden test each built their own `Prober::new()` |
| 4 | The grammar allowed file names that Windows cannot hold, such as `NUL.idx` and `movie.`, and the fuzz hint's U+FFFD made valid Unicode of a name that is not | File names that every platform can write; `hint=` cases for the others; names that are not Unicode in platform-specific tests; the fuzz hint as native bytes on Unix and rejected elsewhere when not UTF-8, as the maintainer chose | 2.1, 2.3, 4.1, 6.2 | Yes: revision 75 parsed `NUL.idx`, `movie.` and `con` as file names and turned the hint's byte `ff` into U+FFFD |

Found while revising:

- **Device names in the layout.** A fixture or a format named `nul` or `com1` cannot exist on Windows either, since `nul.hex` names the device there. The layout check and `Fixture::load` refuse such names now (section 2). Reproduced: revision 75 took `nul` for a fixture's name.
- **A built-in companion's name** is written beside the file, so it follows the file-name rule too, and differs from the file's and the other companions' by more than case, since Windows and macOS compare names without regard to case by default.
- **A sign among hexadecimal digits.** `u8::from_str_radix` takes a leading `+`, so revision 75 read a token `+a` as the byte 10. The token and the hint's percent-encoding now check two hexadecimal digits. Reproduced.
- **No resolver before Phase 14.** Section 9 had task 6 add the cases' custom resolver, though no build of the library has one before Phase 14, task 1.
- **Boxed strings.** The library's `PortablePath` holds its strings as `String`, whose capacity can exceed its length. Task 7 holds them at their length, as the charge of section 4.2 needs.

### 10.3 The third review

The maintainer reviewed revision 76 on 2026-11-27 and kept G3.2 open: 15 findings of priority 2 and 4 of priority 3, with all 55 staged fixture tests passing and `git diff --cached --check` clean. Each holds. The maintainer chose the fixes of findings 1 and 6, and the other fixes follow the review's own recommendation. "Reproduced" means that a scratch test against a copy of the staged revision 76 failed, and that the spike's test of the fix passes.

| # | Finding | What changed | Where | Reproduced |
|---|---|---|---|---|
| 1 | P2: what a file probe gives when its retained-data limit cannot hold its path's charge was undefined, and the raised limit of the golden tests hid it | The path is left out and its entry stays, the limit exhausted with no part stopped, as the maintainer chose; plan section 7.5 and `api.md` sections 5.3 and 6.15 state it, and task 7 tests a file probe at its own limit one byte below the charge and exactly at it | 4.2, 9 | By reading: the Phase 2 library charges no path yet |
| 2 | P2: a partial report's path was taken out of the partial report's usage alone, though the error's usage covers it | A path that the partial report lists comes out of both | 4.2 | Yes: the error's figure stayed at 24 while the partial report's, net of its path of 22 bytes, went to 2 |
| 3 | P2: the toy walker needed a whole body before it entered a list or offered a packet, so the input's end lost what had arrived | A cut list is entered and its children before the end are walked; a cut packet's bytes are offered as a packet that has not ended; then the walk ends truncated, as the accepted walker does | spike | Yes: a cut list gave no stream, and a cut packet holding a whole header gave `not_found` |
| 4 | P2: a path's containment was a string prefix, and the flag and raw form of a lossy path were dropped unchecked | Containment by components, and a path below the folder must be a handed path in exactly its form | 4.2 | Yes: `/tmp/fixture2/file` became `{folder}2/file`, and a Unicode path marked lossy collapsed |
| 5 | P2: the documented regeneration ran the golden writer and the seed check as independent tests | The seed check runs the golden pass first, once in a process | 6.4 | Yes: with a new fixture and the variable set, the seed check failed on its missing `expected.json` |
| 6 | P2: the names of the fixture and the case, and the checkout's folder, counted toward the fixed length | 512 bytes from components of a fixed width for the run and the case, as the maintainer chose; a target folder too long fails with a message that names `CARGO_TARGET_DIR` | 4.2 | Yes: a fixture name of 155 characters, and in the scratch copy, whose path is long, the two samples too |
| 7 | P2: two runs that shared a target folder removed each other's input | The run's component, the process's identifier; the folder-length test uses no name | 4.2 | By reading |
| 8 | P2: `pieces=40,8` capped read calls, so a first read of 8 bytes used up the 40, and the library may join short reads, so the case established no split at the codec | Pieces cut at fixed bytes; the case renamed `short-reads` for what it does test; a stream fixture whose packets split the header, with a test that a lost continuation fails | 2.3, 3, 5 | Yes: reads of 8 and then 64 bytes delivered 8 and 8 |
| 9 | P3: the custom source narrowed a position to `usize` before comparing it with the length, which panics on a 32-bit host | It compares first, and gives no bytes at and beyond the end | spike | By reading; a test reads at `u64::MAX` |
| 10 | P3: the recipe matcher sorted its messages, so `/10` came before `/1` | Members by name and entries by index as it walks | 4.4 | Yes |
| 11 | P2: a requested name with a space or `=` could not be written, and duplicates were not checked after decoding | Requested names percent-encoded, each decoded name once | 2.3 | Yes: `Movie%20%282026%29.sub` stayed encoded |
| 12 | P2: no case reached the two outcomes of lookup without a path: no name, and built-in lookup without a folder | `hint=none` for a nameless case, and built-in lookup with a hint or none through every adapter but the file | 2.3, 4.1 | Yes: built-in lookup beside a hint was refused, and no case could withhold the hint |
| 13 | P2: the responses did not match the resolver's results | Each argument checked as the result takes it, `declined:` with its reason and an empty token among them | 2.3 | Yes: `limit_reached:streams`, `failed:whatever` and `found:` were accepted; `declined:` with a reason and an empty token refused |
| 14 | P3: pieces beside built-in lookup were accepted, though the file adapter's reads are not cut | Refused with the file | 2.3 | Yes |
| 15 | P2: no target ran `fuzz/corpus/toy/`, which the whole toy fixtures seed | A `toy` target in the spike and in task 20 | 6.5, 9 | By reading |
| 16 | P2: task 4's golden harness needed `fuzzing_entry_points::prober`, which task 20 added | Task 4 adds the module with the prober alone, task 5 the toy format, task 20 the rest | 9 | By reading |
| 17 | P2: a fixture of the target `none` without a region passed and gave no seed, and a region of a later target escaped every length check | A fixture of the target `none` marks a region, and every seed is held to the bound before its target is deferred | 2.4, 6.1 | Yes: no seed, and a region of 2 MB, both passed |
| 18 | P2: the toy codec target took any packet length that the context declared, so a seed could claim 100 bytes and hold 8 | The context's length is the bytes' own in packet mode and 0 in stream mode | 6.2 | Yes: the claim parsed |
| 19 | P3: the corpus writer dropped the errors of its listing | A corpus that does not exist yet is empty; any other error fails the writer with its path | 6.4 | Yes: an unreadable target folder kept its stale seed |

Found while revising:

- **The pieces' state.** Cutting at bytes makes `Pieces` a pure function of a position, so `FaultingReader` keeps its position across reads and seeks, and the custom source cuts at the positions that the library asks for.
- **The toy tests' stream inputs** stated their bytes' length as the context in stream mode; with finding 18's rule they state 0.
- **The renamed case** kept its files' content, since the Phase 2 library reads nothing, so `short-reads` changed no expectation.
- **The strict reader and the path's rule.** `RetainedData` exhausted beside no omission and no stopped part is readable: the stub's record checks each omitted count against an exhausted limit, and nothing the other way, as for the diagnostics limit beside a complete report.
- **A file at the corpus's top.** The writer read every entry of the corpus as a target folder; it now skips an entry that is no folder, which an error had hidden before.

### 10.4 The fourth review

The maintainer reviewed revision 77, committed as `a88ceea`, on 2026-11-27, with all 68 fixture-spike tests passing. The review approved D19, with the hidden module of fuzz targets and the repository-only entry points, and the rest of D14, the fixture syntax, the seed routing and the revised rule of path accounting among it. It kept G3.2 open on two findings of the fixture protocol, each of which holds, and both fixes keep the approved public API. The maintainer chose both fixes.

| # | Finding | What changed | Where | Reproduced |
|---|---|---|---|---|
| 1 | Built-in lookup without the file ran through every adapter but the file and expected `Declined` from the async ones too, though an async probe reads only the async option, which has no built-in resolver: it makes no request and reports the companions "not enabled" | Built-in lookup runs through blocking adapters alone, the decline the built-in resolver's own; a library test of Phase 14, task 1, checks the async outcome, as the maintainer chose | 2.3, 4.1 | By reading `api.md` section 13; the spike runs no async adapter, and no lookup before Phase 14 |
| 2 | One shared seekable expectation could not hold the valid outcomes of adapters at a resource limit | `outcomes=per_adapter`, only beside a `limit.` condition, with each adapter's semantic part in `outcomes.json`, refused where every adapter agrees with its class, as the maintainer chose; each adapter's usage held within the case's limits | 2.3, 4.1 | Yes: in the I/O spike, with a working memory of 0, bytes that are no toy file gave `Unknown` with no limit exhausted through `probe_bytes`, and `InsufficientData` with the working memory exhausted through the seekable and the forward-only reader; on a toy file, bytes gave `Identified` |

Found while revising:

- **A class without an adapter.** The golden pass required a forward-only adapter in every case, though a case of built-in lookup with the file runs through `probe_file` alone. A class that no adapter of a case runs now has no part.
- **No sample case yet.** In Phase 2 every adapter gives the error of an unknown format, so a sample's case of `outcomes=per_adapter` fails by the agreement rule. A temporary case under a working memory of 0 showed it, in the check and when writing. With `outcomes=shared`, the same case wrote the shared parts. The spike's tests give the parts themselves, and task 7 adds the first fixture case.
- **The layout.** The kit's list of expectation parts gains `outcomes`, so the layout check accepts its file and the stale-file check covers it.

### 10.5 The fifth review

The maintainer reviewed revision 78 on 2026-11-27, with all 74 fixture-spike tests passing and `git diff --cached --check` clean, and kept G3.2 open. Its summary counts seven issues and its comments give eight, five of priority 2 and three of priority 3; each holds, and each is fixed here. The maintainer chose the fixes of findings 1 and 2, and the other fixes follow the review's own recommendation. "Reproduced" means that a scratch test against revision 78 showed the defect before the fix, and that the spike's test of the fix passes.

| # | Finding | What changed | Where | Reproduced |
|---|---|---|---|---|
| 1 | P2: the default limits can stop adapters differently, but the default case had to share its parts, so the golden pass refused a valid fixture before any named case | A `default: outcomes=per_adapter` line, the default case's only condition; no `limit.` condition needed; the seed check reads the bytes adapter's outcome from `outcomes`, as the maintainer chose | 2.1, 2.3, 4.1, 6.1, 6.4 | Yes: in the I/O spike, a toy file of 675,504 bytes gave 1,269 titles from 31,665 bytes read through bytes in memory, and 1,024 titles at 67,108,864 bytes with the read limit exhausted through the seekable reader |
| 2 | P2: any `limit.` condition qualified a case of outcomes of each adapter, so differences that no limit caused passed, and a run without a seekable adapter gave an empty `outcomes` | In each class, the adapters that exhausted no limit share one part, and some class has two that differ, as the maintainer chose; a run without a seekable adapter fails by its own message | 4.1 | Yes: two seekable adapters reporting `toy` and `flac` with no limit exhausted were accepted, and an empty run gave `{}` |
| 3 | P2: only the root usage was checked, though an error's usage does not cover its partial report's peak working memory | The partial report's usage is checked too | 4.1 | Yes: a partial report's peak of 3 passed a limit of 0 |
| 4 | P2: a missing usage, a missing counter or a malformed twin skipped its check, and the number won over a twin beside it | The usage read by the library's strict `Usage`, each counter in exactly one valid form | 4.1 | Yes: all four passed a limit of 0, `0` beside `"9007199254740993"` among them |
| 5 | P2: the bytes that the test's counters measured were copied into the expectation without a check against the read limit | The bytes delivered stay within the read limit, before anything is compared or written | 4.1 | By reading: no line compared `bytes_delivered` with a limit, and the Phase 2 library reads nothing |
| 6 | P3: the netting of paths took the charge from `retained_data` alone, so a gross figure in its twin stayed gross and failed its limit | Either form read, and the net figure written in its canonical form | 4.2 | Yes: a net figure of 9007199254740991 with a path of 18 bytes, written in its twin, was reported 18 bytes over |
| 7 | P3: the rule that each request has its answer covered the requests that built-in lookup without the file declines itself, which may take no answer, and a limit can stop one adapter before a request that another makes | A request that built-in lookup without the file declines takes no answer; each answer is asked for by at least one adapter of the case | 2.3 | By reading: the spike refuses every lookup but `off` before Phase 14 |
| 8 | P3: the async test of Phase 14 expected no source-table entry, though a report lists its primary input | No companion request or source opened, the primary input listed and no companion, and the companions "not enabled" for an input that expects one | 2.3, plan Phase 14 | By reading |

Found while revising:

- **The bytes read and the bytes delivered.** Plan section 7.5 defines the bytes read as the bytes that every source of the probe delivers, so where the test counts every source, the usage's figure equals the measured delivery, which the test now checks beside the bound of finding 5. A usage that under-reports fails even below the limit.
- **Older tests with a usage of one counter.** The tests of the paths' netting gave a usage of `retained_data` alone, which the strict reader refuses, so the netting left it gross. They now give a whole usage, in a test file of their own, `sources.rs`, beside the twin's test.
- **The default line end to end.** A temporary `default: outcomes=per_adapter` on a sample fixture failed by the agreement rule, since every adapter gives the same error in Phase 2, and `default: limit.working_memory=0` failed as a condition that the default case does not take. Both were reverted. The spike's tests give the parts themselves, and section 9 proposes task 7's fixture of title references for the first default case.
- **The kit's list of parts.** An expectation's doc comment named three parts without `outcomes`.

### 10.6 The sixth review

The maintainer reviewed revision 79 on 2026-11-28, with all 84 fixture-spike tests passing on macOS, the library's also with `serde_json/preserve_order`, and `git diff --cached --check` clean, and kept G3.2 open on five findings, four of priority 2 and one of priority 3. Each holds, and each is fixed here. The maintainer chose the fix of finding 4, and the other fixes follow the review's own recommendation. "Reproduced" means that a scratch test against revision 79 showed the defect before the fix, and that the spike's test of the fix passes.

| # | Finding | What changed | Where | Reproduced |
|---|---|---|---|---|
| 1 | P2: the test of the twin's netting built its handed path with `Path::join` but wrote `/tmp/abcdefg/x.bin` into the record, which on Windows differs by the separator before `x.bin`, so the normalizer would panic before either assertion | The record lists the handed path's own display string | Spike | By reading: `Path::join` writes Windows' own separator, and the comparison is exact; the review verified it by inspection too |
| 2 | P2: a partial report without a usage passed the check as no partial report at all | A partial report needs its usage, wherever the member stands | 4.1 | Yes: a partial report of `sources` alone passed |
| 3 | P2: the check ran after the netting of paths and against the case's limit, while a file probe's limit is raised by its primary path alone, so an overrun smaller than a built-in companion's charge passed | Each record checked as written, against the limits that its adapter was given, before the netting | 4.1, 4.2 | Yes: under a case's limit of 128 with two paths of 18 bytes, 147 against an actual limit of 146 was netted to 111 and passed |
| 4 | P2: the maps keyed by adapter would have more members in a run with the async features than in one without, and a run without them would rewrite them, the seed check's pass among them | The golden tests and the seed check in one target that requires every feature of the library, as the maintainer chose | 4.3, 6.4 | By reading: the spike runs no async adapter; a scratch crate showed that `cargo test` skips a target whose required features are off without a word |
| 5 | P3: the async test of Phase 14 asked every async entry point for a source table and a companions status, which a `Detection` does not have | A report from `probe_*` lists the primary input and gives the status; a detection from `detect_*` asserts its usage alone, no companion request and no source opened | 2.3, plan Phase 14 | By reading `api.md` section 8 |

Found while revising:

- **The limit that a built-in companion meets.** The spike raised the file adapter's limit by the charge of every path that the test handed the library, while section 4.2 raises it by the probed file's path alone. Before Phase 14 the two agree, since the test hands only that path, but the spike would have raised the limit by each built-in companion's too. It now raises the limit by the probed file's path, and the check of finding 3 holds the probe to that limit.
- **`zlib` beside the async features.** The review named the async features. `zlib`, which task 14 declares, changes the semantic part of a compressed header, `expected` among the files, so the target requires it too.
- **The fuzz workflow.** The seed check writes the corpus, so the scheduled workflow must pass `--all-features`, or it would skip the check without a word and write nothing (section 6.4).

### 10.7 The seventh review

The maintainer reviewed revision 80 on 2026-11-28, with all 85 fixture-spike tests passing from a copy of the index, the library's also with `serde_json/preserve_order`, regeneration giving identical expectation files, and `git diff --cached --check` clean, and kept G3.2 open on three findings, two of priority 2 and one of priority 3. Each holds, and each is fixed here. The maintainer chose the fix of finding 2, and the other fixes follow the review's own recommendation. "Reproduced" means that a scratch test against revision 80 showed the defect before the fix, and that the spike's test of the fix passes.

| # | Finding | What changed | Where | Reproduced |
|---|---|---|---|---|
| 1 | P2: a file report could leave out the primary path that its raised limit pays for, so the whole raised limit served the rest of the probe, and both the check and the comparison of the file adapter's part took it | Each source table lists the primary input with exactly the path that its adapter was given, checked after the usage and before the netting | 4.2 | Yes: a complete report that listed its primary input without a path and retained 146 bytes passed `prepared` under a case's limit of 128 raised by the path's 18 |
| 2 | P2: the golden target is skipped without `zlib`, so the behavior without the feature, which Phases 5 and 6 test, had no run: every test run passes `--all-features`, and CI's feature-combination job only builds | From task 14, a step of `node scripts/check.ts` lints and tests the library alone under every feature but `zlib`, as the maintainer chose; the golden files stay under every feature | 4.3, plan 9.2, 9.8 and Phase 3, task 14 | By reading the check script and `ci.yml`; a scratch workspace showed the selection that turns `zlib` off |
| 3 | P3: the test of the budget built an `unknown_format` that opened a companion, and put its sources at the error's root | A complete report with an opened companion, built as the library's serialization tests build theirs and read back by the strict reader | Spike | Yes: the strict reader refuses an `unknown_format` that counts a companion, and reads an error's root `sources` only as a member of a newer version |

Found while revising:

- **The other tables and adapters.** The review asked for a successful file report. The raised limit holds the path in every record of a file probe, so an error's observations and partial report list it too, where they stand. Every other adapter is given no path, so its primary input lists none; a probe that wrote its hint as a path would otherwise have been written into its adapter part unchecked, since only a path below the test's folder was checked.
- **The other tests of paths.** Two tests of the netting built records of the same kind: the twin's test listed its source at an `unknown_format`'s root, and the partial report's test built a `source_failed` of three members. They now use the records that the strict reader reads. The test of a Unicode path marked lossy keeps a record that the reader refuses, since that record is the defect that it tests.
- **The selection of the run.** `--no-default-features` with `--workspace` leaves `zlib` on wherever a member depends on the library with its default features, as the CLI will, so the step selects the library alone (section 4.3).
- **Lint and coverage.** Clippy under every feature does not lint the code that only a build without `zlib` compiles, and coverage leaves it out, so the step lints it as well as testing it.
