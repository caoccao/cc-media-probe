# Spike: the I/O architecture

This disposable spike holds the evidence for gate G3.1 of the [development plan](../../docs/plans/plan-init-project.md), Phase 3, task 1: the smallest end-to-end slice of D8, D9, D15 and D17. One toy container walker is written once as `async` code over an internal positional source. The same code is driven from every input of `api.md` sections 4 and 9, and a header assembler with a toy codec follows the parsing contract of plan section 7.4. Since the async walker missed D8's bar once the first review of 2026-11-27 was fixed, the spike also holds D8's fallback, the same walk as an explicit state machine, for comparison. The second review of the same day was fixed in all three walkers, and section 5 holds the measurements taken after it. The maintainer approved G3.1 on 2026-11-27 at `2de60bf`: D9, D15 and D17 as section 6 shows them, and D8 with the async walker's measured overhead as a one-time exception to its bar, so the shared async walker stays (section 6).

The spike is a workspace of its own, excluded from the root workspace, with its own lock file (plan section 1.2). It depends on the library by path, so it drives the approved `PositionalSource` and `AsyncPositionalSource` and reads the library's `Limits`, as the maintainer chose on 2026-11-27. Nothing depends on it. It stays as the reference for the toy format's walker of task 5 and the I/O layer of task 6, and is deleted in the commit that completes task 6 (plan section 1.2).

## Contents

