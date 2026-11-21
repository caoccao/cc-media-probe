# Amendment 1 to ISO/IEC 14496-15: description from free sources

Amendment 1 (2025) to ISO/IEC 14496-15 ([README](README.md)). It is not the standard.

## 1. Scope

The 2025 amendment covers neural-network post-filter supplemental enhancement information (SEI) and other changes to NAL-unit video carriage. Revised clause 4.10 allows a visual sample entry to contain an optional `seii` box listing SEI message types present in the stream. The box carries identifiers. The messages themselves occur in the bitstream or configuration record. Its placement affects whether it can list messages required for playback. [ISO catalog](https://www.iso.org/standard/90272.html) · [Published amendment preview, clause 4.10](https://cdn.standards.iteh.ai/samples/90272/733ab439812a4edfa0f8aab5e18949d9/ISO-IEC-14496-15-2024-Amd-1-2025.pdf)

## Gaps

The current coverage table has no separate deliverable for post-filter SEI. The preview establishes the change to `seii`. It cannot establish that every configuration-record field in [the base description](iso-iec-14496-15.md) is unaffected by the amendment.

## Sources

`iso-iec-14496-15-amd-1` (published edition and linked preview).
