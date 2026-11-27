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

//! The I/O error kinds that a case's `failed:<kind>` names, by the stable names that the library's records give them
//! (`json-output.md` section 4.4, `source_failure_kind`): each variant of the standard library's stable kinds, its
//! words in lowercase joined by `_`, as the library spells them, and `other`. The table is the library's own,
//! which is private, copied for the test; Phase 14's tests of a failed companion check each name through the
//! library's record.

use std::io::ErrorKind;

/// Each stable I/O error kind with its name.
const FAILURE_KINDS: [(&str, ErrorKind); 39] = [
  ("not_found", ErrorKind::NotFound),
  ("permission_denied", ErrorKind::PermissionDenied),
  ("connection_refused", ErrorKind::ConnectionRefused),
  ("connection_reset", ErrorKind::ConnectionReset),
  ("host_unreachable", ErrorKind::HostUnreachable),
  ("network_unreachable", ErrorKind::NetworkUnreachable),
  ("connection_aborted", ErrorKind::ConnectionAborted),
  ("not_connected", ErrorKind::NotConnected),
  ("addr_in_use", ErrorKind::AddrInUse),
  ("addr_not_available", ErrorKind::AddrNotAvailable),
  ("network_down", ErrorKind::NetworkDown),
  ("broken_pipe", ErrorKind::BrokenPipe),
  ("already_exists", ErrorKind::AlreadyExists),
  ("would_block", ErrorKind::WouldBlock),
  ("not_a_directory", ErrorKind::NotADirectory),
  ("is_a_directory", ErrorKind::IsADirectory),
  ("directory_not_empty", ErrorKind::DirectoryNotEmpty),
  ("read_only_filesystem", ErrorKind::ReadOnlyFilesystem),
  ("stale_network_file_handle", ErrorKind::StaleNetworkFileHandle),
  ("invalid_input", ErrorKind::InvalidInput),
  ("invalid_data", ErrorKind::InvalidData),
  ("timed_out", ErrorKind::TimedOut),
  ("write_zero", ErrorKind::WriteZero),
  ("storage_full", ErrorKind::StorageFull),
  ("not_seekable", ErrorKind::NotSeekable),
  ("quota_exceeded", ErrorKind::QuotaExceeded),
  ("file_too_large", ErrorKind::FileTooLarge),
  ("resource_busy", ErrorKind::ResourceBusy),
  ("executable_file_busy", ErrorKind::ExecutableFileBusy),
  ("deadlock", ErrorKind::Deadlock),
  ("crosses_devices", ErrorKind::CrossesDevices),
  ("too_many_links", ErrorKind::TooManyLinks),
  ("invalid_filename", ErrorKind::InvalidFilename),
  ("argument_list_too_long", ErrorKind::ArgumentListTooLong),
  ("interrupted", ErrorKind::Interrupted),
  ("unsupported", ErrorKind::Unsupported),
  ("unexpected_eof", ErrorKind::UnexpectedEof),
  ("out_of_memory", ErrorKind::OutOfMemory),
  ("other", ErrorKind::Other),
];

/// The kind that `name` names, or `None` for a name that no stable kind has.
pub fn failure_kind(name: &str) -> Option<ErrorKind> {
  FAILURE_KINDS
    .iter()
    .find(|(known, _)| *known == name)
    .map(|(_, kind)| *kind)
}
