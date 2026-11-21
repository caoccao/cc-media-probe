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

// Paths, table syntax, downloads and messages that the specification checks and fetch scripts share (plan section 5.2).

/** The development plan, relative to the repository root, by component. */
export const PLAN_FILE = ['docs', 'plans', 'plan-init-project.md'];
/** Separates the cells of a table row. */
export const CELL_SEPARATOR = '\t';
/** Ends every row of a table. */
export const ROW_END = '\n';
/** Nothing. */
export const EMPTY = '';
/** A space. */
export const SPACE = ' ';
/** A run of whitespace, line breaks included. */
export const WHITESPACE_PATTERN = /\s+/u;
/** Separates the items of a list in a message. */
export const LIST_SEPARATOR = ', ';
/** A calendar date as the tables write it. */
export const DATE_PATTERN = /^\d{4}-\d{2}-\d{2}$/u;
/** The length of the calendar date that starts an ISO 8601 timestamp. */
export const DATE_LENGTH = 10;
/** A SHA-256 checksum in lower-case hexadecimal. */
export const CHECKSUM_PATTERN = /^[0-9a-f]{64}$/u;
/** The hash algorithm of the checksums. */
export const CHECKSUM_ALGORITHM = 'sha256';
/** The encoding of the checksums. */
export const CHECKSUM_ENCODING = 'hex';
/** The shortest time between two requests to one host, in milliseconds, so that no site is asked too often. */
export const REQUEST_INTERVAL = 2_000;
/** Hosts that ask for a longer interval than the default, in milliseconds: the Internet Archive limits how often
 * one client may ask it, and refused connections at 4 seconds on 2026-11-21. */
export const HOST_INTERVALS: ReadonlyMap<string, number> = new Map([['web.archive.org', 6_000]]);
/** The longest interval that backing off reaches, in milliseconds. */
export const MAXIMUM_INTERVAL = 300_000;
/** How much a host's interval grows after each sign of blocking. */
export const BACK_OFF_FACTOR = 2;
/** How often a request that met a sign of blocking is tried again. */
export const MAXIMUM_RETRIES = 4;
/** Statuses that are retried after backing off: signs that a host is limiting or blocking requests (forbidden, too
 * many requests, unavailable) and passing server failures (internal error, bad gateway, gateway timeout). */
export const RETRIED_STATUSES = [403, 429, 500, 502, 503, 504];
/** The response header in which a host asks for a delay. */
export const RETRY_AFTER_HEADER = 'retry-after';
/** A whole number of seconds. */
export const DIGITS_PATTERN = /^\d+$/u;
/** Milliseconds in a second. */
export const MILLISECONDS_PER_SECOND = 1_000;
/** The unit of an interval in messages. */
export const SECONDS_UNIT = ' s';
/** How long one download may take, in milliseconds. */
export const DOWNLOAD_TIMEOUT = 120_000;
/** The status of a successful HTTP response. */
export const HTTP_OK = 200;
/** The request header that names the client. */
export const USER_AGENT_HEADER = 'user-agent';
/** A generic client name, without contact details. */
export const USER_AGENT = 'cc-media-probe-fetch';
/** Redirects are followed. */
export const FOLLOW_REDIRECTS = 'follow';
/** A redirect fails the download. */
export const REFUSE_REDIRECTS = 'error';
/** The response header that names the media type. */
export const CONTENT_TYPE_HEADER = 'content-type';
/** Separates a media type from its parameters. */
export const MEDIA_TYPE_PARAMETERS = ';';
/** Starts a note: something to review that does not fail the run. */
export const NOTE_PREFIX = 'note: ';

/** A header row that differs from the expected columns. */
export const HEADER_DIFFERS = 'the header row differs from the expected columns:';
/** A row with too many or too few cells. */
export const WRONG_CELL_COUNT = 'the row does not have one cell per column';
/** A host that kept signaling blocking after every retry. */
export const BLOCKED = 'the host kept refusing or failing requests after backing off; try again later, status';
/** A host whose interval grew. */
export const BACKING_OFF = 'backing off: the interval of';
/** An HTTP response other than 200. */
export const UNEXPECTED_STATUS = 'unexpected HTTP status';
