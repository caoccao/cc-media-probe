# Descriptions of the paid standards

This directory covers the 16 paid ISO/IEC and IEC documents listed in the [source manifest](../manifest.tsv), including two amendments. The project uses public evidence, including publisher previews, corrigenda and independently published specifications. Each file describes the fields needed by the probe in the project's own words. A description is a `descriptive` source (plan section 5.4); G1.6 approved each one as the substitute for its standard on 2026-11-21. Behavior taken from it is labeled as descriptive in code comments and in `formats.md`.

- **Nothing is copied from the standards.** Field names are given as the standards spell them, because code comments cite them (plan section 5.5); layouts and meanings are restated from the sources.
- **Sources are traceable.** References use manifest identifiers, RFC numbers or direct links to supporting clauses and examples. Additional public references are candidates for the project's source review. A registration establishes a code's meaning. A profile or earlier edition establishes the rules within its stated scope.
- **Implementations are never a source.** Search results that pointed to an implementation's code were not opened (plan section 1.3).
- **Gaps are stated.** Each file records which claims its cited material leaves open and which fields the probe can still identify.

A coverage row cites a description as `<file name>: <section>`, for example `iso-iec-14496-15.md: 2`. A row that needs a structure the cited sources do not establish cites the description's gaps, `iso-iec-14496-15.md: Gaps`. G1.6 decided such rows: each keeps what the cited sources establish, and the rest is deferred to G17.n until another source is approved through GS.2.

| File | Standard | What the project needs from it |
|---|---|---|
| [iso-iec-14496-12.md](iso-iec-14496-12.md) | ISO/IEC 14496-12, ISO base media file format | Box structure, movie and track headers, sample tables, fragments |
| [iso-iec-14496-15.md](iso-iec-14496-15.md) | ISO/IEC 14496-15, NAL-unit video in ISO base media | The AVC, HEVC and VVC configuration records; length-prefixed samples |
| [iso-iec-14496-15-amd-1.md](iso-iec-14496-15-amd-1.md) | Amendment 1 to ISO/IEC 14496-15 | SEI signaling and post-filter metadata, outside the current reported fields |
| [iso-iec-14496-1.md](iso-iec-14496-1.md) | ISO/IEC 14496-1, Systems | Elementary stream and decoder configuration descriptors |
| [iso-iec-14496-14.md](iso-iec-14496-14.md) | ISO/IEC 14496-14, MP4 file format | MPEG-4 sample entries and the elementary stream descriptor box |
| [iso-iec-14496-30.md](iso-iec-14496-30.md) | ISO/IEC 14496-30, timed text in ISO base media | WebVTT and TTML sample entries |
| [iso-iec-14496-30-amd-1.md](iso-iec-14496-30-amd-1.md) | Amendment 1 to ISO/IEC 14496-30 | WebVTT sample-entry children and the scope of TTML and WebVTT timing |
| [iso-iec-23003-5.md](iso-iec-23003-5.md) | ISO/IEC 23003-5, uncompressed audio in ISO base media | The PCM configuration box |
| [iso-iec-14496-3.md](iso-iec-14496-3.md) | ISO/IEC 14496-3, MPEG-4 Audio | AudioSpecificConfig; LATM/LOAS |
| [iso-iec-13818-7.md](iso-iec-13818-7.md) | ISO/IEC 13818-7, AAC | ADTS and ADIF headers |
| [iso-iec-11172-3.md](iso-iec-11172-3.md) | ISO/IEC 11172-3, MPEG-1 audio | The MPEG audio frame header |
| [iso-iec-13818-3.md](iso-iec-13818-3.md) | ISO/IEC 13818-3, MPEG-2 audio | Low-sampling-frequency frame headers and Layer III duration |
| [iso-iec-14496-2.md](iso-iec-14496-2.md) | ISO/IEC 14496-2, MPEG-4 Visual | Visual object sequence and video object layer headers |
| [iso-iec-11172-2.md](iso-iec-11172-2.md) | ISO/IEC 11172-2, MPEG-1 video | The sequence header |
| [iso-iec-11172-1.md](iso-iec-11172-1.md) | ISO/IEC 11172-1, MPEG-1 systems | The MPEG-1 pack header of program streams |
| [iec-61834-2.md](iec-61834-2.md) | IEC 61834-2, DV | The DIF structure and the auxiliary packs |
