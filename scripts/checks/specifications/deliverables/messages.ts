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

// Messages of the deliverables and coverage check.

/** A malformed key. */
export const MALFORMED_KEY = 'not a key of a kind and lower-case words joined by dots or hyphens';
/** Rows out of order. */
export const OUT_OF_ORDER = 'rows are not in ascending order of key';
/** A key listed twice. */
export const DUPLICATE_KEY = 'key listed twice';
/** A cell outside its vocabulary. */
export const NOT_IN_VOCABULARY = 'is not one of';
/** A key that does not start with its kind. */
export const KIND_MISMATCH = 'the key does not start with its kind';
/** A release or gate that does not fit the status. */
export const MALFORMED_GATE_OR_RELEASE =
  'an active row names its release and no gate; a deferred or dropped row names its gate and no release';
/** An active row without a source. */
export const SOURCES_RULE = 'an active row names at least one source';
/** A malformed date. */
export const MALFORMED_DATE = 'the confirmation date is not written as YYYY-MM-DD';
/** A key of one table without its partner in the other. */
export const NO_COVERAGE = 'deliverable without a coverage row';
/** A coverage row without its deliverable. */
export const NO_DELIVERABLE = 'coverage row without a deliverable';
/** A source that names no document of the manifests. */
export const UNKNOWN_SOURCE = 'source names no document of the manifests:';
/** A malformed source. */
export const MALFORMED_SOURCE = 'a source is a document identifier, then a colon and the section:';
/** A promise of section 4 without its deliverable. */
export const MISSING_DELIVERABLE = 'section 4 lists this, but no deliverable has its words:';
/** A deliverable that section 4 does not list. */
export const UNPLANNED_DELIVERABLE = 'no row of section 4 has the words of this deliverable';
/** A missing table. */
export const MISSING_TABLE = 'the table is missing';
