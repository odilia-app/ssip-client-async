// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#![no_std]
#![forbid(clippy::std_instead_of_alloc, clippy::alloc_instead_of_core)]

extern crate alloc;
use alloc::{
	string::{String, ToString},
	vec::Vec,
};
use core::{
	fmt,
	str::FromStr,
};

use thiserror::Error as ThisError;

use strum_macros::Display as StrumDisplay;

/// Return code of SSIP commands
pub type ReturnCode = u16;

/// Message identifier
pub type MessageId = u32;

/// Client identifier
pub type ClientId = u32;

/// Message identifiers
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MessageScope {
    /// Last message from current client
    Last,
    /// Messages from all clients
    All,
    /// Specific message
    Message(MessageId),
}

impl fmt::Display for MessageScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MessageScope::Last => write!(f, "self"),
            MessageScope::All => write!(f, "all"),
            MessageScope::Message(id) => write!(f, "{}", id),
        }
    }
}

/// Client identifiers
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum ClientScope {
    /// Current client
    Current,
    /// All clients
    All,
    /// Specific client
    Client(ClientId),
}

impl fmt::Display for ClientScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientScope::Current => write!(f, "self"),
            ClientScope::All => write!(f, "all"),
            ClientScope::Client(id) => write!(f, "{}", id),
        }
    }
}

/// Priority
#[derive(StrumDisplay, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Priority {
    #[strum(serialize = "progress")]
    Progress,
    #[strum(serialize = "notification")]
    Notification,
    #[strum(serialize = "message")]
    Message,
    #[strum(serialize = "text")]
    Text,
    #[strum(serialize = "important")]
    Important,
}

/// Punctuation mode.
#[derive(StrumDisplay, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PunctuationMode {
    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "some")]
    Some,
    #[strum(serialize = "most")]
    Most,
    #[strum(serialize = "all")]
    All,
}

/// Capital letters recognition mode.
#[derive(StrumDisplay, Debug, Clone, Hash, Eq, PartialEq)]
pub enum CapitalLettersRecognitionMode {
    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "spell")]
    Spell,
    #[strum(serialize = "icon")]
    Icon,
}

