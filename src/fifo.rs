// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::io::{self, BufReader, BufWriter};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::client::{Client, ClientResult};

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

/// Create a pair of FIFO reader and writer
fn new_pair<P>(
    socket_path: P,
    read_timeout: Option<Duration>,
) -> io::Result<(BufReader<UnixStream>, BufWriter<UnixStream>)>
where
    P: AsRef<Path>,
{
    let stream = UnixStream::connect(socket_path.as_ref())?;
    stream.set_read_timeout(read_timeout)?;
    Ok((BufReader::new(stream.try_clone()?), BufWriter::new(stream)))
}

/// New FIFO client
pub fn new_client<P>(
    socket_path: P,
    user: &str,
    application: &str,
    component: &str,
    read_timeout: Option<Duration>,
) -> ClientResult<Client<UnixStream>>
where
    P: AsRef<Path>,
{
    let (input, output) = new_pair(&socket_path, read_timeout)?;
    Client::new(input, output, user, application, component)
}

/// New FIFO client on the standard socket `${XDG_RUNTIME_DIR}/speech-dispatcher/speechd.sock`
pub fn new_default_client(
    user: &str,
    application: &str,
    component: &str,
    read_timeout: Option<Duration>,
) -> ClientResult<Client<UnixStream>> {
    let socket_path = speech_dispatcher_socket()?;
    new_client(
        socket_path.as_path(),
        user,
        application,
        component,
        read_timeout,
    )
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_speech_dispatcher_socket() -> std::io::Result<()> {
        let socket_path = super::speech_dispatcher_socket()?;
        assert!(socket_path
            .to_str()
            .unwrap()
            .ends_with("/speech-dispatcher/speechd.sock"));
        Ok(())
    }
}
