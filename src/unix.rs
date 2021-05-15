use std::io;
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

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

pub fn new(user: &str, application: &str, component: &str) -> ClientResult<Client<UnixStream>> {
    let socket_path = speech_dispatcher_socket()?;
    let stream = UnixStream::connect(socket_path)?;
    let input = io::BufReader::new(stream.try_clone()?);
    let output = io::BufWriter::new(stream);
    Client::new(input, output, user, application, component)
}
