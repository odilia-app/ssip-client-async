use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::thread;

use ssip_client::*;

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
        let mut client =
            ssip_client::new_fifo_client(&server_path, "test", "test", "main", None).unwrap();
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
fn say_one_line() -> io::Result<()> {
    test_client(
        &[
            SET_CLIENT_COMMUNICATION,
            (&["SPEAK\r\n"], "230 OK RECEIVING DATA\r\n"),
            (
                &["Hello, world\r\n", ".\r\n"],
                "225-21\r\n225 OK MESSAGE QUEUED\r\n",
            ),
        ],
        |client| {
            assert_eq!("21", client.say_line("Hello, world").unwrap(),);
            Ok(())
        },
    )
}
