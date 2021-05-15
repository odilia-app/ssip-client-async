use std::fmt;
use std::io::{self, Read, Write};
use thiserror::Error as ThisError;

pub type ReturnCode = u16;

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
        execute_command!(
            &mut input,
            &mut output,
            "SET self CLIENT_NAME {}:{}:{}",
            user,
            application,
            component
        )?;
        Ok(Self { input, output })
    }

    /// Close the connection
    pub fn quit(&mut self) -> ClientStatus {
        execute_command!(&mut self.input, &mut self.output, "QUIT")
    }
}
