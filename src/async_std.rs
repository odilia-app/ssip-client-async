// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021-2022 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::io::{self, Read, Write};

use crate::constants::*;
use crate::protocol::{
    flush_lines, parse_event_id, parse_single_integer, parse_single_value, parse_typed_lines,
    flush_lines_async_std, write_lines_async_std,
};
use crate::types::*;

use async_std::io::{AsyncBufRead, AsyncWrite};

/// Convert boolean to ON or OFF
fn on_off(value: bool) -> &'static str {
    if value {
        "on"
    } else {
        "off"
    }
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
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

macro_rules! send_one_line {
    ($self:expr, $fmt:expr, $( $arg:expr ),+) => {
        flush_lines(&mut $self.output, &[format!($fmt, $( $arg ),+).as_str()])
    };
    ($self:expr, $fmt:expr) => {
        flush_lines(&mut $self.output, &[$fmt])
    }
}

macro_rules! send_toggle {
    ($output:expr, $fmt:expr, $val:expr) => {
        send_one_line!($output, $fmt, on_off($val))
    };
    ($output:expr, $fmt:expr, $arg:expr, $val:expr) => {
        send_one_line!($output, $fmt, $arg, on_off($val))
    };
}

macro_rules! send_range {
    ($output:expr, $fmt:expr, $scope:expr, $val:expr) => {
        send_one_line!(
            $output,
            $fmt,
            $scope,
            std::cmp::max(-100, std::cmp::min(100, $val))
        )
    };
}

