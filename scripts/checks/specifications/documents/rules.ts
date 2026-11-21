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

// The rules of one manifest row (plan section 5.2): the vocabularies, the cells that each retrieval needs and
// forbids, and the order of identifiers.

import { CHECKSUM_PATTERN, DATE_PATTERN, EMPTY, LIST_SEPARATOR } from '../constants.ts';
import {
  ADDRESS_PATTERN,
  AVAILABILITIES,
  CLASSES,
  DOCUMENT_COLUMNS,
  FETCH,
  FOLDER_PATTERN,
  GATE_PATTERN,
  IDENTIFIER_PATTERN,
  LINK,
  MANUAL,
  MEDIA_TYPE_PATTERN,
  NOT_REPRODUCIBLE,
  PAID,
  REQUIRED_COLUMNS,
  RETRIEVALS,
  SNAPSHOT,
  TREE,
} from './constants.ts';
import { compareText } from '../tab-separated.ts';
import { type DocumentEntry, documentCells } from './manifest.ts';
import {
  ARTIFACT_RULE,
  CHECKSUM_RULE,
  EMPTY_CELL,
  MALFORMED_ADDRESS,
  MALFORMED_FOLDER,
  MALFORMED_GATE,
  MALFORMED_IDENTIFIER,
  MALFORMED_MEDIA_TYPE,
  NOT_IN_VOCABULARY,
  OUT_OF_ORDER,
  PAID_RULE,
  REFERENCE_RULE,
  STORED_LINK,
} from './messages.ts';

/** The findings of one row at `location`, given the row before it. */
export function ruleFindings(location: string, entry: DocumentEntry, previous: DocumentEntry | undefined): string[] {
  const findings: string[] = [];
  const add = (message: string): void => {
    findings.push(`${location}: ${message}`);
  };
  if (!IDENTIFIER_PATTERN.test(entry.identifier)) {
    add(`${MALFORMED_IDENTIFIER}: ${entry.identifier}`);
  }
  if (entry.folder !== EMPTY && !FOLDER_PATTERN.test(entry.folder)) {
    add(`${MALFORMED_FOLDER}: ${entry.folder}`);
  }
  if (previous !== undefined && compareText(previous.identifier, entry.identifier) >= 0) {
    add(OUT_OF_ORDER);
  }
  const cells = documentCells(entry);
  for (const column of REQUIRED_COLUMNS) {
    if (cells[DOCUMENT_COLUMNS.indexOf(column)] === EMPTY) {
      add(`${column} ${EMPTY_CELL}`);
    }
  }
  const vocabularies: [string, readonly string[]][] = [
    [entry.availability, AVAILABILITIES],
    [entry.retrieval, RETRIEVALS],
    [entry.classification, CLASSES],
  ];
  for (const [value, vocabulary] of vocabularies) {
    if (!vocabulary.includes(value)) {
      add(`${value} ${NOT_IN_VOCABULARY} ${vocabulary.join(LIST_SEPARATOR)}`);
    }
  }
  findings.push(...addressFindings(location, entry), ...retrievalFindings(location, entry));
  if (entry.approved !== EMPTY && !GATE_PATTERN.test(entry.approved)) {
    add(`${MALFORMED_GATE}: ${entry.approved}`);
  }
  return findings;
}

/** The landing page, the artifact and the immutable reference are addresses where present. */
function addressFindings(location: string, entry: DocumentEntry): string[] {
  const addresses = [entry.landing, entry.artifact, entry.immutableReference].filter(
    (address) => address !== EMPTY && address !== NOT_REPRODUCIBLE,
  );
  const malformed = addresses.filter((address) => !ADDRESS_PATTERN.test(address));
  return malformed.map((address) => `${location}: ${MALFORMED_ADDRESS}: ${address}`);
}

/** What each retrieval needs: an artifact for what the script downloads, an immutable reference or `none` for a
 * snapshot, a media type for everything stored, a checksum only for what is stored, nothing to download for a paid
 * document. */
function retrievalFindings(location: string, entry: DocumentEntry): string[] {
  const findings: string[] = [];
  const add = (message: string): void => {
    findings.push(`${location}: ${message}`);
  };
  const downloaded = entry.retrieval === FETCH || entry.retrieval === SNAPSHOT || entry.retrieval === TREE;
  const stored = downloaded || entry.retrieval === MANUAL;
  if (downloaded && entry.artifact === EMPTY) {
    add(ARTIFACT_RULE);
  }
  const needsReference = entry.retrieval === SNAPSHOT || entry.retrieval === TREE;
  const referenceValid =
    entry.retrieval === TREE ? entry.immutableReference === NOT_REPRODUCIBLE : entry.immutableReference !== EMPTY;
  if (needsReference ? !referenceValid : entry.immutableReference !== EMPTY) {
    add(REFERENCE_RULE);
  }
  if (stored !== MEDIA_TYPE_PATTERN.test(entry.contentType)) {
    add(`${MALFORMED_MEDIA_TYPE}: ${entry.contentType}`);
  }
  if (entry.availability === PAID && (downloaded || entry.artifact !== EMPTY)) {
    add(PAID_RULE);
  }
  const recorded = entry.checksum !== EMPTY || entry.retrieved !== EMPTY;
  if (entry.retrieval === LINK && recorded) {
    add(STORED_LINK);
  } else if (
    recorded &&
    (!CHECKSUM_PATTERN.test(entry.checksum) || !DATE_PATTERN.test(entry.retrieved))
  ) {
    add(CHECKSUM_RULE);
  }
  return findings;
}
