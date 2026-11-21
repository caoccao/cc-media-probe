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

// Rows of the manifest that the tests of the document modules share. This file holds no test of its own.

import type { DocumentEntry } from './manifest.ts';

export const ETSI: DocumentEntry = {
  identifier: 'etsi-ts-102366',
  folder: 'audio/etsi',
  title: 'Digital Audio Compression (AC-3, Enhanced AC-3) Standard',
  publisher: 'ETSI',
  edition: 'V1.4.1 (2017-09)',
  landing: 'https://www.etsi.org/deliver/etsi_ts/102300_102399/102366/01.04.01_60/',
  artifact: 'https://www.etsi.org/deliver/etsi_ts/102300_102399/102366/01.04.01_60/ts_102366v010401p.pdf',
  immutableReference: '',
  contentType: 'application/pdf',
  availability: 'free',
  retrieval: 'fetch',
  appliesTo: 'AC-3 and E-AC-3',
  classification: 'normative',
  checksum: '',
  retrieved: '',
  approved: '',
};

export const SNAPSHOT: DocumentEntry = {
  ...ETSI,
  identifier: 'xiph-ogg-framing',
  folder: 'containers/xiph',
  title: 'Ogg logical bitstream framing',
  publisher: 'Xiph.Org Foundation',
  edition: 'Living; commit 0123456',
  landing: 'https://xiph.org/ogg/doc/framing.html',
  artifact: 'https://xiph.org/ogg/doc/framing.html',
  immutableReference: 'https://raw.githubusercontent.com/xiph/ogg/0123456/doc/framing.html',
  contentType: 'text/plain',
  retrieval: 'snapshot',
  classification: 'secondary',
};
