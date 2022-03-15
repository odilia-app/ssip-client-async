// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::io::{self, Read, Write};
use std::str::FromStr;
use thiserror::Error as ThisError;

use crate::constants::*;
use crate::protocol::{send_lines, write_lines};
use crate::types::{
    CapitalLettersRecognitionMode, ClientScope, Event, KeyName, MessageId, MessageScope,
    NotificationType, Priority, PunctuationMode, ReturnCode, Source, StatusLine, SynthesisVoice,
};

/// Client error, either I/O error or SSIP error.
#[derive(ThisError, Debug)]
pub enum ClientError {
    #[error("Invalid type")]
    InvalidType,
    #[error("I/O: {0}")]
    Io(io::Error),
    #[error("No line in result")]
    NoLine,
    #[error("SSIP: {0}")]
    Ssip(StatusLine),
    #[error("Too many lines")]
    TooManyLines,
    #[error("Truncated message")]
    TruncatedMessage,
    #[error("Unexpected status: {0}")]
    UnexpectedStatus(ReturnCode),
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

/// Client name
pub struct ClientName {
    pub user: String,
    pub application: String,
    pub component: String,
}

impl ClientName {
    pub fn new(user: &str, application: &str) -> Self {
        ClientName::with_component(user, application, "main")
    }

    pub fn with_component(user: &str, application: &str, component: &str) -> Self {
        ClientName {
            user: user.to_string(),
            application: application.to_string(),
            component: component.to_string(),
        }
    }
}

/// Convert boolean to ON or OFF
fn on_off(value: bool) -> &'static str {
    if value {
        "on"
    } else {
        "off"
    }
}

macro_rules! client_send {
    ($name:ident, $doc:expr, $scope:ident, $value_name:ident as $value_type:ty, $fmt:expr, $value:expr) => {
        #[doc=$doc]
        pub fn $name(
            &mut self,
            $scope: ClientScope,
            $value_name: $value_type,
        ) -> ClientResult<&mut Client<S>> {
            let line = match $scope {
                ClientScope::Current => format!($fmt, "self", $value),
                ClientScope::All => format!($fmt, "all", $value),
                ClientScope::Client(id) => format!($fmt, id, $value),
            };

            send_lines(&mut self.output, &[line.as_str()])?;
            Ok(self)
        }
    };
    ($name:ident, $doc:expr, $scope:ident, $value_name:ident as $value_type:ty, $fmt:expr) => {
        client_send!(
            $name,
            $doc,
            $scope,
            $value_name as $value_type,
            $fmt,
            $value_name
        );
    };
    ($name:ident, $doc:expr, $value_name:ident as $value_type:ty, $fmt:expr, $value:expr) => {
        #[doc=$doc]
        pub fn $name(&mut self, $value_name: $value_type) -> ClientResult<&mut Client<S>> {
            send_lines(&mut self.output, &[format!($fmt, $value).as_str()])?;
            Ok(self)
        }
    };
    ($name:ident, $doc:expr, $value_name:ident as $value_type:ty, $fmt:expr) => {
        client_send!($name, $doc, $value_name as $value_type, $fmt, $value_name);
    };
    ($name:ident, $doc:expr, $line:expr) => {
        #[doc=$doc]
        pub fn $name(&mut self) -> ClientResult<&mut Client<S>> {
            send_lines(&mut self.output, &[$line])?;
            Ok(self)
        }
    };
}

macro_rules! client_send_boolean {
    ($name:ident, $doc:expr, $scope:ident, $value_name:ident, $fmt:expr) => {
        client_send!(
            $name,
            $doc,
            $scope,
            $value_name as bool,
            $fmt,
            on_off($value_name)
        );
    };
    ($name:ident, $doc:expr, $value_name:ident, $fmt:expr) => {
        client_send!($name, $doc, $value_name as bool, $fmt, on_off($value_name));
    };
}

macro_rules! client_send_range {
    ($name:ident, $doc:expr, $scope:ident, $value_name:ident, $fmt:expr) => {
        client_send!(
            $name,
            $doc,
            $scope,
            $value_name as i8,
            $fmt,
            std::cmp::max(-100, std::cmp::min(100, $value_name))
        );
    };
}

