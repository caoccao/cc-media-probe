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

// A documentation tree stored as one file (a `tree` row): each page is a JSON document whose topic sections name
// child pages through its references. The walk starts at the artifact, visits every page below it once, one request
// at a time, and stores the pages in order of path, so that the same pages give the same bytes.

import { ROW_END } from '../constants.ts';
import type { Downloader } from '../downloads.ts';
import { field, listField, textField } from '../json-fields.ts';
import { compareText } from '../tab-separated.ts';
import {
  IDENTIFIERS_FIELD,
  MAXIMUM_TREE_PAGES,
  PATH_SEPARATOR,
  REFERENCE_PATH_FIELD,
  REFERENCES_FIELD,
  TOPIC_SECTIONS_FIELD,
  TREE_PAGE_EXTENSION,
  TREE_PAGES_SEGMENT,
} from './constants.ts';
import { TREE_TOO_LARGE, UNREADABLE_PAGE } from './messages.ts';

/** One page of a tree: its path below the site's data address, and its parsed content. */
export interface TreePage {
  readonly path: string;
  readonly page: unknown;
}

/** Walks the tree whose root page is at `rootAddress` and returns the stored form of all its pages. */
export async function collectTree(rootAddress: string, downloader: Downloader): Promise<Buffer> {
  const position = rootAddress.indexOf(TREE_PAGES_SEGMENT);
  const base = rootAddress.slice(0, position);
  const rootPath = rootAddress.slice(position, rootAddress.length - TREE_PAGE_EXTENSION.length);
  const seen = new Set([rootPath]);
  const queue = [rootPath];
  const pages: TreePage[] = [];
  for (let path = queue.shift(); path !== undefined; path = queue.shift()) {
    const downloaded = await downloader(base + path + TREE_PAGE_EXTENSION);
    let page: unknown;
    try {
      page = JSON.parse(downloaded.bytes.toString()) as unknown;
    } catch {
      throw new Error(`${UNREADABLE_PAGE} ${path}`);
    }
    pages.push({ path, page });
    for (const child of childPaths(page, rootPath)) {
      if (!seen.has(child)) {
        if (seen.size >= MAXIMUM_TREE_PAGES) {
          throw new Error(TREE_TOO_LARGE);
        }
        seen.add(child);
        queue.push(child);
      }
    }
  }
  pages.sort((first, second) => compareText(first.path, second.path));
  return Buffer.from(JSON.stringify({ root: rootAddress, pages }) + ROW_END);
}

/** The paths of the pages that the topic sections of `page` name, below `rootPath`. */
export function childPaths(page: unknown, rootPath: string): string[] {
  const references = field(page, REFERENCES_FIELD);
  return listField(page, TOPIC_SECTIONS_FIELD)
    .flatMap((section) => listField(section, IDENTIFIERS_FIELD))
    .map((identifier) => textField(field(references, String(identifier)), REFERENCE_PATH_FIELD))
    .filter((path) => path.startsWith(rootPath + PATH_SEPARATOR));
}