/// Symbolic key names
#[derive(StrumDisplay, Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyName {
    #[strum(serialize = "space")]
    Space,
    #[strum(serialize = "underscore")]
    Underscore,
    #[strum(serialize = "double-quote")]
    DoubleQuote,
    #[strum(serialize = "alt")]
    Alt,
    #[strum(serialize = "control")]
    Control,
    #[strum(serialize = "hyper")]
    Hyper,
    #[strum(serialize = "meta")]
    Meta,
    #[strum(serialize = "shift")]
    Shift,
    #[strum(serialize = "super")]
    Super,
    #[strum(serialize = "backspace")]
    Backspace,
    #[strum(serialize = "break")]
    Break,
    #[strum(serialize = "delete")]
    Delete,
    #[strum(serialize = "down")]
    Down,
    #[strum(serialize = "end")]
    End,
    #[strum(serialize = "enter")]
    Enter,
    #[strum(serialize = "escape")]
    Escape,
    #[strum(serialize = "f1")]
    F1,
    #[strum(serialize = "f2")]
    F2,
    #[strum(serialize = "f3")]
    F3,
    #[strum(serialize = "f4")]
    F4,
    #[strum(serialize = "f5")]
    F5,
    #[strum(serialize = "f6")]
    F6,
    #[strum(serialize = "f7")]
    F7,
    #[strum(serialize = "f8")]
    F8,
    #[strum(serialize = "f9")]
    F9,
    #[strum(serialize = "f10")]
    F10,
    #[strum(serialize = "f11")]
    F11,
    #[strum(serialize = "f12")]
    F12,
    #[strum(serialize = "f13")]
    F13,
    #[strum(serialize = "f14")]
    F14,
    #[strum(serialize = "f15")]
    F15,
    #[strum(serialize = "f16")]
    F16,
    #[strum(serialize = "f17")]
    F17,
    #[strum(serialize = "f18")]
    F18,
    #[strum(serialize = "f19")]
    F19,
    #[strum(serialize = "f20")]
    F20,
    #[strum(serialize = "f21")]
    F21,
    #[strum(serialize = "f22")]
    F22,
    #[strum(serialize = "f23")]
    F23,
    #[strum(serialize = "f24")]
    F24,
    #[strum(serialize = "home")]
    Home,
    #[strum(serialize = "insert")]
    Insert,
    #[strum(serialize = "kp-*")]
    KpMultiply,
    #[strum(serialize = "kp-+")]
    KpPlus,
    #[strum(serialize = "kp--")]
    KpMinus,
    #[strum(serialize = "kp-.")]
    KpDot,
    #[strum(serialize = "kp-/")]
    KpDivide,
    #[strum(serialize = "kp-0")]
    Kp0,
    #[strum(serialize = "kp-1")]
    Kp1,
    #[strum(serialize = "kp-2")]
    Kp2,
    #[strum(serialize = "kp-3")]
    Kp3,
    #[strum(serialize = "kp-4")]
    Kp4,
    #[strum(serialize = "kp-5")]
    Kp5,
    #[strum(serialize = "kp-6")]
    Kp6,
    #[strum(serialize = "kp-7")]
    Kp7,
    #[strum(serialize = "kp-8")]
    Kp8,
    #[strum(serialize = "kp-9")]
    Kp9,
    #[strum(serialize = "kp-enter")]
    KpEnter,
    #[strum(serialize = "left")]
    Left,
    #[strum(serialize = "menu")]
    Menu,
    #[strum(serialize = "next")]
    Next,
    #[strum(serialize = "num-lock")]
    NumLock,
    #[strum(serialize = "pause")]
    Pause,
    #[strum(serialize = "print")]
    Print,
    #[strum(serialize = "prior")]
    Prior,
    #[strum(serialize = "return")]
    Return,
    #[strum(serialize = "right")]
    Right,
    #[strum(serialize = "scroll-lock")]
    ScrollLock,
    #[strum(serialize = "tab")]
    Tab,
    #[strum(serialize = "up")]
    Up,
    #[strum(serialize = "window")]
    Window,
}

/// Notification type
#[derive(StrumDisplay, Debug, Clone, Hash, Eq, PartialEq)]
pub enum NotificationType {
    #[strum(serialize = "begin")]
    Begin,
    #[strum(serialize = "end")]
    End,
    #[strum(serialize = "cancel")]
    Cancel,
    #[strum(serialize = "pause")]
    Pause,
    #[strum(serialize = "resume")]
    Resume,
    #[strum(serialize = "index_mark")]
    IndexMark,
    #[strum(serialize = "all")]
    All,
}

/// Notification event type (returned by server)
#[derive(StrumDisplay, Debug, Clone)]
pub enum EventType {
    Begin,
    End,
    Cancel,
    Pause,
    Resume,
    IndexMark(String),
}

/// Event identifier
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct EventId {
    // Message id
    pub message: String,
    // Client id
    pub client: String,
}

impl EventId {
    // New event identifier
    pub fn new(message: &str, client: &str) -> Self {
        Self {
            message: message.to_string(),
            client: client.to_string(),
        }
    }
}

/// Notification event
#[derive(Debug, Clone)]
pub struct Event {
    pub ntype: EventType,
    pub id: EventId,
}

impl Event {
    pub fn new(ntype: EventType, message: &str, client: &str) -> Event {
        Event {
            ntype,
            id: EventId::new(message, client),
        }
    }

    pub fn begin(message: &str, client: &str) -> Event {
        Event::new(EventType::Begin, message, client)
    }

    pub fn end(message: &str, client: &str) -> Event {
        Event::new(EventType::End, message, client)
    }

    pub fn index_mark(mark: String, message: &str, client: &str) -> Event {
        Event::new(EventType::IndexMark(mark), message, client)
    }

    pub fn cancel(message: &str, client: &str) -> Event {
        Event::new(EventType::Cancel, message, client)
    }

    pub fn pause(message: &str, client: &str) -> Event {
        Event::new(EventType::Pause, message, client)
    }

