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

// Tests of the parsers of the platform tools' output.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import {
  DUMPBIN_DLL_PATTERN,
  OBJDUMP_DLL_PATTERN,
  OTOOL_LIBRARY_PATTERN,
  READELF_NEEDED_PATTERN,
} from './constants.ts';
import { matchingLines } from './dynamic-libraries.ts';

test('reads otool output', () => {
  const output = 'target/debug/cc-media-probe:\n\t/usr/lib/libSystem.B.dylib (compatibility version 1.0.0, current version 1359.40.3)\n';
  assert.deepEqual(matchingLines(output, OTOOL_LIBRARY_PATTERN), ['/usr/lib/libSystem.B.dylib']);
});

test('reads readelf output', () => {
  const output =
    'Dynamic section at offset 0x2d0 contains 3 entries:\n' +
    '  Tag        Type                         Name/Value\n' +
    ' 0x0000000000000001 (NEEDED)             Shared library: [libgcc_s.so.1]\n' +
    ' 0x0000000000000001 (NEEDED)             Shared library: [libc.so.6]\n' +
    ' 0x000000000000000c (INIT)               0x5000\n';
  assert.deepEqual(matchingLines(output, READELF_NEEDED_PATTERN), ['libgcc_s.so.1', 'libc.so.6']);
});

test('reads objdump and dumpbin output', () => {
  const objdump = 'The Import Tables:\n\tDLL Name: KERNEL32.dll\n\tvma:  Hint/Ord Member-Name\n\tDLL Name: ntdll.dll\n';
  assert.deepEqual(matchingLines(objdump, OBJDUMP_DLL_PATTERN), ['KERNEL32.dll', 'ntdll.dll']);
  const dumpbin = '  Image has the following dependencies:\r\n\r\n    KERNEL32.dll\r\n    VCRUNTIME140.dll\r\n\r\n  Summary\r\n';
  assert.deepEqual(matchingLines(dumpbin, DUMPBIN_DLL_PATTERN), ['KERNEL32.dll', 'VCRUNTIME140.dll']);
});
