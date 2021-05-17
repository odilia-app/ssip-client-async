// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::io::{self, BufRead, Write};

use crate::client::{ClientError, ClientResult, ClientStatus, StatusLine};

macro_rules! invalid_input {
    ($msg:expr) => {
        ClientError::from(io::Error::new(io::ErrorKind::InvalidInput, $msg))
    };
    ($fmt:expr, $($arg:tt)*) => {
        invalid_input!(format!($fmt, $($arg)*).as_str())
    };
}

macro_rules! send_lines {
    ($input:expr, $output:expr, $lines:expr) => {
        crate::protocol::send_lines($output, $lines)
            .and_then(|()| crate::protocol::receive_answer($input, None))
    };
    ($input:expr, $output:expr, $lines:expr, $out:expr) => {
        crate::protocol::send_lines($output, $lines)
            .and_then(|()| crate::protocol::receive_answer($input, Some($out)))
    };
}

macro_rules! send_line {
    ($input:expr, $output:expr, $line:expr) => {
        send_lines!($input, $output, &[$line])
    };
    ($input:expr, $output:expr, $fmt:expr, $($arg:tt)*) => {
        send_line!($input, $output, format!($fmt, $($arg)*).as_str())
    };
}

pub(crate) fn write_lines(output: &mut dyn Write, lines: &[&str]) -> ClientResult<()> {
    for line in lines.iter() {
        output.write_all(line.as_bytes())?;
        output.write_all(b"\r\n")?;
    }
    Ok(())
}

pub(crate) fn send_lines(output: &mut dyn Write, lines: &[&str]) -> ClientResult<()> {
    write_lines(output, lines)?;
    output.flush()?;
    Ok(())
}

/// Strip prefix if found
fn strip_prefix(line: &str, prefix: &str) -> String {
    line.strip_prefix(prefix).unwrap_or(line).to_string()
}

/// Parse the status line "OK msg" or "ERR msg"
fn parse_status_line(code: u16, line: &str) -> ClientStatus {
    if (300..700).contains(&code) {
        const TOKEN_ERR: &str = "ERR ";
        let message = strip_prefix(line, TOKEN_ERR);
        Err(ClientError::Ssip(StatusLine { code, message }))
    } else {
        const TOKEN_OK: &str = "OK ";
        let message = strip_prefix(line, TOKEN_OK);
        Ok(StatusLine { code, message })
    }
}

pub(crate) fn receive_answer(
    input: &mut dyn BufRead,
    mut lines: Option<&mut Vec<String>>,
) -> ClientStatus {
    loop {
        let mut line = String::new();
        input.read_line(&mut line).map_err(ClientError::Io)?;
        match line.chars().nth(3) {
            Some(ch) => match ch {
                ' ' => match line[0..3].parse::<u16>() {
                    Ok(code) => return parse_status_line(code, &line[4..].trim_end()),
                    Err(err) => return Err(invalid_input!(err.to_string())),
                },
                '-' => match lines {
                    Some(ref mut lines) => lines.push(line[4..].trim_end().to_string()),
                    None => return Err(invalid_input!("unexpected line: {}", line)),
                },
                ch => {
                    return Err(invalid_input!("expecting space or dash, got {}.", ch));
                }
            },
            None => return Err(invalid_input!("line too short: {}", line)),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::io::BufReader;

    use super::{receive_answer, ClientError};

    #[test]
    fn single_ok_status_line() {
        let mut input = BufReader::new("208 OK CLIENT NAME SET\r\n".as_bytes());
        let status = receive_answer(&mut input, None).unwrap();
        assert_eq!(208, status.code);
        assert_eq!("CLIENT NAME SET", status.message);
    }

    #[test]
    fn single_success_status_line() {
        let mut input = BufReader::new("231 HAPPY HACKING\r\n".as_bytes());
        let status = receive_answer(&mut input, None).unwrap();
        assert_eq!(231, status.code);
        assert_eq!("HAPPY HACKING", status.message);
    }

    #[test]
    fn single_err_status_line() {
        let mut input = BufReader::new("409 ERR RATE TOO HIGH\r\n".as_bytes());
        match receive_answer(&mut input, None).err().unwrap() {
            ClientError::Ssip(status) => {
                assert_eq!(409, status.code);
                assert_eq!("RATE TOO HIGH", status.message);
            }
            err => panic!("{}: invalid error", err),
        }
    }

    #[test]
    fn multi_lines() {
        let mut input = BufReader::new(
            "249-afrikaans\taf\tnone\r\n249-en-rhotic\ten\tr\r\n249 OK VOICE LIST SENT\r\n"
                .as_bytes(),
        );
        let mut lines = Vec::new();
        let status = receive_answer(&mut input, Some(&mut lines)).unwrap();
        assert_eq!(249, status.code);
        assert_eq!("VOICE LIST SENT", status.message);
        assert_eq!(
            vec!["afrikaans\taf\tnone", "en-rhotic\ten\tr"],
            lines.as_slice()
        );
    }
}