/// SSIP client on generic stream
#[cfg(not(feature = "metal-io"))]
pub struct Client<S: Read + Write> {
    input: io::BufReader<S>,
    output: io::BufWriter<S>,
}

#[cfg(feature = "metal-io")]
pub struct Client<S: Read + Write + Source> {
    input: io::BufReader<S>,
    output: io::BufWriter<S>,
    socket: S,
}

impl<S: Read + Write + Source> Client<S> {
    #[cfg(not(feature = "metal-io"))]
    pub(crate) fn new(input: io::BufReader<S>, output: io::BufWriter<S>) -> ClientResult<Self> {
        // https://stackoverflow.com/questions/58467659/how-to-store-tcpstream-with-bufreader-and-bufwriter-in-a-data-structure
        Ok(Self { input, output })
    }

    #[cfg(feature = "metal-io")]
    pub(crate) fn new(
        input: io::BufReader<S>,
        output: io::BufWriter<S>,
        socket: S,
    ) -> ClientResult<Self> {
        Ok(Self {
            socket,
            input,
            output,
        })
    }

    /// Return the only string in the list or an error if there is no line or too many.
    fn parse_single_value(lines: &[String]) -> ClientResult<String> {
        match lines.len() {
            0 => Err(ClientError::NoLine),
            1 => Ok(lines[0].to_string()),
            _ => Err(ClientError::TooManyLines),
        }
    }

    /// Set the client name. It must be the first call on startup.
    pub fn set_client_name(&mut self, client_name: ClientName) -> ClientResult<&mut Client<S>> {
        send_lines(
            &mut self.output,
            &[format!(
                "SET self CLIENT_NAME {}:{}:{}",
                client_name.user, client_name.application, client_name.component
            )
            .as_str()],
        )?;
        Ok(self)
    }

    /// Initiate communitation to send text to speak
    pub fn speak(&mut self) -> ClientResult<&mut Client<S>> {
        send_lines(&mut self.output, &["SPEAK"])?;
        Ok(self)
    }

    /// Send lines
    pub fn send_lines(&mut self, lines: &[&str]) -> ClientResult<&mut Client<S>> {
        const END_OF_DATA: [&str; 1] = ["."];
        write_lines(&mut self.output, lines)?;
        send_lines(&mut self.output, &END_OF_DATA)?;
        Ok(self)
    }

    /// Send a line
    pub fn send_line(&mut self, line: &str) -> ClientResult<&mut Client<S>> {
        const END_OF_DATA: &str = ".";
        self.send_lines(&[line, END_OF_DATA])
    }

    /// Send a char
    pub fn send_char(&mut self, ch: char) -> ClientResult<&mut Client<S>> {
        self.send_lines(&[format!("CHAR {}", ch).as_str()])
    }

    /// Send a symbolic key name
    pub fn say_key_name(&mut self, keyname: KeyName) -> ClientResult<&mut Client<S>> {
        self.send_lines(&[format!("KEY {}", keyname).as_str()])
    }

    /// Action on a message or a group of messages
    fn send_message_command(
        &mut self,
        command: &str,
        scope: MessageScope,
    ) -> ClientResult<&mut Client<S>> {
        let line = match scope {
            MessageScope::Last => format!("{} self", command),
            MessageScope::All => format!("{} all", command),
            MessageScope::Message(id) => format!("{} {}", command, id),
        };
        send_lines(&mut self.output, &[line.as_str()])?;
        Ok(self)
    }

    /// Stop current message
    pub fn stop(&mut self, scope: MessageScope) -> ClientResult<&mut Client<S>> {
        self.send_message_command("STOP", scope)
    }

    /// Cancel current message
    pub fn cancel(&mut self, scope: MessageScope) -> ClientResult<&mut Client<S>> {
        self.send_message_command("CANCEL", scope)
    }

    /// Pause current message
    pub fn pause(&mut self, scope: MessageScope) -> ClientResult<&mut Client<S>> {
        self.send_message_command("PAUSE", scope)
    }

    /// Resume current message
    pub fn resume(&mut self, scope: MessageScope) -> ClientResult<&mut Client<S>> {
        self.send_message_command("RESUME", scope)
    }

