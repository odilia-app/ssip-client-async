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
    let stream = UnixStream::connect(socket_path)?;
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
    use std::os::unix::net::UnixStream;
    use std::path::Path;
    use std::thread;
    use std::time::Duration;

    use super::{new_pair, Client};

    struct Server {
        input: BufReader<UnixStream>,
        output: BufWriter<UnixStream>,
        answers: Vec<&'static str>,
    }

    impl Server {
        fn new<P>(socket_path: &P, answers: &[&'static str]) -> io::Result<Self>
        where
            P: AsRef<Path>,
        {
            let timeout = Duration::new(5, 0);
            let (input, output) = new_pair(&socket_path, Some(timeout))?;
            Ok(Server {
                input,
                output,
                answers: answers.to_vec(),
            })
        }

        fn serve(&mut self) -> io::Result<()> {
            for answer in self.answers.iter() {
                let mut line = String::new();
                self.input.read_line(&mut line)?;
                self.output.write_all(answer.as_bytes())?;
                self.output.flush()?;
            }
            Ok(())
        }

        fn temporary_path() -> io::Result<tempfile::TempPath> {
            let tfile = tempfile::NamedTempFile::new()?;
            Ok(tfile.into_temp_path())
        }

        fn run<P>(
            socket_path: P,
            answers: &'static [&'static str],
        ) -> io::Result<thread::JoinHandle<io::Result<()>>>
        where
            P: AsRef<Path>,
        {
            let unix_path = socket_path.as_ref().to_str().unwrap().to_string();
            unsafe {
                libc::mkfifo(unix_path.as_ptr() as *const libc::c_char, 0o700);
            }
            let server_path = socket_path.as_ref().to_path_buf();
            Ok(thread::spawn(move || -> io::Result<()> {
                let mut server = Server::new(&server_path, answers)?;
                server.serve()?;
                Ok(())
            }))
        }
    }

    /// Create a server and run the client
    fn test_client<F>(answers: &'static [&'static str], process: F) -> io::Result<()>
    where
        F: FnMut(&mut Client<UnixStream>) -> io::Result<()>,
    {
        let socket_path = Server::temporary_path()?;
        let server_path = socket_path.to_path_buf();
        let mut process_wrapper = std::panic::AssertUnwindSafe(process);
        let result = std::panic::catch_unwind(move || {
            let handle = Server::run(&server_path, answers).unwrap();
            let (input, output) = new_pair(&server_path, None)?;
            let mut client = Client::new(input, output, "test", "test", "main").unwrap();
            process_wrapper(&mut client).unwrap();
            handle.join().unwrap()
        });
        socket_path.close().unwrap();
        assert!(result.is_ok());
        Ok(())
    }

    #[test]
    fn connect_and_quit() -> io::Result<()> {
        test_client(
            &["208 OK CLIENT NAME SET\r\n", "231 HAPPY HACKING\r\n"],
            |client| {
                assert_eq!(231, client.quit().unwrap().code);
                Ok(())
            },
        )
    }
}
