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

//! The stable name of an I/O error kind, which an error record keeps as `source_failure_kind()` (`api.md` section
//! 10.2). A table, so that a renamed variant keeps its name.

use std::io::ErrorKind;

use super::input_output_kinds::{
  ADDRESS_IN_USE_NAME, ADDRESS_NOT_AVAILABLE_NAME, ALREADY_EXISTS_NAME, ARGUMENT_LIST_TOO_LONG_NAME, BROKEN_PIPE_NAME,
  CONNECTION_ABORTED_NAME, CONNECTION_REFUSED_NAME, CONNECTION_RESET_NAME, CROSSES_DEVICES_NAME, DEADLOCK_NAME,
  DIRECTORY_NOT_EMPTY_NAME, EXECUTABLE_FILE_BUSY_NAME, FILE_TOO_LARGE_NAME, HOST_UNREACHABLE_NAME, INTERRUPTED_NAME,
  INVALID_DATA_NAME, INVALID_FILENAME_NAME, INVALID_INPUT_NAME, IS_A_DIRECTORY_NAME, NETWORK_DOWN_NAME,
  NETWORK_UNREACHABLE_NAME, NOT_A_DIRECTORY_NAME, NOT_CONNECTED_NAME, NOT_FOUND_NAME, NOT_SEEKABLE_NAME, OTHER_NAME,
  OUT_OF_MEMORY_NAME, PERMISSION_DENIED_NAME, QUOTA_EXCEEDED_NAME, READ_ONLY_FILESYSTEM_NAME, RESOURCE_BUSY_NAME,
  STALE_NETWORK_FILE_HANDLE_NAME, STORAGE_FULL_NAME, TIMED_OUT_NAME, TOO_MANY_LINKS_NAME, UNEXPECTED_END_OF_FILE_NAME,
  UNSUPPORTED_NAME, WOULD_BLOCK_NAME, WRITE_ZERO_NAME,
};

/// The stable name of `kind`: its words in lowercase joined by underscores, and `other` for a kind that this
/// table does not list.
pub(super) fn stable_name(kind: ErrorKind) -> &'static str {
  match kind {
    ErrorKind::NotFound => NOT_FOUND_NAME,
    ErrorKind::PermissionDenied => PERMISSION_DENIED_NAME,
    ErrorKind::ConnectionRefused => CONNECTION_REFUSED_NAME,
    ErrorKind::ConnectionReset => CONNECTION_RESET_NAME,
    ErrorKind::HostUnreachable => HOST_UNREACHABLE_NAME,
    ErrorKind::NetworkUnreachable => NETWORK_UNREACHABLE_NAME,
    ErrorKind::ConnectionAborted => CONNECTION_ABORTED_NAME,
    ErrorKind::NotConnected => NOT_CONNECTED_NAME,
    ErrorKind::AddrInUse => ADDRESS_IN_USE_NAME,
    ErrorKind::AddrNotAvailable => ADDRESS_NOT_AVAILABLE_NAME,
    ErrorKind::NetworkDown => NETWORK_DOWN_NAME,
    ErrorKind::BrokenPipe => BROKEN_PIPE_NAME,
    ErrorKind::AlreadyExists => ALREADY_EXISTS_NAME,
    ErrorKind::WouldBlock => WOULD_BLOCK_NAME,
    ErrorKind::NotADirectory => NOT_A_DIRECTORY_NAME,
    ErrorKind::IsADirectory => IS_A_DIRECTORY_NAME,
    ErrorKind::DirectoryNotEmpty => DIRECTORY_NOT_EMPTY_NAME,
    ErrorKind::ReadOnlyFilesystem => READ_ONLY_FILESYSTEM_NAME,
    ErrorKind::StaleNetworkFileHandle => STALE_NETWORK_FILE_HANDLE_NAME,
    ErrorKind::InvalidInput => INVALID_INPUT_NAME,
    ErrorKind::InvalidData => INVALID_DATA_NAME,
    ErrorKind::TimedOut => TIMED_OUT_NAME,
    ErrorKind::WriteZero => WRITE_ZERO_NAME,
    ErrorKind::StorageFull => STORAGE_FULL_NAME,
    ErrorKind::NotSeekable => NOT_SEEKABLE_NAME,
    ErrorKind::QuotaExceeded => QUOTA_EXCEEDED_NAME,
    ErrorKind::FileTooLarge => FILE_TOO_LARGE_NAME,
    ErrorKind::ResourceBusy => RESOURCE_BUSY_NAME,
    ErrorKind::ExecutableFileBusy => EXECUTABLE_FILE_BUSY_NAME,
    ErrorKind::Deadlock => DEADLOCK_NAME,
    ErrorKind::CrossesDevices => CROSSES_DEVICES_NAME,
    ErrorKind::TooManyLinks => TOO_MANY_LINKS_NAME,
    ErrorKind::InvalidFilename => INVALID_FILENAME_NAME,
    ErrorKind::ArgumentListTooLong => ARGUMENT_LIST_TOO_LONG_NAME,
    ErrorKind::Interrupted => INTERRUPTED_NAME,
    ErrorKind::Unsupported => UNSUPPORTED_NAME,
    ErrorKind::UnexpectedEof => UNEXPECTED_END_OF_FILE_NAME,
    ErrorKind::OutOfMemory => OUT_OF_MEMORY_NAME,
    _ => OTHER_NAME,
  }
}

#[cfg(test)]
mod tests {
  use std::io::ErrorKind;

  use super::stable_name;

  #[test]
  fn names_each_kind_in_snake_case() {
    let kinds = [
      ErrorKind::NotFound,
      ErrorKind::PermissionDenied,
      ErrorKind::ConnectionRefused,
      ErrorKind::ConnectionReset,
      ErrorKind::HostUnreachable,
      ErrorKind::NetworkUnreachable,
      ErrorKind::ConnectionAborted,
      ErrorKind::NotConnected,
      ErrorKind::AddrInUse,
      ErrorKind::AddrNotAvailable,
      ErrorKind::NetworkDown,
      ErrorKind::BrokenPipe,
      ErrorKind::AlreadyExists,
      ErrorKind::WouldBlock,
      ErrorKind::NotADirectory,
      ErrorKind::IsADirectory,
      ErrorKind::DirectoryNotEmpty,
      ErrorKind::ReadOnlyFilesystem,
      ErrorKind::StaleNetworkFileHandle,
      ErrorKind::InvalidInput,
      ErrorKind::InvalidData,
      ErrorKind::TimedOut,
      ErrorKind::WriteZero,
      ErrorKind::StorageFull,
      ErrorKind::NotSeekable,
      ErrorKind::QuotaExceeded,
      ErrorKind::FileTooLarge,
      ErrorKind::ResourceBusy,
      ErrorKind::ExecutableFileBusy,
      ErrorKind::Deadlock,
      ErrorKind::CrossesDevices,
      ErrorKind::TooManyLinks,
      ErrorKind::InvalidFilename,
      ErrorKind::ArgumentListTooLong,
      ErrorKind::Interrupted,
      ErrorKind::Unsupported,
      ErrorKind::UnexpectedEof,
      ErrorKind::OutOfMemory,
      ErrorKind::Other,
    ];
    for kind in kinds {
      let mut expected = String::new();
      for character in format!("{kind:?}").chars() {
        if character.is_uppercase() && !expected.is_empty() {
          expected.push('_');
        }
        expected.extend(character.to_lowercase());
      }
      assert_eq!(stable_name(kind), expected);
    }
  }
}