    client_send!(
        set_priority,
        "Set message priority",
        priority as Priority,
        "SET self PRIORITY {}"
    );

    client_send_boolean!(
        set_debug,
        "Set debug mode. Return the log location",
        value,
        "SET all DEBUG {}"
    );

    client_send!(
        set_output_module,
        "Set output module",
        scope,
        value as &str,
        "SET {} OUTPUT_MODULE {}"
    );

    client_send!(
        get_output_module,
        "Get the current output module",
        "GET OUTPUT_MODULE"
    );

    client_send!(
        list_output_modules,
        "List the available output modules",
        "LIST OUTPUT_MODULES"
    );

    client_send!(
        set_language,
        "Set language code",
        scope,
        value as &str,
        "SET {} LANGUAGE {}"
    );

    client_send!(get_language, "Get the current language", "GET LANGUAGE");

    client_send_boolean!(
        set_ssml_mode,
        "Set SSML mode (Speech Synthesis Markup Language)",
        value,
        "SET self SSML_MODE {}"
    );

    client_send!(
        set_punctuation_mode,
        "Set punctuation mode",
        scope,
        value as PunctuationMode,
        "SET {} PUNCTUATION {}"
    );

    client_send_boolean!(
        set_spelling,
        "Set spelling on or off",
        scope,
        value,
        "SET {} SPELLING {}"
    );

    client_send!(
        set_capital_letter_recogn,
        "Set capital letters recognition mode",
        scope,
        value as CapitalLettersRecognitionMode,
        "SET {} CAP_LET_RECOGN {}"
    );

    client_send!(
        set_voice_type,
        "Set the voice type (MALE1, FEMALE1, …)",
        scope,
        value as &str,
        "SET {} VOICE_TYPE {}"
    );

    client_send!(
        get_voice_type,
        "Get the current pre-defined voice",
        "GET VOICE_TYPE"
    );

    client_send!(
        list_voice_types,
        "List the available symbolic voice names",
        "LIST VOICES"
    );

    client_send!(
        set_synthesis_voice,
        "Set the voice",
        scope,
        value as &str,
        "SET {} SYNTHESIS_VOICE {}"
    );

    client_send!(
        list_synthesis_voices,
        "Lists the available voices for the current synthesizer",
        "LIST SYNTHESIS_VOICES"
    );

    client_send_range!(
        set_rate,
        "Set the rate of speech. n is an integer value within the range from -100 to 100, lower values meaning slower speech.",
        scope,
        value,
        "SET {} RATE {}"
    );

    client_send!(get_rate, "Get the current rate of speech.", "GET RATE");

    client_send_range!(
        set_pitch,
        "Set the pitch of speech. n is an integer value within the range from -100 to 100.",
        scope,
        value,
        "SET {} PITCH {}"
    );

    client_send!(get_pitch, "Get the current pitch value.", "GET PITCH");

    client_send_range!(
        set_volume,
        "Set the volume of speech. n is an integer value within the range from -100 to 100.",
        scope,
        value,
        "SET {} VOLUME {}"
    );

    client_send!(
        set_pause_context,
        "Set the number of (more or less) sentences that should be repeated after a previously paused text is resumed.",
        scope,
        value as u8,
        "SET {} PAUSE_CONTEXT {}"
    );

    client_send_boolean!(
        set_history,
        "Enable or disable history of received messages.",
        scope,
        value,
        "SET {} HISTORY {}"
    );

    client_send!(get_volume, "Get the current volume.", "GET VOLUME");

    client_send!(block_begin, "Open a block", "BLOCK BEGIN");

    client_send!(block_end, "End a block", "BLOCK END");

    client_send!(quit, "Close the connection", "QUIT");

    client_send!(
        enable_notification,
        "Enable notification events",
        value as NotificationType,
        "SET self NOTIFICATION {} on"
    );

    client_send!(
        disable_notification,
        "Disable notification events",
        value as NotificationType,
        "SET self NOTIFICATION {} off"
    );

    /// Receive answer from server
    pub fn receive(&mut self, lines: &mut Vec<String>) -> ClientStatus {
        crate::protocol::receive_answer(&mut self.input, Some(lines))
    }

