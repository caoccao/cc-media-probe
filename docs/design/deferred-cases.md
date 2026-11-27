# Deferred cases

| | |
|---|---|
| **Status** | Written on 2026-11-27 for plan Phase 2, task 13, when the API spike `spikes/api-scenarios/` was deleted, as the maintainer asked at the approval review of revision 66 and chose the same day. |
| **What it keeps** | The inputs and the expected results of the spike's cases that the library cannot run before a later phase: the H.264 details of Phase 4, the Matroska details of Phase 5 and the ISO base media details of Phase 6, whose types arrive there, and the source protocol of Phase 3, whose detection makes the library's first reads. The library's tests took over every other case of the spike's tests (plan Phase 2, tasks 11 and 13). The maintainer's review of tasks 12 and 13, on 2026-11-27, found the source protocol missing here; this document has kept it since. |
| **Its life** | Each phase turns its cases into tests of its own type, in the form that its gate approves, and removes them from this document in the same change. The document is deleted with the last of them. |

In Phase 2, `FormatDetails` and `CodecDetails` hold only their catch-all, so the library's tests read these details as a kind of a newer version, which keeps its members unread. That shows that a reader of this version keeps them, but neither the typed accessors nor the rules that reading a typed detail checks. The spike's stub had a type for each, and these are the cases that it tested.

## Contents

