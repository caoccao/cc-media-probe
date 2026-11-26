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

//! The serialized names and the messages of the errors (`json-output.md` section 6.1; `cli.md` section 8.6).

/// The serialized name of `ErrorKind::SourceFailed`.
pub const SOURCE_FAILED_NAME: &str = "source_failed";
/// The serialized name of `ErrorKind::UnknownFormat`.
pub const UNKNOWN_FORMAT_NAME: &str = "unknown_format";
/// The serialized name of `ErrorKind::FormatNotDetermined`.
pub const FORMAT_NOT_DETERMINED_NAME: &str = "format_not_determined";
/// The serialized name of `ErrorKind::UnsupportedFormat`.
pub const UNSUPPORTED_FORMAT_NAME: &str = "unsupported_format";
/// The serialized name of `ErrorKind::FeatureDisabled`.
pub const FEATURE_DISABLED_NAME: &str = "feature_disabled";
/// The serialized name of `ErrorKind::NoStreamInventory`.
pub const NO_STREAM_INVENTORY_NAME: &str = "no_stream_inventory";

/// The message of `ErrorKind::SourceFailed`.
pub const SOURCE_FAILED_MESSAGE: &str = "the source failed";
/// The message of `ErrorKind::UnknownFormat`.
pub const UNKNOWN_FORMAT_MESSAGE: &str = "the format is unknown";
/// The message of `ErrorKind::FormatNotDetermined`.
pub const FORMAT_NOT_DETERMINED_MESSAGE: &str = "the format could not be determined";
/// The message of `ErrorKind::UnsupportedFormat`.
pub const UNSUPPORTED_FORMAT_MESSAGE: &str = "the format is not supported";
/// The message of `ErrorKind::FeatureDisabled`.
pub const FEATURE_DISABLED_MESSAGE: &str = "the stream information needs a disabled feature";
/// The message of `ErrorKind::NoStreamInventory`.
pub const NO_STREAM_INVENTORY_MESSAGE: &str = "the probe stopped before the stream inventory";
/// The words before the byte at which a source failed.
pub const AT_BYTE_TEXT: &str = "at byte";
/// The message of a source that failed while it was being opened, before any read (`cli.md` section 8.6).
pub const NOT_OPENED_MESSAGE: &str = "the source could not be opened";
/// The most causes of an error's chain that its cause messages follow, so that a cyclic chain ends (`api.md`
/// section 10.2).
pub const MAXIMUM_CAUSES: usize = 32;

/// The serialized name of `CheckState::Validated`.
pub const VALIDATED_NAME: &str = "validated";
/// The serialized name of `CheckState::Rejected`.
pub const REJECTED_NAME: &str = "rejected";
/// The serialized name of `CheckState::NotCheckable`.
pub const NOT_CHECKABLE_NAME: &str = "not_checkable";
/// The serialized name of `CheckState::Outranked`.
pub const OUTRANKED_NAME: &str = "outranked";

/// The member of an error record that holds its kind, which decides the members that it keeps (`json-output.md`
/// section 4.4).
#[cfg(feature = "serde")]
pub const KIND_MEMBER: &str = "kind";

/// The message of an error record with both stopping points, or with a record other than its partial report's.
#[cfg(feature = "serde")]
pub const STOPPING_POINTS_MESSAGE: &str = "an error record holds both a partial report and observations, or a record \
                                           other than its partial report's";

/// The message of a source failure without its kind, or of another known kind with the members of one.
#[cfg(feature = "serde")]
pub const SOURCE_FAILURE_MESSAGE: &str = "only a source failure, which always names its kind, carries the members of \
                                          one or a partial report";

/// The message of an operating system's code that is not a 64-bit integer in canonical decimal.
#[cfg(feature = "serde")]
pub const OPERATING_SYSTEM_CODE_MESSAGE: &str = "an operating system's error code is not a 64-bit integer in canonical \
                                                 decimal";

/// The message of an error's limit reason whose limit the record does not pair with the part that it stopped first
/// (fifth review of the third slice's first part).
#[cfg(feature = "serde")]
pub const ERROR_REASON_MESSAGE: &str = "an error's limit reason names a limit that its record does not say stopped a \
                                        part";

