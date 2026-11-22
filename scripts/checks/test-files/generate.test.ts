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

// Tests of a whole run of the generator, with a stand-in for the tools, and of the state and pruning it relies on.

import assert from 'node:assert/strict';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, parse } from 'node:path';
import { test } from 'node:test';

import { generateTestFiles } from './generate.ts';
import { removeUnusedFiles } from './prune.ts';
import { readState, writeState } from './state.ts';
import { locateTools, usualLocations, type ToolRunner } from './tools.ts';

/** A stand-in that writes its arguments into the output and counts its runs. `missing` names programs that cannot be
 * run, `failing` programs that fail, and `files` the paths of usual locations that exist. */
function standIn(missing: string[] = [], failing: string[] = [], files: string[] = []): ToolRunner & { runs: string[] } {
  const runs: string[] = [];
  return {
    runs,
    version: (program) => (missing.includes(program) ? undefined : `${program} version 1.0`),
    isFile: (path) => files.includes(path),
    run(program, commandArguments) {
      runs.push(program);
      if (failing.includes(program)) {
        return false;
      }
      const output = commandArguments.at(-1) ?? '';
      writeFileSync(output, commandArguments.join(' '));
      return true;
    },
  };
}

const recipesOf = (root: string, ...components: string[]): string =>
  join(root, 'crates', 'cc-media-probe-testkit', 'test-files', ...components);

/** Writes recipes for one H.264 stream and one Matroska file made from it into the repository at `root`. */
function writeRecipes(root: string, mkvmergeArguments = ['--deterministic', 'seed', '{input:0}', '{output}']): void {
  mkdirSync(recipesOf(root, 'video'), { recursive: true });
  mkdirSync(recipesOf(root, 'containers'), { recursive: true });
  writeFileSync(
    recipesOf(root, 'video', 'h264.json'),
    JSON.stringify({ files: [{ name: 'a.h264', tool: 'ffmpeg', arguments: ['-y', '{output}'] }] }),
  );
  writeFileSync(
    recipesOf(root, 'containers', 'matroska.json'),
    JSON.stringify({ files: [{ name: 'a.mkv', tool: 'mkvmerge', inputs: ['video/h264/a.h264'], arguments: mkvmergeArguments }] }),
  );
}

/** A temporary repository with the recipes of `writeRecipes`. */
function repository(): string {
  const root = mkdtempSync(join(tmpdir(), 'generator-'));
  writeRecipes(root);
  return root;
}

const output = (root: string, ...components: string[]): string => join(root, 'target', 'test-files', ...components);

