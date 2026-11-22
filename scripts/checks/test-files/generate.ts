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

// One run of the generator: check the recipes and tools, remove unused files, and generate only what is missing or
// changed (plan section 9.1).

import { existsSync, mkdirSync, renameSync, rmSync } from 'node:fs';
import { dirname, join } from 'node:path';

import {
  CURRENT_SUMMARY,
  GENERATED_FILE,
  GENERATED_SUMMARY,
  GENERATION_FAILED,
  INPUT_FAILED,
  INPUT_PLACEHOLDER_PATTERN,
  OUTPUT_DIRECTORY,
  OUTPUT_PLACEHOLDER,
  PARTIAL_DIRECTORY,
  PATH_SEPARATOR,
  RECIPES_DIRECTORY,
  REMOVED_FILE,
  REMOVED_SUMMARY,
  STATE_FILE,
} from './constants.ts';
import { generationOrder, identities } from './identity.ts';
import { removeUnusedFiles } from './prune.ts';
import { readRecipes, recipeProblems, type Recipe } from './recipes.ts';
import { readState, writeState } from './state.ts';
import { locateTools, SYSTEM_TOOLS, type ToolRunner } from './tools.ts';
import { EMPTY } from '../specifications/constants.ts';

/** Generates the test files of the repository at `root`; returns the exit status: 0, or 1 after any problem. */
export function generateTestFiles(
  root: string,
  runner: ToolRunner = SYSTEM_TOOLS,
  environment: NodeJS.ProcessEnv = process.env,
): number {
  const outputDirectory = join(root, ...OUTPUT_DIRECTORY);
  const { recipes, problems } = readRecipes(join(root, ...RECIPES_DIRECTORY));
  const { order, problems: orderProblems } = generationOrder(recipes);
  const { located, problems: toolProblems } = locateTools(
    recipes.map((recipe) => recipe.tool),
    runner,
    root,
    environment,
  );
  const allProblems = [...problems, ...recipeProblems(recipes), ...orderProblems, ...toolProblems];
  if (allProblems.length > 0) {
    allProblems.forEach((problem) => console.error(problem));
    return 1;
  }
  const versions = new Map([...located].map(([tool, { version }]) => [tool, version]));
  const wanted = identities(order, versions);
  const statePath = join(outputDirectory, STATE_FILE);
  const state = readState(statePath);
  const removed = removeUnusedFiles(outputDirectory, new Set(wanted.keys()));
  removed.forEach((path) => console.log(`${REMOVED_FILE} ${path}`));
  [...state.keys()].filter((path) => !wanted.has(path)).forEach((path) => state.delete(path));
  let generated = 0;
  let current = 0;
  const failed = new Set<string>();
  for (const recipe of order) {
    const identity = wanted.get(recipe.path) ?? EMPTY;
    if (state.get(recipe.path)?.identity === identity && existsSync(filePath(outputDirectory, recipe.path))) {
      current += 1;
      continue;
    }
    state.delete(recipe.path);
    const failedInput = recipe.inputs.find((input) => failed.has(input));
    if (failedInput !== undefined) {
      failed.add(recipe.path);
      console.error(`${recipe.path}: ${INPUT_FAILED} ${failedInput}`);
    } else if (makeFile(recipe, located.get(recipe.tool)?.program ?? recipe.tool, outputDirectory, runner, root)) {
      state.set(recipe.path, { identity, tool: recipe.tool, version: versions.get(recipe.tool) ?? EMPTY });
      writeState(statePath, state);
      generated += 1;
      console.log(`${GENERATED_FILE} ${recipe.path}`);
    } else {
      failed.add(recipe.path);
      console.error(`${recipe.path}: ${GENERATION_FAILED}`);
    }
  }
  writeState(statePath, state);
  rmSync(join(outputDirectory, PARTIAL_DIRECTORY), { recursive: true, force: true });
  console.log(`${GENERATED_SUMMARY} ${generated}, ${CURRENT_SUMMARY} ${current}, ${REMOVED_SUMMARY} ${removed.length}`);
  return failed.size === 0 ? 0 : 1;
}

/** The file-system path of a generated file. */
function filePath(outputDirectory: string, path: string): string {
  return join(outputDirectory, ...path.split(PATH_SEPARATOR));
}

/** Runs one recipe into an unfinished file and moves it into place only when the tool succeeded and wrote it. An
 * outdated file is removed when its new version fails, so no test reads a file that its recipe no longer describes. */
function makeFile(
  recipe: Recipe,
  program: string,
  outputDirectory: string,
  runner: ToolRunner,
  directory: string,
): boolean {
  const target = filePath(outputDirectory, recipe.path);
  const partial = join(outputDirectory, PARTIAL_DIRECTORY, ...recipe.path.split(PATH_SEPARATOR));
  rmSync(partial, { force: true });
  mkdirSync(dirname(partial), { recursive: true });
  const commandArguments = recipe.commandArguments.map((argument) =>
    argument
      .replaceAll(OUTPUT_PLACEHOLDER, partial)
      .replace(INPUT_PLACEHOLDER_PATTERN, (placeholder, position: string) =>
        filePath(outputDirectory, recipe.inputs[Number(position)] ?? placeholder),
      ),
  );
  if (!runner.run(program, commandArguments, directory) || !existsSync(partial)) {
    rmSync(partial, { force: true });
    rmSync(target, { force: true });
    return false;
  }
  mkdirSync(dirname(target), { recursive: true });
  renameSync(partial, target);
  return true;
}
