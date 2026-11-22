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

//! A tokenizer for formatted Rust source.
//!
//! It is not a full Rust lexer. It skips whitespace and comments and yields
//! identifiers, lifetimes, literals and single punctuation characters, each
//! with the line it starts on and whether the next token follows directly.
//! That is enough for the structure check (D23).

mod characters;
mod constants;
mod literal_scanner;
mod token;
mod tokenizer;

pub use token::{LiteralKind, Token, TokenKind};
pub use tokenizer::tokenize;
