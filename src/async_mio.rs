// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2022 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::VecDeque;
use std::io::{self, Read, Write};

use crate::{
    client::{Client, ClientError, ClientName, ClientResult},
    constants::*,
    types::{EventId, Source, SynthesisVoice},
};

#[derive(Debug)]
/// Request for SSIP server.
pub enum Request {
    SetName(ClientName),
    Speak,
    SendLine(String),
    SendLines(Vec<String>),
    SendChar(char),
    Quit,
}

#[derive(Debug)]
/// Response from SSIP server.
pub enum Response {
    LanguageSet,                         // 201
    PrioritySet,                         // 202
    RateSet,                             // 203
    PitchSet,                            // 204
    PunctuationSet,                      // 205
    CapLetRecognSet,                     // 206
    SpellingSet,                         // 207
    ClientNameSet,                       // 208
    VoiceSet,                            // 209
    Stopped,                             // 210
    Paused,                              // 211
    Resumed,                             // 212
    Canceled,                            // 213
    TableSet,                            // 215
    OutputModuleSet,                     // 216
    PauseContextSet,                     // 217
    VolumeSet,                           // 218
    SsmlModeSet,                         // 219
    NotificationSet,                     // 220
    PitchRangeSet,                       // 263
    DebugSet,                            // 262
    HistoryCurSetFirst,                  // 220
    HistoryCurSetLast,                   // 221
    HistoryCurSetPos,                    // 222
    HistoryCurMoveFor,                   // 223
    HistoryCurMoveBack,                  // 224
    MessageQueued,                       // 225,
    SoundIconQueued,                     // 226
    MessageCanceled,                     // 227
    ReceivingData,                       // 230
    Bye,                                 // 231
    HistoryClientListSent(Vec<String>),  // 240
    HistoryMsgsListSent(Vec<String>),    // 241
    HistoryLastMsg(String),              // 242
    HistoryCurPosRet(String),            // 243
    TableListSent(Vec<String>),          // 244
    HistoryClientIdSent(String),         // 245
    MessageTextSent,                     // 246
    HelpSent(Vec<String>),               // 248
    VoicesListSent(Vec<SynthesisVoice>), // 249
    OutputModulesListSent(Vec<String>),  // 250
    GetString(String),                   // 251
    GetInteger(u8),                      // 251
    InsideBlock,                         // 260
    OutsideBlock,                        // 261
    NotImplemented,                      // 299
    EventIndexMark(EventId, String),     // 700
    EventBegin(EventId),                 // 701
    EventEnd(EventId),                   // 702
    EventCanceled(EventId),              // 703
    EventPaused(EventId),                // 704
    EventResumed(EventId),               // 705
}

const INITIAL_REQUEST_QUEUE_CAPACITY: usize = 4;

/// Asynchronous client based on `mio`.
///
///
pub struct AsyncClient<S: Read + Write + Source> {
    client: Client<S>,
    requests: VecDeque<Request>,
}

impl<S: Read + Write + Source> AsyncClient<S> {
    /// New asynchronous client build on top of a synchronous client.
    pub fn new(client: Client<S>) -> Self {
        Self {
            client,
            requests: VecDeque::with_capacity(INITIAL_REQUEST_QUEUE_CAPACITY),
        }
    }

    /// Convert two lines of the response in an event id
    fn parse_event_id(lines: &[String]) -> ClientResult<EventId> {
        match lines.len() {
            0 | 1 => Err(ClientError::TooFewLines),
            2 => Ok(EventId::new(&lines[0], &lines[1])),
            _ => Err(ClientError::TooManyLines),
        }
    }

    /// Register client
    pub fn register(
        &mut self,
        poll: &mio::Poll,
        input_token: mio::Token,
        output_token: mio::Token,
    ) -> io::Result<()> {
        self.client.register(poll, input_token, output_token)
    }

    /// Push a new request in the queue.
    pub fn push(&mut self, request: Request) {
        self.requests.push_back(request);
    }

