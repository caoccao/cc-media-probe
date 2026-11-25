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

//! The stable names of the I/O error kinds (`json-output.md` section 4.4): each variant's words in lowercase,
//! joined by underscores.

/// `std::io::ErrorKind::NotFound`.
pub const NOT_FOUND_NAME: &str = "not_found";
/// `std::io::ErrorKind::PermissionDenied`.
pub const PERMISSION_DENIED_NAME: &str = "permission_denied";
/// `std::io::ErrorKind::ConnectionRefused`.
pub const CONNECTION_REFUSED_NAME: &str = "connection_refused";
/// `std::io::ErrorKind::ConnectionReset`.
pub const CONNECTION_RESET_NAME: &str = "connection_reset";
/// `std::io::ErrorKind::HostUnreachable`.
pub const HOST_UNREACHABLE_NAME: &str = "host_unreachable";
/// `std::io::ErrorKind::NetworkUnreachable`.
pub const NETWORK_UNREACHABLE_NAME: &str = "network_unreachable";
/// `std::io::ErrorKind::ConnectionAborted`.
pub const CONNECTION_ABORTED_NAME: &str = "connection_aborted";
/// `std::io::ErrorKind::NotConnected`.
pub const NOT_CONNECTED_NAME: &str = "not_connected";
/// `std::io::ErrorKind::AddrInUse`, named as the standard library spells it.
pub const ADDRESS_IN_USE_NAME: &str = "addr_in_use";
/// `std::io::ErrorKind::AddrNotAvailable`, named as the standard library spells it.
pub const ADDRESS_NOT_AVAILABLE_NAME: &str = "addr_not_available";
/// `std::io::ErrorKind::NetworkDown`.
pub const NETWORK_DOWN_NAME: &str = "network_down";
/// `std::io::ErrorKind::BrokenPipe`.
pub const BROKEN_PIPE_NAME: &str = "broken_pipe";
/// `std::io::ErrorKind::AlreadyExists`.
pub const ALREADY_EXISTS_NAME: &str = "already_exists";
/// `std::io::ErrorKind::WouldBlock`.
pub const WOULD_BLOCK_NAME: &str = "would_block";
/// `std::io::ErrorKind::NotADirectory`.
pub const NOT_A_DIRECTORY_NAME: &str = "not_a_directory";
/// `std::io::ErrorKind::IsADirectory`.
pub const IS_A_DIRECTORY_NAME: &str = "is_a_directory";
/// `std::io::ErrorKind::DirectoryNotEmpty`.
pub const DIRECTORY_NOT_EMPTY_NAME: &str = "directory_not_empty";
/// `std::io::ErrorKind::ReadOnlyFilesystem`.
pub const READ_ONLY_FILESYSTEM_NAME: &str = "read_only_filesystem";
/// `std::io::ErrorKind::StaleNetworkFileHandle`.
pub const STALE_NETWORK_FILE_HANDLE_NAME: &str = "stale_network_file_handle";
/// `std::io::ErrorKind::InvalidInput`.
pub const INVALID_INPUT_NAME: &str = "invalid_input";
/// `std::io::ErrorKind::InvalidData`.
pub const INVALID_DATA_NAME: &str = "invalid_data";
/// `std::io::ErrorKind::TimedOut`.
pub const TIMED_OUT_NAME: &str = "timed_out";
/// `std::io::ErrorKind::WriteZero`.
pub const WRITE_ZERO_NAME: &str = "write_zero";
/// `std::io::ErrorKind::StorageFull`.
pub const STORAGE_FULL_NAME: &str = "storage_full";
/// `std::io::ErrorKind::NotSeekable`.
pub const NOT_SEEKABLE_NAME: &str = "not_seekable";
/// `std::io::ErrorKind::QuotaExceeded`.
pub const QUOTA_EXCEEDED_NAME: &str = "quota_exceeded";
/// `std::io::ErrorKind::FileTooLarge`.
pub const FILE_TOO_LARGE_NAME: &str = "file_too_large";
/// `std::io::ErrorKind::ResourceBusy`.
pub const RESOURCE_BUSY_NAME: &str = "resource_busy";
/// `std::io::ErrorKind::ExecutableFileBusy`.
pub const EXECUTABLE_FILE_BUSY_NAME: &str = "executable_file_busy";
/// `std::io::ErrorKind::Deadlock`.
pub const DEADLOCK_NAME: &str = "deadlock";
/// `std::io::ErrorKind::CrossesDevices`.
pub const CROSSES_DEVICES_NAME: &str = "crosses_devices";
/// `std::io::ErrorKind::TooManyLinks`.
pub const TOO_MANY_LINKS_NAME: &str = "too_many_links";
/// `std::io::ErrorKind::InvalidFilename`.
pub const INVALID_FILENAME_NAME: &str = "invalid_filename";
/// `std::io::ErrorKind::ArgumentListTooLong`.
pub const ARGUMENT_LIST_TOO_LONG_NAME: &str = "argument_list_too_long";
/// `std::io::ErrorKind::Interrupted`.
pub const INTERRUPTED_NAME: &str = "interrupted";
/// `std::io::ErrorKind::Unsupported`.
pub const UNSUPPORTED_NAME: &str = "unsupported";
/// `std::io::ErrorKind::UnexpectedEof`, named as the standard library spells it.
pub const UNEXPECTED_END_OF_FILE_NAME: &str = "unexpected_eof";
/// `std::io::ErrorKind::OutOfMemory`.
pub const OUT_OF_MEMORY_NAME: &str = "out_of_memory";
/// `std::io::ErrorKind::Other`, and every kind that this table does not list.
pub const OTHER_NAME: &str = "other";
