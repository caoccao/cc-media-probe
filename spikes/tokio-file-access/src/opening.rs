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

//! The opening function and the bound it must meet.

use std::io;
use std::path::Path;

use tokio::io::{AsyncRead, AsyncSeek};

/// What a probe function needs from its reader: the bounds of `asynchronous::tokio::probe_reader`.
pub fn accepts_seekable_reader<Reader: AsyncRead + AsyncSeek + Unpin + Send>(reader: Reader) -> Reader {
  reader
}

/// Opens `path` and passes the file on, as the path-based convenience would.
pub async fn open_for_probing(path: impl AsRef<Path>) -> io::Result<tokio::fs::File> {
  let file = tokio::fs::File::open(path).await?;
  Ok(accepts_seekable_reader(file))
}

/// Proves at compile time that the future of `open_for_probing` is `Send`.
pub fn future_is_send(path: &Path) -> impl Future<Output = io::Result<tokio::fs::File>> + Send + '_ {
  open_for_probing(path)
}

#[cfg(test)]
mod tests {
  use tokio::io::{AsyncReadExt, AsyncSeekExt};

  #[tokio::test(flavor = "current_thread")]
  async fn opens_reads_and_seeks_inside_a_runtime() {
    let path = std::env::temp_dir().join("tokio-file-access-spike.bin");
    std::fs::write(&path, b"0123456789").unwrap();
    let mut file = super::open_for_probing(&path).await.unwrap();
    file.seek(std::io::SeekFrom::Start(4)).await.unwrap();
    let mut buffer = [0_u8; 3];
    file.read_exact(&mut buffer).await.unwrap();
    assert_eq!(&buffer, b"456");
    std::fs::remove_file(&path).unwrap();
  }

  #[test]
  fn fails_outside_a_runtime_instead_of_blocking() {
    let outcome = std::panic::catch_unwind(|| {
      let future = super::open_for_probing("missing");
      let waker = std::task::Waker::noop();
      let mut context = std::task::Context::from_waker(waker);
      let mut future = Box::pin(future);
      let _ = future.as_mut().poll(&mut context);
    });
    assert!(outcome.is_err());
  }
}
