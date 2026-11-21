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

// Patterns and messages of the Markdown link check.

/** A fence line of a code block. */
export const FENCE_PATTERN = /^[ \t]*(?:```|~~~)/u;
/** A heading and its text, without closing number signs. */
export const HEADING_PATTERN = /^#{1,6}[ \t]+(.*?)(?:[ \t]+#+)?[ \t]*$/u;
/** Characters that GitHub drops from a heading's anchor. */
export const NOT_ANCHOR_CHARACTERS = /[^\p{L}\p{N} _-]/gu;
/** Spaces, which become hyphens in an anchor. */
export const SPACE_PATTERN = / /gu;
/** Joins an anchor and its occurrence number. */
export const HYPHEN = '-';
/** Code spans, whose content is not a link. */
export const CODE_SPAN_PATTERN = /`[^`]*`/gu;
/** An inline link's target. */
export const INLINE_LINK_PATTERN = /\]\(([^)]*)\)/gu;
/** A reference definition's target. */
export const REFERENCE_DEFINITION_PATTERN = /^ {0,3}\[[^\]]+\]:[ \t]+(\S+)/u;
/** A link with a scheme, such as `https:` or `mailto:`, which this check skips. */
export const SCHEME_PATTERN = /^[a-z][a-z0-9+.-]*:/iu;
/** Angle brackets around a link target. */
export const ANGLE_BRACKETS_PATTERN = /^<|>$/gu;
/** A title after a link target. */
export const TITLE_PATTERN = /[ \t].*$/u;
/** Separates a path from an anchor. */
export const ANCHOR_SEPARATOR = '#';
/** Nothing. */
export const EMPTY = '';
/** A link to a missing file. */
export const MISSING_FILE = 'link to a missing file';
/** A link to a missing heading. */
export const MISSING_HEADING = 'link to a missing heading';
