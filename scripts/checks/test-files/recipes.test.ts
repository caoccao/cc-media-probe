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

// Tests of reading and validating recipes.

import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';

import { parseRecipes, readRecipes, recipeProblems, type Recipe } from './recipes.ts';

const recipe = (path: string, inputs: string[] = [], commandArguments = ['{output}'], tool = 'ffmpeg'): Recipe => ({
  path,
  tool,
  inputs,
  commandArguments,
});

test('reads the recipes of every <topic>/<format>.json and places their outputs in <topic>/<format>/', () => {
  const directory = mkdtempSync(join(tmpdir(), 'recipes-'));
  try {
    mkdirSync(join(directory, 'video'));
    writeFileSync(
      join(directory, 'video', 'h264.json'),
      JSON.stringify({ files: [{ name: 'small.h264', tool: 'ffmpeg', arguments: ['-y', '{output}'] }] }),
    );
    const { recipes, problems } = readRecipes(directory);
    assert.deepEqual(problems, []);
    assert.deepEqual(recipes, [recipe('video/h264/small.h264', [], ['-y', '{output}'])]);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test('reads nothing from a missing folder and refuses misplaced or malformed recipe files', () => {
  assert.deepEqual(readRecipes(join(tmpdir(), 'no-such-recipes-folder')), { recipes: [], problems: [] });
  const directory = mkdtempSync(join(tmpdir(), 'recipes-'));
  try {
    mkdirSync(join(directory, 'pictures'));
    mkdirSync(join(directory, 'audio'));
    writeFileSync(join(directory, 'audio', 'Opus.json'), '{"files": []}');
    writeFileSync(join(directory, 'audio', 'flac.json'), '{"files": [{"name": "x.flac"}]}');
    writeFileSync(join(directory, 'audio', 'notes.txt'), '');
    const { recipes, problems } = readRecipes(directory);
    assert.deepEqual(recipes, []);
    assert.equal(problems.length, 4);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test('parses only the recipe shape', () => {
  assert.equal(parseRecipes('not json', 'video/h264'), undefined);
  assert.equal(parseRecipes('[]', 'video/h264'), undefined);
  assert.equal(parseRecipes('{"files": [1]}', 'video/h264'), undefined);
  assert.equal(parseRecipes('{"files": [{"name": "a.h264", "tool": "ffmpeg", "arguments": [1]}]}', 'video/h264'), undefined);
  assert.deepEqual(
    parseRecipes('{"files": [{"name": "a.mkv", "tool": "mkvmerge", "inputs": ["video/h264/a.h264"], "arguments": ["{output}", "{input:0}"]}]}', 'containers/matroska'),
    [recipe('containers/matroska/a.mkv', ['video/h264/a.h264'], ['{output}', '{input:0}'], 'mkvmerge')],
  );
});

test('finds duplicate outputs, bad names, unknown tools and inputs, and wrong placeholders', () => {
  assert.deepEqual(recipeProblems([recipe('video/h264/a.h264'), recipe('containers/matroska/a.mkv', ['video/h264/a.h264'], ['{input:0}', '{output}'], 'mkvmerge')]), []);
  const problems = recipeProblems([
    recipe('video/h264/a.h264'),
    recipe('video/h264/a.h264'),
    recipe('video/h264/Bad Name.h264'),
    recipe('video/h264/b.h264', [], ['{output}'], 'encoder'),
    recipe('video/h264/c.h264', ['video/h264/missing.h264']),
    recipe('video/h264/d.h264', [], ['-i', 'x']),
    recipe('video/h264/e.h264', [], ['{input:0}', '{output}']),
  ]);
  assert.equal(problems.length, 6);
  assert.match(problems.join('\n'), /more than one recipe/u);
  assert.match(problems.join('\n'), /unknown tool; the generator runs ffmpeg, mkvmerge: encoder/u);
  assert.match(problems.join('\n'), /no recipe generates the input video\/h264\/missing.h264/u);
});
