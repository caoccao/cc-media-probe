/*
* Copyright (c) 2026. caoccao.com Sam Cao
* All rights reserved.

* Licensed under the Apache License, Version 2.0 (the "License");
* you may not use this file except in compliance with the License.
* You may obtain a copy of the License at

* http://www.apache.org/licenses/LICENSE-2.0

* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific language governing permissions and
* limitations under the License.
*/

// Messages of the RFC checks and of the RFC fetch script.

/** Appendix A was not found in the plan. */
export const NO_APPENDIX = 'Appendix A was not found';
/** A row whose link names another RFC. */
export const LINK_MISMATCH = 'the link names another RFC than the label';
/** A status abbreviation that the legend does not define. */
export const UNKNOWN_STATUS = 'unknown status abbreviation';
/** An RFC listed twice. */
export const DUPLICATE_RFC = 'RFC listed twice';
/** An RFC of Appendix A missing from the manifest. */
export const MISSING_FROM_MANIFEST = 'in Appendix A but not in the manifest';
/** An RFC of the manifest missing from Appendix A. */
export const MISSING_FROM_APPENDIX = 'in the manifest but not in Appendix A';
/** A manifest cell that differs from Appendix A or from the derived value. */
export const DIFFERS = 'differs from the expected value';
/** A malformed number. */
export const MALFORMED_NUMBER = 'not a number';
/** A malformed checksum. */
export const MALFORMED_CHECKSUM = 'not a SHA-256 checksum in lower-case hexadecimal';
/** A malformed date. */
export const MALFORMED_DATE = 'not a date written as YYYY-MM-DD';
/** Rows out of order. */
export const OUT_OF_ORDER = 'rows are not in ascending order';
/** An RFC of the manifest without its text. */
export const MISSING_TEXT = 'the RFC text is missing; run `node scripts/fetch-rfcs.ts`';
/** An RFC text that the manifest does not list. */
export const UNLISTED_TEXT = 'RFC text that the manifest does not list';
/** A stored text whose checksum differs from the manifest. */
export const CHECKSUM_MISMATCH = 'the checksum differs from the manifest';
/** A stale index. */
export const STALE_README = 'differs from the index generated from the manifest; run `node scripts/fetch-rfcs.ts`';
/** An erratum of an RFC that the manifest does not list. */
export const UNLISTED_RFC = 'names an RFC that the manifest does not list';
/** An erratum listed twice. */
export const DUPLICATE_ERRATUM = 'erratum listed twice';
/** A cell outside its vocabulary. */
export const NOT_IN_VOCABULARY = 'is not one of';
/** A relevant erratum without components, or an irrelevant one with them. */
export const COMPONENTS_RULE = 'components are named exactly when the relevance is not `none`';
/** An empty summary. */
export const EMPTY_SUMMARY = 'the summary is empty';

/** A download that failed. */
export const DOWNLOAD_FAILED = 'download failed';
/** A response of another media type. */
export const WRONG_MEDIA_TYPE = 'unexpected media type';
/** A downloaded text that does not name its own number. */
export const NOT_THE_RFC = 'the downloaded text does not name this RFC near its top';
/** A text present without a checksum in the manifest. */
export const UNRECORDED_TEXT = 'present but not recorded in the manifest; delete it and run again';
/** Metadata that could not be read. */
export const UNREADABLE_METADATA = 'the RFC Editor metadata could not be read';
/** An RFC that the RFC Editor now lists as obsoleted. */
export const OBSOLETED = 'is obsoleted by';
/** A title that the RFC Editor now gives differently. */
export const CHANGED_TITLE = 'the RFC Editor now gives the title';
/** A status that the RFC Editor now gives differently. */
export const CHANGED_STATUS = 'the RFC Editor now gives the status';
/** An updating RFC that Appendix A does not list. */
export const UNLISTED_UPDATE = 'is updated by an RFC that Appendix A does not list:';
/** A verified erratum missing from the table. */
export const MISSING_ERRATUM = 'verified erratum missing from the errata table; classify it and add the row';
/** A held erratum that is not in the table. */
export const UNLISTED_HELD_ERRATUM = 'erratum held for document update, not in the table; review it if it is new';
/** A row whose erratum the RFC Editor no longer lists for that RFC. */
export const UNKNOWN_ERRATUM = 'the RFC Editor lists no such erratum for this RFC';
/** A row whose status, type or section changed at the RFC Editor. */
export const ERRATUM_CHANGED = 'the RFC Editor now gives other values; the table has';
/** A missing manifest. */
export const MISSING_MANIFEST = 'the manifest is missing; run `node scripts/fetch-rfcs.ts`';
/** A missing errata table. */
export const MISSING_ERRATA = 'the errata table is missing';
/** The end of a successful run. */
export const UP_TO_DATE = 'the RFC set matches the RFC Editor';
