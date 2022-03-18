// Copyright (c) 2021-2022 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::thread;

use std::os::unix::net::UnixListener;

/// Server on a named socket.
pub struct Server {
    listener: UnixListener,
    communication: Vec<(&'static str, &'static str)>,
}

impl Server {
    /// Create a new server on a named socket.
    ///
    /// Argument `communication` is an array of pairs. The first item is a list of strings
    /// the server will receive and the second item is the answer.
    pub fn new<P>(
        socket_path: P,
        communication: &[(&'static str, &'static str)],
    ) -> io::Result<Self>
    where
        P: AsRef<Path>,
    {
        let listener = UnixListener::bind(socket_path.as_ref())?;
        Ok(Server {
            listener,
            communication: communication.to_vec(),
        })
    }

    fn split_lines(lines: &str) -> Vec<String> {
        lines
            .trim_end()
            .split("\r\n")
            .map(|s| format!("{}\r\n", s))
            .collect::<Vec<String>>()
    }

    pub fn serve(&mut self) -> io::Result<()> {
        let (stream, _) = self.listener.accept()?;
        let mut input = BufReader::new(stream.try_clone()?);
        let mut output = BufWriter::new(stream);
        for (questions, answer) in self.communication.iter() {
            for question in Server::split_lines(questions).iter() {
                let mut line = String::new();
                input.read_line(&mut line)?;
                if line != *question {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("read <{}> instead of <{}>", line, *question),
                    ));
                }
            }
            output.write_all(answer.as_bytes())?;
            output.flush()?;
        }
        Ok(())
    }

    pub fn run<P>(
        socket_path: P,
        communication: &'static [(&'static str, &'static str)],
    ) -> thread::JoinHandle<io::Result<()>>
    where
        P: AsRef<Path>,
    {
        let server_path = socket_path.as_ref().to_path_buf();
        let mut server = Server::new(&server_path, communication).unwrap();
        thread::spawn(move || -> io::Result<()> {
            server.serve()?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod test {

    use super::Server;

    #[test]
    fn test_split_lines() {
        const ONE_LINE: &str = "one line\r\n";
        let one_line = Server::split_lines(ONE_LINE);
        assert_eq!(&[ONE_LINE], one_line.as_slice());
    }
}
