# Descriptions of the paid standards

The 16 ISO/IEC and IEC standards of [`../manifest.tsv`](../manifest.tsv) that must be bought are not used by this project. Each file here describes, in this project's own words, the parts of one of them that the deliverables need, as free public sources document them. A description is a `descriptive` source (plan section 5.4), proposed for approval at G1.6. Behavior taken from it is labeled as descriptive in code comments and in `formats.md`.

- **Nothing is copied from the standards.** Field names are given as the standards spell them, because code comments cite them (plan section 5.5); layouts and meanings are restated from the sources.
- **Every statement names its source,** an identifier of [`../manifest.tsv`](../manifest.tsv) or an RFC. Where a source is a community description, that is said.
- **Implementations are never a source.** Search results that pointed to an implementation's code were not opened (plan section 1.3).
- **Gaps are stated.** What no free source covers is listed at the end of each file, with what the project does instead.

A coverage row cites a description as `<file name>: <section>`, for example `iso-iec-14496-15.md: 2`. A row that needs a structure no free source covers cites the description's gaps, `iso-iec-14496-15.md: Gaps`; G1.6 decides such a row.

| File | Standard | What the project needs from it |
|---|---|---|
| [iso-iec-14496-12.md](iso-iec-14496-12.md) | ISO/IEC 14496-12, ISO base media file format | Box structure, movie and track headers, sample tables, fragments |
| [iso-iec-14496-15.md](iso-iec-14496-15.md) | ISO/IEC 14496-15, NAL-unit video in ISO base media | The AVC, HEVC and VVC configuration records; length-prefixed samples |
| [iso-iec-14496-15-amd-1.md](iso-iec-14496-15-amd-1.md) | Amendment 1 to ISO/IEC 14496-15 | Nothing the deliverables use |
| [iso-iec-14496-1.md](iso-iec-14496-1.md) | ISO/IEC 14496-1, Systems | Elementary stream and decoder configuration descriptors |
| [iso-iec-14496-14.md](iso-iec-14496-14.md) | ISO/IEC 14496-14, MP4 file format | MPEG-4 sample entries and the elementary stream descriptor box |
| [iso-iec-14496-30.md](iso-iec-14496-30.md) | ISO/IEC 14496-30, timed text in ISO base media | WebVTT and TTML sample entries |
| [iso-iec-14496-30-amd-1.md](iso-iec-14496-30-amd-1.md) | Amendment 1 to ISO/IEC 14496-30 | Nothing the deliverables use |
| [iso-iec-23003-5.md](iso-iec-23003-5.md) | ISO/IEC 23003-5, uncompressed audio in ISO base media | The PCM configuration box |
| [iso-iec-14496-3.md](iso-iec-14496-3.md) | ISO/IEC 14496-3, MPEG-4 Audio | AudioSpecificConfig; LATM/LOAS |
| [iso-iec-13818-7.md](iso-iec-13818-7.md) | ISO/IEC 13818-7, AAC | ADTS and ADIF headers |
| [iso-iec-11172-3.md](iso-iec-11172-3.md) | ISO/IEC 11172-3, MPEG-1 audio | The MPEG audio frame header |
| [iso-iec-13818-3.md](iso-iec-13818-3.md) | ISO/IEC 13818-3, MPEG-2 audio | The lower sampling rates of the frame header |
| [iso-iec-14496-2.md](iso-iec-14496-2.md) | ISO/IEC 14496-2, MPEG-4 Visual | Visual object sequence and video object layer headers |
| [iso-iec-11172-2.md](iso-iec-11172-2.md) | ISO/IEC 11172-2, MPEG-1 video | The sequence header |
| [iso-iec-11172-1.md](iso-iec-11172-1.md) | ISO/IEC 11172-1, MPEG-1 systems | The MPEG-1 pack header of program streams |
| [iec-61834-2.md](iec-61834-2.md) | IEC 61834-2, DV | The DIF structure and the auxiliary packs |
