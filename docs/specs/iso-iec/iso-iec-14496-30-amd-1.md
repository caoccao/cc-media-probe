# Amendment 1 to ISO/IEC 14496-30: description from free sources

Amendment 1 (2022) to ISO/IEC 14496-30 ([README](README.md)). It is not the standard.

## 1. Scope

The 2022 amendment, *Timing improvements*, clarifies how sample timing, edit lists and TTML document timing interact. A document is active during its containing sample's interval, and presentation can be trimmed by an edit. It also addresses WebVTT cues whose end time is unknown. [ISO catalog](https://www.iso.org/standard/82615.html) · [Amendment, clauses 4.2, 5.3 and 6.3](https://cdn.standards.iteh.ai/samples/82615/e5c86c4bbf2b4232aa3a5675430c8d91/ISO-IEC-14496-30-2018-Amd-1-2022.pdf)

Clause 6.5 also revises the `wvtt` sample entry. The configuration box remains required, the source label is recommended, and a bitrate box is optional. This matters to the [WebVTT header reader](iso-iec-14496-30.md), which locates `vttC` by box type and size.

## Gaps

The current deliverables cover WebVTT header text and TTML identification. Cue rendering, applying TTML timing and resolving an indefinite cue's duration are outside those fields.

## Sources

`iso-iec-14496-30-amd-1` (published amendment, especially clauses 4.2, 5.3, 6.3 and 6.5).
