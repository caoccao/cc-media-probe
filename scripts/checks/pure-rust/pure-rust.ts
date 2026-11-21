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

// The pure-Rust verification of D4 (plan section 6) for the two published crates: the inventory of their dependency
// graph (layer 1), their build with every native-compiler variable pointing at a missing program and the inspection
// of every build-script output (layer 3), and the dynamic libraries of the linked binary (layer 4). Layer 2 is the
// review recorded in `crates/cc-media-probe-checks/data/build-scripts.tsv`.

import { join } from 'node:path';

import { captureCommand, runCommand } from '../command.ts';
import { CARGO, CHECKS_ARGUMENTS } from '../constants.ts';
import { executableFromMessages } from './built-binary.ts';
import {
  BINARY_NAME,
  BINARY_NOT_FOUND,
  BUILD_ARGUMENTS,
  BUILD_LAYER,
  BUILD_OUTPUT_COMMAND,
  DYNAMIC_LIBRARIES_COMMAND,
  FEATURE_SELECTIONS,
  INVENTORY_COMMAND,
  INVENTORY_LAYER,
  LISTING_FAILED,
  MESSAGE_FORMAT_ARGUMENT,
  MISSING_COMPILER_NAME,
  PLATFORM_NAMES,
  RESULT_LAYER,
  TARGET_DIRECTORY,
  TARGET_DIRECTORY_OPTION,
  UNSUPPORTED_PLATFORM,
} from './constants.ts';
import { listDynamicLibraries } from './dynamic-libraries.ts';

/** Runs the three automated layers; returns whether all of them pass. */
export function verifyPureRust(root: string): boolean {
  console.log(INVENTORY_LAYER);
  if (!runCommand(CARGO, [...CHECKS_ARGUMENTS, INVENTORY_COMMAND], root)) {
    return false;
  }
  console.log(BUILD_LAYER);
  const targetDirectory = join(root, ...TARGET_DIRECTORY);
  const missingCompiler = join(targetDirectory, MISSING_COMPILER_NAME);
  const environment = { ...process.env, CC: missingCompiler, CXX: missingCompiler, AR: missingCompiler };
  const targetArguments = [TARGET_DIRECTORY_OPTION, targetDirectory];
  for (const features of FEATURE_SELECTIONS) {
    if (!runCommand(CARGO, [...BUILD_ARGUMENTS, ...features, ...targetArguments], root, environment)) {
      return false;
    }
  }
  if (!runCommand(CARGO, [...CHECKS_ARGUMENTS, BUILD_OUTPUT_COMMAND, targetDirectory], root)) {
    return false;
  }
  console.log(RESULT_LAYER);
  const platform = PLATFORM_NAMES.get(process.platform);
  if (platform === undefined) {
    console.error(`${UNSUPPORTED_PLATFORM}: ${process.platform}`);
    return false;
  }
  const messages = captureCommand(CARGO, [...BUILD_ARGUMENTS, MESSAGE_FORMAT_ARGUMENT, ...targetArguments], root, environment);
  const binary = messages === undefined ? undefined : executableFromMessages(messages, BINARY_NAME);
  if (binary === undefined) {
    console.error(BINARY_NOT_FOUND);
    return false;
  }
  const libraries = listDynamicLibraries(platform, binary, root);
  if (libraries === undefined) {
    console.error(LISTING_FAILED);
    return false;
  }
  return runCommand(CARGO, [...CHECKS_ARGUMENTS, DYNAMIC_LIBRARIES_COMMAND, platform, ...libraries], root);
}
