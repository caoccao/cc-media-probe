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

// Files, columns, vocabularies and addresses of the manifest of non-RFC documents (plan section 5.2).

/** The directory of the manifest, relative to the repository root, by component. */
export const SPECIFICATIONS_DIRECTORY = ['docs', 'specs'];
/** The git-ignored directory of the stored documents, inside the manifest's directory. */
export const FILES_DIRECTORY = 'files';
/** The manifest. */
export const MANIFEST_FILE = 'manifest.tsv';
/** The columns of the manifest (plan section 5.2). */
export const DOCUMENT_COLUMNS = [
  'identifier',
  'folder',
  'title',
  'publisher',
  'edition',
  'landing_url',
  'artifact_url',
  'immutable_reference',
  'content_type',
  'availability',
  'retrieval',
  'applies_to',
  'class',
  'sha256',
  'retrieved',
  'approved',
];

/** The columns that no row leaves empty. */
export const REQUIRED_COLUMNS = [
  'identifier',
  'folder',
  'title',
  'publisher',
  'edition',
  'landing_url',
  'availability',
  'retrieval',
  'applies_to',
  'class',
];
/** An identifier: lower-case words and numbers joined by hyphens or dots. */
export const IDENTIFIER_PATTERN = /^[a-z0-9]+(?:[.-][a-z0-9]+)*$/u;
/** The folder of a stored document: a topic of Appendix B, then the publisher, joined by a slash. */
export const FOLDER_PATTERN =
  /^(?:containers|video|audio|subtitles|tags-and-registries)\/[a-z0-9]+(?:-[a-z0-9]+)*$/u;
/** Separates the topic from the publisher in a folder. */
export const FOLDER_SEPARATOR = '/';
/** An address of a page or file. */
export const ADDRESS_PATTERN = /^https?:\/\/[^\s]+$/u;
/** A media type. */
export const MEDIA_TYPE_PATTERN = /^[a-z]+\/[a-z0-9.+-]+$/u;
/** A gate, as the approval column names it. */
export const GATE_PATTERN = /^G(?:S|\d+)\.\d+$/u;

/** Whether a document can be obtained. */
export const AVAILABILITIES = ['free', 'paid', 'none'];
/** Must be bought. */
export const PAID = 'paid';
/** How a document is stored. */
export const RETRIEVALS = ['fetch', 'snapshot', 'tree', 'manual', 'link'];
/** The script downloads the artifact, whose address names its edition. */
export const FETCH = 'fetch';
/** The script stores a living page from its immutable reference, or once from the page itself. */
export const SNAPSHOT = 'snapshot';
/** The script walks a documentation tree from the artifact and stores the pages as one file. */
export const TREE = 'tree';
/** The maintainer places the file. */
export const MANUAL = 'manual';
/** Not stored. */
export const LINK = 'link';
/** The role of a document. */
export const CLASSES = ['normative', 'secondary', 'descriptive'];
/** The immutable reference of a living page that has none: the approved copy exists only locally. */
export const NOT_REPRODUCIBLE = 'none';
/** Archive captures, which must never be redirected to another capture. */
export const ARCHIVE_PREFIX = 'https://web.archive.org/web/';

/** The extensions of stored files, by the last segment of the address they come from. */
export const KNOWN_EXTENSIONS = [
  '.csv', '.doc', '.docx', '.htm', '.html', '.json', '.md', '.pdf', '.txt', '.xml', '.zip',
];
/** The extensions of stored files, by media type, where the address has none of the known ones. */
export const MEDIA_TYPE_EXTENSIONS: ReadonlyMap<string, string> = new Map([
  ['application/json', '.json'],
  ['application/msword', '.doc'],
  ['application/pdf', '.pdf'],
  ['application/xhtml+xml', '.xhtml'],
  ['application/xml', '.xml'],
  ['application/zip', '.zip'],
  ['text/csv', '.csv'],
  ['text/html', '.html'],
  ['text/markdown', '.md'],
  ['text/plain', '.txt'],
  ['text/xml', '.xml'],
]);
/** The extension of a file whose media type is not known. */
export const DEFAULT_EXTENSION = '.bin';
/** Separates the path of an address from its query. */
export const QUERY_SEPARATOR = '?';
/** Separates the segments of an address's path. */
export const PATH_SEPARATOR = '/';

/** The draft rows of Appendix A.7: the draft name and its pinned archive address. */
export const DRAFT_ROW_PATTERN = new RegExp(
  String.raw`^\| \[(draft-[a-z0-9-]+)\]\(https://datatracker\.ietf\.org/doc/[a-z0-9-]+/\) \| [^|]+ \| -(\d+) \| ` +
    String.raw`\[archive\]\((https://www\.ietf\.org/archive/id/[a-z0-9-]+\.txt)\) \|`,
  'u',
);
/** The capture groups of a draft row. */
export const DRAFT_NAME_GROUP = 1;
/** The pinned revision. */
export const DRAFT_REVISION_GROUP = 2;
/** The archive address. */
export const DRAFT_ADDRESS_GROUP = 3;
/** Opens Appendix A.7. */
export const DRAFTS_HEADING = '### A.7';
/** Opens the next section, which ends A.7. */
export const NEXT_SECTION_HEADING = '### ';

/** Where a documentation tree keeps its pages, in the address of its root page. */
export const TREE_PAGES_SEGMENT = '/documentation/';
/** Ends the address of a page of the tree. */
export const TREE_PAGE_EXTENSION = '.json';
/** The most pages a tree may have, so that a walk always ends. */
export const MAXIMUM_TREE_PAGES = 5_000;
/** Fields of a page of the tree. */
export const TOPIC_SECTIONS_FIELD = 'topicSections';
/** The pages of a topic section. */
export const IDENTIFIERS_FIELD = 'identifiers';
/** The references of a page, by identifier. */
export const REFERENCES_FIELD = 'references';
/** The path of a reference. */
export const REFERENCE_PATH_FIELD = 'url';
