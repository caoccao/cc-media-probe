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

//! The four reader methods that the handles of both runtimes share (`api.md` section 9.1), written once (R36). Each
//! handle's module imports its runtime's `AsyncRead` and `AsyncSeek`, which the methods' bounds name.

/// Writes `probe_reader`, `probe_stream`, `detect_reader` and `detect_stream` for the handle `$handle`, a struct
/// that borrows its prober as `prober`, with the bounds of the `AsyncRead` and `AsyncSeek` in scope where the macro
/// is called. This build reads nothing of the input, so each outcome is ready at once.
macro_rules! reader_methods {
  ($handle:ident) => {
    impl $handle<'_> {
      /// Probes a seekable reader, the whole input from its byte 0, as [`Prober::probe_reader`] probes a blocking
      /// one. Pass `&mut reader` to keep it, or `Box::pin(reader)` for a reader that is not `Unpin`; its position
      /// afterwards is unspecified. Dropping the future stops the probe, as
      /// [the rule of cancellation](crate::asynchronous#cancellation) says.
      ///
      /// # Errors
      ///
      /// As [`Prober::probe_reader`].
      pub async fn probe_reader<Reader: AsyncRead + AsyncSeek + Unpin>(
        &self,
        reader: Reader,
      ) -> Result<MediaReport, Error> {
        std::future::ready(self.prober.probe_input(reader)).await
      }

      /// Probes a forward-only reader from where it stands, as [`Prober::probe_stream`] probes a blocking one. Pass
      /// `&mut reader` to keep it, or `Box::pin(reader)` for a reader that is not `Unpin`; it is left after exactly
      /// the bytes read from it. Dropping the future stops the probe, as
      /// [the rule of cancellation](crate::asynchronous#cancellation) says.
      ///
      /// # Errors
      ///
      /// As [`Prober::probe_stream`].
      pub async fn probe_stream<Reader: AsyncRead + Unpin>(&self, reader: Reader) -> Result<MediaReport, Error> {
        std::future::ready(self.prober.probe_input(reader)).await
      }

      /// Detects the format of a seekable reader, from its byte 0. Pass `&mut reader` to keep it, or
      /// `Box::pin(reader)` for a reader that is not `Unpin`; its position afterwards is unspecified. Dropping the
      /// future stops the detection, as [the rule of cancellation](crate::asynchronous#cancellation) says.
      ///
      /// # Errors
      ///
      /// `SourceFailed` for a reader that fails, and nothing else.
      pub async fn detect_reader<Reader: AsyncRead + AsyncSeek + Unpin>(
        &self,
        reader: Reader,
      ) -> Result<Detection, Error> {
        std::future::ready(Ok(self.prober.detect_input(reader))).await
      }

      /// Detects the format of a forward-only reader, from where it stands, reading only the pinned prefix. Pass
      /// `&mut reader` to keep it, or `Box::pin(reader)` for a reader that is not `Unpin`; it is left after exactly
      /// the bytes read from it. Dropping the future stops the detection, as
      /// [the rule of cancellation](crate::asynchronous#cancellation) says.
      ///
      /// # Errors
      ///
      /// `SourceFailed` for a reader that fails, and nothing else.
      pub async fn detect_stream<Reader: AsyncRead + Unpin>(&self, reader: Reader) -> Result<Detection, Error> {
        std::future::ready(Ok(self.prober.detect_input(reader))).await
      }
    }
  };
}

pub(super) use reader_methods;
