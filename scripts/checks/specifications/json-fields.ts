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

// Reading fields of parsed JSON documents whose shape is not trusted: a missing or mistyped field reads as nothing.

import { EMPTY } from './constants.ts';

/** The field `name` of `document`, or `undefined` when it is not an object. */
export function field(document: unknown, name: string): unknown {
  return document instanceof Object ? Reflect.get(document, name) : undefined;
}

/** The field `name` as text; nothing when it is missing. */
export function textField(document: unknown, name: string): string {
  const value = field(document, name);
  return value === undefined || value === null ? EMPTY : String(value);
}

/** The field `name` as a list; empty when it is not one. */
export function listField(document: unknown, name: string): unknown[] {
  const value = field(document, name);
  return Array.isArray(value) ? (value as unknown[]) : [];
}
