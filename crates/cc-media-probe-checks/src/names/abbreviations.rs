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

//! The data table of the name check: the known abbreviations, and the whole
//! names that Rust, its standard library or a dependency fixes.
//!
//! The first list was approved at G1.2. Each abbreviation that a review finds
//! joins it (R32).

/// Words that are abbreviations, compared in lower case with each word of a
/// defined name.
pub static ABBREVIATIONS: &[&str] = &[
  "abbr", "acc", "addr", "alloc", "app", "arg", "args", "arr", "att", "attr", "attrs", "aud", "auth", "avg", "bps",
  "buf", "bufs", "calc", "cb", "cfg", "chap", "chk", "cmd", "cmp", "cnt", "col", "cols", "cond", "conf", "config",
  "conn", "ctl", "ctrl", "ctx", "cur", "curr", "db", "dbg", "dec", "decl", "def", "del", "desc", "dest", "diag",
  "dict", "dim", "dir", "dirs", "doc", "docs", "dst", "dur", "elem", "elems", "env", "err", "errs", "esc", "eval",
  "evt", "exe", "exec", "expr", "ext", "fd", "fmt", "fps", "freq", "func", "gen", "hdr", "hdrs", "hex", "hz", "id",
  "idx", "img", "info", "init", "int", "io", "iter", "kbps", "khz", "lang", "len", "lib", "lvl", "max", "mbs", "mem",
  "meta", "mgr", "min", "misc", "ms", "msg", "msgs", "nal", "ns", "num", "nums", "obj", "opt", "opts", "orig", "param",
  "params", "pic", "pid", "pix", "pkg", "pkt", "pkts", "pos", "pps", "prev", "proc", "prop", "props", "pts", "ptr",
  "px", "qty", "rand", "recv", "repo", "req", "res", "resp", "ret", "rng", "sec", "secs", "seg", "seq", "sig", "spec",
  "specs", "sps", "src", "stat", "stats", "std", "str", "sync", "sys", "sz", "temp", "tmp", "trk", "ts", "txt", "typ",
  "usr", "util", "utils", "val", "vals", "var", "vars", "vec", "ver", "vid", "vps",
];

/// Whole names that contain a listed word but are fixed by Rust, its
/// standard library or a dependency, such as `fmt` in a `Display`
/// implementation.
pub static FIXED_NAMES: &[&str] = &[
  "Err",
  "IntoIter",
  "as_str",
  "cmp",
  "fill_buf",
  "fmt",
  "from_str",
  "into_iter",
  "iter",
  "iter_mut",
  "len",
  "max",
  "min",
  "partial_cmp",
  "poll_fill_buf",
  "to_str",
  "visit_seq",
  "visit_str",
];