1. [How to read it](#1-how-to-read-it)
2. [H.264 (plan Phase 4)](#2-h264-plan-phase-4)
3. [Matroska (plan Phase 5)](#3-matroska-plan-phase-5)
4. [ISO base media (plan Phase 6)](#4-iso-base-media-plan-phase-6)
5. [The source protocol (plan Phase 3)](#5-the-source-protocol-plan-phase-3)

## 1. How to read it

- **The input** is the JSON that the spike's stub read, as it wrote it. The stub's detail types kept Serde's default form: a variant is a map of its name (`{"IsoBaseMedia": {...}}`), and an enum's value is its name in `UpperCamelCase` (`"ConfigurationRecord"`). The library's form is that of `json-output.md` section 8: the `kind` first, the names in `snake_case`, and identifiers as decimal strings. A phase rewrites each input in its type's form before it reads it.
- **The expected results** are the stub's assertions, accessor by accessor. The names of the accessors are the stub's, which `api.md` sections 11.4 and 11.5 sketch; each phase's gate reviews its type's names (plan Phase 2, task 2).
- **The source** names the review that asked for the case and the spike's test that held it.

## 2. H.264 (plan Phase 4)

### 2.1 The picture parameter sets and the links that were read

**Source.** The twelfth review of `api.md`, section 11.5; the spike's `tests/h264.rs`. A caller sees why a slice's reference resolved to nothing: the details keep every picture parameter set read and the identifiers that the first picture's activation named.

**Input.** An `avc1` record whose only sequence parameter set is 0, with a picture parameter set that names set 1, and a second picture parameter set from a linked parameter-set track, cut short after its own identifier:

```json
{
  "sequence_parameter_sets": [{
    "origin": "ConfigurationRecord",
    "profile_indication": 100,
    "level_indication": 31,
    "sequence_parameter_set_identifier": 0
  }],
  "picture_parameter_sets": [
    {
      "origin": "ConfigurationRecord",
      "picture_parameter_set_identifier": 0,
      "sequence_parameter_set_identifier": 1
    },
    { "origin": "ParameterSetTrack", "picture_parameter_set_identifier": 2 }
  ],
  "activation": { "picture_parameter_set_identifier": 0 },
  "conflicts": []
}
```

**Expected.**

| Accessor | Result |
|---|---|
| `sequence_parameter_sets()[0].sequence_parameter_set_identifier()` | `Some(0)` |
| `sequence_parameter_sets()[0].origin()` | `ConfigurationRecord` |
| `picture_parameter_sets()[0].picture_parameter_set_identifier()` | `Some(0)` |
| `picture_parameter_sets()[0].sequence_parameter_set_identifier()` | `Some(1)`, a set that the record does not hold |
| `picture_parameter_sets()[1].origin()` | `ParameterSetTrack` |
| `picture_parameter_sets()[1].sequence_parameter_set_identifier()` | `None`: the set was cut short after `pic_parameter_set_id` (ITU-T H.264 7.3.2.2) |
| `activation().picture_parameter_set_identifier()` | `Some(0)` |
| `activation().buffering_period_sequence_parameter_set_identifier()` | `None` |
| `primary_sequence_parameter_set()` | `None`: only an observed chain establishes the primary set (plan Phase 4, the H.264 row; the maintainer's choice at the seventh review) |

The stub's other accessors, which the case does not set: `constraint_set_flags()`, and `picture_width_in_macroblocks_minus_one()`, which follows `seq_parameter_set_id` (7.3.2.1.1), so a primary set can end before it, as the tenth review found and scenario 15.10 shows. The origins are `ConfigurationRecord`, such as an `avcC` box or a Matroska `CodecPrivate`; a sample of the stream itself, as `avc3` allows; and `ParameterSetTrack`, a sample of the track that `avcp` links (plan Phase 6, task 4).

### 2.2 The lists of version 1.0 are required

**Source.** The nineteenth review of `api.md`; the spike's `tests/record_rules.rs`, `the_lists_and_flags_of_version_1_0_are_required`.

**Input.** `{"sequence_parameter_sets": [], "picture_parameter_sets": [], "conflicts": []}`, and the same without `picture_parameter_sets`.

**Expected.** The first reads; the second is refused, with an error that names the missing member `picture_parameter_sets`. A list of the form is written when empty, so a reader of version 1.0 never meets it absent (`json-output.md` section 5.2).

## 3. Matroska (plan Phase 5)

No test of the spike read Matroska's details. The typed part of scenario 15.10, which `api.md` keeps as an approved sketch, holds what the stub's type gave and Phase 5 adds to the shipped example, as `examples.md` section 4 says:

- `document_type()`, the `DocType`, always present; `writing_application()`, the `WritingApp`, optional.
- `track(stream_index)` finds a track's details by the stream's index.
- `flag_default()` is a stored value, `Some` with `is_stored()` true when the track entry stored the flag, `Some` with the default 1 when the entry omitted it, and `None` when the entry was cut short before the flag could be read.
- `track_number()` and `codec_identifier()` are optional even though RFC 9559 makes `TrackNumber` and `CodecID` mandatory: EBML fixes no order among a master's children, so a cut can fall before any of them.

The stub's other accessors: `document_type_version()`, `document_type_read_version()` and `timestamp_scale()`, stored values with their defaults, each absent only when the input ended before the element could be read, and `muxing_application()`.

## 4. ISO base media (plan Phase 6)

### 4.1 A first sample's description that was not kept

**Source.** The fifth review of `api.md`; the spike's `tests/report.rs`, `a_known_description_that_was_not_kept_leaves_the_codec_absent`. The library's tests hold its codec part: a codec without candidates and without every candidate is absent and displays as "a description that was not read".

**Input.** The first fragment selects description 2, which the retained-data limit kept the probe from keeping:

```json
{
  "IsoBaseMedia": {
    "fragment_track_identifiers": ["1"],
    "tracks": [{ "stream_index": 0, "first_sample_description_index": 2 }]
  }
}
```

**Expected.** `track(0).first_sample_description_index()` is `Some(2)`: the index is kept while the description it selects is not (plan Phase 6, task 7).

### 4.2 A description cut off before its format code

**Source.** The eleventh review of `api.md`; the spike's `tests/report.rs`, `an_iso_description_cut_off_before_its_format_code_is_a_description_that_was_not_read`. The codec part is held by the library's tests, as in section 4.1.

**Input.** The first sample selects description 1, cut off before its format code:

```json
{
  "IsoBaseMedia": {
    "fragment_track_identifiers": [],
    "tracks": [{ "stream_index": 0, "first_sample_description_index": 1 }]
  }
}
```

**Expected.** `track(0).first_sample_description_index()` is `Some(1)`. A track with sample descriptions is never "a codec that the input never named", so its codec misses a candidate.

### 4.3 Track identifiers in an error's observations

**Source.** The fourth review of `api.md`; the spike's `tests/document.rs`, `an_identifier_is_a_string_at_every_width`.

**Input.** A document whose entry is the error `no_stream_inventory`, whose observations keep the 32-bit `track_ID` of each track fragment header read (ISO/IEC 14496-12, `tfhd`):

```json
{
  "format_details": {
    "IsoBaseMedia": { "fragment_track_identifiers": ["1", "4294967295"], "tracks": [] }
  },
  "diagnostics": []
}
```

**Expected.** `fragment_track_identifiers()` is `[1, 4294967295]`, `u32::MAX` the last; the document is written back exactly as read; and the same document with `"fragment_track_identifiers": [1]`, a number, is refused, since an identifier is a string at every width (`json-output.md` section 5.4).

### 4.4 A track identifier in canonical decimal at its own width

**Source.** The thirteenth review of `api.md`; the spike's `tests/wire_values.rs`, `an_integer_identifier_is_canonical_decimal_at_its_own_width`.

**Input.** `{"IsoBaseMedia": {"fragment_track_identifiers": [identifier], "tracks": []}}` for each identifier below.

**Expected.** `"4294967295"` reads; `"01"`, `"+1"`, `"4294967296"`, `"-1"` and `""` are each refused: the text is canonical decimal within 32 bits.

### 4.5 The tracks are required and written when empty

**Source.** The nineteenth review of `api.md`; the spike's `tests/record_rules.rs`, `the_lists_and_flags_of_version_1_0_are_required`.

**Input.** `{"fragment_track_identifiers": [], "tracks": []}`, and the same without `tracks`.

**Expected.** The first reads and is written back exactly, `"tracks": []` included; the second is refused, with an error that names the missing member `tracks`.

## 5. The source protocol (plan Phase 3)

In Phase 2 the library reads nothing from any input: detection gives `Unknown`, and a probe `UnknownFormat`, before a read (plan Phase 2, task 11). The spike's stub read a prefix of 16 bytes from every source, and its `tests/async_source.rs` checked the reads against `api.md` sections 4.4 and 9.3. Its inputs are sources rather than JSON, so each case below describes the source's behavior. The stub's own requests are given where a case compared them; the library's first read can have another position and length, so a test of Phase 3 compares requests with each other, as the expected results do. The fourth case of that file, a source that is not `Unpin`, probed in a box and pinned in place, is held by the library's tests in `src/asynchronous/source.rs` and `src/asynchronous/entry.rs`. Plan Phase 3 lists the tests that these cases join: an async source with pending reads, a custom source that is never asked for zero bytes, and a custom async source reused after a dropped probe (plan section 8.7).

### 5.1 A pending read is asked again with the same request

**Source.** `api.md` section 9.3, "Repeated requests", which the spike checked from its first build (plan Phase 2, task 3); the spike's `a_pending_read_is_asked_again_with_the_same_request`.

**Input.** An async positional source of unknown length that records every call of `poll_read_at` with its position and the length of its buffer. It answers `Poll::Pending` to every other call, starting with the first, after waking the call's waker at once, and otherwise fills the whole buffer with bytes `0x1A`. It is probed by value with `asynchronous::probe_source` and a minimal executor.

**Expected.** The probe ends with its ordinary error. Each pending call is followed by a call with the same position and buffer length, and no other call comes in between: with the stub's single read, exactly two calls, both `(0, 16)`. No call asks for zero bytes (`api.md` section 4.4).

### 5.2 A source reused after a dropped probe answers the new request

**Source.** The first review of `api.md`, a finding of priority 1 (its section 16); the spike's `a_source_reused_after_a_dropped_probe_answers_the_new_request`.

**Input.** An async positional source over the 256 bytes 0 to 255, of known length, as the source of `api.md` section 15.7 behaves. It keeps the range of its request in flight. A call for another range than the one in flight, or with none in flight, starts a request for that range, wakes the waker and answers `Poll::Pending`. A call for the range in flight answers that range's bytes and clears it. Every call is recorded.

1. A probe borrows it as `&mut source` and is polled once with a waker that does nothing, so its read is pending, and is then dropped.
2. The caller reads 4 bytes at position 100 through `poll_read_at` on the same source, with a minimal executor.
3. A second probe borrows the same source and runs to its end.

**Expected.** After step 1 the source holds one request, the probe's first read (the stub's `(0, 16)`), and the dropped probe never calls again. Step 2 gives `[100, 101, 102, 103]`, the bytes of the new range, not those of the abandoned request: it is asked twice, once to start and once to answer. Step 3 ends with the ordinary error, its read asked twice too. The stub's full record was `(0, 16), (100, 4), (100, 4), (0, 16), (0, 16)`.

### 5.3 A blocking probe never asks for zero bytes

**Source.** The eighth review of `api.md`, which gave its section 4.4, "An empty buffer", and plan section 7.5 the rule; the spike's `a_blocking_probe_never_asks_for_zero_bytes`.

**Input.** A blocking positional source of unknown length that records the length of every buffer passed to `read_at` and fills it whole, probed as `&mut source` with `probe_source`.

**Expected.** The probe ends with its ordinary error; the source was asked at least once; and every buffer had at least one byte. The same holds for the async source of section 5.1, whose record is checked the same way. A read that the remaining allowance clamps to zero bytes is not issued, so every 0 that a source returns answers a read of at least one byte.
