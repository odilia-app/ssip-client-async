// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use core::error::Error;
use core::fmt;
use std::io;
use std::str::FromStr;

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
            MessageScope::Message(id) => write!(f, "{id}"),
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
            ClientScope::Client(id) => write!(f, "{id}"),
        }
    }
}

/// Priority
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "dbus", derive(zvariant::Type))]
pub enum Priority {
    Progress,
    Notification,
    Message,
    Text,
    Important,
}

/// NOTE: The [`fmt::Display`] implementation is how we format the enum for implementing the SSIP
/// protocol.
///
/// The [`fmt::Display`] impl either:
///
/// 1. Can not be changed,
/// 2. Must implement a different trait.
impl fmt::Display for Priority {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Priority::Progress => fmt.write_str("important"),
            Priority::Notification => fmt.write_str("notification"),
            Priority::Message => fmt.write_str("message"),
            Priority::Text => fmt.write_str("text"),
            Priority::Important => fmt.write_str("message"),
        }
    }
}

/// Punctuation mode.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "dbus", derive(zvariant::Type))]
pub enum PunctuationMode {
    None,
    Some,
    Most,
    All,
}

/// NOTE: the display impl is used to write to SSIP command buffers.
impl fmt::Display for PunctuationMode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            PunctuationMode::None => "none",
            PunctuationMode::Some => "some",
            PunctuationMode::Most => "most",
            PunctuationMode::All => "all",
        };
        fmt.write_str(s)
    }
}

/// Capital letters recognition mode.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "dbus", derive(zvariant::Type))]
pub enum CapitalLettersRecognitionMode {
    None,
    Spell,
    Icon,
}

/// NOTE: The Display implementation is used when constructing SSIP commands.
impl fmt::Display for CapitalLettersRecognitionMode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            CapitalLettersRecognitionMode::None => "none",
            CapitalLettersRecognitionMode::Spell => "spell",
            CapitalLettersRecognitionMode::Icon => "icon",
        };
        fmt.write_str(s)
    }
}

/// Symbolic key names
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "dbus", derive(zvariant::Type))]
pub enum KeyName {
    Space,
    Underscore,
    DoubleQuote,
    Alt,
    Control,
    Hyper,
    Meta,
    Shift,
    Super,
    Backspace,
    Break,
    Delete,
    Down,
    End,
    Enter,
    Escape,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    Home,
    Insert,
    KpMultiply,
    KpPlus,
    KpMinus,
    KpDot,
    KpDivide,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpEnter,
    Left,
    Menu,
    Next,
    NumLock,
    Pause,
    Print,
    Prior,
    Return,
    Right,
    ScrollLock,
    Tab,
    Up,
    Window,
}

/// NOTE: Display impl is used for SSIP command creation.
impl fmt::Display for KeyName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            KeyName::Space => "space",
            KeyName::Underscore => "underscore",
            KeyName::DoubleQuote => "double-quote",
            KeyName::Alt => "alt",
            KeyName::Control => "control",
            KeyName::Hyper => "hyper",
            KeyName::Meta => "meta",
            KeyName::Shift => "shift",
            KeyName::Super => "super",
            KeyName::Backspace => "backspace",
            KeyName::Break => "break",
            KeyName::Delete => "delete",
            KeyName::Down => "down",
            KeyName::End => "end",
            KeyName::Enter => "enter",
            KeyName::Escape => "escape",
            KeyName::F1 => "f1",
            KeyName::F2 => "f2",
            KeyName::F3 => "f3",
            KeyName::F4 => "f4",
            KeyName::F5 => "f5",
            KeyName::F6 => "f6",
            KeyName::F7 => "f7",
            KeyName::F8 => "f8",
            KeyName::F9 => "f9",
            KeyName::F10 => "f10",
            KeyName::F11 => "f11",
            KeyName::F12 => "f12",
            KeyName::F13 => "f13",
            KeyName::F14 => "f14",
            KeyName::F15 => "f15",
            KeyName::F16 => "f16",
            KeyName::F17 => "f17",
            KeyName::F18 => "f18",
            KeyName::F19 => "f19",
            KeyName::F20 => "f20",
            KeyName::F21 => "f21",
            KeyName::F22 => "f22",
            KeyName::F23 => "f23",
            KeyName::F24 => "f24",
            KeyName::Home => "home",
            KeyName::Insert => "insert",
            KeyName::KpMultiply => "kp-*",
            KeyName::KpPlus => "kp-+",
            KeyName::KpMinus => "kp--",
            KeyName::KpDot => "kp-.",
            KeyName::KpDivide => "kp-/",
            KeyName::Kp0 => "kp-0",
            KeyName::Kp1 => "kp-1",
            KeyName::Kp2 => "kp-2",
            KeyName::Kp3 => "kp-3",
            KeyName::Kp4 => "kp-4",
            KeyName::Kp5 => "kp-5",
            KeyName::Kp6 => "kp-6",
            KeyName::Kp7 => "kp-7",
            KeyName::Kp8 => "kp-8",
            KeyName::Kp9 => "kp-9",
            KeyName::KpEnter => "kp-enter",
            KeyName::Left => "left",
            KeyName::Menu => "menu",
            KeyName::Next => "next",
            KeyName::NumLock => "num-lock",
            KeyName::Pause => "pause",
            KeyName::Print => "print",
            KeyName::Prior => "prior",
            KeyName::Return => "return",
            KeyName::Right => "right",
            KeyName::ScrollLock => "scroll-lock",
            KeyName::Tab => "tab",
            KeyName::Up => "up",
            KeyName::Window => "window",
        };
        f.write_str(s)
    }
}

