# Source dossier

The input to gate G1.5 (plan sections 5.4 and 14): for every entry of plan section 4 that is marked † because no public normative specification exists, the public descriptions that were found, what they cover and leave open, whether one is hosted inside an implementation's source repository, and the disposition proposed for it. It starts from Appendix B.6 of the [development plan](../plans/plan-init-project.md#b6-candidate-sources-for-formats-without-a-normative-specification).

| | |
|---|---|
| Status | Proposed for G1.5; nothing here is approved yet |
| Researched | 2026-11-21 |
| Sources read | The prose of each candidate only. No implementation's code was opened, and links to code were not followed (plan section 1.3). |
| Where they are | Each proposed source is a row of [`docs/specs/manifest.tsv`](../specs/manifest.tsv) with the class `descriptive` and an empty `approved` cell; `node scripts/fetch-specs.ts` stores it under `docs/specs/files/`. Wiki pages are pinned to a revision of their source, other living pages to an Internet Archive capture. |

**Dispositions** (plan section 5.4, R17): *approve* the named description as a `descriptive` source; reduce a format to *detection only*; reduce a codec to *identification only*; *defer* to after 1.0; or, for detect-only types, *drop* from the inventory or keep as *hint only*. A descriptive source is never treated as normative: behavior taken from it is labeled in code comments and in `formats.md`.

## Summary

| Entry | Proposed source (manifest identifier) | Proposed disposition | Plan fallback |
|---|---|---|---|
| OGM | None usable | Detection only, as Ogg with an `ogm` hint | Detect-only |
| IVF | `multimedia-wiki-ivf` | Approve | Detect-only |
| RealMedia container | `realnetworks-rmff` | Approve | Container-level reporting, or detect-only |
| RealAudio file headers and type-specific data | `multimedia-wiki-realmedia` | Approve | Detect-only |
| RealVideo type-specific data | None located | Identification only | Container-level reporting |
| AAC in RealMedia | None located | Identification only | Identified only in RealMedia |
| Windows Media Audio extra data | None usable | Wave-format fields only | Wave-format fields only |
| Wave64 | `multimedia-wiki-wave64` | Approve | Detect-only |
| TTA | `tausoft-tta` | Approve | Detect-only |
| ALAC configuration | `apple-alac-magic-cookie` | Approve | Container-declared values only |
| SubRip | `loc-subrip`, with the Matroska subtitle mapping | Approve | Detect-only |
| SSA/ASS | `kotus-ssa-v4-plus`, with the Matroska subtitle mapping | Approve | Detect-only |
| USF | `titlevision-usf` | Approve | Detect-only |
| VobSub index and DVD sub-pictures | `dvdinfo-spu`, with the Matroska subtitle mapping | Approve | Detect-only |
| PGS | `thescorpius-pgs` | Approve | Detect-only |
| HDMV TextST | Patent texts, not yet read | Defer, unless G1.5 approves a patent text | Detect-only, or defer |
| XSUB | `multimedia-wiki-xsub` | Approve | Identification only |
| Blu-ray playlists and clip information | Partial notes only | Defer the Blu-ray part of Phase 15 | Defer Phase 15 |
| DVD LPCM headers | `dvdinfo-lpcm` | Approve | Container-declared values only |
| Blu-ray LPCM headers; DVD private-stream sub-stream numbers | None located | Container-declared values only; open question below | Container-declared values only |
| Kate | `xiph-wiki-kate` | Identification only | Identification only |
| Chapter comments in Vorbis comments | `xiph-wiki-chapter-extension` | Approve | Chapter comments stay ordinary tags |
| Dirac in Ogg | `dirac-ogg-mapping` | Approve | Identified only in Ogg |
| MPEG audio VBR headers | `mp3-tech-info-tag` for the Xing/Info layout | Approve for Xing/Info; VBRI not read | No VBR header is read |
| Windows Media Video 7/8 extra data | None located | Bitmap header fields only | Bitmap header fields only |
| VobSub palette in MP4 | None located | Identification only | Identified only in MP4 |
| ID3v1 | `id3-v1` | Approve | Presence reported, content not read |
| DVD-Video information files | `dvdinfo-ifo`, `dvdinfo-ifo-vts`, `dvdinfo-pgc` | Approve | Defer |
| VobButtons | `dvdinfo-pci`, with the pinned Matroska codec draft | Matroska identification; `btn` files hint only | Matroska identification only |
| MicroDVD | `multimedia-wiki-microdvd` | Detection only | Dropped from the inventory |
| RIFF CD-XA, HD-DVD subtitles, Windows Television, Internet Video Recording, RealAudio metafiles | None located | Drop from the inventory | Dropped from the inventory |

