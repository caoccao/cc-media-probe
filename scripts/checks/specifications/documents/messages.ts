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

// Messages of the checks and the fetch script of the non-RFC documents.

/** A malformed identifier. */
export const MALFORMED_IDENTIFIER = 'not an identifier of lower-case words joined by hyphens or dots';
/** A malformed folder. */
export const MALFORMED_FOLDER =
  'not a folder of a topic (containers, video, audio, subtitles, tags-and-registries) and a publisher';
/** Rows out of order. */
export const OUT_OF_ORDER = 'rows are not in ascending order of identifier';
/** An empty cell that must be filled. */
export const EMPTY_CELL = 'is empty';
/** A cell outside its vocabulary. */
export const NOT_IN_VOCABULARY = 'is not one of';
/** A malformed address. */
export const MALFORMED_ADDRESS = 'not an address';
/** A malformed gate. */
export const MALFORMED_GATE = 'not a gate';
/** A malformed or misplaced media type. */
export const MALFORMED_MEDIA_TYPE = 'a stored document needs a media type, and a link has none';
/** A downloaded document without an artifact. */
export const ARTIFACT_RULE = 'a fetched, snapshot or tree row needs an artifact address';
/** A misplaced or missing immutable reference. */
export const REFERENCE_RULE =
  'a snapshot needs an immutable reference or `none`, a tree needs `none`, and other rows have none';
/** A paid document with something to download. */
export const PAID_RULE = 'a paid document has no artifact and is not downloaded';
/** A link with a checksum. */
export const STORED_LINK = 'a link is not stored, so it has no checksum or date';
/** A malformed checksum or date. */
export const CHECKSUM_RULE = 'the checksum must be SHA-256 in lower-case hexadecimal, with its date';
/** A duplicate identifier across the tables. */
export const DUPLICATE_IDENTIFIER = 'identifier listed twice';
/** A draft of Appendix A.7 without its pinned row. */
export const MISSING_DRAFT = 'no manifest row fetches the pinned revision of this draft:';
/** A stored file that differs from its checksum. */
export const CHECKSUM_MISMATCH = 'the stored file differs from the manifest\'s checksum';
/** A stored file without a recorded checksum. */
export const UNRECORDED_FILE = 'present but not recorded in the manifest; delete it and run again';
/** A stored file that no row names. */
export const UNLISTED_FILE = 'stored file that no manifest row names';
/** An identifier on the command line that no row has. */
export const UNKNOWN_IDENTIFIER = 'no manifest row has this identifier';
/** A missing manifest. */
export const MISSING_MANIFEST = 'the manifest is missing';

/** A download that failed. */
export const DOWNLOAD_FAILED = 'download failed';
/** A response of another media type. */
export const WRONG_MEDIA_TYPE = 'unexpected media type';
/** A downloaded document with other bytes than recorded. */
export const CHANGED_DOCUMENT =
  'the downloaded document differs from the recorded checksum; review the change and supersede the row through GS.2';
/** A missing manual document. */
export const PLACE_MANUALLY = 'place the document here by hand:';
/** A documentation tree that is larger than the limit. */
export const TREE_TOO_LARGE = 'the documentation tree has more pages than the limit';
/** A page of a tree that is not JSON. */
export const UNREADABLE_PAGE = 'a page of the documentation tree is not JSON:';
/** The end of a successful run. */
export const UP_TO_DATE = 'every stored document matches the manifest';