    pub fn resume(message: &str, client: &str) -> Event {
        Event::new(EventType::Resume, message, client)
    }
}

/// Synthesis voice
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SynthesisVoice {
    pub name: String,
    pub language: Option<String>,
    pub dialect: Option<String>,
}

impl SynthesisVoice {
    pub fn new(name: &str, language: Option<&str>, dialect: Option<&str>) -> SynthesisVoice {
        SynthesisVoice {
            name: name.to_string(),
            language: language.map(|s| s.to_string()),
            dialect: dialect.map(|s| s.to_string()),
        }
    }
    /// Parse Option::None or string "none" into Option::None
    fn parse_none(token: Option<&str>) -> Option<String> {
        match token {
            Some(s) => match s {
                "none" => None,
                s => Some(s.to_string()),
            },
            None => None,
        }
    }
}

impl FromStr for SynthesisVoice {
    type Err = ClientError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('\t');
        match iter.next() {
            Some(name) => Ok(SynthesisVoice {
                name: name.to_string(),
                language: SynthesisVoice::parse_none(iter.next()),
                dialect: SynthesisVoice::parse_none(iter.next()),
            }),
            None => Err(ClientError::unexpected_eof("missing synthesis voice name")),
        }
    }
}

/// Command status line
///
/// Consists in a 3-digits code and a message. It can be a success or a failure.
///
/// Examples:
/// - 216 OK OUTPUT MODULE SET
/// - 409 ERR RATE TOO HIGH
#[derive(Debug, PartialEq, Eq)]
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
    #[error("Not ready")]
    NotReady,
    #[error("SSIP: {0}")]
    Ssip(StatusLine),
    #[error("Too few lines")]
    TooFewLines,
    #[error("Too many lines")]
    TooManyLines,
    #[error("Unexpected status: {0}")]
    UnexpectedStatus(ReturnCode),
		#[error("Unexpected EOF: {0}")]
		UnexpectedEof(&'static str),
		#[error("Invalid data: {0}")]
		InvalidData(&'static str),
}

impl ClientError {
    /// Invalid data I/O error
    pub fn invalid_data(msg: &'static str) -> Self {
        ClientError::InvalidData(msg)
    }

    /// Unexpected EOF I/O error
    pub fn unexpected_eof(msg: &'static str) -> Self {
        ClientError::UnexpectedEof(msg)
    }
}

/// Client result.
pub type ClientResult<T> = Result<T, ClientError>;

/// Client result consisting in a single status line
pub type ClientStatus = ClientResult<StatusLine>;

/// Client name
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

/// Cursor motion in history
#[derive(StrumDisplay, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CursorDirection {
    #[strum(serialize = "backward")]
    Backward,
    #[strum(serialize = "forward")]
    Forward,
}

/// Sort direction in history
#[derive(StrumDisplay, Debug, Clone, Eq, PartialEq, Hash)]
pub enum SortDirection {
    #[strum(serialize = "asc")]
    Ascending,
    #[strum(serialize = "desc")]
    Descending,
}

/// Property messages are ordered by in history
#[derive(StrumDisplay, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SortKey {
    #[strum(serialize = "client_name")]
    ClientName,
    #[strum(serialize = "priority")]
    Priority,
    #[strum(serialize = "message_type")]
    MessageType,
    #[strum(serialize = "time")]
    Time,
    #[strum(serialize = "user")]
    User,
}

/// Sort ordering
#[derive(StrumDisplay, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ordering {
    #[strum(serialize = "text")]
    Text,
    #[strum(serialize = "sound_icon")]
    SoundIcon,
    #[strum(serialize = "char")]
    Char,
    #[strum(serialize = "key")]
    Key,
}

/// Position in history
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum HistoryPosition {
    First,
    Last,
    Pos(u16),
}

impl fmt::Display for HistoryPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HistoryPosition::First => write!(f, "first"),
            HistoryPosition::Last => write!(f, "last"),
            HistoryPosition::Pos(n) => write!(f, "pos {}", n),
        }
    }
}

