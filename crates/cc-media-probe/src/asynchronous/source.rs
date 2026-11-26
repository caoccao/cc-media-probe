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

//! `AsyncPositionalSource`, the trait of a caller's own async source (`api.md` section 9.3), with the three
//! implementations of the library, which take the bounds that futures-io gives its own `AsyncRead`.

use std::io;
use std::ops::DerefMut;
use std::pin::Pin;
use std::task::{Context, Poll};

/// An async source that is neither a file nor a reader, such as ranges of a remote object. The trait is
/// poll-based, so it is object-safe, and the probe future is `Send` exactly when the source is. It needs neither
/// runtime feature, and the rules of [`PositionalSource`](crate::source::PositionalSource) apply unchanged, the
/// rule for an empty buffer included.
///
/// **Repeated requests.** Within one probe, after a call returns `Poll::Pending`, the probe's next call to the same
/// method repeats the request: the same `position` and a buffer of the same length. The probe makes no other
/// request of the source in between, and gives up a pending request only by being dropped. The promise ends with
/// the probe: a source that outlives a dropped probe, as one passed as `&mut source` does, can still hold that
/// probe's request when a second probe calls. Such a source therefore keeps the range of its request in flight,
/// and starts a new request when a call asks for another range.
///
/// **Waking.** Before a call returns `Poll::Pending`, the source arranges for the waker of that call's `context` to
/// be woken once the request can make progress, as `Future::poll` asks of a future. After several such calls, the
/// waker to wake is the latest call's. The probe relies on that wake: a source that returns `Poll::Pending` without
/// arranging one can leave the probe waiting forever. A wake is not the only reason for a call, though. An executor
/// may poll the probe's future before the source wakes it, as on a spurious wakeup that Tokio allows itself, or
/// when another future of the same task is woken. The source is then called again with the same request while that
/// request is still outstanding, and it arranges the wake again, for this call's waker, and returns `Poll::Pending`
/// once more. A source that keeps an inner future, such as that of a range request, and polls it with the
/// `context` that it is given meets this rule.
///
/// The library implements the trait for `&mut Source` and `Box<Source>` where `Source` is `Unpin`, and for
/// `Pin<Pointer>`, which serves a source that is not `Unpin`: pass it as `Box::pin(source)`, or pin it in place
/// with `std::pin::pin!` and pass `pinned.as_mut()`. A caller with an `async` read function can keep its future in
/// the source and poll it from `poll_read_at`.
///
/// ```
/// # fn main() -> Result<(), cc_media_probe::Error> {
/// use std::io;
/// use std::pin::Pin;
/// use std::task::{Context, Poll};
///
/// use cc_media_probe::asynchronous::{self, AsyncPositionalSource};
///
/// /// Ranges of an object in memory, ready at once, as a remote object served from a cache would be.
/// struct Object {
///     data: Vec<u8>,
/// }
///
/// impl AsyncPositionalSource for Object {
///     fn poll_read_at(
///         self: Pin<&mut Self>,
///         _context: &mut Context<'_>,
///         position: u64,
///         buffer: &mut [u8],
///     ) -> Poll<io::Result<usize>> {
///         let start = usize::try_from(position).map_or(self.data.len(), |start| start.min(self.data.len()));
///         let mut available = self.data.get(start..).unwrap_or_default();
///         Poll::Ready(io::Read::read(&mut available, buffer))
///     }
///
///     fn poll_length(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
///         Poll::Ready(Ok(u64::try_from(self.data.len()).ok()))
///     }
/// }
///
/// let mut object = Object { data: b"RIFF".to_vec() };
/// let detection = futures_lite::future::block_on(asynchronous::detect_source(&mut object))?;
/// println!("{}", detection.outcome().as_str());
/// let pinned = Box::pin(object);
/// let detection = futures_lite::future::block_on(asynchronous::detect_source(pinned))?;
/// println!("{}", detection.outcome().as_str());
/// # Ok(())
/// # }
/// ```
pub trait AsyncPositionalSource {
  /// Reads at most `buffer.len()` bytes starting at `position`. Ready with 0 for an empty buffer, and for a nonempty
  /// one only at or past the end. The library never asks for zero bytes. `Poll::Pending` comes only after the
  /// waker of `context` is arranged to be woken, as the trait's rule of waking says.
  ///
  /// # Errors
  ///
  /// The error with which the source failed. `ErrorKind::Interrupted` is retried; any other error ends the probe.
  fn poll_read_at(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>>;

  /// The total length, or `None` when the source does not know it. `Poll::Pending` comes only after the waker
  /// of `context` is arranged to be woken, as the trait's rule of waking says.
  ///
  /// # Errors
  ///
  /// The error with which the source failed. `ErrorKind::Interrupted` is retried, as for a read; any other error
  /// ends the probe.
  fn poll_length(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>>;
}

/// Writes the implementation for a pointer to an `Unpin` source, `&mut Source` or `Box<Source>`, which polls the
/// source through `Pin::new`, as futures-io does for its own `AsyncRead`. One body serves both pointers (R36).
macro_rules! unpin_pointer_source {
  ($(#[$attribute:meta])* $pointer:ty) => {
    $(#[$attribute])*
    impl<Source: AsyncPositionalSource + Unpin + ?Sized> AsyncPositionalSource for $pointer {
      fn poll_read_at(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
        position: u64,
        buffer: &mut [u8],
      ) -> Poll<io::Result<usize>> {
        Pin::new(&mut **self.get_mut()).poll_read_at(context, position, buffer)
      }

      fn poll_length(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
        Pin::new(&mut **self.get_mut()).poll_length(context)
      }
    }
  };
}

unpin_pointer_source!(
  /// A borrowed source, polled through `Pin::new`, so it is `Unpin`.
  &mut Source
);
unpin_pointer_source!(
  /// A boxed source, polled through `Pin::new` as a borrowed one is.
  Box<Source>
);

/// A pinned source of any kind, such as `Pin<Box<Source>>` or `Pin<&mut Source>` for a source that is not `Unpin`.
impl<Pointer> AsyncPositionalSource for Pin<Pointer>
where
  Pointer: DerefMut + Unpin,
  Pointer::Target: AsyncPositionalSource,
{
  fn poll_read_at(
    self: Pin<&mut Self>,
    context: &mut Context<'_>,
    position: u64,
    buffer: &mut [u8],
  ) -> Poll<io::Result<usize>> {
    self.get_mut().as_mut().poll_read_at(context, position, buffer)
  }

  fn poll_length(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
    self.get_mut().as_mut().poll_length(context)
  }
}

#[cfg(test)]
pub(crate) mod tests {
  use std::io::{self, Read};
  use std::marker::PhantomPinned;
  use std::pin::{Pin, pin};
  use std::task::{Context, Poll, Waker};

  use super::AsyncPositionalSource;

  /// Ranges of bytes in memory, ready at once. It is not `Unpin`, so only a pinned pointer passes it.
  pub(crate) struct ReadyRanges {
    data: Vec<u8>,
    _pinned: PhantomPinned,
  }

  impl ReadyRanges {
    /// The ranges of `data`.
    pub(crate) fn new(data: &[u8]) -> ReadyRanges {
      ReadyRanges {
        data: data.to_vec(),
        _pinned: PhantomPinned,
      }
    }
  }

  impl AsyncPositionalSource for ReadyRanges {
    fn poll_read_at(
      self: Pin<&mut Self>,
      _context: &mut Context<'_>,
      position: u64,
      buffer: &mut [u8],
    ) -> Poll<io::Result<usize>> {
      let start = usize::try_from(position).map_or(self.data.len(), |start| start.min(self.data.len()));
      let mut available = self.data.get(start..).unwrap_or_default();
      Poll::Ready(available.read(buffer))
    }

    fn poll_length(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
      Poll::Ready(Ok(u64::try_from(self.data.len()).ok()))
    }
  }

  /// A source that is `Unpin`, so a borrow and a box pass it.
  struct Movable(Pin<Box<ReadyRanges>>);

  impl AsyncPositionalSource for Movable {
    fn poll_read_at(
      mut self: Pin<&mut Self>,
      context: &mut Context<'_>,
      position: u64,
      buffer: &mut [u8],
    ) -> Poll<io::Result<usize>> {
      self.0.as_mut().poll_read_at(context, position, buffer)
    }

    fn poll_length(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<io::Result<Option<u64>>> {
      self.0.as_mut().poll_length(context)
    }
  }

  /// Polls a read of three bytes at `position` and the length once each, as the library would.
  fn poll_once<Source: AsyncPositionalSource + Unpin>(mut source: Source, position: u64) -> (usize, Option<u64>) {
    let mut context = Context::from_waker(Waker::noop());
    let mut buffer = [0_u8; 3];
    let read = Pin::new(&mut source).poll_read_at(&mut context, position, &mut buffer);
    let length = Pin::new(&mut source).poll_length(&mut context);
    match (read, length) {
      (Poll::Ready(Ok(count)), Poll::Ready(Ok(length))) => (count, length),
      _ => (usize::MAX, None),
    }
  }

  #[test]
  fn polls_through_each_pointer() {
    let mut movable = Movable(Box::pin(ReadyRanges::new(b"abcd")));
    assert_eq!(poll_once(&mut movable, 2), (2, Some(4)));
    assert_eq!(poll_once(Box::new(movable), 0), (3, Some(4)));
    let boxed: Box<dyn AsyncPositionalSource + Send + Unpin> = Box::new(Movable(Box::pin(ReadyRanges::new(b"ab"))));
    assert_eq!(poll_once(boxed, 0), (2, Some(2)));
    assert_eq!(poll_once(Box::pin(ReadyRanges::new(b"abcd")), 4), (0, Some(4)));
    let mut pinned = pin!(ReadyRanges::new(b"abcdef"));
    assert_eq!(poll_once(pinned.as_mut(), 1), (3, Some(6)));
  }
}
