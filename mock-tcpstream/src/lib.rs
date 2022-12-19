//! # Mock TcpStream
//!
//! A library for mocking `std::net::TcpStream` or `tokio::net::TcpStream` in tests.

use futures_io::{AsyncRead as FuturesIoAsyncRead, AsyncWrite as FuturesIoAsyncWrite};
use std::cmp::min;
use std::io::{Read, Result, Write};
use std::net::{SocketAddr, ToSocketAddrs};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

// TODO: Implement std_async traits as well?
// TODO: Implement Tokio Stream?

/// A mocked `std::net::TcpStream` or `tokio::net::TcpStream`.
///
/// This idea was pretty much copied and pasted from
/// [here](https://rust-lang.github.io/async-book/09_example/03_tests.html).
/// (This was published under an MIT license at the time of this crate's
/// creation.)
#[derive(Debug, Clone, Default)]
pub struct MockTcpStream {
    /// A buffer for all received--but yet unread--data.
    ///
    /// Even though this is a public field, you generally should not read from
    /// this directly. The point of this library is for you to use the
    /// [`std::io::Read`] or [`AsyncRead`](tokio::io::AsyncRead)
    /// implementations to read data received on this mocked stream.
    ///
    /// When you use the above implementations to read from this buffer, this
    /// buffer is cleared (which mimics the behavior of a `TcpStream`).
    ///
    /// Receiving data can be done using `MockTcpStream.receive()`, which simply
    /// extends this buffer with new data.
    pub received_data: Arc<Mutex<Vec<u8>>>,

    /// A buffer for all transmitted data. Every call to `Write` or `AsyncWrite`
    /// extends this buffer, but does nothing with the data beyond this. It is
    /// up to the library user to read this data however desired.
    ///
    /// The most common use case of this data will be to extend another
    /// [`MockTcpStream::received_data`] with the entirety of this buffer, then
    /// clear this buffer.
    pub transmitted_data: Arc<Mutex<Vec<u8>>>,

    /**
     * An optional socket address that will be used to populate the
     * `to_socket_addrs()` return value.
     */
    pub address: Option<SocketAddr>,
}

impl MockTcpStream {
    /// Returns a new `MockTcpStream`.
    pub fn new() -> Self {
        MockTcpStream::default()
    }

    /// Receive some data in this `MockTcpStream`.
    pub fn receive(&mut self, data: &[u8]) -> std::io::Result<usize> {
        let mut received_data = self.received_data.lock().unwrap();
        received_data.extend(data);
        Ok(data.len())
    }

    /// Emit data fom this `MockTcpStream`. If there is no data to be emitted,
    /// the returned `Vec` will have a length of zero.
    pub fn transmit(&mut self) -> Vec<u8> {
        let transmitted_data = self.transmitted_data.lock().unwrap();
        transmitted_data.to_vec()
    }
}

impl PartialEq for MockTcpStream {
    /// Compares two `MockTcpStream`s for equality, byte-for-byte, in both the
    /// [received_data](MockTcpStream::received_data) and
    /// [transmitted_data](MockTcpStream::transmitted_data) buffers.
    fn eq(&self, other: &Self) -> bool {
        let my_rx = self.received_data.lock().unwrap();
        let your_rx = other.received_data.lock().unwrap();
        if my_rx.eq(&your_rx.to_vec()) {
            return false;
        }

        let my_tx = self.transmitted_data.lock().unwrap();
        let your_tx = other.transmitted_data.lock().unwrap();
        my_tx.eq(&your_tx.to_vec())
    }
}

impl Read for MockTcpStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let mut new_vec: Vec<u8> = Vec::new();
        let mut read_bytes: usize = 0;
        {
            let received_data = self.received_data.lock().unwrap();
            let size: usize = min(received_data.len(), buf.len());
            /* For `copy_from_slice()`, the length of both slices must be equal. */
            buf[..size].copy_from_slice(&received_data[..size]);
            new_vec.extend(&received_data[size..]);
            read_bytes += size;
        }
        self.received_data = Arc::new(Mutex::new(new_vec));
        Ok(read_bytes)
    }
}