/// The message of an error record whose known kind does not allow its format, its hint, its candidates or its reason,
/// lacks the source that failed, or lists more causes than the chain is followed for (`json-output.md` section 4.4).
#[cfg(feature = "serde")]
pub const ERROR_KIND_MESSAGE: &str = "an error record's kind does not allow its format, its hint or its candidates, or \
                                      it lists more than 32 causes";

/// The message of observations whose diagnostic names a stream, or of a first reason that names one beside no partial
/// report, which no probe knows before its inventory (`json-output.md` section 10).
#[cfg(feature = "serde")]
pub const OBSERVED_STREAM_MESSAGE: &str = "observations name a stream, which no probe knows before its inventory";

/// The message of an error record whose partial report is of another format, or whose candidate or observed entry
/// names a limit that the record does not say stopped a part (`json-output.md` section 10).
#[cfg(feature = "serde")]
pub const ERROR_REFERENCES_MESSAGE: &str = "an error record's partial report is of another format, or a reason names a \
                                            limit that its record does not say stopped a part";

/// The message of a candidate's check whose reason its state does not allow (`json-output.md` section 4.4).
#[cfg(feature = "serde")]
pub const CANDIDATE_CHECK_MESSAGE: &str = "a candidate's check has a reason that its state does not allow";

/// The message of candidates that name a format twice, rise in confidence, or stand in an order that no checks in
/// their rank give (`api.md` section 5.1).
#[cfg(feature = "serde")]
pub const CANDIDATES_MESSAGE: &str = "an error's candidates name a format twice or are not in the rank and order of \
                                      their checks";

/// The message of a record whose first reason of detection is not the first cause of the candidates, or not the
/// reason of an undetermined format (`api.md` section 5.3).
#[cfg(feature = "serde")]
pub const DETECTION_REASON_MESSAGE: &str = "an error record's first reason of detection is not the first cause that \
                                            its candidates or its reason give";

/// The message of an error whose usage counts fewer bytes than its partial report or its observed sources read
/// (plan section 7.5).
#[cfg(feature = "serde")]
pub const ERROR_USAGE_MESSAGE: &str = "an error's usage counts fewer bytes than its partial report or its sources read";

/// The message of a source failure that names a source whose entry did not fail (`api.md` section 10.1).
#[cfg(feature = "serde")]
pub const FAILED_SOURCE_MESSAGE: &str = "a source failure names a source whose entry did not fail";

/// The message of an error whose format is not that of the candidate that validated (`api.md` section 5.1).
#[cfg(feature = "serde")]
pub const VALIDATED_FORMAT_MESSAGE: &str = "an error's format is not the format of the candidate that validated";

/// The message of an error record whose limit that stopped detection stopped another part first, or that names a
/// part beyond detection, which its probe never reached (`api.md` section 5.3).
#[cfg(feature = "serde")]
pub const DETECTION_STOPS_MESSAGE: &str = "an error record says that a limit of detection stopped another part \
                                           first, or names a part that its probe never reached";

/// The message of an error whose observations hold a companion that was not opened beside no first reason of the
/// companions (`api.md` sections 5.3 and 6.15).
#[cfg(feature = "serde")]
pub const OBSERVED_REFUSAL_MESSAGE: &str = "an error's observations hold a companion that was not opened, and its \
                                            record has no first reason of the companions";

/// The message of an error record whose probe never ran its prober and that exhausts or counts a counter of a later
/// activity, or observes a companion (`json-output.md` section 4.4; sixth and seventh reviews of the third slice's
/// first part).
#[cfg(feature = "serde")]
pub const ACTIVITY_COUNTER_MESSAGE: &str = "an error record exhausts or counts a counter, or observes a companion, of \
                                            an activity that its probe never ran";

/// The kinds of an error whose probe ended at detection, before discovery began: an unknown, undetermined or
/// unsupported format (`api.md` section 5.1).
#[cfg(feature = "serde")]
pub const DETECTION_ENDINGS: [super::ErrorKind; 3] = [
  super::ErrorKind::UnknownFormat,
  super::ErrorKind::FormatNotDetermined,
  super::ErrorKind::UnsupportedFormat,
];
