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

// The manifest of non-RFC documents and drafts (plan section 5.2): one row per stored file or link, in ascending
// order of identifier. The checksum and the date are recorded by the fetch script.

import { EMPTY } from '../constants.ts';
import { compareText, formatTable, parseTable } from '../tab-separated.ts';
import {
  DEFAULT_EXTENSION,
  DOCUMENT_COLUMNS,
  FOLDER_SEPARATOR,
  KNOWN_EXTENSIONS,
  MEDIA_TYPE_EXTENSIONS,
  NOT_REPRODUCIBLE,
  PATH_SEPARATOR,
  QUERY_SEPARATOR,
  SNAPSHOT,
  TREE,
} from './constants.ts';
import { ruleFindings } from './rules.ts';

/** One row of the manifest. */
export interface DocumentEntry {
  readonly identifier: string;
  readonly folder: string;
  readonly title: string;
  readonly publisher: string;
  readonly edition: string;
  readonly landing: string;
  readonly artifact: string;
  readonly immutableReference: string;
  readonly contentType: string;
  readonly availability: string;
  readonly retrieval: string;
  readonly appliesTo: string;
  readonly classification: string;
  readonly checksum: string;
  readonly retrieved: string;
  readonly approved: string;
}

/** The entries of the manifest `text`, and findings for malformed rows; findings name `file`. */
export function readDocuments(file: string, text: string): { entries: DocumentEntry[]; findings: string[] } {
  const table = parseTable(file, text, DOCUMENT_COLUMNS);
  const findings = [...table.findings];
  const entries: DocumentEntry[] = [];
  for (const row of table.rows) {
    const [
      identifier = EMPTY,
      folder = EMPTY,
      title = EMPTY,
      publisher = EMPTY,
      edition = EMPTY,
      landing = EMPTY,
      artifact = EMPTY,
      immutableReference = EMPTY,
      contentType = EMPTY,
      availability = EMPTY,
      retrieval = EMPTY,
      appliesTo = EMPTY,
      classification = EMPTY,
      checksum = EMPTY,
      retrieved = EMPTY,
      approved = EMPTY,
    ] = row.cells;
    const entry: DocumentEntry = {
      identifier,
      folder,
      title,
      publisher,
      edition,
      landing,
      artifact,
      immutableReference,
      contentType,
      availability,
      retrieval,
      appliesTo,
      classification,
      checksum,
      retrieved,
      approved,
    };
    findings.push(...ruleFindings(`${file}:${row.line}`, entry, entries.at(-1)));
    entries.push(entry);
  }
  return { entries, findings };
}

/** The cells of one entry, in column order. */
export function documentCells(entry: DocumentEntry): string[] {
  return [
    entry.identifier,
    entry.folder,
    entry.title,
    entry.publisher,
    entry.edition,
    entry.landing,
    entry.artifact,
    entry.immutableReference,
    entry.contentType,
    entry.availability,
    entry.retrieval,
    entry.appliesTo,
    entry.classification,
    entry.checksum,
    entry.retrieved,
    entry.approved,
  ];
}

/** The manifest's text for `entries`, in ascending order of identifier. */
export function formatDocuments(entries: readonly DocumentEntry[]): string {
  const sorted = [...entries].sort((first, second) => compareText(first.identifier, second.identifier));
  return formatTable(DOCUMENT_COLUMNS, sorted.map(documentCells));
}

/** The address that the fetch script downloads for `entry`: the immutable reference of a reproducible snapshot,
 * otherwise the artifact. */
export function sourceAddress(entry: DocumentEntry): string {
  const reproducible = entry.retrieval === SNAPSHOT && entry.immutableReference !== NOT_REPRODUCIBLE;
  return reproducible ? entry.immutableReference : entry.artifact;
}

/** The path of the stored file below `docs/specs/files/`, by component: its topic, its publisher, its name. */
export function storedPath(entry: DocumentEntry): string[] {
  return [...entry.folder.split(FOLDER_SEPARATOR), storedFileName(entry)];
}

/** The name of the stored file in its folder: the identifier, with the extension of the source address
 * when it has a known one, otherwise the extension of the media type. A tree is stored as one JSON file. */
export function storedFileName(entry: DocumentEntry): string {
  const lastSegment = (sourceAddress(entry).split(QUERY_SEPARATOR)[0] ?? EMPTY).split(PATH_SEPARATOR).at(-1) ?? EMPTY;
  const fromAddress = KNOWN_EXTENSIONS.find((extension) => lastSegment.toLowerCase().endsWith(extension));
  const fromType = MEDIA_TYPE_EXTENSIONS.get(entry.contentType) ?? DEFAULT_EXTENSION;
  return entry.identifier + (entry.retrieval === TREE ? fromType : (fromAddress ?? fromType));
}