1. [What it holds](#1-what-it-holds)
2. [The toy format](#2-the-toy-format)
3. [The toy codec](#3-the-toy-codec)
4. [The cases](#4-the-cases)
5. [The measurements](#5-the-measurements)
6. [What it shows for G3.1](#6-what-it-shows-for-g31)
7. [Running it](#7-running-it)

## 1. What it holds

The library's code rules hold here too: the root workspace's lint table, R29 to R33, and the copyright header. The structure check of `cc-media-probe-checks` finds nothing in the spike.

- **`src/source/`, the internal source of D8.** `InternalSource` has two read paths. `read_now` serves, as plain code, what the source can give at once: any range of bytes, a block that the cache holds, or a range that the pinned prefix or the window holds. `read_at` is `async` and makes a read that needs the input. Both clamp every read to the allowance and never ask for zero bytes. The walker reads through `fill!`, which awaits only when a read must wait. Under the trait:
  - the raw inputs: a caller's `PositionalSource`, a `Read + Seek` reader and a forward-only `Read`, which are blocking; a caller's `AsyncPositionalSource`; and the seekable and forward-only readers of Tokio and of futures-io;
  - `BlockCache` for the seekable path, whose bookkeeping, `Blocks`, issues no read itself. Its first block is required. Every further block is optional: one that the working memory cannot hold marks nothing, and the cache reuses its least recently used block instead. A buffer that the walk requires takes the optional blocks back before it names the limit, so a larger cache never gives less than a smaller one;
  - `Window` for the forward-only path (D9), with the bytes that detection read pinned for the whole probe. It never drops the byte that a read asks for, even with a chunk larger than the window, and it delivers what detection read before it reports an exhausted allowance;
  - `Memory` for bytes, read in place.

  A blocking raw read returns `std::future::ready`, so it is made before its future exists and never waits. Every source also has `Holdings`: whether it delivered the byte before a position, and its optional blocks to give back. A fill whose read fails keeps the count of bytes that arrived before the error.
- **`src/executor/`.** `poll_once` polls a future once with `Waker::noop`, which is stable since Rust 1.85, so the executor needs no `unsafe` code. The blocking entry points drive the whole probe with it. A future that is not ready on its first poll ends with `ProbeError::Stalled`, which no blocking input causes.
- **`src/walker/`, the walker.** One traversal in file order, with an explicit stack of open lists. It offers every packet that it reaches, in bounded pieces, to the assembler of its stream. The per-element path is one `async fn` whose only awaits are those of `fill!`, so an element costs no nested future; only the rare title reference has a future of its own. `Walk` and `Packet` hold the state that is not I/O, and their methods are plain code: the element decisions, the pieces of a packet, the declarations, the diagnostics and the source table. `probe` runs detection, the primary input's walk, and then each companion, with the primary input's buffers released once its walk is over. Since the second review:
  - a header read never reaches past its list: a list whose rest cannot hold a header is left, with a diagnostic, before the read;
  - a read that gives nothing at an element's position ends the input cleanly only where the byte before it was delivered. Otherwise the walker reads that byte (plan section 7.5), and a jump past the end truncates the element that it jumped over;
  - a read that fails inside a packet offers the bytes that arrived before the error, so the same failure gives the same fields however the pieces are cut;
  - a title reference is followed only to a title element that the walk recorded before it, which needs no read to check;
  - the catalog lists each companion once, in the order found, through an `IdentifierSet`: an open-addressing table of fixed-size keys under a hash with random keys, so a repeated identifier is found in a bounded number of steps. With lookup on, the queue of requests holds each identifier's offset beside it and duplicates no name;
  - the record of titles, the catalog's set and queue, the stack of open lists and the list of identities are working storage, each charged before it grows.
- **`src/assembler/` and `src/codec/`, D17.** The parser takes the inputs of plan section 7.4: the stream's framing mode and representation, the context of the call with the packet's known length, the bytes, the continuation value, and whether the packet or stream ended. The per-stream header assembler holds at most the per-stream header limit, the continuation value's fixed size included, and charges that size to working memory too. It calls the parser in the three situations of plan section 7.4, keeps the continuation value, and drops what the parser finished. In stream mode only the end of the input ends the stream, and the parser is told so. No call examines more bytes than the work units and the cap on bytes examined, 16 times the limit, leave, so every field established was paid for, and a stream that still waits once its bytes examined reach the cap ends with the header limit named, or the work units on a tie. A source that fails ends each waiting stream with the fields read and the reason "source failed". The toy codec's parser is a pure function, which decides each unit as soon as its bytes allow (section 3).
- **`src/budget/`, D16, reduced.** It covers the bytes read, the working memory with its peak, the per-stream header bytes and the work units. Every buffer is charged before it is reserved, its real capacity is read back, and a contiguous buffer or a list grows with both allocations charged. A charge of working memory is an attempt that marks nothing. A buffer that the probe requires goes through `require`: one that does not fit takes back the optional blocks of a `Reclaim` owner, the cache, and is charged once more, and only one that still does not fit, or that the allocator refuses, marks the working memory exhausted.
- **`src/companion/`, after `api.md` section 13.**
  - The request, the five answers, a blocking resolver trait and an async one.
  - The built-in resolver's three steps: resolve the path, following every link; check containment in the resolved root, and that the path names a regular file, since opening a named pipe waits for a writer; open the resolved path; then resolve it again and require a regular file of the handle.
  - The built-in identity: the device and file numbers on Unix, and the resolved path otherwise. Only the built-in resolver compares a companion with the primary file. The primary file's numbers are a fixed-size value taken when it is opened, and become a charged identity only once a companion is requested.
  - The trusted root resolved when the first request needs it. A root or a path that cannot be resolved fails the request with the file system's error; a path that no longer resolves after the open is declined as changed.
  - The companion-request and sources-opened limits, checked just before each call, and the identities held for the rest of the probe, each charged to working memory before it is reserved.
- **Entry points.** `blocking` has the five inputs of `api.md` section 4.1. `asynchronous` has `probe_source`, with `asynchronous::tokio` and `asynchronous::futures` each holding a seekable and a forward-only entry point (D15).
- **`src/handwritten/`.** The same probe written by hand as blocking code: bytes, and a seekable reader through the same block bookkeeping. It shares the walker's state, element decisions, assembler and parser, and has no companions. It is the baseline of the overhead measurement, so the comparison isolates what writing the walker once as `async` code costs.
- **`src/machine/`, D8's fallback.** The same walk as an explicit state machine, written for comparison because the async walker missed the bar once the review's fixes were in (section 5). The machine never reads. It says which range it needs, takes in what a read delivered, and runs the same plain code between reads as the other two walkers: `Walk`, `Packet`, the detection's decisions, the assembler and the parser. Each phase holds what the walker keeps across a read. A blocking loop drives it over the hand-written probe's sources, and an async loop over the internal source, which awaits only a read that must wait, so one machine serves both modes. Like the hand-written probe, it has no companions and no forward-only window.
- **`src/builder/`.** `ToyFile` and the codec's units in both representations, the draft of the test kit's builders of task 5, with stray bytes that belong to no element. A unit whose body its length field cannot hold is refused.
- **`src/options/`.** The internal sizes, each a `NonZeroUsize`, so no size of 0 can stall a read or a packet's pieces.

## 2. The toy format

The draft of the permanent toy format of task 5, as the maintainer chose on 2026-11-27. All integers are big-endian.

```text
File header, 12 bytes
  offset  length  field
  0       6       signature "CCToy" and a zero byte
  6       1       version, 1
  7       1       flags, 0: none is defined
  8       4       offset of the first element, at least 12; the bytes between are padding

Element
  0       4       code, four ASCII characters
  4       4       length of the body
  8       n       body
```

| Code | Body | Walker |
|---|---|---|
| `STRM` | 7 bytes: stream number, framing (0 packet mode, 1 stream mode), representation (0 units with one-byte lengths, 1 units with two-byte lengths), codec code | Declares a stream; only the codec `TCOD` gets an assembler, and another codec is "unsupported"; an unknown framing or representation is a bad body |
| `PACK` | Stream number, then the payload | Offers the payload in pieces to the stream's assembler while it wants more; a stream that no declaration names gives a diagnostic |
| `TITL` | UTF-8 text, at most 255 bytes | A title |
| `TREF` | 8 bytes: the absolute offset of an earlier `TITL` element | Follows it backward when the walk recorded a `TITL` element there before the reference; anything else, title-shaped bytes inside another element's body among them, is a bad body, and nothing is read for it |
| `LIST` | Child elements | Entered with the explicit stack, within the nesting limit; a child that runs beyond it ends the list, a rest too short for a header ends it before any read, and the end of the input inside it is a truncated element at the list |
| `CMPN` | An identifier: 1 to 16 lowercase ASCII letters and digits | The companion `<identifier>.toyc`, in the probed file's folder; a companion's own `CMPN` is not followed |

Any other code is skipped with a diagnostic. An element costs a work unit once its header is read, so an allowance of exactly the elements present completes the traversal. The input ends cleanly at an element's position only where the byte before it exists; the end inside an element that the walker skipped truncates that element. Detection reads the file header and the first element's code, which must be one of the six. A read cut short by the end of the input or by a limit gives insufficient data while its bytes can still begin a toy file, and "unknown" once they rule it out: another signature byte, another version, or bytes that none of the six codes begins with. On a forward-only input, a sample beyond the pinned prefix gives insufficient data too.

## 3. The toy codec

The codec `TCOD` carries units: a kind byte, a length field and the body. The stream declaration states its representation, which is how long the length field is: one byte, or two bytes, big-endian, which holds a parameters unit of more than 127 parameters.

| Kind | Body | Meaning |
|---|---|---|
| 0 | Any | Payload, without a header |
| 1 | 6 bytes: width, height (16 bits each), parameter count, flags | The setup. A count of 0 completes the header; flag bit 0 says that the parameters cannot be read without decoding, a final partial state |
| 2 | The count's number of 16-bit parameters | The parameters, whose length only the setup tells: the second unit needs a fact of the first, which the continuation value keeps |

Any other kind, parameters before a setup, a setup whose length is not 6, or parameters of another length than the setup's count gives are malformed. The parser decides each as soon as those bytes are there: the kind byte, or the kind and the length field, so no limit is named for a body that was never needed.

- **In packet mode,** a packet that starts with a payload unit has no header, which its kind byte alone tells. At a unit boundary the parser asks for the kind byte alone, since a payload unit after a setup finishes the packet, and the header continues in a later packet. A unit cut by its packet's end is malformed. The container knows each packet's length, so a unit whose stated end lies beyond it is malformed once its header is read, and the rest of the packet is never needed.
- **In stream mode,** payload units are skipped and finished, and a setup is finished once read, so the assembler drops its bytes and keeps its fact. The end of a delivery is not the end of the stream; the end of the input is, and running out of bytes there is never malformed.

## 4. The cases

`tests/cases/` holds 92 cases: 27 of them were added for the first review of 2026-11-27, and 17 for the second, which the list marks "second review". `src/source/window.rs` holds one embedded test.

- **`parity.rs`: the same report from every input.**
  - The typical file goes through bytes, a seekable reader, a forward-only reader, a custom positional source and a file. It goes through a custom async source, one ready at once and one that answers pending to every other call. It goes through the seekable and forward-only readers of Tokio, a `tokio::fs::File` among them, and of futures-io, with both features of the library enabled.
  - The hand-written probe gives the same report as the async one, usage included.
- **`assembly.rs`: the header assembly.**
  - A header is split at every byte position across two deliveries, and at every pair of positions across three. Deliveries and pieces of one byte work too.
  - Two packets delivered back to back stay two, joined by the continuation.
  - A packet of 1 MiB, far larger than a header limit of 64 bytes, has its header in its first piece. On a seekable input fewer than 1,024 bytes are read; on a forward-only one the rest is read past, and fewer than 4,096 bytes are held.
  - A second unit is read with the fact of a first unit whose bytes were dropped.
  - A header arrives after five headerless packets.
  - A long run of headerless payload meets the cap on bytes examined.
  - The end of the input cuts a header short.
  - A parser that needs more than the limit ends with the limit named.
  - A setup that needs decoding is final, and bad units are malformed.
  - Two streams compete for the last working memory.
  - A stream that waits through a thousand packets holds no more than one piece and its continuation value.
- **`forward.rs`: the forward-only window (D9).**
  - The second stream's only header is taken while the first stream still waits, and it is far behind the window by the time the first completes.
  - A reference behind the window gives a diagnostic for that range only, and the walk goes on with the next sibling; one within the window is read.
  - A detection sample beyond the pinned prefix gives insufficient data with the range unavailable, while the same file through a seekable reader is identified.
  - A forward-only reader starts where it stands and is left after exactly the bytes read.
- **`protocol.rs`: the source protocol of `api.md` sections 4.4 and 9.3, through the cache and the window, which G3.1 confirms.** These are the cases of `docs/design/deferred-cases.md` section 5, run against the spike.
  - A pending request is asked again with the same range, through a custom async source and through a Tokio reader on the window.
  - A source reused after a dropped probe answers the new request, with the record `(0, 64), (100, 4), (100, 4), (0, 64), (0, 64)`.
  - A blocking probe never asks for zero bytes, under eight allowances, and every byte that the source delivered is counted.
  - Every probe future is `Send` for a `Send` input, which the build checks, and a source that holds an `Rc` still runs.
- **`limits.rs`: the assembler's limits.**
  - One-byte deliveries of a stream parse as one delivery does, under a header limit of 96 bytes.
  - A parser call examines no more bytes than the work units leave, and an allowance of exactly the elements and bytes examined parses the header.
  - The bytes examined for a stream never pass their cap.
  - The continuation value counts toward the per-stream header limit and is charged to working memory.
  - A stream that still waits when its 1,024 packets have reached the cap of 16 times a 64-byte limit ends with the header limit named, and with the work units on a tie (second review).
- **`units.rs`: what a unit's first bytes decide (second review), the three walkers agreeing.**
  - A setup followed by a payload kind parses with pieces of 8 bytes as with pieces of 9, under a header limit of 9 bytes and the continuation value.
  - An unknown kind, a setup of another length, parameters before any setup, and parameters that the setup's count contradicts are malformed before their 200-byte bodies arrive, in both framing modes, with no limit named.
  - A two-byte length that contradicts the setup is malformed at once.
- **`representation.rs`: the parser's inputs beyond the bytes.**
  - Two-byte lengths hold a parameters unit of 200 parameters, which the one-byte builder refuses, in both framing modes.
  - The same header in each representation gives the same fields, and units read in the wrong representation run past their packet.
  - An unknown representation byte is a bad body.
  - A known packet length makes a parameters unit that runs past it malformed on the first piece, under a header limit of 64 bytes, with the rest of the packet unread.
  - A payload unit is known by its kind byte alone, so a header allowance of one byte still reaches the third packet.
  - The builders refuse a length that the field cannot hold.
- **`traversal.rs`: the traversal's edges, the async and hand-written probes agreeing.**
  - The end of the input inside an open list, or a nested one, cuts that list short, and a closed list does not.
  - An allowance of exactly the elements present completes the traversal.
  - A title reference never reads through itself.
  - A chunk larger than the window keeps the bytes that a read asks for.
  - A larger cache never reads less under the same working memory.
  - Detection judges the byte that a one-byte allowance gave on a forward-only input, and bytes that already rule the format out are not insufficient data.
  - Since the second review, also through the state machine where it reads the input:
    - the end of the input 25 bytes inside a skipped final packet truncates it, from bytes, a seekable reader and a forward-only reader, while the whole file ends cleanly;
    - a list with one stray byte is left without reading past it, so an allowance of exactly the bytes needed reads both titles;
    - a reference into a skipped element's body, at bytes shaped like a title, is a bad body;
    - a first element that no known code begins, such as `Z` or `TZ`, rules the format out, while `TI` and nothing at all are insufficient data.
- **`failures.rs`: a failing source (R19).**
  - On a positional source, a seekable reader, a forward-only reader, an async source and the hand-written probe, the partial report keeps a setup as cut short, a stream with payload only as not found, and a stream not begun as not attempted, each with the reason "source failed", returns every charge, and records the primary input as failed.
  - A file that cannot be opened is a failed source.
  - A companion that fails after its setup keeps its fields, and the primary input stays opened.
  - The bytes that a read delivered before it failed are offered to their stream: with pieces of 8 and of 16 bytes, each of the seven inputs keeps the setup as cut short, with one packet offered (second review).
- **`budget.rs`: the budget.**
  - An allowance of zero issues no read.
  - Ten allowances are exact on every blocking input.
  - A stop names its limit for the streams that still wait.
  - The working memory bounds the peak, down to a limit of 0, and a limit one byte below the peak stops nothing, since it costs only an optional cache block.
  - The work units stop the traversal.
  - Every buffer returns its charge by the end.
- **`companions.rs`: the companions, on macOS alone, as the maintainer chose on 2026-11-27.**
  - The built-in resolver opens a companion beside the probed file.
  - A link to a file inside the root, in a layout folder, is followed, and a link that leads outside the root is declined.
  - A directory, a missing companion, and an input without a folder are told apart.
  - One identifier named twice makes one request, and two names of one file give an alias.
  - A hard link to the probed file is an alias of source 0.
  - With lookup off no request is made, and an invalid identifier makes none either.
  - The request limit refuses a further companion before any request.
  - A custom resolver serves the companion of bytes, and an async one serves an async source.
  - A failing companion ends the probe with its identifier and the partial report.
  - The allowance is shared by the primary input and its companion.
- **`memory.rs`: working memory between the cache and the walk (second review), the three walkers agreeing.**
  - A cache block taken during detection gives way to the scratch buffers, with one block or two.
  - An optional block that does not fit marks nothing.
  - A scratch buffer that the allocator refuses names working memory.
  - The stack of 100 open lists is charged to working memory, and one that does not fit stops the traversal.
- **`catalog.rs`: the companions that a file names (second review).**
  - The queue of 200 named companions is charged to working memory, under a request limit of 0, and one that does not fit stops the traversal.
  - 20,000 companions named twice each are listed once each, in the order found.
- **`machine.rs`: D8's fallback gives the async walker's report, usage included,** through its blocking driver from bytes and from a seekable reader, and through its async driver from a source that answers pending to every other call. It covers eight files that reach the traversal's edges, among them the typical file, a cut list, an overlapping reference, long headerless runs, both representations and three that detection refuses, under six sets of limits and sizes.
- **`lookup.rs`: companion lookup's limits and identities, on macOS alone.**
  - A missing companion leaves the open allowance to the next, and a refused one gets no request.
  - A custom token equal to the primary file's identity is no alias of source 0, while two equal tokens make an alias.
  - A token is held under working memory: one that cannot be held is "limit reached", and every token's charge returns at the end.
  - A named pipe is declined before an open that would wait for a writer.
  - Built-in lookup charges no identity when no companion is named, so a working memory that the file needs exactly marks nothing (second review).
  - A folder that cannot be resolved fails the request with its cause, while an input without a folder is declined (second review).
  - The cases that need Unix calls are compiled on Unix alone, so the cases compile for Windows too.
- **`src/source/window.rs`: a chunk that does not fit frees the ring with its charge.**

## 5. The measurements

Measured on 2026-11-27 on an Apple M4 Max with macOS and Rust 1.100.0, in the release profile, by Criterion 0.8.2 without default features. Three walkers run on the same toy files:

- the async probe, driven by `poll_once`;
- the hand-written one, the baseline;
- D8's fallback, the state machine through its blocking driver.

"Few" has 16 packets after the headers, and "many" has 100,000. They are read from memory, through the block cache over a reader in memory, and from a file. Before a case is timed, every probe of it runs once and must give the expected report, and since the second review every timed probe must succeed, or the run ends with status 1.

**The bar,** as the maintainer chose on 2026-11-27: the async walker within 10% of the hand-written one on the two in-memory cases, where no I/O hides the cost. Each case is judged by its median, and the bar must hold in each of three runs. Otherwise the spike writes the fallback's state machine for comparison. At the review the maintainer also chose one codegen unit for the benchmark's profile (below).

The machine was busy throughout. During the runs after the first review its load average stood between 42 and 68, with other applications and a virtual machine running, and during those after the second review between 8.5 and 32.9. The maintainer chose to record that rather than wait for a quiet machine, and chose at the second review to rerun all six runs, since its fixes changed the per-element path of all three walkers.

**The gate's three runs, after the second review's fixes, with one codegen unit.** The in-memory medians, with the async probe's difference from the hand-written one by the median, the mean and the slope:

| Run | Input | Async | Hand-written | Median | Mean | Slope |
|---|---|---|---|---|---|---|
| 1 | few | 807.5 ns | 728.6 ns | +10.8% | +11.4% | +10.4% |
| 1 | many | 1.571 ms | 1.459 ms | +7.7% | +7.4% | +7.5% |
| 2 | few | 817.7 ns | 759.7 ns | +7.6% | +7.5% | +7.7% |
| 2 | many | 1.644 ms | 1.460 ms | +12.6% | +12.7% | +14.4% |
| 3 | few | 817.2 ns | 735.4 ns | +11.1% | +12.0% | +12.9% |
| 3 | many | 1.573 ms | 1.476 ms | +6.6% | +7.2% | +8.0% |

The bar fails in every run again: with few packets in runs 1 and 3, and with many in run 2. So D8 still does not hold by the maintainer's rule. Through the cache and from a file, the async probe was −19.5% to +12.6% in these runs, the widest spread with few packets from a file.

**The three walkers together, in three more runs after the second review's fixes.** The medians, with each walker's difference from the hand-written one by the median:

| Run | Group | Input | Async | State machine | Hand-written | Async | State machine |
|---|---|---|---|---|---|---|---|
| 4 | memory | few | 809.3 ns | 924.1 ns | 740.3 ns | +9.3% | +24.8% |
| 4 | memory | many | 1.550 ms | 2.172 ms | 1.459 ms | +6.3% | +48.9% |
| 4 | cursor | few | 1.917 µs | 2.047 µs | 1.952 µs | −1.8% | +4.8% |
| 4 | cursor | many | 2.576 ms | 3.487 ms | 2.966 ms | −13.2% | +17.6% |
| 4 | file | few | 17.54 µs | 17.72 µs | 17.46 µs | +0.4% | +1.5% |
| 4 | file | many | 2.771 ms | 3.585 ms | 3.086 ms | −10.2% | +16.2% |
| 5 | memory | few | 811.9 ns | 915.4 ns | 726.7 ns | +11.7% | +26.0% |
| 5 | memory | many | 1.553 ms | 2.177 ms | 1.468 ms | +5.8% | +48.3% |
| 5 | cursor | few | 1.940 µs | 2.056 µs | 2.108 µs | −8.0% | −2.5% |
| 5 | cursor | many | 2.774 ms | 3.462 ms | 2.946 ms | −5.9% | +17.5% |
| 5 | file | few | 18.33 µs | 20.02 µs | 18.84 µs | −2.7% | +6.3% |
| 5 | file | many | 2.720 ms | 3.557 ms | 3.047 ms | −10.7% | +16.7% |
| 6 | memory | few | 781.9 ns | 906.3 ns | 715.6 ns | +9.3% | +26.6% |
| 6 | memory | many | 1.574 ms | 2.234 ms | 1.551 ms | +1.5% | +44.0% |
| 6 | cursor | few | 1.949 µs | 2.088 µs | 1.945 µs | +0.2% | +7.4% |
| 6 | cursor | many | 2.704 ms | 3.604 ms | 2.996 ms | −9.8% | +20.3% |
| 6 | file | few | 20.11 µs | 19.71 µs | 23.18 µs | −13.3% | −14.9% |
| 6 | file | many | 2.775 ms | 3.690 ms | 3.201 ms | −13.3% | +15.3% |

In memory the state machine is +24.8% to +26.6% with few packets and +44.0% to +48.9% with many. In the same runs the async walker is +9.3% to +11.7% with few packets, which misses the bar in run 5, and +1.5% to +6.3% with many. Through the cache and from a file, the machine is −14.9% to +20.3%, and the async walker −13.3% to +0.4%. Both walkers take the second review's fixes, and both slowed alike: with few packets in memory, the async walker's medians moved from 745 to 757 ns after the first review to 782 to 818 ns, and the hand-written walker's from 679 to 686 ns to 716 to 760 ns.

**After the first review: the gate's three runs, with one codegen unit.** The in-memory medians, with the async probe's difference from the hand-written one by the median, the mean and the slope:

| Run | Input | Async | Hand-written | Median | Mean | Slope |
|---|---|---|---|---|---|---|
| 1 | few | 956.8 ns | 857.9 ns | +11.5% | +12.0% | +8.8% |
| 1 | many | 1.567 ms | 1.460 ms | +7.4% | +2.0% | +8.8% |
| 2 | few | 836.3 ns | 695.2 ns | +20.3% | +18.2% | +17.2% |
| 2 | many | 1.532 ms | 1.386 ms | +10.5% | +11.3% | +10.2% |
| 3 | few | 790.0 ns | 661.4 ns | +19.4% | +20.5% | +18.3% |
| 3 | many | 1.524 ms | 1.354 ms | +12.6% | +15.8% | +15.5% |

The bar fails in every run, so D8 does not hold by the maintainer's rule. Through the cache and from a file, the async probe was within 10% or faster in these runs, from −18.2% to +0.4%.

**After the first review: the three walkers together, in three more runs.** The medians, with each walker's difference from the hand-written one by the median:

| Run | Group | Input | Async | State machine | Hand-written | Async | State machine |
|---|---|---|---|---|---|---|---|
| 1 | memory | few | 749.4 ns | 903.5 ns | 681.9 ns | +9.9% | +32.5% |
| 1 | memory | many | 1.500 ms | 2.158 ms | 1.392 ms | +7.7% | +55.0% |
| 1 | cursor | few | 1.90 µs | 2.04 µs | 1.90 µs | +0.2% | +7.6% |
| 1 | cursor | many | 2.529 ms | 3.502 ms | 2.855 ms | −11.4% | +22.7% |
| 1 | file | few | 19.35 µs | 17.76 µs | 18.56 µs | +4.3% | −4.3% |
| 1 | file | many | 2.669 ms | 3.602 ms | 3.003 ms | −11.1% | +19.9% |
| 2 | memory | few | 745.3 ns | 906.2 ns | 686.1 ns | +8.6% | +32.1% |
| 2 | memory | many | 1.482 ms | 2.144 ms | 1.367 ms | +8.5% | +56.9% |
| 2 | cursor | few | 1.87 µs | 2.01 µs | 1.87 µs | −0.2% | +7.4% |
| 2 | cursor | many | 2.497 ms | 3.481 ms | 2.831 ms | −11.8% | +23.0% |
| 2 | file | few | 17.09 µs | 17.29 µs | 17.69 µs | −3.4% | −2.2% |
| 2 | file | many | 2.635 ms | 3.597 ms | 2.968 ms | −11.2% | +21.2% |
| 3 | memory | few | 757.1 ns | 914.7 ns | 678.8 ns | +11.5% | +34.7% |
| 3 | memory | many | 1.558 ms | 2.620 ms | 1.512 ms | +3.0% | +73.2% |
| 3 | cursor | few | 2.10 µs | 2.03 µs | 1.88 µs | +11.7% | +7.6% |
| 3 | cursor | many | 2.509 ms | 3.467 ms | 2.827 ms | −11.2% | +22.7% |
| 3 | file | few | 18.51 µs | 18.42 µs | 17.82 µs | +3.9% | +3.3% |
| 3 | file | many | 2.675 ms | 3.810 ms | 3.358 ms | −20.3% | +13.5% |

In memory the state machine's overhead over the hand-written baseline is three times the async walker's or more: +32% to +35% with few packets, and +55% to +73% with many. In the same runs the async walker is +8.6% to +11.5% with few packets, which misses the bar in run 3, and +3.0% to +8.5% with many.

Through the cache and from a file, the machine is −4% to +23%, and the async walker −20% to +12%. Each read passes through several dispatches on the machine's phase: where the read goes, what it delivered, the next phase and its range. Each phase also copies the element and the packet that it holds. This is the straightforward form of the fallback; it was neither profiled nor tuned.

**Codegen units.** The first three runs after the fixes used the default release profile, of 16 codegen units. Their medians put the async probe at +17.7%, +18.9% and +20.1% with few packets, and +8.1%, +8.8% and +9.3% with many. That build wrapped each source's walk in one more `async fn`.

A temporary harness, since deleted, alternated the two probes. It ran 301 rounds of 2,000 probes with few packets and 201 rounds of 2 with many, and took the median of each round's ratio, which cancels drift in the machine's load. It compared that build with the final one, which has no such `async fn`:

| Build | Units | Hand-written, few | Async, few | Few | Many |
|---|---|---|---|---|---|
| Nested `async fn` | 16 | 672 to 679 ns | 816 to 825 ns | +21.5%, +22.2% | +12.7%, +13.7% |
| Final | 16 | 786 to 841 ns | 826 to 882 ns | +5.1% to +5.7% | −10.2% to −9.7% |
| Nested `async fn` | 1 | 691 ns | 797 ns | +15.2% | +8.4% |
| Final | 1 | 692 ns | 771 ns | +11.6% | +9.6% |

The hand-written code is the same in all four builds. With 16 units, its speed moved by about 15% between builds, because the partition into units decides what the compiler inlines across them. The 10% bar then measured the partition rather than the async code. One codegen unit removed the swing of the baseline, and the maintainer chose it for the gate.

**The staged build, before the review.** Its table gave Criterion's typical estimate under a heading of medians: the slope, or the mean for `cursor/many` and `file/many`, which Criterion sampled flat. Its saved medians were +8.4% (few) and −9.8% (many) in memory, −2.1% and −18.3% through the cache, and +2.0% and −4.4% from a file. A fresh run in the review gave +9.4% by the median and +19.8% by the slope for `memory/few`. With 16 units, those figures share the baseline's swing above.

**The walker as first written missed the bar.** These figures are Criterion's typical estimate, with the default profile. Its every read was an `async fn`, and each element went through nested `async fn`s for its body, its packet and the packet's pieces:

| Group | Input | Async | Hand-written | Difference |
|---|---|---|---|---|
| memory | few | 1.137 µs | 0.690 µs | +64.9% |
| memory | many | 3.311 ms | 1.685 ms | +96.5% |
| cursor | few | 2.390 µs | 1.851 µs | +29.1% |
| cursor | many | 3.521 ms | 2.737 ms | +28.6% |
| file | few | 20.81 µs | 20.29 µs | +2.6% |
| file | many | 3.885 ms | 2.719 ms | +42.9% |

Profiles taken with macOS `sample` showed the time in the futures themselves: a third of it in the future of the read helper, and much of the rest in building, copying and dropping the walker's nested futures, which hold 120 to 592 bytes each. Shorter runs of the in-memory group, each against the hand-written probe, tried one change at a time:

| Change | Many packets | Few packets |
|---|---|---|
| As first written | +89% | +64% |
| `#[inline]` on the read helper | no change | no change |
| The source's bytes served without the source's own future | −1% to −3% | −1% to −3% |
| The reads that can finish at once made as plain code, before the helper's future | +66% | +49% |
| The packet's path in the walker's own loop, without nested `async fn`s | +30% | +30% |
| No future at all where nothing waits, `fill!` | −5% | +10% |

The last two changes are the rule that the final walker follows: an element's path is one `async fn` with no nested one, and a read that the source can give at once builds no future. With it, the async probe costs about what the hand-written one does. This first round met the bar with the default profile, before the review; the table of codegen units above shows how far the default profile moves the baseline.

**Heap.** `tests/peak/` runs each probe under `peak_alloc` on a file of 20,000 packets for a waiting stream, with the default sizes. It fails when the measured heap peak exceeds the charged working memory by more than 64 KiB, when the walkers' peaks differ, or, since the second review, when a probe did not walk the whole file: each must read the title, parse the first header, offer all 20,000 packets to the waiting stream until the input ends, and exhaust no limit.

| Probe | Measured heap peak | Charged working-memory peak |
|---|---|---|
| async, bytes | 5,111 bytes | 4,407 bytes |
| hand-written, bytes | 5,111 bytes | 4,407 bytes |
| state machine, bytes | 5,111 bytes | 4,407 bytes |
| async, seekable reader | 529,719 bytes | 528,695 bytes |
| hand-written, seekable reader | 529,719 bytes | 528,695 bytes |
| state machine, seekable reader | 529,719 bytes | 528,695 bytes |
| async, forward-only reader | 1,250,295 bytes | 1,249,591 bytes |

The difference is 704 bytes from memory and through the window, and 1,024 bytes through the cache. It is the report's own records and the small fixed structures that the spike does not charge: the list of blocks and the stream table. The three walkers allocate the same. Each peak is 16 bytes above the build after the first review: the record of the file's one title element, which the second review's fix charges.

**The size of each probe future,** which a caller's task holds, in the release profile:

- `asynchronous::probe_source`: 2,176 bytes;
- `asynchronous::tokio::probe_reader`: 2,200 bytes;
- `asynchronous::tokio::probe_stream`: 2,360 bytes;
- `asynchronous::futures::probe_reader`: 2,200 bytes;
- the state machine's `machine::probe_source`: 1,152 bytes.

After the first review they were 2,112 to 2,296 bytes, and the machine's 1,072: the second review's fixes added 64 bytes to each async future and 80 to the machine's. The build staged before the first review had 2,048 to 2,232 bytes; the table it was staged with gave those of an earlier build.

## 6. What it shows for G3.1

- **D8, one code base for both modes, misses its bar after each review's fixes, and its fallback costs more.**
  - Over blocking inputs the async walker was ready on its first poll in every case and every benchmark, so `poll_once` never stalled.
  - After the second review's fixes, with one codegen unit, the async walker is +7.6% to +11.1% with few packets and +6.6% to +12.6% with many in the gate's three runs, by the median (section 5), and misses the bar in each run. So D8 does not hold by the maintainer's rule. In three more runs it is +9.3% to +11.7% and +1.5% to +6.3%. Through the cache and from a file it is within the noise or faster. After the first review's fixes it was +11.5% to +20.3% and +7.4% to +12.6% in the gate's runs.
  - The fallback, written as the rule asks, is +24.8% to +26.6% and +44.0% to +48.9% in memory under the same conditions, twice the async walker's overhead or more. It has the same heap peaks, and a future of 1,152 bytes against 2,176. The spike shows no form of the walker that is cheaper than the async one.
  - The condition that the first build found still holds: a walker awaits only where a read must wait, and keeps an element's path in one `async fn`. One more `async fn` around each source's walk, which this revision first had, cost a further 3.6 points with few packets in the alternating rounds with one codegen unit, and nothing measurable with many.
  - The measurements were taken on a busy machine, at a load average of 42 to 68 after the first review and 8.5 to 32.9 after the second.
  - **Decided at G3.1, on 2026-11-27.** The maintainer accepted the async walker's measured overhead as a one-time exception to D8's bar and kept the shared async walker, since the fallback measured more than twice its overhead with the same heap peaks. The bar stays in the plan as written; the exception covers this decision only.
- **The rest of G3.1, decided on 2026-11-27.** The maintainer confirmed D9, D15 and D17 as the next three items show them, and took the sizes that the spike proposes, below, and the toy format of section 2 as the starting points of tasks 5 to 7. The public defaults of `Limits` belong to G3.3.
- **D9, the forward-only window, holds.**
  - A read behind the window gives "range unavailable" for that range only, and the walker goes on with the next sibling.
  - The bytes that detection read stay pinned for the whole probe, charged to working memory. The first build released them once the walker read past them, which lost a reference into them while the window would still have held the bytes after them.
  - A detection sample beyond the prefix's capacity reads nothing and gives insufficient data with the range unavailable.
  - The window drops only bytes before the read position, so a chunk larger than the window still keeps the byte that a read asks for; the review of 2026-11-27 found a build that dropped it and read later bytes in its place. Detection gets what the prefix read before it hears that the allowance ran out, as from any other input.
  - The ring is kept only once the chunk beside it is reserved, so a chunk that does not fit frees the ring with its charge.
  - The window delivers every byte in order, so it knows whether it delivered the byte before a position: the end of the input inside an element that the walker skipped is found without a read of the input (second review).
- **D15, the async surface, holds.** Both modules work with both features enabled. Each probe future is `Send` for a `Send` input, which the build checks. The block cache and the window keep the promise of `api.md` section 9.3, since a pending request is held by the future that made it. Tokio's seek waits for an operation in flight before it starts, as Tokio's own `seek` does.
- **D17, the parsing contract, fits the toy codec in both framing modes and both representations.** The parser takes the representation that the stream declaration states, and the packet's known length as context. The spike found these things that the permanent assembler and parsers should keep, the last eight through the two reviews of 2026-11-27:
  - An assembler grows its buffer only when the bytes do not fit. The first build doubled it on every packet, so a waiting stream held the whole 1 MiB per-stream limit, which the measured heap peak showed.
  - A parser that can tell "no header present" from a unit's first byte says so at once. The first toy parser waited for a payload unit's whole body, which cost a second piece per headerless packet, and the build that the review read still waited for its length byte.
  - In stream mode the end of a delivery is not the end of the stream. The reviewed build called the parser at the end of every delivery as if the stream had ended, so one-byte deliveries met the cap on bytes examined where one delivery parsed. Only the end of the input ends the stream, and the parser is told so once.
  - A call examines no more bytes than the work units and the cap on bytes examined leave. The reviewed build charged the work after the call, so a call could establish fields that the budget did not cover. A parser that needs the bytes beyond the allowance ends with the limit that set it.
  - The continuation value's fixed size counts toward the per-stream header limit and is charged to working memory while the stream waits (plan sections 7.4 and 7.5).
  - A source that fails leaves each waiting stream cut short, not found or not attempted, with the reason "source failed", instead of an unfinished assembler with its charge.
  - A known packet length lets a parser call a unit that runs past it malformed on the first piece, before a limit is blamed for it.
  - At a unit boundary in packet mode, a parser asks for the smallest prefix that decides the next step, the kind byte alone, since a payload kind ends the packet's part of the header. The reviewed build asked for the kind and the length together, so a header that filled its limit parsed with pieces of 9 bytes and failed with pieces of 8.
  - A unit whose first bytes contradict the syntax is malformed as soon as they are there: an unknown kind, a unit before the one it depends on, or a length that the kind or the continuation rules out. The reviewed build waited for the body, and named the header limit for a body that was never needed.
  - A stream that still waits once its bytes examined reach their cap ends with the limit named, even when no later call comes. The reviewed build named the cap only when a later call was cut, so a stream that reached it exactly and then met the end of the input reported a truncation.
- **Companion lookup works as `api.md` section 13 has it,** with four corrections that the first review of 2026-11-27 found and four that the second found:
  - The companion-request and sources-opened limits are checked just before each call, so a missing companion leaves the open allowance to the next.
  - Only the built-in resolver compares a companion with the primary file; a custom resolver's token is compared with earlier companions' only.
  - Each identity is charged to working memory before it is kept, so one that working memory cannot hold makes the entry "limit reached", and every charge returns at the end.
  - The built-in resolver checks that the resolved path names a regular file before it opens it, since opening a named pipe for reading waits for a writer (POSIX `open`).
  - The queue of requests is working storage, charged before it grows, and holds only each identifier's offset beside the listed identifier. The reviewed build queued owned names and identifiers outside the budget, even with a request limit of 0.
  - An identifier is found again in a bounded number of steps, through a table keyed by a hash with random keys. The reviewed build scanned two growing lists for every declaration, which is quadratic in the companions named.
  - The primary file's identity is a fixed-size value during the walk, and becomes a charged identity only once a companion is requested. The reviewed build held an uncharged copy throughout and charged a second even when no companion was named, which named working memory for a file that names none.
  - A folder or path that the built-in resolver cannot resolve fails the request with the file system's error, which ends the probe with its cause (R19). The reviewed build took any failure for an absent folder or a changed path.
- **The sizes that the spike proposes,** which are internal and not limits of `Limits`:
  - a block of 64 KiB, and 8 blocks in the cache;
  - a window of 1 MiB, a pinned prefix of at most 64 KiB, and reads of 64 KiB on a forward-only input;
  - a piece of 4 KiB;
  - a cap on bytes examined of 16 times the per-stream header limit, which names `HeaderBytesPerStream` when reached.
- **Working memory between the cache and the walk (D16, which G3.3 decides),** after the second review: a cache block beyond the first is optional, so one that does not fit marks nothing, and a buffer that the walk requires takes such blocks back before it names the limit. The reviewed build let a block taken during detection keep the walk from reserving its scratch buffers, marked working memory for a read that completed, and recorded nothing when the allocator refused a buffer that the charge allowed. The record of title elements, the catalog, the stack of open lists and the list of identities are charged too.
- **What the spike leaves out:**
  - the length of a source: the toy needs none, and the library's walkers do for results that depend on the end (plan section 7.5);
  - retained-data accounting: the report's titles, diagnostics, streams, listed companions and source table, and the small fixed structures that the measured heap peak shows uncharged;
  - the built-in resolver on Linux and Windows, and Windows junctions, which stable Rust cannot create: the maintainer chose macOS alone for this one-time check, and Phase 14's tests run on all three platforms;
  - an async built-in resolver;
  - the request cache beyond one request per identifier;
  - a regular file replaced by a named pipe between the type check and the open, which would still wait: the same condition as containment, that nobody changes the folder tree during the probe, covers it.

## 7. Running it

In `spikes/input/`:

- `cargo clippy --all-targets`, with the root workspace's lint table in the manifest;
- `cargo test`, which runs the 92 cases, the embedded test and the heap measurement;
- `cargo test --release --test peak` for the heap measurement in the release profile;
- `cargo bench --bench overhead` for Criterion, with the bench profile's one codegen unit; the gate takes three runs.

From the repository root, `cargo run -q --locked -p cc-media-probe-checks -- structure --root spikes/input` applies the structure rules of R30 to R33 and the header.