/// Notification type
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "dbus", derive(zvariant::Type))]
pub enum NotificationType {
    Begin,
    End,
    Cancel,
    Pause,
    Resume,
    IndexMark,
    All,
}

impl fmt::Display for NotificationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            NotificationType::Begin => "begin",
            NotificationType::End => "end",
            NotificationType::Cancel => "cancel",
            NotificationType::Pause => "pause",
            NotificationType::Resume => "resume",
            NotificationType::IndexMark => "index_mark",
            NotificationType::All => "all",
        };
        f.write_str(s)
    }
}

/// Notification event type (returned by server)
#[derive(Debug, Clone)]
pub enum EventType {
    Begin,
    End,
    Cancel,
    Pause,
    Resume,
    IndexMark(String),
}

// TODO: very suspicious that this is not correct, even though it _is_ the behaviour of `strum`
impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventType::Begin => f.write_str("begin"),
            EventType::End => f.write_str("end"),
            EventType::Cancel => f.write_str("cancel"),
            EventType::Pause => f.write_str("pause"),
            EventType::Resume => f.write_str("resume"),
            EventType::IndexMark(value) => write!(f, "index_mark({value})"),
        }
    }
}

/// Event identifier
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "dbus", derive(zvariant::Type))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "dbus", derive(zvariant::Type))]
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
#[derive(Debug)]
pub enum ClientError {
    Io(io::Error),
    NotReady,
    Ssip(StatusLine),
    TooFewLines,
    TooManyLines,
    UnexpectedStatus(ReturnCode),
}

impl fmt::Display for ClientError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::Io(ioe) => {
                fmt.write_str("I/O: ")?;
                ioe.fmt(fmt)
            }
            ClientError::NotReady => fmt.write_str("Not ready"),
            ClientError::Ssip(status_line) => {
                fmt.write_str("SSIP: ")?;
                status_line.fmt(fmt)
            }
            ClientError::TooFewLines => fmt.write_str("Too few lines"),
            ClientError::TooManyLines => fmt.write_str("Too many lines"),
            ClientError::UnexpectedStatus(return_code) => {
                fmt.write_str("Unexpected status: ")?;
                return_code.fmt(fmt)
            }
        }
    }
}
impl Error for ClientError {}

impl ClientError {
    /// Create I/O error
    pub fn io_error(kind: io::ErrorKind, msg: &str) -> Self {
        Self::Io(io::Error::new(kind, msg))
    }

    /// Invalid data I/O error
    pub fn invalid_data(msg: &str) -> Self {
        ClientError::io_error(io::ErrorKind::InvalidData, msg)
    }

    /// Unexpected EOF I/O error
    pub fn unexpected_eof(msg: &str) -> Self {
        ClientError::io_error(io::ErrorKind::UnexpectedEof, msg)
    }
}

impl From<io::Error> for ClientError {
    fn from(err: io::Error) -> Self {
        if err.kind() == io::ErrorKind::WouldBlock {
            ClientError::NotReady
        } else {
            ClientError::Io(err)
        }
    }
}

/// Client result.
pub type ClientResult<T> = Result<T, ClientError>;

/// Client result consisting in a single status line
pub type ClientStatus = ClientResult<StatusLine>;

/// Client name
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "dbus", derive(zvariant::Type))]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "dbus", derive(zvariant::Type))]
pub enum CursorDirection {
    Backward,
    Forward,
}

impl fmt::Display for CursorDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CursorDirection::Backward => "backward",
            CursorDirection::Forward => "forward",
        };
        f.write_str(s)
    }
}

/// Sort direction in history
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "dbus", derive(zvariant::Type))]
pub enum SortDirection {
    Ascending,
    Descending,
}

impl fmt::Display for SortDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SortDirection::Ascending => "asc",
            SortDirection::Descending => "desc",
        };
        f.write_str(s)
    }
}

/// Property messages are ordered by in history
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "dbus", derive(zvariant::Type))]
pub enum SortKey {
    ClientName,
    Priority,
    MessageType,
    Time,
    User,
}

impl fmt::Display for SortKey {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            SortKey::ClientName => "client_name",
            SortKey::Priority => "priority",
            SortKey::MessageType => "message_type",
            SortKey::Time => "time",
            SortKey::User => "user",
        };
        fmt.write_str(s)
    }
}

/// Sort ordering
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "dbus", derive(zvariant::Type))]
pub enum Ordering {
    Text,
    SoundIcon,
    Char,
    Key,
}
impl fmt::Display for Ordering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Ordering::Text => "text",
            Ordering::SoundIcon => "sound_icon",
            Ordering::Char => "char",
            Ordering::Key => "key",
        };
        f.write_str(s)
    }
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
            HistoryPosition::Pos(n) => write!(f, "pos {n}"),
        }
    }
}

/// History client status
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "dbus", derive(zvariant::Type))]
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

    use std::io;
    use std::str::FromStr;

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
                Err(ClientError::Io(err)) if err.kind() == io::ErrorKind::InvalidData => (),
                Err(_) => panic!("expecting error 'invalid data' parsing \"{line}\""),
            }
        }
        for line in &["8 joe:speechd_client:main", "8", ""] {
            match HistoryClientStatus::from_str(line) {
                Ok(_) => panic!("parsing should have failed"),
                Err(ClientError::Io(err)) if err.kind() == io::ErrorKind::UnexpectedEof => (),
                Err(_) => panic!("expecting error 'unexpected EOF' parsing \"{line}\""),
            }
        }
    }
}
