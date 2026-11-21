# Source dossier

The input to gate G1.5 (plan sections 5.4 and 14): for every entry of plan section 4 that is marked † because no public normative specification exists, the public descriptions that were found, what they cover and leave open, whether one is hosted inside an implementation's source repository, and the disposition proposed for it. It starts from Appendix B.6 of the [development plan](../plans/plan-init-project.md#b6-candidate-sources-for-formats-without-a-normative-specification).

| | |
|---|---|
| Status | Decided at G1.5 on 2026-11-21. Every proposal was accepted. At the maintainer's request, HDMV TextST and the VBRI layout were researched further and their sources approved, and a source was added for the DVD sub-stream numbers. The [development log](../plans/plan-init-project-log.md) records the decisions. |
| Researched | 2026-11-21 |
| Sources read | The prose of each candidate only. No implementation's code was opened, and links to code were not followed (plan section 1.3). |
| Where they are | Each source is a row of [`docs/specs/manifest.tsv`](../specs/manifest.tsv) with the class `descriptive`; its `approved` cell names G1.5, or GS.2 for the sources found during the gate. `node scripts/fetch-specs.ts` stores it under `docs/specs/files/`. Wiki pages are pinned to a revision of their source, other living pages to an Internet Archive capture. |

**Dispositions** (plan section 5.4, R17): *approve* the named description as a `descriptive` source; reduce a format to *detection only*; reduce a codec to *identification only*; *defer* to after 1.0; or, for detect-only types, *drop* from the inventory or keep as *hint only*. A descriptive source is never treated as normative: behavior taken from it is labeled in code comments and in `formats.md`.

## Summary

