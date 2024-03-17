// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021-2022 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::io;
use std::path::{Path, PathBuf};

const SPEECHD_APPLICATION_NAME: &str = "speech-dispatcher";
const SPEECHD_SOCKET_NAME: &str = "speechd.sock";

struct FifoPath {
    path: Option<PathBuf>,
}

impl FifoPath {
    fn new() -> FifoPath {
        FifoPath { path: None }
    }

    fn set<P>(&mut self, path: P)
    where
        P: AsRef<Path>,
    {
        self.path = Some(path.as_ref().to_path_buf());
    }

    /// Return the standard socket according to the [freedesktop.org](https://www.freedesktop.org/) specification.
    fn default_path() -> io::Result<PathBuf> {
        match dirs::runtime_dir() {
            Some(runtime_dir) => Ok(runtime_dir
                .join(SPEECHD_APPLICATION_NAME)
                .join(SPEECHD_SOCKET_NAME)),
            None => Err(io::Error::new(
                io::ErrorKind::NotFound,
                "unix socket not found",
            )),
        }
    }

    fn get(&self) -> io::Result<PathBuf> {
        match &self.path {
            Some(path) => Ok(path.to_path_buf()),
            _ => FifoPath::default_path(),
        }
    }
}

#[cfg(not(feature = "async-mio"))]
mod synchronous {
    use std::io::{self, BufReader, BufWriter};
    pub use std::os::unix::net::UnixStream;
    use std::path::Path;
    use std::time::Duration;

    use crate::client::Client;
    use crate::net::StreamMode;

    use super::FifoPath;

    pub struct Builder {
        path: FifoPath,
        mode: StreamMode,
    }

    impl Builder {
        pub fn new() -> Self {
            Self {
                path: FifoPath::new(),
                mode: StreamMode::Blocking,
            }
        }

        pub fn path<P>(&mut self, socket_path: P) -> &mut Self
        where
            P: AsRef<Path>,
        {
            self.path.set(socket_path);
            self
        }

        pub fn timeout(&mut self, read_timeout: Duration) -> &mut Self {
            self.mode = StreamMode::TimeOut(read_timeout);
            self
        }

        pub fn nonblocking(&mut self) -> &mut Self {
            self.mode = StreamMode::NonBlocking;
            self
        }

        pub fn build(&self) -> io::Result<Client<UnixStream>> {
            let input = UnixStream::connect(self.path.get()?)?;
            match self.mode {
                StreamMode::Blocking => input.set_nonblocking(false)?,
                StreamMode::NonBlocking => input.set_nonblocking(true)?,
                StreamMode::TimeOut(timeout) => input.set_read_timeout(Some(timeout))?,
            }
            let output = input.try_clone()?;
            Ok(Client::new(BufReader::new(input), BufWriter::new(output)))
        }
    }
}

#[cfg(not(feature = "async-mio"))]
pub use synchronous::{Builder, UnixStream};

#[cfg(feature = "async-mio")]
mod asynchronous {
    pub use mio::net::UnixStream;
    use std::io::{self, BufReader, BufWriter};
    use std::os::unix::net::UnixStream as StdUnixStream;
    use std::path::Path;

    use crate::client::Client;

    use super::FifoPath;

    pub struct Builder {
        path: FifoPath,
    }

    impl Builder {
        pub fn new() -> Self {
            Self {
                path: FifoPath::new(),
            }
        }

        fn non_blocking(socket: StdUnixStream) -> io::Result<StdUnixStream> {
            socket.set_nonblocking(true)?;
            Ok(socket)
        }

        pub fn path<P>(&mut self, socket_path: P) -> &mut Self
        where
            P: AsRef<Path>,
        {
            self.path.set(socket_path);
            self
        }

        pub fn build(&self) -> io::Result<Client<UnixStream>> {
            let stream = StdUnixStream::connect(self.path.get()?)?;
            Ok(Client::new(
                BufReader::new(UnixStream::from_std(Self::non_blocking(
                    stream.try_clone()?,
                )?)),
                BufWriter::new(UnixStream::from_std(Self::non_blocking(stream)?)),
            ))
        }
    }
}
#[cfg(feature = "tokio")]
pub mod asynchronous_tokio {
    pub use tokio::net::{
      UnixStream,
      unix::OwnedReadHalf,
      unix::OwnedWriteHalf,
    };
    use tokio::io::{self, BufReader as AsyncBufReader, BufWriter as AsyncBufWriter};
    use std::path::Path;

    use crate::tokio::AsyncClient;

    use super::FifoPath;

    pub struct Builder {
        path: FifoPath,
    }

    impl Builder {
        pub fn new() -> Self {
            Self {
                path: FifoPath::new(),
            }
        }

        pub fn path<P>(&mut self, socket_path: P) -> &mut Self
        where
            P: AsRef<Path>,
        {
            self.path.set(socket_path);
            self
        }

        pub async fn build(&self) -> io::Result<AsyncClient<AsyncBufReader<OwnedReadHalf>, AsyncBufWriter<OwnedWriteHalf>>> {
            let (read_stream, write_stream) = UnixStream::connect(self.path.get()?).await?
              .into_split();
            Ok(AsyncClient::new(
                AsyncBufReader::new(read_stream),
                AsyncBufWriter::new(write_stream),
            ))
        }
    }
}

#[cfg(feature = "async-mio")]
pub use asynchronous::{Builder, UnixStream};

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_fifo_path() -> std::io::Result<()> {
        if std::env::var("XDG_RUNTIME_DIR").is_ok() {
            let socket_path = super::FifoPath::new();
            assert!(socket_path
                .get()?
                .to_str()
                .unwrap()
                .ends_with("/speech-dispatcher/speechd.sock"));
        }
        Ok(())
    }
}