/// History client status
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct HistoryClientStatus {
    pub id: ClientId,
    pub name: String,
    pub connected: bool,
}

impl HistoryClientStatus {
    pub fn new(id: ClientId, name: &str, connected: bool) -> Self {
        Self {
            id,
            name: name.to_string(),
            connected,
        }
    }
}

impl FromStr for HistoryClientStatus {
    type Err = ClientError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.splitn(3, ' ');
        match iter.next() {
            Some("") => Err(ClientError::unexpected_eof("expecting client id")),
            Some(client_id) => match client_id.parse::<u32>() {
                Ok(id) => match iter.next() {
                    Some(name) => match iter.next() {
                        Some("0") => Ok(HistoryClientStatus::new(id, name, false)),
                        Some("1") => Ok(HistoryClientStatus::new(id, name, true)),
                        Some(_) => Err(ClientError::invalid_data("invalid client status")),
                        None => Err(ClientError::unexpected_eof("expecting client status")),
                    },
                    None => Err(ClientError::unexpected_eof("expecting client name")),
                },
                Err(_) => Err(ClientError::invalid_data("invalid client id")),
            },
            None => Err(ClientError::unexpected_eof("expecting client id")),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
/// Request for SSIP server.
pub enum Request {
    SetName(ClientName),
    // Speech related requests
    Speak,
    SendLine(String),
    SendLines(Vec<String>),
    SpeakChar(char),
    SpeakKey(KeyName),
    // Flow control
    Stop(MessageScope),
    Cancel(MessageScope),
    Pause(MessageScope),
    Resume(MessageScope),
    // Setter and getter
    SetPriority(Priority),
    SetDebug(bool),
    SetOutputModule(ClientScope, String),
    GetOutputModule,
    ListOutputModules,
    SetLanguage(ClientScope, String),
    GetLanguage,
    SetSsmlMode(bool),
    SetPunctuationMode(ClientScope, PunctuationMode),
    SetSpelling(ClientScope, bool),
    SetCapitalLettersRecognitionMode(ClientScope, CapitalLettersRecognitionMode),
    SetVoiceType(ClientScope, String),
    GetVoiceType,
    ListVoiceTypes,
    SetSynthesisVoice(ClientScope, String),
    ListSynthesisVoices,
    SetRate(ClientScope, i8),
    GetRate,
    SetPitch(ClientScope, i8),
    GetPitch,
    SetVolume(ClientScope, i8),
    GetVolume,
    SetPauseContext(ClientScope, u32),
    SetNotification(NotificationType, bool),
    // Blocks
    Begin,
    End,
    // History
    SetHistory(ClientScope, bool),
    HistoryGetClients,
    HistoryGetClientId,
    HistoryGetClientMsgs(ClientScope, u32, u32),
    HistoryGetLastMsgId,
    HistoryGetMsg(MessageId),
    HistoryCursorGet,
    HistoryCursorSet(ClientScope, HistoryPosition),
    HistoryCursorMove(CursorDirection),
    HistorySpeak(MessageId),
    HistorySort(SortDirection, SortKey),
    HistorySetShortMsgLength(u32),
    HistorySetMsgTypeOrdering(Vec<Ordering>),
    HistorySearch(ClientScope, String),
    // Misc.
    Quit,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
/// Response from SSIP server.
pub enum Response {
    LanguageSet,                                     // 201
    PrioritySet,                                     // 202
    RateSet,                                         // 203
    PitchSet,                                        // 204
    PunctuationSet,                                  // 205
    CapLetRecognSet,                                 // 206
    SpellingSet,                                     // 207
    ClientNameSet,                                   // 208
    VoiceSet,                                        // 209
    Stopped,                                         // 210
    Paused,                                          // 211
    Resumed,                                         // 212
    Canceled,                                        // 213
    TableSet,                                        // 215
    OutputModuleSet,                                 // 216
    PauseContextSet,                                 // 217
    VolumeSet,                                       // 218
    SsmlModeSet,                                     // 219
    NotificationSet,                                 // 220
    PitchRangeSet,                                   // 263
    DebugSet,                                        // 262
    HistoryCurSetFirst,                              // 220
    HistoryCurSetLast,                               // 221
    HistoryCurSetPos,                                // 222
    HistoryCurMoveFor,                               // 223
    HistoryCurMoveBack,                              // 224
    MessageQueued,                                   // 225,
    SoundIconQueued,                                 // 226
    MessageCanceled,                                 // 227
    ReceivingData,                                   // 230
    Bye,                                             // 231
    HistoryClientListSent(Vec<HistoryClientStatus>), // 240
    HistoryMsgsListSent(Vec<String>),                // 241
    HistoryLastMsg(String),                          // 242
    HistoryCurPosRet(String),                        // 243
    TableListSent(Vec<String>),                      // 244
    HistoryClientIdSent(ClientId),                   // 245
    MessageTextSent,                                 // 246
    HelpSent(Vec<String>),                           // 248
    VoicesListSent(Vec<SynthesisVoice>),             // 249
    OutputModulesListSent(Vec<String>),              // 250
    Get(String),                                     // 251
    InsideBlock,                                     // 260
    OutsideBlock,                                    // 261
    NotImplemented,                                  // 299
    EventIndexMark(EventId, String),                 // 700
    EventBegin(EventId),                             // 701
    EventEnd(EventId),                               // 702
    EventCanceled(EventId),                          // 703
    EventPaused(EventId),                            // 704
    EventResumed(EventId),                           // 705
}

#[cfg(test)]
mod tests {

		use alloc::format;
		use core::str::FromStr;

    use super::{ClientError, HistoryClientStatus, HistoryPosition, MessageScope, SynthesisVoice};

    #[test]
    fn parse_synthesis_voice() {
        // Voice with dialect
        let v1 =
            SynthesisVoice::from_str("Portuguese (Portugal)+Kaukovalta\tpt\tKaukovalta").unwrap();
        assert_eq!("Portuguese (Portugal)+Kaukovalta", v1.name);
        assert_eq!("pt", v1.language.unwrap());
        assert_eq!("Kaukovalta", v1.dialect.unwrap());

        // Voice without dialect
        let v2 = SynthesisVoice::from_str("Esperanto\teo\tnone").unwrap();
        assert_eq!("Esperanto", v2.name);
        assert_eq!("eo", v2.language.unwrap());
        assert!(v2.dialect.is_none());
    }

    #[test]
    fn format_message_scope() {
        assert_eq!("self", format!("{}", MessageScope::Last).as_str());
        assert_eq!("all", format!("{}", MessageScope::All).as_str());
        assert_eq!("123", format!("{}", MessageScope::Message(123)).as_str());
    }

    #[test]
    fn format_history_position() {
        assert_eq!("first", format!("{}", HistoryPosition::First).as_str());
        assert_eq!("last", format!("{}", HistoryPosition::Last).as_str());
        assert_eq!("pos 15", format!("{}", HistoryPosition::Pos(15)).as_str());
    }

    #[test]
    fn parse_history_client_status() {
        assert_eq!(
            HistoryClientStatus::new(10, "joe:speechd_client:main", false),
            HistoryClientStatus::from_str("10 joe:speechd_client:main 0").unwrap()
        );
        assert_eq!(
            HistoryClientStatus::new(11, "joe:speechd_client:main", true),
            HistoryClientStatus::from_str("11 joe:speechd_client:main 1").unwrap()
        );
        for line in &[
            "9 joe:speechd_client:main xxx",
            "xxx joe:speechd_client:main 1",
        ] {
            match HistoryClientStatus::from_str(line) {
                Ok(_) => panic!("parsing should have failed"),
                Err(ClientError::InvalidData(_)) => (),
                Err(_) => panic!("expecting error 'invalid data' parsing \"{}\"", line),
            }
        }
        for line in &["8 joe:speechd_client:main", "8", ""] {
            match HistoryClientStatus::from_str(line) {
                Ok(_) => panic!("parsing should have failed"),
                Err(ClientError::UnexpectedEof(_)) => (),
                Err(_) => panic!("expecting error 'unexpected EOF' parsing \"{}\"", line),
            }
        }
    }
}
