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

macro_rules! send_command {
    ($output:expr, $cmd:expr) => {
        crate::protocol::send_command($output, $cmd)
    };
    ($output:expr, $fmt:expr, $($arg:tt)*) => {
        crate::protocol::send_command($output, format!($fmt, $($arg)*).as_str())
    };
}

macro_rules! execute_command {
    ($input:expr, $output:expr, $cmd:expr) => {
        send_command!($output, $cmd)
            .and_then(|()|
                      crate::protocol::receive_answer($input, None))
    };
    ($input:expr, $output:expr, $fmt:expr, $($arg:tt)*) => {
        send_command!($output, $fmt, $($arg)*)
            .and_then(|()|
                      crate::protocol::receive_answer($input, None))
    };
}

pub(crate) fn send_command(output: &mut dyn Write, command: &str) -> ClientResult<()> {
    output.write_all(command.as_bytes())?;
    output.write_all(b"\r\n")?;
    output.flush()?;
    Ok(())
}

/// Parse the status line "OK msg" or "ERR msg"
fn parse_status_line(code: u16, line: &str) -> ClientStatus {
    const TOKEN_OK: &str = "OK ";
    const OFFSET_OK: usize = TOKEN_OK.len();
    const TOKEN_ERR: &str = "ERR ";
    const OFFSET_ERR: usize = TOKEN_ERR.len();
    if line.starts_with(TOKEN_OK) {
        let message = line[OFFSET_OK..].to_string();
        Ok(StatusLine { code, message })
    } else if line.starts_with(TOKEN_ERR) {
        let message = line[OFFSET_ERR..].to_string();
        Err(ClientError::Ssip(StatusLine { code, message }))
    } else {
        let status = StatusLine {
            code,
            message: line.to_string(),
        };
        if (300..700).contains(&code) {
            Err(ClientError::Ssip(status))
        } else {
            Ok(status)
        }
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
            None => return Err(invalid_input!("line too short")),
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
