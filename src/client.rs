// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::fmt;
use std::io::{self, Read, Write};
use thiserror::Error as ThisError;

use crate::constants::{ReturnCode, OK_RECEIVING_DATA};

/// Command status line
///
/// Consists in a 3-digits code and a message. It can be a success or a failure.
///
/// Examples:
/// - 216 OK OUTPUT MODULE SET
/// - 409 ERR RATE TOO HIGH
#[derive(Debug, PartialEq)]
pub struct StatusLine {
    pub code: ReturnCode,
    pub message: String,
}

impl fmt::Display for StatusLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.code, self.message)
    }
}

/// Client error, either I/O error or SSIP error.
#[derive(ThisError, Debug)]
pub enum ClientError {
    #[error("I/O: {0}")]
    Io(io::Error),
    #[error("SSIP: {0}")]
    Ssip(StatusLine),
}

impl From<io::Error> for ClientError {
    fn from(err: io::Error) -> Self {
        ClientError::Io(err)
    }
}

/// Client result.
pub type ClientResult<T> = Result<T, ClientError>;

/// Client result consisting in a single status line
pub type ClientStatus = ClientResult<StatusLine>;

/// Message identifier
pub type MessageId = String;

macro_rules! client_method {
    ($name:ident, $doc:expr, $line:expr) => {
        #[doc=$doc]
        pub fn $name(&mut self) -> ClientStatus {
            send_line!(&mut self.input, &mut self.output, $line)
        }
    };
}

/// SSIP client on generic stream
pub struct Client<S: Read + Write> {
    input: io::BufReader<S>,
    output: io::BufWriter<S>,
}

impl<S: Read + Write> Client<S> {
    pub(crate) fn new(
        mut input: io::BufReader<S>,
        mut output: io::BufWriter<S>,
        user: &str,
        application: &str,
        component: &str,
    ) -> ClientResult<Self> {
        // https://stackoverflow.com/questions/58467659/how-to-store-tcpstream-with-bufreader-and-bufwriter-in-a-data-structure
        send_line!(
            &mut input,
            &mut output,
            "SET self CLIENT_NAME {}:{}:{}",
            user,
            application,
            component
        )?;
        Ok(Self { input, output })
    }

    /// Send text to server
    pub fn speak(&mut self, lines: &[&str]) -> ClientResult<MessageId> {
        let status = send_line!(&mut self.input, &mut self.output, "SPEAK")?;
        if status.code == OK_RECEIVING_DATA {
            const END_OF_DATA: [&str; 1] = ["."];
            crate::protocol::write_lines(&mut self.output, lines)?;
            let mut answer = Vec::new();
            send_lines!(&mut self.input, &mut self.output, &END_OF_DATA, &mut answer)
                .map(|_| answer[0].to_string())
        } else {
            Err(ClientError::Ssip(status))
        }
    }

    /// Send a single line to the server
    pub fn speak1(&mut self, line: &str) -> ClientResult<MessageId> {
        let lines: [&str; 1] = [line];
        self.speak(&lines)
    }

    client_method!(quit, "Close the connection", "QUIT");
}