test('generates every file once, in input order, and keeps them on later runs', () => {
  const root = repository();
  try {
    const tools = standIn();
    assert.equal(generateTestFiles(root, tools, {}), 0);
    assert.deepEqual(tools.runs, ['ffmpeg', 'mkvmerge']);
    assert.match(readFileSync(output(root, 'containers', 'matroska', 'a.mkv'), 'utf8'), /test-files.video.h264.a\.h264/u);
    assert.equal(existsSync(output(root, '.partial')), false);
    const state = readState(output(root, 'generation-state.json'));
    assert.deepEqual([...state.keys()], ['containers/matroska/a.mkv', 'video/h264/a.h264']);
    assert.equal(state.get('video/h264/a.h264')?.version, 'ffmpeg version 1.0');

    assert.equal(generateTestFiles(root, tools, {}), 0);
    assert.deepEqual(tools.runs, ['ffmpeg', 'mkvmerge']);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('generates again only a changed recipe and what depends on it, and removes unused files', () => {
  const root = repository();
  try {
    assert.equal(generateTestFiles(root, standIn(), {}), 0);
    writeFileSync(output(root, 'video', 'h264', 'stray.h264'), 'left over');
    writeRecipes(root, ['--deterministic', 'other', '{input:0}', '{output}']);
    const tools = standIn();
    assert.equal(generateTestFiles(root, tools, {}), 0);
    assert.deepEqual(tools.runs, ['mkvmerge']);
    assert.match(readFileSync(output(root, 'containers', 'matroska', 'a.mkv'), 'utf8'), /--deterministic other/u);
    assert.equal(existsSync(output(root, 'video', 'h264', 'stray.h264')), false);

    rmSync(recipesOf(root, 'containers'), { recursive: true });
    assert.equal(generateTestFiles(root, standIn(), {}), 0);
    assert.equal(existsSync(output(root, 'containers')), false);
    assert.deepEqual([...readState(output(root, 'generation-state.json')).keys()], ['video/h264/a.h264']);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('stops before touching anything when a tool is not on the system path', () => {
  const root = repository();
  try {
    mkdirSync(output(root), { recursive: true });
    writeFileSync(output(root, 'kept.bin'), 'kept');
    const tools = standIn(['mkvmerge']);
    assert.equal(generateTestFiles(root, tools, {}), 1);
    assert.deepEqual(tools.runs, []);
    assert.equal(existsSync(output(root, 'kept.bin')), true);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('leaves no file of a failed recipe or of what needs it, and tries again next time', () => {
  const root = repository();
  try {
    const failing = standIn([], ['ffmpeg']);
    assert.equal(generateTestFiles(root, failing, {}), 1);
    assert.deepEqual(failing.runs, ['ffmpeg']);
    assert.equal(existsSync(output(root, 'video', 'h264', 'a.h264')), false);
    assert.equal(existsSync(output(root, 'containers', 'matroska', 'a.mkv')), false);
    const working = standIn();
    assert.equal(generateTestFiles(root, working, {}), 0);
    assert.deepEqual(working.runs, ['ffmpeg', 'mkvmerge']);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('reads a missing or damaged state as empty and writes it sorted', () => {
  const directory = mkdtempSync(join(tmpdir(), 'state-'));
  try {
    const file = join(directory, 'nested', 'generation-state.json');
    assert.equal(readState(file).size, 0);
    writeState(file, new Map([
      ['b', { identity: '2', tool: 'ffmpeg', version: 'v' }],
      ['a', { identity: '1', tool: 'ffmpeg', version: 'v' }],
    ]));
    assert.deepEqual([...readState(file).keys()], ['a', 'b']);
    writeFileSync(file, '{ damaged');
    assert.equal(readState(file).size, 0);
    writeFileSync(file, '{"a": {"identity": 1}}');
    assert.equal(readState(file).size, 0);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test('prunes only unwanted files and the folders they leave empty', () => {
  const directory = mkdtempSync(join(tmpdir(), 'prune-'));
  try {
    assert.deepEqual(removeUnusedFiles(join(directory, 'missing'), new Set()), []);
    mkdirSync(join(directory, 'video', 'h264'), { recursive: true });
    mkdirSync(join(directory, 'audio', 'opus'), { recursive: true });
    mkdirSync(join(directory, '.partial', 'video'), { recursive: true });
    writeFileSync(join(directory, 'video', 'h264', 'kept.h264'), '');
    writeFileSync(join(directory, 'audio', 'opus', 'old.opus'), '');
    writeFileSync(join(directory, 'generation-state.json'), '{}');
    assert.deepEqual(removeUnusedFiles(directory, new Set(['video/h264/kept.h264'])), ['audio/opus/old.opus']);
    assert.equal(existsSync(join(directory, 'audio')), false);
    assert.equal(existsSync(join(directory, '.partial')), false);
    assert.equal(existsSync(join(directory, 'generation-state.json')), true);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test('finds a tool on the system path first, then in a usual location, and reports one found in neither', () => {
  const directory = tmpdir();
  const bundled = join(parse(directory).root, 'Applications', 'MKVToolNix.app', 'Contents', 'MacOS', 'mkvmerge');
  const found = locateTools(['mkvmerge', 'ffmpeg', 'ffmpeg'], standIn(['mkvmerge'], [], [bundled]), directory, {});
  assert.deepEqual(found.problems, []);
  assert.deepEqual([...found.located], [
    ['ffmpeg', { program: 'ffmpeg', version: 'ffmpeg version 1.0' }],
    ['mkvmerge', { program: bundled, version: `${bundled} version 1.0` }],
  ]);
  const missing = locateTools(['mkvmerge'], standIn(['mkvmerge']), directory, {});
  assert.deepEqual([...missing.located], []);
  assert.deepEqual(missing.problems, [
    'mkvmerge is neither on the system path nor in its usual install locations; install it to generate the test files',
  ]);
});

test('lists the usual locations below the root and below folders that environment variables name', () => {
  const directory = tmpdir();
  const root = parse(directory).root;
  const locations = usualLocations('mkvmerge', directory, { ProgramFiles: join(root, 'Programs'), ProgramData: '' });
  assert.ok(locations.includes(join(root, 'usr', 'bin', 'mkvmerge')));
  assert.ok(locations.includes(join(root, 'opt', 'homebrew', 'bin', 'mkvmerge')));
  assert.ok(locations.includes(join(root, 'Applications', 'MKVToolNix.app', 'Contents', 'MacOS', 'mkvmerge')));
  assert.ok(locations.includes(join(root, 'Programs', 'MKVToolNix', 'mkvmerge.exe')));
  assert.equal(locations.some((location) => location.includes('chocolatey')), false);
});

test('runs a tool found in a usual location by its full path', () => {
  const root = repository();
  try {
    const bundled = join(parse(root).root, 'Applications', 'MKVToolNix.app', 'Contents', 'MacOS', 'mkvmerge');
    const tools = standIn(['mkvmerge'], [], [bundled]);
    assert.equal(generateTestFiles(root, tools, {}), 0);
    assert.deepEqual(tools.runs, ['ffmpeg', bundled]);
    assert.equal(readState(output(root, 'generation-state.json')).get('containers/matroska/a.mkv')?.version, `${bundled} version 1.0`);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
