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

use crate::constants::OK_RECEIVING_DATA;
use crate::types::{
    CapitalLettersRecognitionMode, ClientTarget, KeyName, MessageId, MessageTarget, Priority,
    PunctuationMode, StatusLine, SynthesisVoice,
};

/// Client error, either I/O error or SSIP error.
#[derive(ThisError, Debug)]
pub enum ClientError {
    #[error("I/O: {0}")]
    Io(io::Error),
    #[error("SSIP: {0}")]
    Ssip(StatusLine),
    #[error("No line in result")]
    NoLine,
    #[error("Too many lines")]
    TooManyLines,
    #[error("Invalid type")]
    InvalidType,
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

macro_rules! client_setter {
    ($name:ident, $doc:expr, $target_name:ident, $value_name:ident as $value_type:ty, $fmt:expr, $value:expr) => {
        #[doc=$doc]
        pub fn $name(
            &mut self,
            $target_name: ClientTarget,
            $value_name: $value_type,
        ) -> ClientStatus {
            let line = match $target_name {
                ClientTarget::Current => format!($fmt, "self", $value),
                ClientTarget::All => format!($fmt, "all", $value),
                ClientTarget::Client(id) => format!($fmt, id, $value),
            };
            send_lines!(&mut self.input, &mut self.output, &[line.as_str()])
        }
    };
    ($name:ident, $doc:expr, $target_name:ident, $value_name:ident as $value_type:ty, $fmt:expr) => {
        client_setter!(
            $name,
            $doc,
            $target_name,
            $value_name as $value_type,
            $fmt,
            $value_name
        );
    };
    ($name:ident, $doc:expr, $value_name:ident as $value_type:ty, $fmt:expr, $value:expr) => {
        #[doc=$doc]
        pub fn $name(&mut self, $value_name: $value_type) -> ClientStatus {
            send_line!(&mut self.input, &mut self.output, $fmt, $value)
        }
    };
    ($name:ident, $doc:expr, $value_name:ident as $value_type:ty, $fmt:expr) => {
        client_setter!($name, $doc, $value_name as $value_type, $fmt, $value_name);
    };
    ($name:ident, $doc:expr, $line:expr) => {
        #[doc=$doc]
        pub fn $name(&mut self) -> ClientStatus {
            send_line!(&mut self.input, &mut self.output, $line)
        }
    };
}

macro_rules! client_boolean_setter {
    ($name:ident, $doc:expr, $target_name:ident, $value_name:ident, $fmt:expr) => {
        client_setter!(
            $name,
            $doc,
            $target_name,
            $value_name as bool,
            $fmt,
            if $value_name { "ON" } else { "OFF" }
        );
    };
    ($name:ident, $doc:expr, $value_name:ident, $fmt:expr) => {
        client_setter!(
            $name,
            $doc,
            $value_name as bool,
            $fmt,
            if $value_name { "ON" } else { "OFF" }
        );
    };
}

macro_rules! client_range_setter {
    ($name:ident, $doc:expr, $target_name:ident, $value_name:ident, $fmt:expr) => {
        client_setter!(
            $name,
            $doc,
            $target_name,
            $value_name as i8,
            $fmt,
            std::cmp::max(-100, std::cmp::min(100, $value_name))
        );
    };
}

macro_rules! client_getter {
    ($name:ident, $doc:expr, $line:expr) => {
        #[doc=$doc]
        pub fn $name(&mut self) -> ClientResult<Vec<String>> {
            let mut result = Vec::new();
            send_lines!(&mut self.input, &mut self.output, &[&$line], &mut result)?;
            Ok(result)
        }
    };
}

macro_rules! client_single_getter {
    ($name:ident, $doc:expr, $value_type:ty, $line:expr) => {
        #[doc=$doc]
        pub fn $name(&mut self) -> ClientResult<$value_type> {
            let mut lines = Vec::new();
            send_lines!(&mut self.input, &mut self.output, &[&$line], &mut lines)?;
            let result = Client::<S>::parse_single_value(&lines)?
                .parse()
                .map_err(|_| ClientError::InvalidType)?;
            Ok(result)
        }
    };
    ($name:ident, $doc:expr, $line:expr) => {
        #[doc=$doc]
        pub fn $name(&mut self) -> ClientResult<String> {
            let mut lines = Vec::new();
            send_lines!(&mut self.input, &mut self.output, &[&$line], &mut lines)?;
            Client::<S>::parse_single_value(&lines)
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

    fn parse_single_value(lines: &[String]) -> ClientResult<String> {
        match lines.len() {
            0 => Err(ClientError::NoLine),
            1 => Ok(lines[0].to_string()),
            _ => Err(ClientError::TooManyLines),
        }
    }

    /// Send text to server
    pub fn say_text(&mut self, lines: &[&str]) -> ClientResult<MessageId> {
        let status = send_line!(&mut self.input, &mut self.output, "SPEAK")?;
        if status.code == OK_RECEIVING_DATA {
            const END_OF_DATA: [&str; 1] = ["."];
            crate::protocol::write_lines(&mut self.output, lines)?;
            let mut answer = Vec::new();
            send_lines!(&mut self.input, &mut self.output, &END_OF_DATA, &mut answer)?;
            Client::<S>::parse_single_value(&answer)
        } else {
            Err(ClientError::Ssip(status))
        }
    }

    /// Send a single line to the server
    pub fn say_line(&mut self, line: &str) -> ClientResult<MessageId> {
        let lines: [&str; 1] = [line];
        self.say_text(&lines)
    }

    /// Send a char to the server
    pub fn say_char(&mut self, ch: char) -> ClientResult<MessageId> {
        let line = format!("CHAR {}", ch);
        let mut answer = Vec::new();
        send_lines!(&mut self.input, &mut self.output, &[&line], &mut answer)?;
        Client::<S>::parse_single_value(&answer)
    }

    /// Send a symbolic key name
    pub fn say_key_name(&mut self, keyname: KeyName) -> ClientResult<MessageId> {
        let line = format!("KEY {}", keyname);
        let mut answer = Vec::new();
        send_lines!(&mut self.input, &mut self.output, &[&line], &mut answer)?;
        Client::<S>::parse_single_value(&answer)
    }

    /// Action on a message or a group of messages
    fn send_message_command(&mut self, command: &str, target: MessageTarget) -> ClientStatus {
        let line = match target {
            MessageTarget::Last => format!("{} self", command),
            MessageTarget::All => format!("{} all", command),
            MessageTarget::Message(id) => format!("{} {}", command, id),
        };
        send_lines!(&mut self.input, &mut self.output, &[line.as_str()])
    }

    /// Stop current message
    pub fn stop(&mut self, target: MessageTarget) -> ClientStatus {
        self.send_message_command("STOP", target)
    }

    /// Cancel current message
    pub fn cancel(&mut self, target: MessageTarget) -> ClientStatus {
        self.send_message_command("CANCEL", target)
    }

    /// Pause current message
    pub fn pause(&mut self, target: MessageTarget) -> ClientStatus {
        self.send_message_command("PAUSE", target)
    }

    /// Resume current message
    pub fn resume(&mut self, target: MessageTarget) -> ClientStatus {
        self.send_message_command("RESUME", target)
    }

    client_setter!(
        set_priority,
        "Set message priority",
        priority as Priority,
        "SET self PRIORITY {}"
    );

    client_boolean_setter!(set_debug, "Set debug mode", value, "SET all DEBUG {}");

    client_setter!(
        set_output_module,
        "Set output module",
        target,
        value as &str,
        "SET {} OUTPUT_MODULE {}"
    );

    client_single_getter!(
        get_output_module,
        "Get the current output module",
        "GET OUTPUT_MODULE"
    );

    client_getter!(
        list_output_module,
        "List the available output modules",
        "LIST OUTPUT_MODULES"
    );

    client_setter!(
        set_language,
        "Set language code",
        target,
        value as &str,
        "SET {} LANGUAGE {}"
    );

    client_boolean_setter!(
        set_ssml_mode,
        "Set SSML mode (Speech Synthesis Markup Language)",
        value,
        "SET self SSML_MODE {}"
    );

    client_setter!(
        set_punctuation_mode,
        "Set punctuation mode",
        target,
        value as PunctuationMode,
        "SET {} PUNCTUATION {}"
    );

    client_boolean_setter!(
        set_spelling,
        "Set spelling on or off",
        target,
        value,
        "SET {} SPELLING {}"
    );

    client_setter!(
        set_capital_letter_recogn,
        "Set capital letters recognition mode",
        target,
        value as CapitalLettersRecognitionMode,
        "SET {} CAP_LET_RECOGN {}"
    );

    client_setter!(
        set_voice_type,
        "Set the voice type (MALE1, FEMALE1, …)",
        target,
        value as &str,
        "SET {} VOICE_TYPE {}"
    );

    client_getter!(
        get_voice_type,
        "Get the current pre-defined voice",
        "GET VOICE_TYPE"
    );

    client_getter!(
        list_voices,
        "List the available symbolic voice names",
        "LIST VOICES"
    );

    client_setter!(
        set_synthesis_voice,
        "Set the voice",
        target,
        value as &str,
        "SET {} SYNTHESIS_VOICE {}"
    );

    /// Lists the available voices for the current synthesizer.
    pub fn list_synthesis_voice(&mut self) -> ClientResult<Vec<SynthesisVoice>> {
        let mut result = Vec::new();
        send_lines!(
            &mut self.input,
            &mut self.output,
            &["LIST SYNTHESIS_VOICES"],
            &mut result
        )?;
        let mut voices = Vec::new();
        for name in result.iter() {
            let voice = SynthesisVoice::from_str(name.as_str())?;
            voices.push(voice);
        }
        Ok(voices)
    }

    client_range_setter!(
        set_rate,
        "Set the rate of speech. n is an integer value within the range from -100 to 100, lower values meaning slower speech.",
        target,
        value,
        "SET {} RATE {}"
    );

    client_single_getter!(get_rate, "Get the current rate of speech.", u8, "GET RATE");

    client_range_setter!(
        set_pitch,
        "Set the pitch of speech. n is an integer value within the range from -100 to 100.",
        target,
        value,
        "SET {} PITCH {}"
    );

    client_single_getter!(get_pitch, "Get the current pitch value.", u8, "GET PITCH");

    client_range_setter!(
        set_volume,
        "Set the volume of speech. n is an integer value within the range from -100 to 100.",
        target,
        value,
        "SET {} VOLUME {}"
    );

    client_setter!(
        set_pause_context,
        "Set the number of (more or less) sentences that should be repeated after a previously paused text is resumed.",
        target,
        value as u8,
        "SET {} PAUSE_CONTEXT {}"
    );

    client_boolean_setter!(
        set_history,
        "Enable or disable history of received messages.",
        target,
        value,
        "SET {} HISTORY {}"
    );

    client_single_getter!(get_volume, "Get the current volume.", u8, "GET VOLUME");

    client_setter!(block_begin, "Open a block", "BLOCK BEGIN");

    client_setter!(block_end, "End a block", "BLOCK END");

    client_setter!(quit, "Close the connection", "QUIT");
}