## Entries

### Containers

**OGM.** The only candidate, a MultimediaWiki page last edited in 2006, is a one-sentence stub: it says that OGM is an incompatible variant of Ogg and documents no structure. No other public description of the OGM stream headers was found. *Proposed:* detection only. An OGM file is read as Ogg; its streams whose headers are OGM's own are reported as of unknown codec, and the `ogm` extension orders the candidates as a hint (plan section 4.4).

**IVF.** `multimedia-wiki-ivf` (revision 15981, 2025) gives the 32-byte file header (signature, version, header length, FourCC, width, height, time base, frame count) and the 12-byte frame header (size and 64-bit timestamp), little-endian. It leaves open what values other than 0 mean in the version field, and does not constrain the header length. The page links to sample files and test vectors, which this project does not use (R8). *Proposed:* approve.

**RealMedia.** Two sources together cover the container:

- `realnetworks-rmff` is the vendor's own reference, "Appendix E: RealMedia File Format (RMFF) Reference" of its 2003 SDK documentation, stored from a capture of 2015. It specifies the file header, the properties and media-properties headers, logical streams, the content description, the data and index sections and the metadata section. It does not specify the type-specific data that each codec puts into the media-properties header.
- `multimedia-wiki-realmedia` (revision 15732, 2023) adds the RealAudio file headers of versions 3 and 4 (version 5 partly, with fields marked unknown) and the RealAudio type-specific data. It lists the RealVideo FourCCs without their type-specific data, and says that its description of data packets comes from working demuxer code; for that section the vendor reference is preferred.

*Proposed:* approve both for what they cover; RealVideo and AAC in RealMedia are reduced to identification only, with container-declared values, because no description of their type-specific data was found. This changes the RealVideo rows of plan section 4.3.

**Wave64.** `multimedia-wiki-wave64` (revision 15287, 2017) gives the chunk header (a 128-bit GUID and a 64-bit size that includes the header) and the GUIDs of the known chunks. The vendor's white paper it links to no longer answers and has no archive capture. The payload of the format chunk is a wave format structure, which the approved Microsoft documents describe. *Proposed:* approve.

**DVD-Video information files.** `dvdinfo-ifo`, `dvdinfo-ifo-vts` and `dvdinfo-pgc` give the byte layouts of the video manager and title set headers, their attributes, and program chains. The cell and time tables are only partly described. *Proposed:* approve for Phase 15, where the title-set reader is limited to what they describe.

**Blu-ray playlists and clip information.** A wiki user page documents the playlist header and marks but omits the play items and the stream number table, and marks the extension data as unknown. A published patent describes the mark types. Nothing was found for clip information files. *Proposed:* defer the Blu-ray part of Phase 15, as the plan's fallback says.

### Audio

**TTA.** `tausoft-tta` is the format authors' own "Detailed description of the TTA multichannel audio data format", with the header structure and the frame structure. Their site refused connections on 2026-11-21, so it is stored from a capture of 2026-04-17. The MultimediaWiki page is a stub. *Proposed:* approve.

**ALAC configuration.** `apple-alac-magic-cookie` is Apple's own text description of the configuration ("magic cookie"): the 24-byte specific configuration, the optional channel layout, and how it is wrapped in MP4 sample entries and in CAF. It is hosted in Apple's repository of the codec's implementation; only this file is opened. *Proposed:* approve.

**Windows Media Audio extra data.** The MultimediaWiki page gives sizes for the extra data of versions 1 and 2 and some of their flag bits, and nothing for Professional, Lossless or Voice. *Proposed:* the fallback, wave-format fields only; the extra data is kept as raw bytes.

**MPEG audio VBR headers.** `mp3-tech-info-tag` describes the Xing/Info header and its extension, including encoder delay and padding. For the VBRI layout the candidate article could not be read: its page is rendered by script, and its capture holds no text. No document from the originator of either layout was found. *Proposed:* approve for Xing/Info; a file with a VBRI header gets a duration estimate only.

