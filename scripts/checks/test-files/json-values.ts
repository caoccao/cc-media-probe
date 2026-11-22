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

// The shapes of parsed JSON values that the recipe and state files use.

import { OBJECT_TYPE, TEXT_TYPE } from './constants.ts';

/** Whether `value` is a plain JSON object. */
export function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === OBJECT_TYPE && value !== null && !Array.isArray(value);
}

/** Whether `value` is a string. */
export function isText(value: unknown): value is string {
  return typeof value === TEXT_TYPE;
}

/** Whether `value` is a list of strings. */
export function isTextList(value: unknown): value is string[] {
  return Array.isArray(value) && value.every(isText);
}
