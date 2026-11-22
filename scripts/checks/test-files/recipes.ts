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

// Reading and validating the recipes: `<topic>/<format>.json` files below the test kit's `test-files/` folder.

import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { basename, join } from 'node:path';

import {
  BAD_FILE_NAME,
  DUPLICATE_OUTPUT,
  FILE_ENCODING,
  FILE_NAME_PATTERN,
  FORMAT_PATTERN,
  INPUT_PLACEHOLDER_PATTERN,
  INPUT_POSITION_GROUP,
  MALFORMED_RECIPES,
  MISPLACED_RECIPES,
  NO_OUTPUT_PLACEHOLDER,
  OUTPUT_PLACEHOLDER,
  PATH_SEPARATOR,
  RECIPE_EXTENSION,
  TOOLS,
  TOPICS,
  UNKNOWN_INPUT,
  UNKNOWN_PLACEHOLDER,
  UNKNOWN_TOOL,
} from './constants.ts';
import { isRecord, isText, isTextList } from './json-values.ts';
import { LIST_SEPARATOR } from '../specifications/constants.ts';

/** One file to generate: where it goes, which tool makes it, from which other generated files, with which arguments. */
export interface Recipe {
  /** The file's path below the output folder, with `/`: `<topic>/<format>/<name>`. */
  readonly path: string;
  readonly tool: string;
  /** The paths of the generated files it is made from. */
  readonly inputs: readonly string[];
  /** The tool's arguments, with `{output}` and `{input:N}` placeholders. */
  readonly commandArguments: readonly string[];
}

/** The recipes that were read, and the problems that stop the run. */
export interface RecipeSet {
  readonly recipes: readonly Recipe[];
  readonly problems: readonly string[];
}

/** Reads every recipe file below `directory`, which need not exist. */
export function readRecipes(directory: string): RecipeSet {
  const recipes: Recipe[] = [];
  const problems: string[] = [];
  if (!existsSync(directory)) {
    return { recipes, problems };
  }
  for (const topic of readdirSync(directory, { withFileTypes: true })) {
    const topicPath = join(directory, topic.name);
    if (!topic.isDirectory() || !TOPICS.includes(topic.name)) {
      problems.push(`${topicPath}: ${MISPLACED_RECIPES}`);
      continue;
    }
    for (const entry of readdirSync(topicPath, { withFileTypes: true })) {
      const file = join(topicPath, entry.name);
      const format = basename(entry.name, RECIPE_EXTENSION);
      if (!entry.isFile() || !entry.name.endsWith(RECIPE_EXTENSION) || !FORMAT_PATTERN.test(format)) {
        problems.push(`${file}: ${MISPLACED_RECIPES}`);
        continue;
      }
      const parsed = parseRecipes(readFileSync(file, FILE_ENCODING), [topic.name, format].join(PATH_SEPARATOR));
      if (parsed === undefined) {
        problems.push(`${file}: ${MALFORMED_RECIPES}`);
      } else {
        recipes.push(...parsed);
      }
    }
  }
  return { recipes, problems };
}

/** The recipes of one file, whose outputs go to the folder `folder`, or `undefined` when the text has another shape. */
export function parseRecipes(text: string, folder: string): Recipe[] | undefined {
  let document: unknown;
  try {
    document = JSON.parse(text);
  } catch {
    return undefined;
  }
  if (!isRecord(document)) {
    return undefined;
  }
  const { files } = document;
  if (!Array.isArray(files)) {
    return undefined;
  }
  const recipes: Recipe[] = [];
  for (const file of files) {
    if (!isRecord(file)) {
      return undefined;
    }
    const { name, tool, inputs = [], arguments: commandArguments } = file;
    if (!isText(name) || !isText(tool) || !isTextList(inputs) || !isTextList(commandArguments)) {
      return undefined;
    }
    recipes.push({ path: [folder, name].join(PATH_SEPARATOR), tool, inputs, commandArguments });
  }
  return recipes;
}

/** The problems of a set of recipes taken together: names, tools, inputs and placeholders. */
export function recipeProblems(recipes: readonly Recipe[]): string[] {
  const problems: string[] = [];
  const paths = new Set<string>();
  for (const recipe of recipes) {
    if (paths.has(recipe.path)) {
      problems.push(`${recipe.path}: ${DUPLICATE_OUTPUT}`);
    }
    paths.add(recipe.path);
  }
  for (const recipe of recipes) {
    const name = recipe.path.split(PATH_SEPARATOR).at(-1) ?? recipe.path;
    if (!FILE_NAME_PATTERN.test(name)) {
      problems.push(`${recipe.path}: ${BAD_FILE_NAME}`);
    }
    if (!TOOLS.has(recipe.tool)) {
      problems.push(`${recipe.path}: ${UNKNOWN_TOOL} ${[...TOOLS.keys()].join(LIST_SEPARATOR)}: ${recipe.tool}`);
    }
    for (const input of recipe.inputs.filter((candidate) => !paths.has(candidate))) {
      problems.push(`${recipe.path}: ${UNKNOWN_INPUT} ${input}`);
    }
    if (!recipe.commandArguments.some((argument) => argument.includes(OUTPUT_PLACEHOLDER))) {
      problems.push(`${recipe.path}: ${NO_OUTPUT_PLACEHOLDER}`);
    }
    const positions = recipe.commandArguments.flatMap((argument) =>
      [...argument.matchAll(INPUT_PLACEHOLDER_PATTERN)].map((match) => Number(match[INPUT_POSITION_GROUP])),
    );
    if (positions.some((position) => position >= recipe.inputs.length)) {
      problems.push(`${recipe.path}: ${UNKNOWN_PLACEHOLDER}`);
    }
  }
  return problems;
}