    /// Check status of answer, discard lines.
    pub fn check_status(&mut self, expected_code: ReturnCode) -> ClientResult<&mut Client<S>> {
        crate::protocol::receive_answer(&mut self.input, None).and_then(|status| {
            if status.code == expected_code {
                Ok(self)
            } else {
                Err(ClientError::UnexpectedStatus(status.code))
            }
        })
    }

    /// Receive lines
    pub fn receive_lines(&mut self, expected_code: ReturnCode) -> ClientResult<Vec<String>> {
        let mut lines = Vec::new();
        let status = self.receive(&mut lines)?;
        if status.code == expected_code {
            Ok(lines)
        } else {
            Err(ClientError::UnexpectedStatus(status.code))
        }
    }

    /// Receive a single string
    pub fn receive_string(&mut self, expected_code: ReturnCode) -> ClientResult<String> {
        self.receive_lines(expected_code)
            .and_then(|lines| Client::<S>::parse_single_value(&lines))
    }

    /// Receive integer
    pub fn receive_u8(&mut self, expected_code: ReturnCode) -> ClientResult<u8> {
        self.receive_string(expected_code)
            .and_then(|s| s.parse().map_err(|_| ClientError::InvalidType))
    }

    /// Receive message id
    pub fn receive_message_id(&mut self) -> ClientResult<MessageId> {
        self.receive_string(OK_MESSAGE_QUEUED)
            .and_then(|s| s.parse().map_err(|_| ClientError::InvalidType))
    }

    /// Receive a list of synthesis voices
    pub fn receive_synthesis_voices(&mut self) -> ClientResult<Vec<SynthesisVoice>> {
        self.receive_lines(OK_VOICES_LIST_SENT).and_then(|lines| {
            let mut voices = Vec::new();
            for name in lines.iter() {
                let voice = SynthesisVoice::from_str(name.as_str())?;
                voices.push(voice);
            }
            Ok(voices)
        })
    }

    /// Receive a notification
    pub fn receive_event(&mut self) -> ClientResult<Event> {
        let mut lines = Vec::new();
        crate::protocol::receive_answer(&mut self.input, Some(&mut lines)).and_then(|status| {
            if lines.len() < 2 {
                Err(ClientError::TruncatedMessage)
            } else {
                let message = lines[0].to_owned();
                let client = lines[1].to_owned();
                match status.code {
                    700 => {
                        if lines.len() != 3 {
                            Err(ClientError::TruncatedMessage)
                        } else {
                            let mark = lines[3].to_owned();
                            Ok(Event::index_mark(mark, message, client))
                        }
                    }
                    701 => Ok(Event::begin(message, client)),
                    702 => Ok(Event::end(message, client)),
                    703 => Ok(Event::cancel(message, client)),
                    704 => Ok(Event::pause(message, client)),
                    705 => Ok(Event::resume(message, client)),
                    _ => Err(ClientError::InvalidType),
                }
            }
        })
    }

    /// Check the result of `set_client_name`.
    pub fn check_client_name_set(&mut self) -> ClientResult<&mut Client<S>> {
        self.check_status(OK_CLIENT_NAME_SET)
    }

    /// Register the socket for polling.
    #[cfg(feature = "metal-io")]
    pub fn register(&mut self, poll: &mio::Poll, token: mio::Token) -> ClientResult<()> {
        poll.registry().register(
            &mut self.socket,
            token,
            mio::Interest::READABLE | mio::Interest::WRITABLE,
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[cfg(not(feature = "metal-io"))]
    use std::net::TcpStream;

    #[cfg(feature = "metal-io")]
    use mio::net::TcpStream;

    use super::{Client, ClientError};

    #[test]
    fn parse_single_value() {
        let result = Client::<TcpStream>::parse_single_value(&[String::from("one")]).unwrap();
        assert_eq!("one", result);
        let err_empty = Client::<TcpStream>::parse_single_value(&[]);
        assert!(matches!(err_empty, Err(ClientError::NoLine)));
        let err_too_many =
            Client::<TcpStream>::parse_single_value(&[String::from("one"), String::from("two")]);
        assert!(matches!(err_too_many, Err(ClientError::TooManyLines)));
    }
}