| Entry | Proposed source (manifest identifier) | Proposed disposition | Plan fallback |
|---|---|---|---|
| OGM | None usable | Detection only, as Ogg with an `ogm` hint | Detect-only |
| IVF | `multimedia-wiki-ivf` | Approve | Detect-only |
| RealMedia container | `realnetworks-rmff` | Approve | Container-level reporting, or detect-only |
| RealAudio file headers and type-specific data | `multimedia-wiki-realmedia` | Approve | Detect-only |
| RealVideo type-specific data | None located | Identification only; in RealMedia, container-level reporting (task 17) | Container-level reporting |
| AAC in RealMedia | None located | Identification only | Identified only in RealMedia |
| Windows Media Audio extra data | None usable; for 0x0161, `microsoft-asf-specification` (task 17) | Wave-format fields only; for 0x0161, its three fields (task 17) | Wave-format fields only |
| Windows Media Audio version 1 format tag (task 17) | `multimedia-wiki-twocc` | Approved through GS.2 | Reported as an unknown format tag |
| Wave64 | `multimedia-wiki-wave64` | Approve | Detect-only |
| TTA | `tausoft-tta` | Approve | Detect-only |
| ALAC configuration | `apple-alac-magic-cookie` | Approve | Container-declared values only |
| SubRip | `loc-subrip`, with the Matroska subtitle mapping | Approve | Detect-only |
| SSA/ASS | `kotus-ssa-v4-plus`, with the Matroska subtitle mapping | Approve | Detect-only |
| USF | `titlevision-usf`; for the XML syntax, `w3c-xml-1-0` (task 17) | Approve | Detect-only |
| VobSub index and DVD sub-pictures | `dvdinfo-spu`, with the Matroska subtitle mapping | Approve | Detect-only |
| PGS | `thescorpius-pgs` | Approve; no frame rate (task 17) | Detect-only |
| HDMV TextST | `jpo-jp-5339002`, with `uspto-us-8326118` | Approve; the fields are checked against the segment length; an event count and no time range (task 17) | Detect-only, or defer |
| XSUB | `multimedia-wiki-xsub` | Approve | Identification only |
| Blu-ray playlists and clip information | Partial notes only | Defer the Blu-ray part of Phase 15 | Defer Phase 15 |
| Blu-ray transport streams: 192-byte packets and stream types (task 17) | `bda-bd-j-baseline-application`, `uspto-us-8208795` | Approved through GS.2; TrueHD, E-AC-3 and DTS-HD types not covered | 188-byte packets only; Blu-ray types reported as user private |
| DVD LPCM headers | `dvdinfo-lpcm` | Approve | Container-declared values only |
| Blu-ray LPCM headers | None located | Container-declared values only | Container-declared values only |
| DVD private-stream sub-stream numbers | `dvdinfo-dvdmpeg`, since task 17 the author's current page | Approve for the ranges, which give DTS its own since task 17; an audio sub-stream's codec is identified by its sync word | Container-declared values only |
| Kate | `xiph-wiki-kate` | Identification only | Identification only |
| Chapter comments in Vorbis comments | `xiph-wiki-chapter-extension` | Approve | Chapter comments stay ordinary tags |
| Dirac in Ogg | `dirac-ogg-mapping` | Approve | Identified only in Ogg |
| MPEG audio VBR headers | `mp3-tech-info-tag` for the Xing/Info layout; `codeproject-mpeg-audio-frame-header` for VBRI and, since task 17, for where the Xing/Info header starts | Approve; for VBRI only the identifier, version, byte count and frame count | No VBR header is read |
| TrueHD interleaved with AC-3 (task 17) | `uspto-us-8086331` | Approved through GS.2; each frame read by its own sync word and length | Pure TrueHD and MLP files only |
| WavPack standard sample rates (task 17) | None located | Rate from the stored WAVE header, otherwise not available | Not available |
| ID3v2 tags in WAVE (task 17) | None located for the chunk identifier | Recognized by the ID3v2 header at the start of a chunk's data | Reported as an unknown chunk |
| Dolby Vision metadata in raw video byte streams (task 17) | `uspto-us-10462491` for HEVC; none located for H.264, H.266 or enhancement-layer units | Approved through GS.2; in HEVC, a NAL unit whose header bytes are 0x7C01 reports Dolby Vision metadata | Not detected in raw streams |
| IVF codec codes for VP9 and AV1 (task 17) | None located | Codec identified from the content of the first frame | Code reported raw, codec unidentified |
| Windows Media Video 7/8 extra data | None located | Bitmap header fields only | Bitmap header fields only |
| VobSub palette in MP4 | None located | Identification only | Identified only in MP4 |
| ID3v1 | `id3-v1` | Approve | Presence reported, content not read |
| DVD-Video information files | `dvdinfo-ifo` (since task 17 the author's current page), `dvdinfo-ifo-vts`, `dvdinfo-pgc` | Approve | Defer |
| VobButtons | `dvdinfo-pci`, with the pinned Matroska codec draft | Matroska identification; `btn` files hint only | Matroska identification only |
| MicroDVD | `multimedia-wiki-microdvd` | Detection only | Dropped from the inventory |
| RIFF CD-XA, HD-DVD subtitles, Windows Television, Internet Video Recording, RealAudio metafiles | None located | Drop from the inventory | Dropped from the inventory |

## Entries

### Containers

**OGM.** The only candidate, a MultimediaWiki page last edited in 2006, is a one-sentence stub: it says that OGM is an incompatible variant of Ogg and documents no structure. No other public description of the OGM stream headers was found. *Proposed:* detection only. An OGM file is read as Ogg; its streams whose headers are OGM's own are reported as of unknown codec, and the `ogm` extension orders the candidates as a hint (plan section 4.4).

**IVF.** `multimedia-wiki-ivf` (revision 15981, 2025) gives the 32-byte file header (signature, version, header length, FourCC, width, height, time base, frame count) and the 12-byte frame header (size and 64-bit timestamp), little-endian. It leaves open what values other than 0 mean in the version field, and does not constrain the header length. The page links to sample files and test vectors, which this project does not use (R8). *Proposed:* approve.

*Added in task 17:* the page names the codec code only by its example `VP80`, for VP8. No document gives the codes of VP9 and AV1, or says that AV1 frames in IVF are temporal units in the low-overhead format; only implementations do, and they are not sources (plan section 1.3). The maintainer decided that any other code is reported as found, and that the codec is identified only when the first frame matches exactly one of a VP8 key frame (RFC 6386, section 9.1), a VP9 key frame with its frame marker and sync code (VP9 sections 6.2 and 7.2), or an AV1 temporal unit that starts with a temporal delimiter (AV1 sections 5.2, 5.3 and 7.5).

**RealMedia.** Two sources together cover the container:

- `realnetworks-rmff` is the vendor's own reference, "Appendix E: RealMedia File Format (RMFF) Reference" of its 2003 SDK documentation, stored from a capture of 2015. It specifies the file header, the properties and media-properties headers, logical streams, the content description, the data and index sections and the metadata section. It does not specify the type-specific data that each codec puts into the media-properties header.
- `multimedia-wiki-realmedia` (revision 15732, 2023) adds the RealAudio file headers of versions 3 and 4 (version 5 partly, with fields marked unknown) and the RealAudio type-specific data. It lists the RealVideo FourCCs without their type-specific data, and says that its description of data packets comes from working demuxer code; for that section the vendor reference is preferred.

*Proposed:* approve both for what they cover; RealVideo and AAC in RealMedia are reduced to identification only, with container-declared values, because no description of their type-specific data was found. This changes the RealVideo rows of plan section 4.3.

*Added in task 17:* neither source says where the codec code of a RealVideo stream sits in its type-specific data, and neither names the MIME type of a video stream; the wiki's RealVideo page gives no layout either. The maintainer decided that such a stream is reported at container level: its kind from the top-level type of its MIME type string, its declared bit rates and duration, and no codec name. AAC streams are identified by the codes `raac` and `racp` of their RealAudio type-specific data, which the wiki lists.

**Wave64.** `multimedia-wiki-wave64` (revision 15287, 2017) gives the chunk header (a 128-bit GUID and a 64-bit size that includes the header) and the GUIDs of the known chunks. The vendor's white paper it links to no longer answers and has no archive capture. The payload of the format chunk is a wave format structure, which the approved Microsoft documents describe. *Proposed:* approve.

**DVD-Video information files.** `dvdinfo-ifo`, `dvdinfo-ifo-vts` and `dvdinfo-pgc` give the byte layouts of the video manager and title set headers, their attributes, and program chains. The cell and time tables are only partly described. *Proposed:* approve for Phase 15, where the title-set reader is limited to what they describe.

*Added in task 17:* the stored `dvdinfo-ifo` was an older copy on dvd.sourceforge.net. The author's current page on mpucoder.com, stored from an archive capture of 2026-08-07, replaced it through GS.2. It gives the sample rate as 48 or 96 kilosamples per second where the older copy read "48Kbps", corrects the offset of a reserved field to 0x0244, adds "not specified" to the aspect values and the reserved resolutions, and separates the title attribute tables from the menu ones. The program-chain and title-set pages of the author's site differ from the stored copies only in fields that Phase 15 does not read, so they stay. The maintainer also decided two readings that the pages leave open. Chapters count an angle block by its first cell, because the cells of a block are alternative angles. Each attribute entry is matched to its stream in the object files through the stream control tables of the program chain, with the sub-stream ranges of `dvdinfo-dvdmpeg`.

**Blu-ray playlists and clip information.** A wiki user page documents the playlist header and marks but omits the play items and the stream number table, and marks the extension data as unknown. A published patent describes the mark types. Nothing was found for clip information files. *Proposed:* defer the Blu-ray part of Phase 15, as the plan's fallback says.

**Blu-ray transport streams** (*added in task 17*). The verification of the transport-stream rows found that no approved source covered the 192-byte packets of BDAV streams or the Blu-ray stream types, which ITU-T H.222.0 leaves in its user-private range. Two sources were found and approved by the maintainer through GS.2:

- `bda-bd-j-baseline-application`, the Blu-ray Disc Association's "BD-J Baseline Application and Logical Model Definition for BD-ROM" (March 2005), still served by the association. Its section 4.1 gives the source packet: a 4-byte `TP_extra_header`, then the 188-byte transport packet, 32 of them to an aligned unit of 6144 bytes. It does not divide the header into fields, so the header is skipped.
- `uspto-us-8208795`, a published patent (Panasonic, 2012). Its description of FIGS. 9A to 9E gives the stream coding types of a playlist's stream table: 0x80 LPCM, 0x81 AC-3 and 0x82 DTS for audio, 0x90 presentation graphics, 0x91 interactive graphics and 0x92 text subtitles. No source found says that the program map uses the same values; the maintainer decided that these values are read as stream types in 192-byte streams only.

No source was found for the types of TrueHD, E-AC-3 or DTS-HD. The only page that lists them is an anonymous programming site that names no sources and gives a text-subtitle value that contradicts the patent; the other lists come from implementations, which are not sources (plan section 1.3). TrueHD in transport streams is deferred to G17.n.

### Video

**Dolby Vision metadata in raw video byte streams** (*added in task 17*). Phase 12 asks raw H.264, HEVC and VVC streams to detect dynamic-range enhancement units. The approved Dolby Vision documents for ISO base media files and transport streams say only that the metadata unit (RPU) is "a special NAL unit" of AVC and HEVC streams; Dolby's profiles document says "unspecified NAL units". The published patent `uspto-us-10462491` (Dolby, 2019) describes the HEVC enhancement-layer stream of a dual-layer stream, a byte stream of NAL units, in its Table 1: a unit whose first two bytes after the start code are 0x7C01 (HEVC type 62, which H.265 leaves unspecified) is Dolby Vision metadata, and every other unit is enhancement-layer video. The maintainer approved it through GS.2, and decided that such a unit in any HEVC byte stream reports Dolby Vision metadata as present. No source was found for H.264, H.266, or enhancement-layer units carried in the same stream as the base layer. Only implementations give those, and they are not sources (plan section 1.3). Their detection is deferred to G17.n.

### Audio

**TTA.** `tausoft-tta` is the format authors' own "Detailed description of the TTA multichannel audio data format", with the header structure and the frame structure. Their site refused connections on 2026-11-21, so it is stored from a capture of 2026-04-17. The MultimediaWiki page is a stub. *Proposed:* approve.

**ALAC configuration.** `apple-alac-magic-cookie` is Apple's own text description of the configuration ("magic cookie"): the 24-byte specific configuration, the optional channel layout, and how it is wrapped in MP4 sample entries and in CAF. It is hosted in Apple's repository of the codec's implementation; only this file is opened. *Proposed:* approve.

**Windows Media Audio extra data.** The MultimediaWiki page gives sizes for the extra data of versions 1 and 2 and some of their flag bits, and nothing for Professional, Lossless or Voice. *Proposed:* the fallback, wave-format fields only; the extra data is kept as raw bytes.

*Added in task 17:* the approved ASF specification, section 11.1.1, defines the type-specific data of format tag 0x0161 as the wave format followed by `dwSamplesPerBlock`, `wEncodeOptions` and `dwSuperBlockAlign`, without the meaning of the option bits. The maintainer decided that these three fields are read for 0x0161, in every carriage. The format tags come from Microsoft's page of audio subtypes, `microsoft-audio-subtype-guids`, which lists 0x000A and 0x0161 to 0x0163. No Microsoft document gives 0x0160, for version 1; `multimedia-wiki-twocc` (revision 16072, 2025), the wiki's list of format tags, does, and was approved through GS.2 for that value alone.

**MPEG audio VBR headers.** `mp3-tech-info-tag` describes the Xing/Info header and its extension, including encoder delay and padding. For the VBRI layout, the candidate article's current page is rendered by script, but a capture of 2012, `codeproject-mpeg-audio-frame-header`, holds its text. Its table gives the header 32 bytes after the end of the first frame header: the identifier `VBRI`, a 16-bit version, delay, quality, a 32-bit byte count at offset 10, a 32-bit frame count at offset 14, and the fields of its table of contents, all big-endian. Two rows contradict themselves: the delay is called a float but is 2 bytes long, and the scale factor is called a DWORD but is 2 bytes long. The article also ships code, which was not opened. No document from the originator of either layout was found. *Decided at G1.5:* approve `mp3-tech-info-tag` for Xing/Info, and the article for the VBRI identifier, version, byte count and frame count only.

*Added in task 17:* the Info tag page places the `Xing` string only "mostly at 0x24". The article says where the Xing/Info header starts: after the 4-byte frame header and the Layer III side information, whose sizes its table 2.2.1 gives (32 bytes for MPEG-1 stereo, 17 for MPEG-1 mono and MPEG-2 or 2.5 stereo, 9 for MPEG-2 or 2.5 mono). The maintainer approved the article through GS.2 for its sections 2.2 and 2.3.1 as well.

**TrueHD interleaved with AC-3** (*added in task 17*). The Dolby TrueHD description does not mention the variant. The published patent `uspto-us-8086331` (Panasonic, 2011) describes it with its FIG. 7 as the "DD/MLP" audio stream of BD-ROM: AC-3 and MLP audio frames are multiplexed in one primary audio stream, at very different rates, and the AC-3 frames are the base data. It gives no stream type and no exact order of the frames. The maintainer approved it through GS.2; each frame is read by its own sync word and length.

**WavPack standard sample rates** (*added in task 17*). The WavPack 5 format document gives the rate as a 4-bit index into "15 standard rates", 15 meaning a custom rate, and does not list them; neither does its text of 2009 nor the MultimediaWiki page, and the custom-rate sub-block has no documented layout. Only implementations list the rates, and they are not sources (plan section 1.3). The maintainer decided that a native file reports the rate from the WAVE header that its sub-block `ID_RIFF_HEADER` stores, and otherwise leaves it unavailable, with the index kept raw. Matroska stores the rate itself.

**ID3v2 tags in WAVE** (*added in task 17*). No document assigns the RIFF chunk that holds an ID3v2 tag. Community reports name `id3 `, `ID3 ` and `ID32`, and the Microsoft page on ID3 tags that an encyclopedia cites for WAVE does not mention it. The maintainer decided that a chunk is read as an ID3v2 tag when its data starts with a valid ID3v2 header whose tag size fits the chunk, whatever its identifier, which is reported as found.

**DVD LPCM headers.** `dvdinfo-lpcm` gives every field of the LPCM audio pack header with its bit layout. *Proposed:* approve for program streams. No description of the Blu-ray LPCM header was found, so LPCM in transport streams keeps container-declared values only.

### Subtitles

**SubRip.** `loc-subrip`, the Library of Congress description, states the cue structure and notes that the format has no standard encoding. The Matroska subtitle mapping, from the pinned codec draft, covers SubRip in Matroska. The MultimediaWiki page has examples only. *Proposed:* approve.

**SSA/ASS.** `kotus-ssa-v4-plus` is "Sub Station Alpha v4.00+ Script Format", the Word document of 2002 that describes the sections, style and event formats and embedded fonts, stored from a capture. With the Matroska subtitle mapping it covers both carriages. *Proposed:* approve.

**USF.** `titlevision-usf` is the USF specification with its DTD, as now hosted by Titlevision. The pinned Matroska codec draft cites version 1.1 of 2010 as its reference for USF. *Proposed:* approve.

*Added in task 17:* the specification describes the elements of a USF document, not the XML syntax they are written in. W3C XML 1.0 (Fifth Edition) of 2008, still the current edition, is a new `snapshot` row, `w3c-xml-1-0`, approved through GS.2 for that syntax and for the contract of G14.1: the prolog, comments, tags, references, the predefined entities and the encoding of a document.

**VobSub.** `dvdinfo-spu` gives the sub-picture unit: sizes, control sequences and display commands. It does not give the palette, which the Matroska subtitle mapping carries in its index-style private data, nor the index file. *Proposed:* approve; the index file is read only as far as the mapping describes the same text.

*Added in task 17:* that description is section 5.1 of the pinned codec draft. Its example index file starts with the line that names version 7, and gives the `size`, `palette`, `langidx`, `id`, `timestamp` and `filepos` lines.

**PGS.** `thescorpius-pgs` (2017, updated 2019) gives the segment header and the five segment types with their fields, and names the two patents it is based on. *Proposed:* approve. Its worked example comes from a real file and is not used as a fixture (R8).

*Added in task 17:* the article says that the frame-rate byte of the presentation composition segment is "Always 0x10. Can be ignored." and gives no meaning to its values, so no frame rate is reported.

**HDMV TextST.** Appendix B.6 found nothing, and the pinned Matroska codec draft cites the section of the Blu-ray specification, which is not public. Two published patents describe the stream:

- `jpo-jp-5339002`, Sony's JP 5339002 B2 (granted 2013), in Japanese. Its figures 11 to 17 are syntax tables with the size and mnemonic of every field: the stream (a dialog style segment, a 16-bit count of presentation segments, then the presentation segments), the segment descriptor (an 8-bit type, 0x81 for the style segment and 0x82 for a presentation segment, and a 16-bit length), and the dialog style set with its region styles and user styles. Its paragraphs 0083 to 0099 restate the sizes in prose. Rows that the figures shade, the font style and the two outline fields, are features of the invention, so the patent alone does not establish them as part of the disc format.
- `uspto-us-8326118`, LG's US 8,326,118 B2 (granted 2012), in English. Its figures 9A to 9D give the same structures by field name, without sizes, and its text explains each field.

*Decided at G1.5:* approve both, Sony's for the layout and LG's as corroboration. The parser checks the fields it reads against `segment_length`, and a segment whose fields do not add up gives a partial result.

*Added in task 17:* in a transport stream, the first PES packet of the stream is the style segment and the others are presentation segments (Sony's figure 10, LG's FIG. 6). LG's FIG. 8 lets the presentation segments run to the end of the file, where Sony's figure 11 puts a 16-bit count before them; Sony's layout applies. LG's FIG. 10 names the start and end timestamps of a presentation segment without their size, and Sony's patent does not describe that segment, so the probe reports the number of presentation segments and no time range.

**XSUB.** `multimedia-wiki-xsub` (revision 10542, 2008) gives the packet header: timestamp string, dimensions, coordinates, palette and the length of the run-length data. It links to an implementation's source, which was not opened. *Proposed:* approve.

**Kate.** `xiph-wiki-kate` describes the header packets at a high level, enough to identify a Kate stream. *Proposed:* identification only, as the fallback says.

**MicroDVD.** `multimedia-wiki-microdvd` (revision 15428) gives the line form, frame numbers in braces followed by text. *Proposed:* detection only, instead of dropping the type.

### Tags, chapters and other carriages

**Chapter comments.** `xiph-wiki-chapter-extension` defines the `CHAPTERxxx` and `CHAPTERxxxNAME` comments and their time format. *Proposed:* approve.

**Dirac in Ogg.** The codec project's wiki page points to the official mapping, `dirac-ogg-mapping`, a PDF of the Dirac project stored from a capture of 2012. *Proposed:* approve. As the codec project's own document it could be classed `normative` at G1.5.

**ID3v1.** `id3-v1` describes ID3v1 and ID3v1.1. *Proposed:* approve.

**ASF pictures** (*added in task 17*). Phase 13 maps ASF attributes to tags, including pictures. Microsoft's list of ASF attributes, approved through GS.2 as `microsoft-asf-attribute-list`, names the attributes and their data types, `WM/Picture` among them. Its `WM_PICTURE` structure is documented only in memory, with pointers, so no document gives the bytes a file stores. *Decided:* `WM/Picture` is kept as a binary tag, and no picture fields are decoded.

**VobButtons.** `dvdinfo-pci` gives the layout of presentation control packets, including the button table; the pinned Matroska codec draft names the track type. Nothing describes a signature of `btn` files. *Proposed:* identification in Matroska; `btn` files by extension hint only (R22).

### Nothing located

- **Windows Media Video 7/8 extra data:** bitmap header fields only.
- **VobSub palette in MP4:** the layout of its decoder-specific information is not described; identification only. No registry identifies the track: the MP4 registration authority lists stream type 0x38 and object type 0xE0 only within user-private ranges, does not register the handler `subp`, and registers `mp4s` only as generic MPEG-4 Systems. *Added in task 17:* `doom9-vobsub-in-mp4`, page 4 of a forum thread of 2006 whose posters examined such files, reports the handler `subp`, the sample entry `mp4s`, stream type 0x38, object type 0xE0, a 90 kHz timescale and samples that are DVD sub-picture bitmaps (posts 71, 77 and 80). The maintainer approved it through GS.2 for identification only: a track is VobSub when all four codes agree. A layout of the palette was found only on an implementation project's issue tracker, which is not a source here (plan section 1.3).
- **DVD private-stream sub-stream numbers:** the pages read first say that the first payload byte is a sub-stream number but list no values. During G1.5, the same site's page on how DVD differs from MPEG, `dvdinfo-dvdmpeg`, was found: 0x20 to 0x3F are sub-pictures, 0x80 to 0x87 audio (AC-3, DTS, SDDS) and 0xA0 to 0xA7 LPCM. *Decided at G1.5:* approve it for these ranges; an audio sub-stream's codec is identified by its sync word, which the approved ETSI specifications of AC-3 and DTS define, whatever its number. *Added in task 17:* the stored page was an older copy. The author's current page, stored from an archive capture of 2026-08-06, replaced it through GS.2: 0x80 to 0x87 are AC-3, 0x88 to 0x8F DTS, and SDDS is no longer listed. Identification by sync word stays; Phase 15 uses the ranges to find the stream of each attribute entry of a title set.
- **RIFF CD-XA, HD-DVD subtitles, Windows Television, Internet Video Recording, RealAudio metafiles:** no public description; drop from the inventory, as the fallback says.

## What G1.5 changes in the plan

G1.5 accepted every proposal, so task 16 updates the plan, together with the outcomes of G1.6:

- The RealVideo rows of section 4.3 become identification only, as do AAC in RealMedia, the VobSub palette in MP4 and Kate.
- OGM and MicroDVD stay as detect-only types; RIFF CD-XA, HD-DVD subtitles, Windows Television, Internet Video Recording and RealAudio metafiles leave the inventory.
- The Blu-ray part of Phase 15 is deferred.
- HDMV TextST keeps its parser in Phase 14, from the patent texts, and its deferred coverage rows become active.
- The VBRI layout is read for its byte and frame counts.
- Program streams from DVD identify their private streams by sub-stream number and, for audio, by sync word.

VobSub keeps its companion lookup in 0.7.0 (R18).
