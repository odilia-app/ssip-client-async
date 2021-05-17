// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::str::FromStr;
use strum_macros::Display as StrumDisplay;

/// Return code of SSIP commands
pub type ReturnCode = u16;

/// Successful completion: OK LANGUAGE SET
pub const OK_LANGUAGE_SET: ReturnCode = 201;

/// Successful completion: OK PRIORITY SET
pub const OK_PRIORITY_SET: ReturnCode = 202;

/// Successful completion: OK RATE SET
pub const OK_RATE_SET: ReturnCode = 203;

/// Successful completion: OK PITCH SET
pub const OK_PITCH_SET: ReturnCode = 204;

/// Successful completion: OK PUNCTUATION SET
pub const OK_PUNCTUATION_SET: ReturnCode = 205;

/// Successful completion: OK CAP LET RECOGNITION SET
pub const OK_CAP_LET_RECOGN_SET: ReturnCode = 206;

/// Successful completion: OK SPELLING SET
pub const OK_SPELLING_SET: ReturnCode = 207;

/// Successful completion: OK CLIENT NAME SET
pub const OK_CLIENT_NAME_SET: ReturnCode = 208;

/// Successful completion: OK VOICE SET
pub const OK_VOICE_SET: ReturnCode = 209;

/// Successful completion: OK STOPPED
pub const OK_STOPPED: ReturnCode = 210;

/// Successful completion: OK PAUSED
pub const OK_PAUSED: ReturnCode = 211;

/// Successful completion: OK RESUMED
pub const OK_RESUMED: ReturnCode = 212;

/// Successful completion: OK CANCELED
pub const OK_CANCELED: ReturnCode = 213;

/// Successful completion: OK TABLE SET
pub const OK_TABLE_SET: ReturnCode = 215;

/// Successful completion: OK OUTPUT MODULE SET
pub const OK_OUTPUT_MODULE_SET: ReturnCode = 216;

/// Successful completion: OK PAUSE CONTEXT SET
pub const OK_PAUSE_CONTEXT_SET: ReturnCode = 217;

/// Successful completion: OK VOLUME SET
pub const OK_VOLUME_SET: ReturnCode = 218;

/// Successful completion: OK SSML MODE SET
pub const OK_SSML_MODE_SET: ReturnCode = 219;

/// Successful completion: OK NOTIFICATION SET
pub const OK_NOTIFICATION_SET: ReturnCode = 220;

/// Successful completion: OK CURSOR SET FIRST
pub const OK_CUR_SET_FIRST: ReturnCode = 220;

/// Successful completion: OK CURSOR SET LAST
pub const OK_CUR_SET_LAST: ReturnCode = 221;

/// Successful completion: OK CURSOR SET TO POSITION
pub const OK_CUR_SET_POS: ReturnCode = 222;

/// Successful completion: OK CURSOR MOVED FORWARD
pub const OK_CUR_MOV_FOR: ReturnCode = 223;

/// Successful completion: OK CURSOR MOVED BACKWARD
pub const OK_CUR_MOV_BACK: ReturnCode = 224;

/// Successful completion: OK MESSAGE QUEUED
pub const OK_MESSAGE_QUEUED: ReturnCode = 225;

/// Successful completion: OK SOUND ICON QUEUED
pub const OK_SND_ICON_QUEUED: ReturnCode = 226;

/// Successful completion: OK MESSAGE CANCELED
pub const OK_MSG_CANCELED: ReturnCode = 227;

/// Successful completion: OK RECEIVING DATA
pub const OK_RECEIVING_DATA: ReturnCode = 230;

// Successful completion: HAPPY HACKING
pub const OK_BYE: ReturnCode = 231;

/// Successful completion: OK CLIENTS LIST SENT
pub const OK_CLIENT_LIST_SENT: ReturnCode = 240;

/// Successful completion: OK MSGS LIST SENT
pub const OK_MSGS_LIST_SENT: ReturnCode = 241;

/// Successful completion: OK LAST MSG SAID
pub const OK_LAST_MSG: ReturnCode = 242;

/// Successful completion: OK CURSOR POSITION RETURNED
pub const OK_CUR_POS_RET: ReturnCode = 243;

/// Successful completion: OK TABLE LIST SEND
pub const OK_TABLE_LIST_SENT: ReturnCode = 244;

/// Successful completion: OK CLIENT ID SENT
pub const OK_CLIENT_ID_SENT: ReturnCode = 245;

/// Successful completion: OK MESSAGE TEXT SENT
pub const OK_MSG_TEXT_SENT: ReturnCode = 246;

