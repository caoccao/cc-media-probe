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

// Tests of the order of recipes and the identity of generated files.

import assert from 'node:assert/strict';
import { test } from 'node:test';

import { generationOrder, identities } from './identity.ts';
import type { Recipe } from './recipes.ts';

const video: Recipe = { path: 'video/h264/a.h264', tool: 'ffmpeg', inputs: [], commandArguments: ['{output}'] };
const audio: Recipe = { path: 'audio/opus/a.opus', tool: 'ffmpeg', inputs: [], commandArguments: ['{output}'] };
const movie: Recipe = {
  path: 'containers/matroska/a.mkv',
  tool: 'mkvmerge',
  inputs: [video.path, audio.path],
  commandArguments: ['{output}', '{input:0}', '{input:1}'],
};
const versions = new Map([
  ['ffmpeg', 'ffmpeg version 9.1'],
  ['mkvmerge', 'mkvmerge v104.0'],
]);

test('puts every input before the recipe that uses it', () => {
  const { order, problems } = generationOrder([movie, video, audio]);
  assert.deepEqual(problems, []);
  assert.deepEqual(
    order.map((recipe) => recipe.path),
    [video.path, audio.path, movie.path],
  );
});

test('reports inputs that form a circle', () => {
  const first: Recipe = { ...video, inputs: [audio.path] };
  const second: Recipe = { ...audio, inputs: [video.path] };
  const { problems } = generationOrder([first, second]);
  assert.equal(problems.length, 1);
  assert.match(problems[0] ?? '', /circle: video\/h264\/a.h264, audio\/opus\/a.opus/u);
});

test('keeps an identity while nothing changes and changes it with the version, the arguments or an input', () => {
  const base = identities([video, audio, movie], versions);
  assert.deepEqual(identities([video, audio, movie], versions), base);
  assert.match(base.get(movie.path) ?? '', /^[0-9a-f]{64}$/u);

  const newerTool = identities([video, audio, movie], new Map([...versions, ['ffmpeg', 'ffmpeg version 9.2']]));
  assert.notEqual(newerTool.get(video.path), base.get(video.path));
  assert.notEqual(newerTool.get(movie.path), base.get(movie.path));

  const changedAudio = { ...audio, commandArguments: ['-ac', '1', '{output}'] };
  const changed = identities([video, changedAudio, movie], versions);
  assert.equal(changed.get(video.path), base.get(video.path));
  assert.notEqual(changed.get(audio.path), base.get(audio.path));
  assert.notEqual(changed.get(movie.path), base.get(movie.path));
});
