# RFCs

The RFCs that this project builds on, as Appendix A of the [development plan](../plans/plan-init-project.md#appendix-a--rfc-list) lists them. Each `rfcNNNN.txt` is the RFC Editor's plain-text copy, byte for byte. [manifest.tsv](manifest.tsv) records where each one came from, its SHA-256 and the day it was retrieved, and `node scripts/check.ts` verifies every file against it.

[errata.tsv](errata.tsv) lists every verified erratum of these RFCs, and every erratum held for document update that affects parsing, each with its relevance to this project and the components it affects (plan section 5.3). Code and fixtures that depend on a corrected rule cite the erratum.

This file is generated from the manifest by `node scripts/fetch-rfcs.ts`; do not edit it by hand. The same script downloads a missing RFC, and reports what changed at the RFC Editor: an RFC that was obsoleted, a title or status that changed, an updating RFC that Appendix A does not list, and a new erratum.

## Index

| RFC | Title | Status | Role |
|---|---|---|---|
| [1950](rfc1950.txt) | ZLIB Compressed Data Format Specification version 3.3 | Informational | Compressed headers in Matroska and QuickTime |
| [1951](rfc1951.txt) | DEFLATE Compressed Data Format Specification version 1.3 | Informational | Compressed headers in Matroska and QuickTime |
| [2046](rfc2046.txt) | Multipurpose Internet Mail Extensions (MIME) Part Two: Media Types | Draft Standard | Media type syntax |
| [2083](rfc2083.txt) | PNG (Portable Network Graphics) Specification Version 1.0 | Historic | Cover-art dimension sniffing |
| [2119](rfc2119.txt) | Key words for use in RFCs to Indicate Requirement Levels | Best Current Practice | Reading requirement language in the listed RFCs |
| [2250](rfc2250.txt) | RTP Payload Format for MPEG1/MPEG2 Video | Proposed Standard | MPEG video headers; systems streams |
| [2361](rfc2361.txt) | WAVE and AVI Codec Registries | Informational | WAVE format tags and video four-character codes |
| [2435](rfc2435.txt) | RTP Payload Format for JPEG-compressed Video | Proposed Standard | JPEG frame parameters |
| [2586](rfc2586.txt) | The Audio/L16 MIME content type | Informational | 16-bit linear PCM |
| [2658](rfc2658.txt) | RTP Payload Format for PureVoice(tm) Audio | Proposed Standard | QCELP identification in 3G2 |
| [2781](rfc2781.txt) | UTF-16, an encoding of ISO 10646 | Informational | Text decoding |
| [3003](rfc3003.txt) | The audio/mpeg Media Type | Proposed Standard | MPEG audio identification |
| [3190](rfc3190.txt) | RTP Payload Format for 12-bit DAT Audio and 20- and 24-bit Linear Sampled Audio | Proposed Standard | Linear PCM variants |
| [3339](rfc3339.txt) | Date and Time on the Internet: Timestamps | Proposed Standard | Date output format |
| [3533](rfc3533.txt) | The Ogg Encapsulation Format Version 0 | Informational | Ogg pages and logical streams |
| [3551](rfc3551.txt) | RTP Profile for Audio and Video Conferences with Minimal Control | Internet Standard | Baseline audio encodings vocabulary |
| [3558](rfc3558.txt) | RTP Payload Format for Enhanced Variable Rate Codecs (EVRC) and Selectable Mode Vocoders (SMV) | Proposed Standard | EVRC and SMV identification in 3G2 |
| [3625](rfc3625.txt) | The QCP File Format and Media Types for Speech Data | Informational | QCELP media types and codec identification |
| [3629](rfc3629.txt) | UTF-8, a transformation format of ISO 10646 | Internet Standard | Text decoding |
| [3640](rfc3640.txt) | RTP Payload Format for Transport of MPEG-4 Elementary Streams | Proposed Standard | AudioSpecificConfig usage |
| [3839](rfc3839.txt) | MIME Type Registrations for 3rd Generation Partnership Project (3GPP) Multimedia files | Proposed Standard | 3GP media types |
| [3986](rfc3986.txt) | Uniform Resource Identifier (URI): Generic Syntax | Internet Standard | External data references |
| [4175](rfc4175.txt) | RTP Payload Format for Uncompressed Video | Proposed Standard | Sampling and depth vocabulary |
| [4184](rfc4184.txt) | RTP Payload Format for AC-3 Audio | Proposed Standard | AC-3 frame structure |
| [4337](rfc4337.txt) | MIME Type Registration for MPEG-4 | Proposed Standard | MP4 media types |
| [4352](rfc4352.txt) | RTP Payload Format for the Extended Adaptive Multi-Rate Wideband (AMR-WB+) Audio Codec | Proposed Standard | AMR-WB+ identification in 3GP |
| [4393](rfc4393.txt) | MIME Type Registrations for 3GPP2 Multimedia Files | Proposed Standard | 3G2 media types |
| [4396](rfc4396.txt) | RTP Payload Format for 3rd Generation Partnership Project (3GPP) Timed Text | Proposed Standard | Timed-text sample description |
| [4421](rfc4421.txt) | RTP Payload Format for Uncompressed Video: Additional Colour Sampling Modes | Proposed Standard | Updates RFC 4175 |
| [4425](rfc4425.txt) | RTP Payload Format for Video Codec 1 (VC-1) | Proposed Standard | VC-1 sequence layer, profiles |
| [4539](rfc4539.txt) | Media Type Registration for the Society of Motion Picture and Television Engineers (SMPTE) Material Exchange Format (MXF) | Informational | MXF media type |
| [4598](rfc4598.txt) | Real-time Transport Protocol (RTP) Payload Format for Enhanced AC-3 (E-AC-3) Audio | Proposed Standard | E-AC-3 frame structure |
| [4629](rfc4629.txt) | RTP Payload Format for ITU-T Rec. H.263 Video | Proposed Standard | H.263 picture header and profiles |
| [4647](rfc4647.txt) | Matching of Language Tags | Best Current Practice | Language lookup helpers |
| [4648](rfc4648.txt) | The Base16, Base32, and Base64 Data Encodings | Proposed Standard | Embedded pictures in comments; hex rendering |
| [4867](rfc4867.txt) | RTP Payload Format and File Storage Format for the Adaptive Multi-Rate (AMR) and Adaptive Multi-Rate Wideband (AMR-WB) Audio Codecs | Proposed Standard | AMR file format; AMR in 3GP |
| [5215](rfc5215.txt) | RTP Payload Format for Vorbis Encoded Audio | Proposed Standard | Vorbis configuration headers |
| [5219](rfc5219.txt) | A More Loss-Tolerant RTP Payload Format for MP3 Audio | Proposed Standard | MP3 frame structure |
| [5334](rfc5334.txt) | Ogg Media Types | Proposed Standard | Ogg media types and codec identification |
| [5484](rfc5484.txt) | Associating Time-Codes with RTP Streams | Proposed Standard | SMPTE time-code fields, for QuickTime timecode tracks |
| [5574](rfc5574.txt) | RTP Payload Format for the Speex Codec | Proposed Standard | Speex parameters |
| [5646](rfc5646.txt) | Tags for Identifying Languages | Best Current Practice | Language tags on streams, chapters and tags |
| [5691](rfc5691.txt) | RTP Payload Format for Elementary Streams with MPEG Surround Multi-Channel Audio | Proposed Standard | MPEG Surround signaling |
| [6184](rfc6184.txt) | RTP Payload Format for H.264 Video | Proposed Standard | NAL unit header, parameter sets, profile-level-id |
| [6190](rfc6190.txt) | RTP Payload Format for Scalable Video Coding | Proposed Standard | H.264 scalable extension NAL header |
| [6381](rfc6381.txt) | The 'Codecs' and 'Profiles' Parameters for "Bucket" Media Types | Proposed Standard | Codec strings, reported for the codecs that have one |
| [6386](rfc6386.txt) | VP8 Data Format and Decoding Guide | Informational | VP8 frame header |
| [6416](rfc6416.txt) | RTP Payload Format for MPEG-4 Audio/Visual Streams | Proposed Standard | LATM, StreamMuxConfig, MPEG-4 Visual configuration |
| [6469](rfc6469.txt) | RTP Payload Format for DV (IEC 61834) Video | Proposed Standard | DV DIF structure |
| [6716](rfc6716.txt) | Definition of the Opus Audio Codec | Proposed Standard | Opus packet structure |
| [6838](rfc6838.txt) | Media Type Specifications and Registration Procedures | Best Current Practice | Media type naming rules |
| [7493](rfc7493.txt) | The I-JSON Message Format | Proposed Standard | Exactly representable number range in the JSON output |
| [7587](rfc7587.txt) | RTP Payload Format for the Opus Speech and Audio Codec | Proposed Standard | Opus parameters |
| [7741](rfc7741.txt) | RTP Payload Format for VP8 Video | Proposed Standard | VP8 payload header |
| [7798](rfc7798.txt) | RTP Payload Format for High Efficiency Video Coding (HEVC) | Proposed Standard | NAL unit header, parameter sets, profile-tier-level |
| [7845](rfc7845.txt) | Ogg Encapsulation for the Opus Audio Codec | Proposed Standard | Opus identification and comment headers |
| [7903](rfc7903.txt) | Windows Image Media Types | Informational | Media type of BMP cover art |
| [8081](rfc8081.txt) | The "font" Top-Level Media Type | Proposed Standard | Font attachments |
| [8174](rfc8174.txt) | Ambiguity of Uppercase vs Lowercase in RFC 2119 Key Words | Best Current Practice | Updates RFC 2119 |
| [8216](rfc8216.txt) | HTTP Live Streaming | Informational | The timestamp-map header that this protocol adds to a WebVTT header (its section 3.5). Nothing else of the protocol is used |
| [8251](rfc8251.txt) | Updates to the Opus Audio Codec | Proposed Standard | Amendments to RFC 6716 |
| [8259](rfc8259.txt) | The JavaScript Object Notation (JSON) Data Interchange Format | Internet Standard | JSON output |
| [8486](rfc8486.txt) | Ambisonics in an Ogg Opus Container | Proposed Standard | Opus channel mapping families 2 and 3 |
| [8794](rfc8794.txt) | Extensible Binary Meta Language | Proposed Standard | EBML structure underlying Matroska |
| [9043](rfc9043.txt) | FFV1 Video Coding Format Versions 0, 1, and 3 | Informational | FFV1 configuration record and key-frame parameters |
| [9328](rfc9328.txt) | RTP Payload Format for Versatile Video Coding (VVC) | Proposed Standard | VVC NAL unit header and parameter sets |
| [9557](rfc9557.txt) | Date and Time on the Internet: Timestamps with Additional Information | Proposed Standard | Updates RFC 3339 |
| [9559](rfc9559.txt) | Matroska Media Container Format Specification | Proposed Standard | Matroska and WebM |
| [9562](rfc9562.txt) | Universally Unique IDentifiers (UUIDs) | Proposed Standard | Extended box types, GUID-keyed objects |
| [9628](rfc9628.txt) | RTP Payload Format for VP9 Video | Proposed Standard | VP9 frame structure |
| [9639](rfc9639.txt) | Free Lossless Audio Codec (FLAC) | Proposed Standard | Native FLAC, FLAC in Ogg, FLAC headers in other containers |
| [9649](rfc9649.txt) | WebP Image Format | Informational | WebP container and dimensions, for cover-art inspection |
| [9694](rfc9694.txt) | Guidelines for the Definition of New Top-Level Media Types | Best Current Practice | Updates RFC 6838 |
| [9924](rfc9924.txt) | Advanced Professional Video | Informational | APV identification and header |

## License notice

The RFC texts in this directory are not covered by the project's Apache-2.0 license. No file here is modified, so each keeps its own copyright notice and legal text as published.

- RFCs published from November 10, 2008 on are copyright the IETF Trust and the persons identified as their authors. They are distributed here under the [IETF Trust Legal Provisions](https://trustee.ietf.org/license-info), which permit anyone to copy, publish, display and distribute them in full and without modification: section 3.c.i for the IETF stream, and section 8.c.iii for the Independent Submission, IAB and IRTF streams.
- Earlier RFCs carry the copyright notice of the Internet Society or of the IETF Trust, which permits copying and distributing them unmodified with the notice kept, or state that their distribution is unlimited.
- RFC 1950 and RFC 1951 carry their authors' copyright, with permission to copy and distribute them for any purpose, provided that the notice is kept and any change is marked.