impl Write for MockTcpStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let mut transmitted_data = self.transmitted_data.lock().unwrap();
        transmitted_data.extend(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl AsyncRead for MockTcpStream {
    fn poll_read(self: Pin<&mut Self>, _: &mut Context, buf: &mut ReadBuf) -> Poll<Result<()>> {
        let mut new_vec: Vec<u8> = Vec::new();
        {
            let received_data = self.received_data.lock().unwrap();
            let size: usize = min(received_data.len(), buf.capacity());
            /* For `copy_from_slice()`, the length of both slices must be equal. */
            buf.put_slice(&received_data[..size]);
            new_vec.extend(&received_data[size..]);
        }
        self.get_mut().received_data = Arc::new(Mutex::new(new_vec));
        Poll::Ready(Ok(()))
    }
}

impl AsyncWrite for MockTcpStream {
    fn poll_write(self: Pin<&mut Self>, _: &mut Context, buf: &[u8]) -> Poll<Result<usize>> {
        {
            // Scope to marginally shorten the lifespan of the mutex lock.
            let mut transmitted_data = self.transmitted_data.lock().unwrap();
            transmitted_data.extend(buf);
        }
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn is_write_vectored(&self) -> bool {
        true
    }

    fn poll_write_vectored(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> std::task::Poll<Result<usize>> {
        let mut len: usize = 0;
        {
            // Scope to marginally shorten the lifespan of the mutex lock.
            let mut transmitted_data = self.transmitted_data.lock().unwrap();
            for buf in bufs {
                transmitted_data.extend(buf.iter());
                len += buf.len();
            }
        }
        Poll::Ready(Ok(len))
    }
}

impl FuturesIoAsyncRead for MockTcpStream {
    fn poll_read(self: Pin<&mut Self>, _: &mut Context, buf: &mut [u8]) -> Poll<Result<usize>> {
        let mut new_vec: Vec<u8> = Vec::new();
        let mut bytes_read: usize = 0;
        {
            let received_data = self.received_data.lock().unwrap();
            let size: usize = min(received_data.len(), buf.len());
            /* For `copy_from_slice()`, the length of both slices must be equal. */
            buf[..size].copy_from_slice(&received_data[..size]);
            new_vec.extend(&received_data[size..]);
            bytes_read += size;
        }
        self.get_mut().received_data = Arc::new(Mutex::new(new_vec));
        Poll::Ready(Ok(bytes_read))
    }
}

impl FuturesIoAsyncWrite for MockTcpStream {
    fn poll_write(self: Pin<&mut Self>, _: &mut Context, buf: &[u8]) -> Poll<Result<usize>> {
        {
            // Scope to marginally shorten the lifespan of the mutex lock.
            let mut transmitted_data = self.transmitted_data.lock().unwrap();
            transmitted_data.extend(buf);
        }
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_write_vectored(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> std::task::Poll<Result<usize>> {
        let mut len: usize = 0;
        {
            // Scope to marginally shorten the lifespan of the mutex lock.
            let mut transmitted_data = self.transmitted_data.lock().unwrap();
            for buf in bufs {
                transmitted_data.extend(buf.iter());
                len += buf.len();
            }
        }
        Poll::Ready(Ok(len))
    }
}

impl Unpin for MockTcpStream {}

impl ToSocketAddrs for MockTcpStream {
    type Iter = std::option::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        Ok(self.address.into_iter())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[std::prelude::rust_2021::test]
    fn test_mock_sync_write() {
        let mut stream = MockTcpStream::new();
        std::io::Write::write(&mut stream, &[1, 2, 3, 4, 5]).unwrap();
        {
            // Scoped so the mutex lock drops.
            let tx_data = stream.transmitted_data.lock().unwrap();
            assert_eq!(tx_data.len(), 5);
        }
        std::io::Write::write(&mut stream, &[6, 7, 8, 9]).unwrap();
        {
            // Scoped so the mutex lock drops.
            let tx_data = stream.transmitted_data.lock().unwrap();
            assert_eq!(tx_data.len(), 9);
        }
        std::io::Write::flush(&mut stream).unwrap();
        {
            // Scoped so the mutex lock drops.
            let tx_data = stream.transmitted_data.lock().unwrap();
            assert_eq!(tx_data.len(), 9);
        }
        // assert_eq!(std::io::Write::is_write_vectored(&stream), true);
        // std::io::Write::write_all_vectored(&mut stream, &[
        //     vec![ 1, 2, 3 ],
        //     vec![ 4, 5, 6 ],
        // ]).unwrap();
        // { // Scoped so the mutex lock drops.
        //     let tx_data = stream.transmitted_data.lock().unwrap();
        //     assert_eq!(tx_data.len(), 15);
        // }
        let transmitted = stream.transmit();
        assert_eq!(transmitted.len(), 9);
    }

    #[tokio::test]
    async fn test_mock_async_write() {
        let mut stream = MockTcpStream::new();
        tokio::io::AsyncWriteExt::write(&mut stream, &[1, 2, 3, 4, 5])
            .await
            .unwrap();
        {
            // Scoped so the mutex lock drops.
            let tx_data = stream.transmitted_data.lock().unwrap();
            assert_eq!(tx_data.len(), 5);
        }
        tokio::io::AsyncWriteExt::write(&mut stream, &[6, 7, 8, 9])
            .await
            .unwrap();
        {
            // Scoped so the mutex lock drops.
            let tx_data = stream.transmitted_data.lock().unwrap();
            assert_eq!(tx_data.len(), 9);
        }
        tokio::io::AsyncWriteExt::flush(&mut stream).await.unwrap();
        {
            // Scoped so the mutex lock drops.
            let tx_data = stream.transmitted_data.lock().unwrap();
            assert_eq!(tx_data.len(), 9);
        }
        // assert_eq!(std::io::Write::is_write_vectored(&stream), true);
        // std::io::Write::write_all_vectored(&mut stream, &[
        //     vec![ 1, 2, 3 ],
        //     vec![ 4, 5, 6 ],
        // ]).unwrap();
        // { // Scoped so the mutex lock drops.
        //     let tx_data = stream.transmitted_data.lock().unwrap();
        //     assert_eq!(tx_data.len(), 15);
        // }
        let transmitted = stream.transmit();
        assert_eq!(transmitted.len(), 9);
    }

    #[std::prelude::rust_2021::test]
    fn test_mock_sync_read() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 22, 33, 44];
        let mut stream = MockTcpStream::new();
        stream.receive(&input).unwrap();
        {
            // Scoped so the mutex lock drops.
            let rx_data = stream.received_data.lock().unwrap();
            assert_eq!(input.len(), rx_data.len());
        }
        {
            // Scoped so the mutex lock drops.
            let mut readbuf = [0; 7];
            let bytes_read = std::io::Read::read(&mut stream, &mut readbuf).unwrap();
            assert_eq!(bytes_read, 7);
            let rx_data = stream.received_data.lock().unwrap();
            assert_eq!(rx_data.len(), input.len() - 7);
        }
        {
            // Scoped so the mutex lock drops.
            let mut readbuf = [0; 20];
            let bytes_read = std::io::Read::read(&mut stream, &mut readbuf).unwrap();
            assert_eq!(bytes_read, input.len() - 7);
            let rx_data = stream.received_data.lock().unwrap();
            assert_eq!(rx_data.len(), 0);
        }
    }

    #[tokio::test]
    async fn test_mock_async_read() {
        let input = [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 22, 33, 44];
        let mut stream = MockTcpStream::new();
        stream.receive(&input).unwrap();
        {
            // Scoped so the mutex lock drops.
            let rx_data = stream.received_data.lock().unwrap();
            assert_eq!(input.len(), rx_data.len());
        }
        {
            // Scoped so the mutex lock drops.
            let mut readbuf = [0; 7];
            let bytes_read = tokio::io::AsyncReadExt::read(&mut stream, &mut readbuf)
                .await
                .unwrap();
            assert_eq!(bytes_read, 7);
            let rx_data = stream.received_data.lock().unwrap();
            assert_eq!(rx_data.len(), input.len() - 7);
        }
        {
            // Scoped so the mutex lock drops.
            let mut readbuf = [0; 20];
            let bytes_read = tokio::io::AsyncReadExt::read(&mut stream, &mut readbuf)
                .await
                .unwrap();
            assert_eq!(bytes_read, input.len() - 7);
            let rx_data = stream.received_data.lock().unwrap();
            assert_eq!(rx_data.len(), 0);
        }
    }
}
