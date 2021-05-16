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

pub fn new_client(
    user: &str,
    application: &str,
    component: &str,
) -> ClientResult<Client<UnixStream>> {
    let socket_path = speech_dispatcher_socket()?;
    let (input, output) = new_pair(&socket_path, None)?;
    Client::new(input, output, user, application, component)
}

#[cfg(test)]
mod tests {

    use std::io::{self, BufRead, BufReader, BufWriter, Write};
    use std::os::unix::net::{UnixListener, UnixStream};
    use std::path::{Path, PathBuf};
    use std::thread;

    use crate::constants::*;

    use super::{new_pair, Client};

    struct Server {
        listener: UnixListener,
        communication: Vec<(&'static [&'static str], &'static str)>,
    }

    impl Server {
        fn new<P>(
            socket_path: &P,
            communication: &[(&'static [&'static str], &'static str)],
        ) -> io::Result<Self>
        where
            P: AsRef<Path>,
        {
            let listener = UnixListener::bind(socket_path)?;
            Ok(Server {
                listener,
                communication: communication.to_vec(),
            })
        }

        fn serve(&mut self) -> io::Result<()> {
            let (stream, _) = self.listener.accept()?;
            let mut input = BufReader::new(stream.try_clone()?);
            let mut output = BufWriter::new(stream);
            for (questions, answer) in self.communication.iter() {
                for question in questions.iter() {
                    let mut line = String::new();
                    input.read_line(&mut line)?;
                    if line != dbg!(*question) {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("read <{}> instead of <{}>", dbg!(line), *question),
                        ));
                    }
                }
                output.write_all(answer.as_bytes())?;
                output.flush()?;
            }
            Ok(())
        }

        fn temporary_path() -> PathBuf {
            let tid = unsafe { libc::pthread_self() } as u64;
            std::env::temp_dir().join(format!("ssip-client-test-{}-{}", std::process::id(), tid))
        }

        fn run<P>(
            socket_path: P,
            communication: &'static [(&'static [&'static str], &'static str)],
        ) -> thread::JoinHandle<io::Result<()>>
        where
            P: AsRef<Path>,
        {
            let server_path = socket_path.as_ref().to_path_buf();
            let mut server = Server::new(&server_path, communication).unwrap();
            let handle = thread::spawn(move || -> io::Result<()> {
                server.serve()?;
                Ok(())
            });
            handle
        }
    }

    /// Create a server and run the client
    fn test_client<F>(
        communication: &'static [(&'static [&'static str], &'static str)],
        process: F,
    ) -> io::Result<()>
    where
        F: FnMut(&mut Client<UnixStream>) -> io::Result<()>,
    {
        let socket_path = Server::temporary_path();
        assert!(!socket_path.exists());
        let server_path = socket_path.to_path_buf();
        let mut process_wrapper = std::panic::AssertUnwindSafe(process);
        let result = std::panic::catch_unwind(move || {
            let handle = Server::run(&server_path, communication);
            let (input, output) = new_pair(&server_path, None)?;
            let mut client = Client::new(input, output, "test", "test", "main").unwrap();
            process_wrapper(&mut client).unwrap();
            handle.join().unwrap()
        });
        std::fs::remove_file(socket_path)?;
        assert_eq!((), result.unwrap().unwrap());
        Ok(())
    }

    const SET_CLIENT_COMMUNICATION: (&'static [&'static str], &'static str) = (
        &["SET self CLIENT_NAME test:test:main\r\n"],
        "208 OK CLIENT NAME SET\r\n",
    );

    #[test]
    fn connect_and_quit() -> io::Result<()> {
        test_client(
            &[
                SET_CLIENT_COMMUNICATION,
                (&["QUIT\r\n"], "231 HAPPY HACKING\r\n"),
            ],
            |client| {
                assert_eq!(OK_BYE, client.quit().unwrap().code);
                Ok(())
            },
        )
    }

    #[test]
    fn speak_one_line() -> io::Result<()> {
        test_client(
            &[
                SET_CLIENT_COMMUNICATION,
                (&["SPEAK\r\n"], "230 OK RECEIVING DATA\r\n"),
                (&["Hello, world\r\n", ".\r\n"], "225 OK MESSAGE QUEUED\r\n"),
            ],
            |client| {
                assert_eq!(
                    OK_MESSAGE_QUEUED,
                    client.speak1("Hello, world").unwrap().code,
                );
                Ok(())
            },
        )
    }
}
