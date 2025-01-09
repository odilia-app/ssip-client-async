use crate::{
	Error,
	EventId,
	StatusLine,
	ClientStatus,
};
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use core::{
	str::FromStr,
	fmt::Error as FmtError,
};

/// Return the only string in the list or an error if there is no line or too many.
pub(crate) fn parse_single_value(lines: &[String]) -> Result<String, Error> {
    match lines.len() {
        0 => Err(Error::TooFewLines),
        1 => Ok(lines[0].to_string()),
        _ => Err(Error::TooManyLines),
    }
}

/// Convert two lines of the response in an event id
pub(crate) fn parse_event_id(lines: &[String]) -> Result<EventId, Error> {
    match lines.len() {
        0 | 1 => Err(Error::TooFewLines),
        2 => Ok(EventId::new(&lines[0], &lines[1])),
        _ => Err(Error::TooManyLines),
    }
}

/// Parse single integer value
pub(crate) fn parse_single_integer<T>(lines: &[String]) -> Result<T, Error>
where
    T: FromStr,
{
    parse_single_value(lines)?.parse::<T>().map_err(|_| {
        Error::InvalidData(
            "invalid integer value",
        )
    })
}

pub(crate) fn parse_typed_lines<T>(lines: &[String]) -> Result<Vec<T>, Error>
where
    T: FromStr<Err = Error>,
{
    lines
        .iter()
        .map(|line| T::from_str(line.as_str()))
        .collect()
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
        Err(Error::Ssip(StatusLine { code, message }))
    } else {
        const TOKEN_OK: &str = "OK ";
        let message = strip_prefix(line, TOKEN_OK);
        Ok(StatusLine { code, message })
    }
}

/*
/// Read lines from server until a status line is found asyncronously.
pub(crate) fn receive_bytes(
    input: &mut W,
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
*/
