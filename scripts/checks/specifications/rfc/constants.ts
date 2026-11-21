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

// Files, columns, addresses, vocabularies and messages of the RFC set (plan sections 5.2 and 5.3, Appendix A).

/** The directory of the RFC texts, relative to the repository root, by component. */
export const RFC_DIRECTORY = ['docs', 'rfc'];
/** The RFC manifest. */
export const MANIFEST_FILE = 'manifest.tsv';
/** The errata table. */
export const ERRATA_FILE = 'errata.tsv';
/** The generated index. */
export const README_FILE = 'README.md';
/** The columns of the RFC manifest (plan section 5.2). */
export const MANIFEST_COLUMNS = ['number', 'title', 'status', 'url', 'sha256', 'retrieved', 'role'];
/** The columns of the errata table (plan section 5.3). */
export const ERRATA_COLUMNS = ['rfc', 'erratum', 'status', 'type', 'section', 'relevance', 'components', 'summary'];
/** Starts the file name of an RFC text. */
export const RFC_FILE_PREFIX = 'rfc';
/** Ends the file name of an RFC text. */
export const RFC_FILE_EXTENSION = '.txt';
/** An RFC text as Git names it, with its number in the first group. */
export const RFC_FILE_PATTERN = /^docs\/rfc\/rfc(\d+)\.txt$/u;
/** An RFC text as the directory lists it, with its number in the first group. */
export const STORED_TEXT_PATTERN = /^rfc(\d+)\.txt$/u;
/** A decimal digit. */
export const DIGIT_PATTERN = /\d/u;
/** A number as the tables write it. */
export const NUMBER_PATTERN = /^[1-9]\d*$/u;

/** The RFC Editor's plain-text copy of an RFC; the number follows. */
export const TEXT_ADDRESS_PREFIX = 'https://www.rfc-editor.org/rfc/rfc';
/** Ends the address of an RFC's metadata, which follows the text's prefix and the number. */
export const METADATA_ADDRESS_EXTENSION = '.json';
/** The RFC Editor's list of every erratum. */
export const ERRATA_ADDRESS = 'https://www.rfc-editor.org/api/v1/errata.json';
/** The media type of an RFC text. */
export const TEXT_MEDIA_TYPE = 'text/plain';
/** The media type of the metadata and the errata list. */
export const JSON_MEDIA_TYPE = 'application/json';
/** Every RFC names its own number near the top, so that an error page is never stored as an RFC. */
export const NUMBER_LINE_PREFIX = 'Request for Comments: ';
/** Names an RFC in messages; the number follows. */
export const RFC_LABEL = 'RFC ';
/** Names an erratum in messages; the number follows. */
export const ERRATUM_LABEL = 'erratum ';
/** How the RFC Editor names an RFC in its metadata; the number follows. */
export const DOCUMENT_PREFIX = 'RFC';

/** Opens Appendix A of the plan. */
export const APPENDIX_HEADING = '## Appendix A';
/** Opens the next appendix, which ends Appendix A. */
export const LEVEL_TWO_HEADING = '## ';
/** A row of an RFC table in Appendix A: the number, linked to its information page, then title, status and role. */
export const APPENDIX_ROW_PATTERN =
  /^\| \[(\d+)\]\(https:\/\/www\.rfc-editor\.org\/info\/rfc(\d+)\) \| ([^|]+?) \| ([^|]+?) \| ([^|]+?) \|/u;
/** The capture groups of a row of Appendix A. */
export const APPENDIX_NUMBER_GROUP = 1;
/** The number in the link target. */
export const APPENDIX_LINK_GROUP = 2;
/** The title. */
export const APPENDIX_TITLE_GROUP = 3;
/** The status abbreviation. */
export const APPENDIX_STATUS_GROUP = 4;
/** The role. */
export const APPENDIX_ROLE_GROUP = 5;
/** The statuses that Appendix A abbreviates, by abbreviation. */
export const STATUS_NAMES: ReadonlyMap<string, string> = new Map([
  ['PS', 'Proposed Standard'],
  ['IS', 'Internet Standard'],
  ['DS', 'Draft Standard'],
  ['BCP', 'Best Current Practice'],
  ['Info', 'Informational'],
  ['Hist', 'Historic'],
]);

/** The statuses of an erratum in the table. */
export const VERIFIED = 'verified';
/** Held for document update. */
export const HELD = 'held';
/** The types of an erratum. */
export const ERRATUM_TYPES = ['technical', 'editorial'];
/** Its relevance to this project. */
export const RELEVANCES = ['parsing', 'output', 'none'];
/** Not relevant: such a row names no component. */
export const NO_RELEVANCE = 'none';
/** The RFC Editor's statuses, by the table's. */
export const RECORD_STATUSES: ReadonlyMap<string, string> = new Map([
  ['Verified', VERIFIED],
  ['Held for Document Update', HELD],
]);
/** The RFC Editor's types, by the table's. */
export const RECORD_TYPES: ReadonlyMap<string, string> = new Map([
  ['Technical', 'technical'],
  ['Editorial', 'editorial'],
]);

/** Field names of the RFC Editor's metadata. */
export const DOCUMENT_FIELD = 'doc_id';
/** The title. */
export const TITLE_FIELD = 'title';
/** The status, in capitals. */
export const STATUS_FIELD = 'status';
/** The RFCs that obsolete it. */
export const OBSOLETED_BY_FIELD = 'obsoleted_by';
/** The RFCs that update it. */
export const UPDATED_BY_FIELD = 'updated_by';
/** Field names of the errata list. */
export const ERRATUM_FIELD = 'errata_id';
/** The RFC of an erratum. */
export const ERRATUM_DOCUMENT_FIELD = 'doc-id';
/** Its status. */
export const ERRATUM_STATUS_FIELD = 'errata_status_code';
/** Its type. */
export const ERRATUM_TYPE_FIELD = 'errata_type_code';
/** Its section. */
export const ERRATUM_SECTION_FIELD = 'section';
