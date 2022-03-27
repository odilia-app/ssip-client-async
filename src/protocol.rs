// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021-2022 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use log::debug;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

use crate::types::{ClientError, ClientResult, ClientStatus, EventId, StatusLine};

macro_rules! invalid_input {
    ($msg:expr) => {
        ClientError::from(io::Error::new(io::ErrorKind::InvalidInput, $msg))
    };
    ($fmt:expr, $($arg:tt)*) => {
        invalid_input!(format!($fmt, $($arg)*).as_str())
    };
}

/// Return the only string in the list or an error if there is no line or too many.
pub(crate) fn parse_single_value(lines: &[String]) -> ClientResult<String> {
    match lines.len() {
        0 => Err(ClientError::TooFewLines),
        1 => Ok(lines[0].to_string()),
        _ => Err(ClientError::TooManyLines),
    }
}

/// Convert two lines of the response in an event id
pub(crate) fn parse_event_id(lines: &[String]) -> ClientResult<EventId> {
    match lines.len() {
        0 | 1 => Err(ClientError::TooFewLines),
        2 => Ok(EventId::new(&lines[0], &lines[1])),
        _ => Err(ClientError::TooManyLines),
    }
}

/// Parse single integer value
pub(crate) fn parse_single_integer<T>(lines: &[String]) -> ClientResult<T>
where
    T: FromStr,
{
    parse_single_value(lines)?.parse::<T>().map_err(|_| {
        ClientError::Io(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid integer value",
        ))
    })
}

pub(crate) fn parse_typed_lines<T>(lines: &[String]) -> ClientResult<Vec<T>>
where
    T: FromStr<Err = ClientError>,
{
    lines
        .iter()
        .map(|line| T::from_str(line.as_str()))
        .collect::<ClientResult<Vec<T>>>()
}

/// Write lines separated by CRLF.
pub(crate) fn write_lines(output: &mut dyn Write, lines: &[&str]) -> ClientResult<()> {
    for line in lines.iter() {
        debug!("SSIP(out): {}", line);
        output.write_all(line.as_bytes())?;
        output.write_all(b"\r\n")?;
    }
    Ok(())
}

/// Write lines separated by CRLF and flush the output.
pub(crate) fn flush_lines(output: &mut dyn Write, lines: &[&str]) -> ClientResult<()> {
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

/// Read lines from server until a status line is found.
pub(crate) fn receive_answer(
    input: &mut dyn BufRead,
    mut lines: Option<&mut Vec<String>>,
) -> ClientStatus {
    loop {
        let mut line = String::new();
        input.read_line(&mut line).map_err(ClientError::Io)?;
        debug!("SSIP(in): {}", line.trim_end());
        match line.chars().nth(3) {
            Some(ch) => match ch {
                ' ' => match line[0..3].parse::<u16>() {
                    Ok(code) => return parse_status_line(code, line[4..].trim_end()),
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
            None if line.is_empty() => return Err(invalid_input!("empty line")),
            None => return Err(invalid_input!("line too short: {}", line)),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::io::BufReader;

    use super::{receive_answer, ClientError, ClientResult};

    use crate::types::SynthesisVoice;

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

    #[test]
    fn parse_single_value() -> ClientResult<()> {
        let no_lines = Vec::new();
        assert!(matches!(
            super::parse_single_value(&no_lines),
            Err(ClientError::TooFewLines)
        ));

        let one = String::from("one");
        let one_line = vec![one.to_owned()];
        assert_eq!(one, super::parse_single_value(&one_line)?);

        let two_lines = vec![one, String::from("two")];
        assert!(matches!(
            super::parse_single_value(&two_lines),
            Err(ClientError::TooManyLines)
        ));

        Ok(())
    }

    #[test]
    fn parse_event_id() -> ClientResult<()> {
        let no_lines = Vec::new();
        assert!(matches!(
            super::parse_event_id(&no_lines),
            Err(ClientError::TooFewLines)
        ));

        let one_line = vec![String::from("one")];
        assert!(matches!(
            super::parse_event_id(&one_line),
            Err(ClientError::TooFewLines)
        ));

        let mid = String::from("message");
        let cid = String::from("client");
        let two_lines = vec![mid.to_owned(), cid.to_owned()];
        let event_id = super::parse_event_id(&two_lines)?;
        assert_eq!(mid, event_id.message);
        assert_eq!(cid, event_id.client);

        let three_lines = vec![
            String::from("one"),
            String::from("two"),
            String::from("three"),
        ];
        assert!(matches!(
            super::parse_event_id(&three_lines),
            Err(ClientError::TooManyLines)
        ));

        Ok(())
    }

    #[test]
    fn parse_synthesis_voices() -> ClientResult<()> {
        let lines = ["en", "afrikaans\taf", "lancashire\ten\tuk-north"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let voices = super::parse_typed_lines::<SynthesisVoice>(&lines)?;
        assert_eq!(3, voices.len());
        assert_eq!("en", voices[0].name.as_str());
        assert_eq!(Some(String::from("af")), voices[1].language);
        assert_eq!(Some(String::from("uk-north")), voices[2].dialect);
        Ok(())
    }
}
