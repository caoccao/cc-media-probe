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

// The `rust-version` that the published packages declare (R12), read from the output of `cargo metadata`.

import { PUBLISHED_PACKAGES } from '../constants.ts';
import { DIFFERENT_DECLARATIONS, MISSING_PACKAGE, NO_DECLARATION, UNREADABLE_METADATA } from './constants.ts';

/** The declared release, or the findings that leave none. */
export interface Declaration {
  readonly version: string | undefined;
  readonly findings: readonly string[];
}

/** The part of a package in `cargo metadata` that is read here; Cargo fixes the field names. */
interface PackageMetadata {
  readonly name?: unknown;
  readonly rust_version?: unknown;
}

/** The part of `cargo metadata` that is read here. */
interface WorkspaceMetadata {
  readonly packages?: unknown;
}

/** The release that every published package declares in `metadata`, the output of `cargo metadata`. */
export function declaredRustVersion(metadata: string): Declaration {
  const packages = readPackages(metadata);
  if (packages === undefined) {
    return { version: undefined, findings: [UNREADABLE_METADATA] };
  }
  const findings: string[] = [];
  const versions = new Set<string>();
  for (const name of PUBLISHED_PACKAGES) {
    const declared = packages.find((entry) => entry?.name === name);
    if (declared === undefined || declared === null) {
      findings.push(`${name}: ${MISSING_PACKAGE}`);
      continue;
    }
    const version = declared.rust_version;
    if (version === undefined || version === null) {
      findings.push(`${name}: ${NO_DECLARATION}`);
      continue;
    }
    versions.add(String(version));
  }
  if (findings.length > 0) {
    return { version: undefined, findings };
  }
  const [version, ...others] = versions;
  return version === undefined || others.length > 0
    ? { version: undefined, findings: [DIFFERENT_DECLARATIONS] }
    : { version, findings: [] };
}

/** The packages of `metadata`, or `undefined` when it is not JSON with a list of packages. */
function readPackages(metadata: string): readonly (PackageMetadata | null)[] | undefined {
  try {
    const packages = (JSON.parse(metadata) as WorkspaceMetadata | null)?.packages;
    return Array.isArray(packages) ? (packages as (PackageMetadata | null)[]) : undefined;
  } catch {
    return undefined;
  }
}