**DVD LPCM headers.** `dvdinfo-lpcm` gives every field of the LPCM audio pack header with its bit layout. *Proposed:* approve for program streams. No description of the Blu-ray LPCM header was found, so LPCM in transport streams keeps container-declared values only.

### Subtitles

**SubRip.** `loc-subrip`, the Library of Congress description, states the cue structure and notes that the format has no standard encoding. The Matroska subtitle mapping, from the pinned codec draft, covers SubRip in Matroska. The MultimediaWiki page has examples only. *Proposed:* approve.

**SSA/ASS.** `kotus-ssa-v4-plus` is "Sub Station Alpha v4.00+ Script Format", the Word document of 2002 that describes the sections, style and event formats and embedded fonts, stored from a capture. With the Matroska subtitle mapping it covers both carriages. *Proposed:* approve.

**USF.** `titlevision-usf` is the USF specification with its DTD, as now hosted by Titlevision. The pinned Matroska codec draft cites version 1.1 of 2010 as its reference for USF. *Proposed:* approve.

**VobSub.** `dvdinfo-spu` gives the sub-picture unit: sizes, control sequences and display commands. It does not give the palette, which the Matroska subtitle mapping carries in its index-style private data, nor the index file. *Proposed:* approve; the index file is read only as far as the mapping describes the same text.

**PGS.** `thescorpius-pgs` (2017, updated 2019) gives the segment header and the five segment types with their fields, and names the two patents it is based on. *Proposed:* approve. Its worked example comes from a real file and is not used as a fixture (R8).

**HDMV TextST.** Appendix B.6 found nothing. Published patents describe the dialog style segment and its region styles in syntax tables, and the pinned Matroska codec draft cites the section of the Blu-ray specification, which is not public. The patents were not read yet. *Proposed:* defer, unless G1.5 asks for a patent text to be read and approved.

**XSUB.** `multimedia-wiki-xsub` (revision 10542, 2008) gives the packet header: timestamp string, dimensions, coordinates, palette and the length of the run-length data. It links to an implementation's source, which was not opened. *Proposed:* approve.

**Kate.** `xiph-wiki-kate` describes the header packets at a high level, enough to identify a Kate stream. *Proposed:* identification only, as the fallback says.

**MicroDVD.** `multimedia-wiki-microdvd` (revision 15428) gives the line form, frame numbers in braces followed by text. *Proposed:* detection only, instead of dropping the type.

### Tags, chapters and other carriages

**Chapter comments.** `xiph-wiki-chapter-extension` defines the `CHAPTERxxx` and `CHAPTERxxxNAME` comments and their time format. *Proposed:* approve.

**Dirac in Ogg.** The codec project's wiki page points to the official mapping, `dirac-ogg-mapping`, a PDF of the Dirac project stored from a capture of 2012. *Proposed:* approve. As the codec project's own document it could be classed `normative` at G1.5.

**ID3v1.** `id3-v1` describes ID3v1 and ID3v1.1. *Proposed:* approve.

**VobButtons.** `dvdinfo-pci` gives the layout of presentation control packets, including the button table; the pinned Matroska codec draft names the track type. Nothing describes a signature of `btn` files. *Proposed:* identification in Matroska; `btn` files by extension hint only (R22).

### Nothing located

- **Windows Media Video 7/8 extra data:** bitmap header fields only.
- **VobSub palette in MP4:** the object type is registered, the layout of its decoder-specific information is not described; identification only.
- **DVD private-stream sub-stream numbers:** the located pages say that the first payload byte is a sub-stream number but list no values. Without them, program streams from DVD cannot tell their private streams apart by number. *Open question for G1.5:* look for a further source, or identify the codec of each private stream from its own sync pattern, which the approved codec specifications define.
- **RIFF CD-XA, HD-DVD subtitles, Windows Television, Internet Video Recording, RealAudio metafiles:** no public description; drop from the inventory, as the fallback says.

## What G1.5 changes in the plan

If the proposals are approved as written, task 16 updates the plan:

- The RealVideo rows of section 4.3 become identification only.
- MicroDVD stays as a detect-only type.
- The Blu-ray part of Phase 15 is deferred.
- The VBRI layout is not read.

VobSub keeps its companion lookup in 0.7.0 (R18).