/// Successful completion: OK HELP SENT
pub const OK_HELP_SENT: ReturnCode = 248;

/// Successful completion: OK VOICE LIST SENT
pub const OK_VOICES_LIST_SENT: ReturnCode = 249;

/// Successful completion: OK MODULE LIST SENT
pub const OK_OUTPUT_MODULES_LIST_SENT: ReturnCode = 250;

/// Successful completion: OK GET RETURNED
pub const OK_GET: ReturnCode = 251;

/// Successful completion: OK INSIDE BLOCK
pub const OK_INSIDE_BLOCK: ReturnCode = 260;

/// Successful completion: OK OUTSIDE BLOCK
pub const OK_OUTSIDE_BLOCK: ReturnCode = 261;

/// Successful completion: OK DEBUGGING SET
pub const OK_DEBUG_SET: ReturnCode = 262;

/// Successful completion: OK PITCH RANGE SET
pub const OK_PITCH_RANGE_SET: ReturnCode = 263;

/// Successful completion: OK BUT NOT IMPLEMENTED -- DOES NOTHING
pub const OK_NOT_IMPLEMENTED: ReturnCode = 299;

/// Server error: ERR INTERNAL
pub const ERR_INTERNAL: ReturnCode = 300;

/// Server error: ERR COULDNT SET PRIORITY
pub const ERR_COULDNT_SET_PRIORITY: ReturnCode = 301;

/// Server error: ERR COULDNT SET LANGUAGE
pub const ERR_COULDNT_SET_LANGUAGE: ReturnCode = 302;

/// Server error: ERR COULDNT SET RATE
pub const ERR_COULDNT_SET_RATE: ReturnCode = 303;

/// Server error: ERR COULDNT SET PITCH
pub const ERR_COULDNT_SET_PITCH: ReturnCode = 304;

/// Server error: ERR COULDNT SET PUNCT MODE
pub const ERR_COULDNT_SET_PUNCTUATION: ReturnCode = 305;

/// Server error: ERR COULDNT SET CAP LET RECOGNITION
pub const ERR_COULDNT_SET_CAP_LET_RECOG: ReturnCode = 306;

/// Server error: ERR COULDNT SET SPELLING
pub const ERR_COULDNT_SET_SPELLING: ReturnCode = 308;

/// Server error: ERR COULDNT SET VOICE
pub const ERR_COULDNT_SET_VOICE: ReturnCode = 309;

/// Server error: ERR COULDNT SET TABLE
pub const ERR_COULDNT_SET_TABLE: ReturnCode = 310;

/// Server error: ERR COULDNT SET CLIENT_NAME
pub const ERR_COULDNT_SET_CLIENT_NAME: ReturnCode = 311;

/// Server error: ERR COULDNT SET OUTPUT MODULE
pub const ERR_COULDNT_SET_OUTPUT_MODULE: ReturnCode = 312;

/// Server error: ERR COULDNT SET PAUSE CONTEXT
pub const ERR_COULDNT_SET_PAUSE_CONTEXT: ReturnCode = 313;

/// Server error: ERR COULDNT SET VOLUME
pub const ERR_COULDNT_SET_VOLUME: ReturnCode = 314;

/// Server error: ERR COULDNT SET SSML MODE
pub const ERR_COULDNT_SET_SSML_MODE: ReturnCode = 315;

/// Server error: ERR COULDNT SET NOTIFICATION
pub const ERR_COULDNT_SET_NOTIFICATION: ReturnCode = 316;

/// Server error: ERR COULDNT SET DEBUGGING
pub const ERR_COULDNT_SET_DEBUG: ReturnCode = 317;

/// Server error: ERR NO SOUND ICONS
pub const ERR_NO_SND_ICONS: ReturnCode = 320;

/// Server error: ERR MODULE CANT REPORT VOICES
pub const ERR_CANT_REPORT_VOICES: ReturnCode = 321;

/// Server error: ERR NO OUTPUT MODULE LOADED
pub const ERR_NO_OUTPUT_MODULE: ReturnCode = 321;

/// Server error: ERR ALREADY INSIDE BLOCK
pub const ERR_ALREADY_INSIDE_BLOCK: ReturnCode = 330;

/// Server error: ERR ALREADY OUTSIDE BLOCK
pub const ERR_ALREADY_OUTSIDE_BLOCK: ReturnCode = 331;

/// Server error: ERR NOT ALLOWED INSIDE BLOCK
pub const ERR_NOT_ALLOWED_INSIDE_BLOCK: ReturnCode = 332;

/// Server error: ERR COULDNT SET PITCH RANGE
pub const ERR_COULDNT_SET_PITCH_RANGE: ReturnCode = 340;