    /// Return true if there is a pending request.
    pub fn has_next(&self) -> bool {
        !self.requests.is_empty()
    }

    /// Write one pending request if any.
    ///
    /// Instance of `mio::Poll` generates a writable event only once until the socket returns `WouldBlock`.
    /// This error is mapped to `ClientError::NotReady`.
    pub fn send_next(&mut self) -> ClientResult<()> {
        match self.requests.pop_front() {
            Some(request) => match request {
                Request::SetName(client_name) => self.client.set_client_name(client_name),
                Request::Speak => self.client.speak(),
                Request::SendLine(line) => self.client.send_line(&line),
                Request::SendLines(lines) => self.client.send_lines(
                    lines
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<&str>>()
                        .as_slice(),
                ),
                Request::SendChar(ch) => self.client.send_char(ch),
                Request::Quit => self.client.quit(),
            }
            .map(|_| ()),
            None => Ok(()),
        }
    }

    /// Receive one response.
    ///
    /// Must be called each time a readable event is returned by `mio::Poll`.
    pub fn receive_next(&mut self) -> ClientResult<Response> {
        const MSG_CURSOR_SET_FIRST: &str = "OK CURSOR SET FIRST";
        let mut lines = Vec::new();
        let status = self.client.receive(&mut lines)?;
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
            OK_CLIENT_LIST_SENT => Ok(Response::HistoryClientListSent(lines)),
            OK_MSGS_LIST_SENT => Ok(Response::HistoryMsgsListSent(lines)),
            OK_LAST_MSG => Ok(Response::HistoryLastMsg(Client::<S>::parse_single_value(
                &lines,
            )?)),
            OK_CUR_POS_RET => Ok(Response::HistoryCurPosRet(Client::<S>::parse_single_value(
                &lines,
            )?)),
            OK_TABLE_LIST_SENT => Ok(Response::TableListSent(lines)),
            OK_CLIENT_ID_SENT => Ok(Response::HistoryClientIdSent(
                Client::<S>::parse_single_value(&lines)?,
            )),
            OK_MSG_TEXT_SENT => Ok(Response::MessageTextSent),
            OK_HELP_SENT => Ok(Response::HelpSent(lines)),
            OK_VOICES_LIST_SENT => Ok(Response::VoicesListSent(
                Client::<S>::parse_synthesis_voices(&lines)?,
            )),
            OK_OUTPUT_MODULES_LIST_SENT => Ok(Response::OutputModulesListSent(lines)),
            OK_GET => {
                let sval = Client::<S>::parse_single_value(&lines)?;
                Ok(match sval.parse::<u8>() {
                    Ok(uval) => Response::GetInteger(uval),
                    Err(_) => Response::GetString(sval),
                })
            }
            OK_INSIDE_BLOCK => Ok(Response::InsideBlock),
            OK_OUTSIDE_BLOCK => Ok(Response::OutsideBlock),
            OK_NOT_IMPLEMENTED => Ok(Response::NotImplemented),
            EVENT_INDEX_MARK => {
                if lines.len() == 3 {
                    Ok(Response::EventIndexMark(
                        Self::parse_event_id(&lines)?,
                        lines[2].to_owned(),
                    ))
                } else {
                    Err(ClientError::TooFewLines)
                }
            }
            EVENT_BEGIN => Ok(Response::EventBegin(Self::parse_event_id(&lines)?)),
            EVENT_END => Ok(Response::EventEnd(Self::parse_event_id(&lines)?)),
            EVENT_CANCELED => Ok(Response::EventCanceled(Self::parse_event_id(&lines)?)),
            EVENT_PAUSED => Ok(Response::EventPaused(Self::parse_event_id(&lines)?)),
            EVENT_RESUMED => Ok(Response::EventResumed(Self::parse_event_id(&lines)?)),
            _ => panic!("error should have been caught earlier"),
        }
    }
}
