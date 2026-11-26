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

//! The names and messages of the strict reader of the serialized form (R33; `json-output.md` sections 2 and 10).

/// The member of a root record that holds its version (`json-output.md` section 4.1).
pub const SCHEMA_VERSION_MEMBER: &str = "schema_version";

/// The member of an enum that carries data and of a typed detail that names its kind (`json-output.md` section 6.2).
pub const KIND_MEMBER: &str = "kind";

/// The suffix of the twin that carries a quantity beyond 2^53 - 1 as text (`json-output.md` section 5.5).
pub const EXACT_SUFFIX: &str = "_exact";

/// The name of the one member of the map in which `serde_json`'s `arbitrary_precision` feature hands over a number
/// that is not an integer within 64 bits (`json-output.md` section 2).
pub const ARBITRARY_PRECISION_NAME: &str = "$serde_json::private::Number";

/// The name of the newtype request by which a record asks the deserializer for the scope of its read; only the
/// strict deserializer answers it (`api.md` section 12, after its fourteenth and nineteenth reviews).
pub const READ_SCOPE_NAME: &str = "$cc_media_probe::form::scope";

/// The identifier of the strict deserializer's answer to that request, recognized by its address, which no input
/// can give.
pub static READ_SCOPE_TOKEN: [u8; READ_SCOPE_TOKEN_LENGTH] = *b"scope";

/// The length of that identifier.
pub const READ_SCOPE_TOKEN_LENGTH: usize = 5;

/// The number of elements of the strict deserializer's answer: the version of the root record whose body it reads,
/// and the deserializer itself.
pub const READ_SCOPE_LENGTH: usize = 2;

/// The message of a value that JSON does not carry: a non-finite number, a byte string, a tagged value, an integer
/// beyond 64 bits or a member name that is not a string (`json-output.md` section 2).
pub const NOT_JSON_VALUE_MESSAGE: &str = "a value that JSON does not carry";

/// The message of a member that holds null, which the form never writes (`json-output.md` section 2).
pub const NULL_MEMBER_MESSAGE: &str = "a member holds null";

/// The message of a map that names a member twice (RFC 7493 section 2.3).
pub const DUPLICATE_MEMBER_MESSAGE: &str = "a member is named twice";

/// The message of a record written as an array of its values (`json-output.md` section 2).
pub const POSITIONAL_RECORD_MESSAGE: &str = "a record is written as an array of its values";

/// The message of the private number's name in an object of a known type (`json-output.md` section 2).
pub const RESERVED_NAME_MESSAGE: &str = "an object of a known type has a member named like serde_json's private number";

/// The message of an answer to the request for the scope that is not the strict deserializer's.
pub const READ_SCOPE_MESSAGE: &str = "the answer to the request for the scope of a read is malformed";

/// What a reader of the scope's answer expects.
pub const READ_SCOPE_EXPECTING: &str = "a value of the serialized form";

/// What a reader of the answer's identifier expects.
pub const READ_SCOPE_IDENTIFIER_EXPECTING: &str = "the identifier of an enum";

/// What a reader of a value held before the version expects.
pub const HELD_VALUE_EXPECTING: &str = "any value";

/// What a reader of a value kept as written expects.
pub const KEPT_VALUE_EXPECTING: &str = "a value of JSON other than null";

/// What a reader of a root record expects.
pub const ROOT_RECORD_EXPECTING: &str = "a root record of the serialized form";

/// What a reader of a record whose outcome decides its kept members expects.
pub const OUTCOME_RECORD_EXPECTING: &str = "a record of the serialized form";

/// The name under which a record whose outcome decides its kept members asks for its map.
pub const OUTCOME_RECORD_NAME: &str = "record";

/// The message of a root record without a version: the version error (`json-output.md` section 4.1).
pub const MISSING_VERSION_MESSAGE: &str = "the version error: the root record has no schema_version";

/// The message of a version that is not a string: the version error.
pub const VERSION_TEXT_MESSAGE: &str = "the version error: schema_version is not a string";

/// The message of a version that is not two canonical decimal numbers of 32 bits: the version error.
pub const VERSION_FORM_MESSAGE: &str = "the version error: schema_version is not two canonical decimal numbers";

/// The message of a root record of another major version than the reader's: the version error.
pub const OTHER_MAJOR_VERSION_MESSAGE: &str = "the version error: the root record has version";

/// The label of the reader's own version in the message of another major version.
pub const READER_VERSION_LABEL: &str = "this reader reads";

/// The message of a nested root record whose version is not that of the record that holds it: the version error.
pub const NESTED_VERSION_MESSAGE: &str = "the version error: a nested root record has version";

/// The label of the holder's version in the message of a nested root record of another version.
pub const HOLDER_VERSION_LABEL: &str = "the record that holds it has";

/// The message of a twin that is not the only member of its quantity, or not in its own range in canonical decimal
/// (`json-output.md` section 5.5).
pub const TWIN_MESSAGE: &str = "a quantity is given by both members of its twin, by neither, or outside its range";

/// The message of a quantity beyond 2^53 - 1 (`json-output.md` section 5.5).
pub const QUANTITY_MESSAGE: &str = "a quantity lies beyond 2^53 - 1";

/// The message of a decimal string that is not canonical decimal within its width (`json-output.md` section 5.4).
pub const DECIMAL_MESSAGE: &str = "a decimal string is not canonical decimal within its width";

/// The message of hexadecimal text that is not whole bytes of lowercase digits (`json-output.md` section 5.6).
pub const HEXADECIMAL_MESSAGE: &str = "hexadecimal text is not whole bytes of lowercase digits";

/// The message of a value without its provenance or a provenance without its value (`json-output.md` section 7.1).
pub const PROVENANCE_MESSAGE: &str = "a value and its provenance are not present together";

/// The message of a map of an enum that carries data without its kind, without the member that its kind needs, or
/// with a member of another known kind (`json-output.md` section 6.2).
pub const KIND_MESSAGE: &str = "an enum that carries data lacks its kind or its kind's member, or has another kind's";

/// The decimal point of a JSON number (RFC 8259 section 6).
pub const DECIMAL_POINT: char = '.';

/// The two markers of a JSON number's exponent.
pub const EXPONENT_MARKERS: [char; 2] = ['e', 'E'];

/// The minus sign of a JSON number.
pub const MINUS_SIGN: char = '-';

/// The plus sign of a JSON number's exponent.
pub const PLUS_SIGN: char = '+';

/// The digit that no canonical integer begins with, unless it is that digit alone.
pub const ZERO_DIGIT: char = '0';

/// Negative zero, which a build without `serde_json`'s `arbitrary_precision` reads as a floating-point number.
pub const NEGATIVE_ZERO: &str = "-0";

/// What an error calls a number that is not finite.
pub const NUMBER_UNEXPECTED: &str = "a number";