/// Server error: ERR NOT YET IMPLEMENTED
pub const ERR_NOT_IMPLEMENTED: ReturnCode = 380;

/// Client error: ERR NO CLIENT
pub const ERR_NO_CLIENT: ReturnCode = 401;

/// Client error: ERR NO SUCH CLIENT
pub const ERR_NO_SUCH_CLIENT: ReturnCode = 402;

/// Client error: ERR NO MESSAGE
pub const ERR_NO_MESSAGE: ReturnCode = 403;

/// Client error: ERR POSITION TOO LOW
pub const ERR_POS_LOW: ReturnCode = 404;

/// Client error: ERR POSITION TOO HIGH
pub const ERR_POS_HIGH: ReturnCode = 405;

/// Client error: ERR ID DOESNT EXIST
pub const ERR_ID_NOT_EXIST: ReturnCode = 406;

/// Client error: ERR UNKNOWN ICON
pub const ERR_UNKNOWN_ICON: ReturnCode = 407;

/// Client error: ERR UNKNOWN PRIORITY
pub const ERR_UNKNOWN_PRIORITY: ReturnCode = 408;

/// Client error: ERR RATE TOO HIGH
pub const ERR_RATE_TOO_HIGH: ReturnCode = 409;

/// Client error: ERR RATE TOO LOW
pub const ERR_RATE_TOO_LOW: ReturnCode = 410;

/// Client error: ERR PITCH TOO HIGH
pub const ERR_PITCH_TOO_HIGH: ReturnCode = 411;

/// Client error: ERR PITCH TOO LOW
pub const ERR_PITCH_TOO_LOW: ReturnCode = 412;

/// Client error: ERR VOLUME TOO HIGH
pub const ERR_VOLUME_TOO_HIGH: ReturnCode = 413;

/// Client error: ERR VOLUME TOO LOW
pub const ERR_VOLUME_TOO_LOW: ReturnCode = 414;

/// Client error: ERR PITCH RANGE TOO HIGH
pub const ERR_PITCH_RANGE_TOO_HIGH: ReturnCode = 415;

/// Client error: ERR PITCH RANGE TOO LOW
pub const ERR_PITCH_RANGE_TOO_LOW: ReturnCode = 416;

/// Client error: ERR INVALID COMMAND
pub const ERR_INVALID_COMMAND: ReturnCode = 500;

/// Client error: ERR INVALID ENCODING
pub const ERR_INVALID_ENCODING: ReturnCode = 501;

/// Client error: ERR MISSING PARAMETER
pub const ERR_MISSING_PARAMETER: ReturnCode = 510;

/// Client error: ERR PARAMETER NOT A NUMBER
pub const ERR_NOT_A_NUMBER: ReturnCode = 511;

/// Client error: ERR PARAMETER NOT A STRING
pub const ERR_NOT_A_STRING: ReturnCode = 512;

/// Client error: ERR PARAMETER NOT ON OR OFF
pub const ERR_PARAMETER_NOT_ON_OFF: ReturnCode = 513;

/// Client error: ERR PARAMETER INVALID
pub const ERR_PARAMETER_INVALID: ReturnCode = 514;

/// Message identifier
pub type MessageId = String;

/// Client identifier
pub type ClientId = String;

/// Message identifiers
#[derive(Debug)]
pub enum MessageTarget {
    /// Last message from current client
    Last,
    /// Messages from all clients
    All,
    /// Specific message
    Message(MessageId),
}

/// Client identifiers
#[derive(Debug)]
pub enum ClientTarget {
    /// Current client
    Current,
    /// All clients
    All,
    /// Specific message
    Message(MessageId),
}

/// Priority
#[derive(StrumDisplay, Debug)]
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
#[derive(StrumDisplay, Debug)]
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
#[derive(StrumDisplay, Debug)]
pub enum CapitalLettersRecognitionMode {
    #[strum(serialize = "none")]
    None,
    #[strum(serialize = "spell")]
    Spell,
    #[strum(serialize = "icon")]
    Icon,
}

/// Symbolic key names
#[derive(StrumDisplay, Debug)]
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

/// Synthesis voice
pub struct SynthesisVoice {
    pub name: String,
    pub language: Option<String>,
    pub dialect: Option<String>,
}

impl SynthesisVoice {
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
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_whitespace();
        Ok(SynthesisVoice {
            name: String::from(iter.next().unwrap()),
            language: SynthesisVoice::parse_none(iter.next()),
            dialect: SynthesisVoice::parse_none(iter.next()),
        })
    }
}
