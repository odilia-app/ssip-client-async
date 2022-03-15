// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::io;
use std::path::PathBuf;

const SPEECHD_APPLICATION_NAME: &str = "speech-dispatcher";
const SPEECHD_SOCKET_NAME: &str = "speechd.sock";

/// Return the standard socket according to the [freedesktop.org](https://www.freedesktop.org/) specification.
fn speech_dispatcher_socket() -> io::Result<PathBuf> {
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

#[cfg(not(feature = "metal-io"))]
mod synchronous {
    use std::io::{BufReader, BufWriter};
    pub use std::os::unix::net::UnixStream;
    use std::path::Path;
    use std::time::Duration;

    use crate::client::{Client, ClientResult};

    /// New FIFO client
    pub fn new_fifo_client<P>(
        socket_path: P,
        read_timeout: Option<Duration>,
    ) -> ClientResult<Client<UnixStream>>
    where
        P: AsRef<Path>,
    {
        let stream = UnixStream::connect(socket_path.as_ref())?;
        stream.set_read_timeout(read_timeout)?;
        Client::new(BufReader::new(stream.try_clone()?), BufWriter::new(stream))
    }

    /// New FIFO client on the standard socket `${XDG_RUNTIME_DIR}/speech-dispatcher/speechd.sock`
    pub fn new_default_fifo_client(
        read_timeout: Option<Duration>,
    ) -> ClientResult<Client<UnixStream>> {
        let socket_path = super::speech_dispatcher_socket()?;
        new_fifo_client(socket_path.as_path(), read_timeout)
    }
}

#[cfg(not(feature = "metal-io"))]
pub use synchronous::{new_default_fifo_client, new_fifo_client, UnixStream};

#[cfg(feature = "metal-io")]
mod asynchronous {
    pub use mio::net::UnixStream;
    use std::io::{BufReader, BufWriter};
    use std::os::unix::net::UnixStream as StdUnixStream;
    use std::path::Path;

    use crate::client::{Client, ClientResult};

    /// New FIFO client
    pub fn new_fifo_client<P>(socket_path: P) -> ClientResult<Client<UnixStream>>
    where
        P: AsRef<Path>,
    {
        let stream = StdUnixStream::connect(socket_path.as_ref())?;
        stream.set_nonblocking(true)?;
        Client::new(
            BufReader::new(UnixStream::from_std(stream.try_clone()?)),
            BufWriter::new(UnixStream::from_std(stream.try_clone()?)),
            UnixStream::from_std(stream),
        )
    }

    /// New FIFO client on the standard socket `${XDG_RUNTIME_DIR}/speech-dispatcher/speechd.sock`
    pub fn new_default_fifo_client() -> ClientResult<Client<UnixStream>> {
        let socket_path = super::speech_dispatcher_socket()?;
        new_fifo_client(socket_path.as_path())
    }
}

#[cfg(feature = "metal-io")]
pub use asynchronous::{new_default_fifo_client, new_fifo_client, UnixStream};

#[cfg(test)]
mod tests {

    #[test]
    fn test_speech_dispatcher_socket() -> std::io::Result<()> {
        if std::env::var("XDG_RUNTIME_DIR").is_ok() {
            let socket_path = super::speech_dispatcher_socket()?;
            assert!(socket_path
                .to_str()
                .unwrap()
                .ends_with("/speech-dispatcher/speechd.sock"));
        }
        Ok(())
    }
}
