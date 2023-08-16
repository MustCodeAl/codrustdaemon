
use tokio::net::UnixStream;
use tokio::io::{BufReader, AsyncRead, Error};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::pin::Pin;
use std::task::Context;
use futures::prelude::*; // Import everything from futures::prelude

/// `SharedStream` wraps an `Arc<Mutex<UnixStream>>` and implements `AsyncRead`.
struct SharedStream(Arc<Mutex<UnixStream>>);

impl SharedStream {
    /// Creates a new `SharedStream` from an `Arc<Mutex<UnixStream>>`.
    fn new(stream: Arc<Mutex<UnixStream>>) -> Self {
        Self(stream)
    }
}

impl AsyncRead for SharedStream {
    /// `poll_read` is a method required by the `AsyncRead` trait.
    /// It tries to read from the `UnixStream` into the provided buffer.
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<Result<(), Error>> {
        let lock_future = self.0.lock();
        futures::pin_mut!(lock_future);
        let mut stream_guard = futures::ready!(lock_future.poll(cx));
        Pin::new(&mut *stream_guard).poll_read(cx, buf)
    }
}

/// Client represents a client connection to a Unix socket.
struct Client {
    reader: Option<BufReader<SharedStream>>,
    conn: Option<Arc<Mutex<UnixStream>>>,
}

impl Client {
    /// Establishes a connection to a Unix socket at the given address.
    async fn dial(&mut self, address: &str) -> Result<(), Error> {
        let stream = UnixStream::connect(address).await?;
        let shared_stream = Arc::new(Mutex::new(stream));

        self.conn = Some(Arc::clone(&shared_stream));
        self.reader = Some(Self::create_reader(Arc::clone(&shared_stream)));

        Ok(())
    }

    /// Creates a `BufReader` for a given `UnixStream`.
    fn create_reader(stream: Arc<Mutex<UnixStream>>) -> BufReader<SharedStream> {
        BufReader::new(SharedStream::new(stream))
    }

    /// Closes the Client connection.
    fn close(&mut self) {
        self.reader.take(); // remove the reader
        self.conn.take(); // remove the connection
    }
}
