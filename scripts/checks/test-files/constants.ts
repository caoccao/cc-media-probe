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

// Paths, recipe syntax, tools and messages of the test-file generator (plan section 9.1).

/** The recipes, relative to the repository root, by component: `<topic>/<format>.json` below it. */
export const RECIPES_DIRECTORY = ['crates', 'cc-media-probe-testkit', 'test-files'];
/** The generated files, relative to the repository root, by component: `<topic>/<format>/<name>` below it. */
export const OUTPUT_DIRECTORY = ['target', 'test-files'];
/** The record of what was generated, in the output directory. */
export const STATE_FILE = 'generation-state.json';
/** Added to the state file's name while it is written. */
export const STATE_PARTIAL_SUFFIX = '.partial';
/** Where a file is written before it is complete, in the output directory. */
export const PARTIAL_DIRECTORY = '.partial';
/** The extension of a recipe file. */
export const RECIPE_EXTENSION = '.json';
/** The topics of `docs/specs/files/`, which the recipes and the generated files follow. */
export const TOPICS = ['containers', 'video', 'audio', 'subtitles', 'tags-and-registries'];
/** A format's folder name: lower-case words and digits joined by hyphens. */
export const FORMAT_PATTERN = /^[a-z0-9]+(?:-[a-z0-9]+)*$/u;
/** A generated file's name: no separator, no leading dot, at most one dot before each extension. */
export const FILE_NAME_PATTERN = /^[a-z0-9][a-z0-9-]*(?:\.[a-z0-9]+)+$/u;
/** Separates the components of a path that a recipe writes. */
export const PATH_SEPARATOR = '/';
/** Stands for the file being generated in a recipe's arguments. */
export const OUTPUT_PLACEHOLDER = '{output}';
/** Stands for an input in a recipe's arguments; the group is its position in `inputs`. */
export const INPUT_PLACEHOLDER_PATTERN = /\{input:(\d+)\}/gu;
/** The group of an input placeholder that holds the position. */
export const INPUT_POSITION_GROUP = 1;
/** What `typeof` gives for a string. */
export const TEXT_TYPE = 'string';
/** What `typeof` gives for an object, arrays and `null` included. */
export const OBJECT_TYPE = 'object';
/** The encoding of the recipe and state files. */
export const FILE_ENCODING = 'utf8';
/** The indentation of the state file. */
export const STATE_INDENTATION = 2;
/** Ends the state file. */
export const LINE_END = '\n';
/** The digest that identifies a generated file. */
export const IDENTITY_ALGORITHM = 'sha256';
/** The encoding of an identity. */
export const IDENTITY_ENCODING = 'hex';
/** Usual install folders below the file-system root: the system and local folders, Homebrew on Apple silicon,
 * MacPorts, snap packages, and the command-line tools inside the MKVToolNix application bundle on macOS. */
export const ROOT_TOOL_FOLDERS: readonly (readonly string[])[] = [
  ['usr', 'bin'],
  ['usr', 'local', 'bin'],
  ['opt', 'homebrew', 'bin'],
  ['opt', 'local', 'bin'],
  ['snap', 'bin'],
  ['Applications', 'MKVToolNix.app', 'Contents', 'MacOS'],
];
/** Usual install folders below a folder that an environment variable names, as on Windows: the MKVToolNix
 * installer's default, Chocolatey's shims, the links of WinGet, and Scoop's shims. */
export const VARIABLE_TOOL_FOLDERS: readonly (readonly [string, readonly string[]])[] = [
  ['ProgramFiles', ['MKVToolNix']],
  ['ProgramData', ['chocolatey', 'bin']],
  ['LOCALAPPDATA', ['Microsoft', 'WinGet', 'Links']],
  ['USERPROFILE', ['scoop', 'shims']],
];
/** The suffixes a tool's executable may have: none, or that of Windows. */
export const EXECUTABLE_SUFFIXES = ['', '.exe'];
/** Splits a tool's version report into lines. */
export const LINE_PATTERN = /\r?\n/u;
/** The tools a recipe may run, each with the arguments that make it print its version on the first line. */
export const TOOLS: ReadonlyMap<string, readonly string[]> = new Map([
  ['ffmpeg', ['-version']],
  ['mkvmerge', ['--version']],
]);

/** A recipe file that is not valid JSON or not of the recipe shape. */
export const MALFORMED_RECIPES = 'not a recipe file: expected {"files": [{"name", "tool", "arguments", "inputs"?}]}';
/** A recipe file outside `<topic>/<format>.json`. */
export const MISPLACED_RECIPES = 'a recipe file must be <topic>/<format>.json, with a topic of docs/specs/files';
/** A generated file's name that the pattern refuses. */
export const BAD_FILE_NAME = 'a file name must be lower-case words, digits and hyphens with an extension';
/** Two recipes for one file. */
export const DUPLICATE_OUTPUT = 'more than one recipe generates';
/** A tool that the generator does not run. */
export const UNKNOWN_TOOL = 'unknown tool; the generator runs';
/** An input that no recipe generates. */
export const UNKNOWN_INPUT = 'no recipe generates the input';
/** An input placeholder without its input. */
export const UNKNOWN_PLACEHOLDER = 'the arguments name an input that `inputs` does not have';
/** A recipe whose arguments never name its output. */
export const NO_OUTPUT_PLACEHOLDER = 'the arguments must contain {output}';
/** Recipes that depend on each other in a circle. */
export const CIRCULAR_INPUTS = 'recipes whose inputs form a circle';
/** A tool that is neither on the system path nor in a usual location. */
export const TOOL_MISSING =
  'is neither on the system path nor in its usual install locations; install it to generate the test files';
/** A tool that failed or wrote nothing. */
export const GENERATION_FAILED = 'failed to generate';
/** An input that failed, which leaves the file that needs it. */
export const INPUT_FAILED = 'not generated, because an input failed:';
/** The summary of a run. */
export const GENERATED_SUMMARY = 'test files: generated';
/** The count of files that were already current. */
export const CURRENT_SUMMARY = 'current';
/** The count of files that were removed. */
export const REMOVED_SUMMARY = 'removed';
/** Introduces a removed file. */
export const REMOVED_FILE = 'removed unused test file';
/** Introduces a generated file. */
export const GENERATED_FILE = 'generated';