/// SSIP client on generic async stream
///
/// There are two ways to send requests and receive responses:
/// * Either with the generic [`Client::send`] and [`Client::receive`]
/// * Or with the specific methods such as [`Client::set_rate`], ..., [`Client::get_rate`], ...
pub struct AsyncClient<R: AsyncBufRead + Unpin, W: AsyncWrite + Unpin> {
    input: R,
    output: W,
}
impl<R: AsyncBufRead + Unpin, W: AsyncWrite + Unpin> AsyncClient<R, W> {
    pub(crate) fn new(input: R, output: W) -> Self {
        Self { input, output }
    }
    /// Send lines of text (terminated by a single dot).
    pub async fn send_lines(&mut self, lines: &[String]) -> ClientResult<&mut Self> {
        const END_OF_DATA: [&str; 1] = ["."];
        write_lines_tokio(
            &mut self.output,
            lines
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        ).await?;
        flush_lines_tokio(&mut self.output, &END_OF_DATA).await?;
        Ok(self)
    }
    /// Receive answer from server
    async fn receive_answer(&mut self, lines: &mut Vec<String>) -> ClientStatus {
        crate::protocol::receive_answer_tokio(&mut self.input, Some(lines)).await
    }
    /// Receive one response.
    pub async fn receive(&mut self) -> ClientResult<Response> {
        const MSG_CURSOR_SET_FIRST: &str = "OK CURSOR SET FIRST";
        let mut lines = Vec::new();
        let status = self.receive_answer(&mut lines).await?;
        match status.code {
            OK_LANGUAGE_SET => Ok(Response::LanguageSet),
            OK_PRIORITY_SET => Ok(Response::PrioritySet),
            OK_RATE_SET => Ok(Response::RateSet),
            OK_PITCH_SET => Ok(Response::PitchSet),
            OK_PUNCTUATION_SET => Ok(Response::PunctuationSet),
            OK_CAP_LET_RECOGN_SET => Ok(Response::CapLetRecognSet),
            OK_SPELLING_SET => Ok(Response::SpellingSet),
            OK_CLIENT_NAME_SET => Ok(Response::ClientNameSet),
            OK_VOICE_SET => Ok(Response::VoiceSet),
            OK_STOPPED => Ok(Response::Stopped),
            OK_PAUSED => Ok(Response::Paused),
            OK_RESUMED => Ok(Response::Resumed),
            OK_CANCELED => Ok(Response::Canceled),
            OK_TABLE_SET => Ok(Response::TableSet),
            OK_OUTPUT_MODULE_SET => Ok(Response::OutputModuleSet),
            OK_PAUSE_CONTEXT_SET => Ok(Response::PauseContextSet),
            OK_VOLUME_SET => Ok(Response::VolumeSet),
            OK_SSML_MODE_SET => Ok(Response::SsmlModeSet),
            // Warning OK_CUR_SET_FIRST == OK_NOTIFICATION_SET == 220. Matching message to make the difference
            OK_NOTIFICATION_SET => {
                if status.message == MSG_CURSOR_SET_FIRST {
                    //OK_CUR_SET_FIRST => Ok(Response::HistoryCurSetFirst)
                    Ok(Response::HistoryCurSetFirst)
                } else {
                    Ok(Response::NotificationSet)
                }
            }
            OK_CUR_SET_LAST => Ok(Response::HistoryCurSetLast),
            OK_CUR_SET_POS => Ok(Response::HistoryCurSetPos),
            OK_PITCH_RANGE_SET => Ok(Response::PitchRangeSet),
            OK_DEBUG_SET => Ok(Response::DebugSet),
            OK_CUR_MOV_FOR => Ok(Response::HistoryCurMoveFor),
            OK_CUR_MOV_BACK => Ok(Response::HistoryCurMoveBack),
            OK_MESSAGE_QUEUED => Ok(Response::MessageQueued),
            OK_SND_ICON_QUEUED => Ok(Response::SoundIconQueued),
            OK_MSG_CANCELED => Ok(Response::MessageCanceled),
            OK_RECEIVING_DATA => Ok(Response::ReceivingData),
            OK_BYE => Ok(Response::Bye),
            OK_CLIENTS_LIST_SENT => Ok(Response::HistoryClientListSent(parse_typed_lines::<
                HistoryClientStatus,
            >(&lines)?)),
            OK_MSGS_LIST_SENT => Ok(Response::HistoryMsgsListSent(lines)),
            OK_LAST_MSG => Ok(Response::HistoryLastMsg(parse_single_value(&lines)?)),
            OK_CUR_POS_RET => Ok(Response::HistoryCurPosRet(parse_single_value(&lines)?)),
            OK_TABLE_LIST_SENT => Ok(Response::TableListSent(lines)),
            OK_CLIENT_ID_SENT => Ok(Response::HistoryClientIdSent(parse_single_integer(&lines)?)),
            OK_MSG_TEXT_SENT => Ok(Response::MessageTextSent),
            OK_HELP_SENT => Ok(Response::HelpSent(lines)),
            OK_VOICES_LIST_SENT => Ok(Response::VoicesListSent(
                parse_typed_lines::<SynthesisVoice>(&lines)?,
            )),
            OK_OUTPUT_MODULES_LIST_SENT => Ok(Response::OutputModulesListSent(lines)),
            OK_GET => Ok(Response::Get(parse_single_value(&lines)?)),
            OK_INSIDE_BLOCK => Ok(Response::InsideBlock),
            OK_OUTSIDE_BLOCK => Ok(Response::OutsideBlock),
            OK_NOT_IMPLEMENTED => Ok(Response::NotImplemented),
            EVENT_INDEX_MARK => match lines.len() {
                0 | 1 | 2 => Err(ClientError::TooFewLines),
                3 => Ok(Response::EventIndexMark(
                    parse_event_id(&lines)?,
                    lines[2].to_owned(),
                )),
                _ => Err(ClientError::TooManyLines),
            },
            EVENT_BEGIN => Ok(Response::EventBegin(parse_event_id(&lines)?)),
            EVENT_END => Ok(Response::EventEnd(parse_event_id(&lines)?)),
            EVENT_CANCELED => Ok(Response::EventCanceled(parse_event_id(&lines)?)),
            EVENT_PAUSED => Ok(Response::EventPaused(parse_event_id(&lines)?)),
            EVENT_RESUMED => Ok(Response::EventResumed(parse_event_id(&lines)?)),
            _ => panic!("error should have been caught earlier"),
        }
    }
}

