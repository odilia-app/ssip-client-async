// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021-2022 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.


use crate::constants::*;
use crate::protocol::{
    parse_event_id, parse_single_integer, parse_single_value, parse_typed_lines,
    flush_lines_tokio, write_lines_tokio,
};
use crate::types::*;

macro_rules! send_one_line {
    ($self:expr, $fmt:expr, $( $arg:expr ),+) => {
        flush_lines_tokio(&mut $self.output, &[format!($fmt, $( $arg ),+).as_str()]).await
    };
    ($self:expr, $fmt:expr) => {
        flush_lines_tokio(&mut $self.output, &[$fmt]).await
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

use tokio::io::{AsyncBufRead, AsyncRead, AsyncWrite};

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

/// SSIP client on generic async stream
///
/// There are two ways to send requests and receive responses:
/// * Either with the generic [`AsyncClient::send`] and [`AsyncClient::receive`]
/// * Or with the specific methods such as [`AsyncClient::set_rate`], ..., [`AsyncClient::get_rate`], ...
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
    pub async fn send_line(&mut self, line: &str) -> ClientResult<&mut Self> {
      self.send(Request::SendLine(line.to_string())).await
    }
    /// Receive answer from server
    async fn receive_answer(&mut self, lines: Option<&mut Vec<String>>) -> ClientStatus {
        crate::protocol::receive_answer_tokio(&mut self.input, lines).await
    }
    /// Receive one response.
    pub async fn receive(&mut self) -> ClientResult<Response> {
        const MSG_CURSOR_SET_FIRST: &str = "OK CURSOR SET FIRST";
        let mut lines = Vec::new();
        let status = self.receive_answer(Some(&mut lines)).await?;
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
    /// Send a request
    pub async fn send(&mut self, request: Request) -> ClientResult<&mut Self> {
        match request {
            Request::SetName(client_name) => send_one_line!(
                self,
                "SET self CLIENT_NAME {}:{}:{}",
                client_name.user,
                client_name.application,
                client_name.component
            ),
            Request::Speak => send_one_line!(self, "SPEAK"),
            Request::SendLine(line) => send_one_line!(self, &line).map(|_| ()),
            Request::SendLines(lines) => self.send_lines(&lines).await.map(|_| ()),
            Request::SpeakChar(ch) => send_one_line!(self, "CHAR {}", ch),
            Request::SpeakKey(key) => send_one_line!(self, "KEY {}", key),
            Request::Stop(scope) => send_one_line!(self, "STOP {}", scope),
            Request::Cancel(scope) => send_one_line!(self, "CANCEL {}", scope),
            Request::Pause(scope) => send_one_line!(self, "PAUSE {}", scope),
            Request::Resume(scope) => send_one_line!(self, "RESUME {}", scope),
            Request::SetPriority(prio) => send_one_line!(self, "SET self PRIORITY {}", prio),
            Request::SetDebug(value) => send_toggle!(self, "SET all DEBUG {}", value),
            Request::SetOutputModule(scope, value) => {
                send_one_line!(self, "SET {} OUTPUT_MODULE {}", scope, value)
            }
            Request::GetOutputModule => send_one_line!(self, "GET OUTPUT_MODULE"),
            Request::ListOutputModules => send_one_line!(self, "LIST OUTPUT_MODULES"),
            Request::SetLanguage(scope, lang) => {
                send_one_line!(self, "SET {} LANGUAGE {}", scope, lang)
            }
            Request::GetLanguage => send_one_line!(self, "GET LANGUAGE"),
            Request::SetSsmlMode(value) => send_toggle!(self, "SET self SSML_MODE {}", value),
            Request::SetPunctuationMode(scope, mode) => {
                send_one_line!(self, "SET {} PUNCTUATION {}", scope, mode)
            }
            Request::SetSpelling(scope, value) => {
                send_toggle!(self, "SET {} SPELLING {}", scope, value)
            }
            Request::SetCapitalLettersRecognitionMode(scope, mode) => {
                send_one_line!(self, "SET {} CAP_LET_RECOGN {}", scope, mode)
            }
            Request::SetVoiceType(scope, value) => {
                send_one_line!(self, "SET {} VOICE_TYPE {}", scope, value)
            }
            Request::GetVoiceType => send_one_line!(self, "GET VOICE_TYPE"),
            Request::ListVoiceTypes => send_one_line!(self, "LIST VOICES"),
            Request::SetSynthesisVoice(scope, value) => {
                send_one_line!(self, "SET {} SYNTHESIS_VOICE {}", scope, value)
            }
            Request::ListSynthesisVoices => send_one_line!(self, "LIST SYNTHESIS_VOICES"),
            Request::SetRate(scope, value) => send_range!(self, "SET {} RATE {}", scope, value),
            Request::GetRate => send_one_line!(self, "GET RATE"),
            Request::SetPitch(scope, value) => send_range!(self, "SET {} PITCH {}", scope, value),
            Request::GetPitch => send_one_line!(self, "GET PITCH"),
            Request::SetVolume(scope, value) => {
                send_range!(self, "SET {} VOLUME {}", scope, value)
            }
            Request::GetVolume => send_one_line!(self, "GET VOLUME"),
            Request::SetPauseContext(scope, value) => {
                send_one_line!(self, "SET {} PAUSE_CONTEXT {}", scope, value)
            }
            Request::SetHistory(scope, value) => {
                send_toggle!(self, "SET {} HISTORY {}", scope, value)
            }
            Request::SetNotification(ntype, value) => {
                send_toggle!(self, "SET self NOTIFICATION {} {}", ntype, value)
            }
            Request::Begin => send_one_line!(self, "BLOCK BEGIN"),
            Request::End => send_one_line!(self, "BLOCK END"),
            Request::HistoryGetClients => send_one_line!(self, "HISTORY GET CLIENT_LIST"),
            Request::HistoryGetClientId => send_one_line!(self, "HISTORY GET CLIENT_ID"),
            Request::HistoryGetClientMsgs(scope, start, number) => send_one_line!(
                self,
                "HISTORY GET CLIENT_MESSAGES {} {}_{}",
                scope,
                start,
                number
            ),
            Request::HistoryGetLastMsgId => send_one_line!(self, "HISTORY GET LAST"),
            Request::HistoryGetMsg(id) => send_one_line!(self, "HISTORY GET MESSAGE {}", id),
            Request::HistoryCursorGet => send_one_line!(self, "HISTORY CURSOR GET"),
            Request::HistoryCursorSet(scope, pos) => {
                send_one_line!(self, "HISTORY CURSOR SET {} {}", scope, pos)
            }
            Request::HistoryCursorMove(direction) => {
                send_one_line!(self, "HISTORY CURSOR {}", direction)
            }
            Request::HistorySpeak(id) => send_one_line!(self, "HISTORY SAY {}", id),
            Request::HistorySort(direction, key) => {
                send_one_line!(self, "HISTORY SORT {} {}", direction, key)
            }
            Request::HistorySetShortMsgLength(length) => {
                send_one_line!(self, "HISTORY SET SHORT_MESSAGE_LENGTH {}", length)
            }
            Request::HistorySetMsgTypeOrdering(ordering) => {
                send_one_line!(
                    self,
                    "HISTORY SET MESSAGE_TYPE_ORDERING \"{}\"",
                    ordering
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            Request::HistorySearch(scope, condition) => {
                send_one_line!(self, "HISTORY SEARCH {} \"{}\"", scope, condition)
            }
            Request::Quit => send_one_line!(self, "QUIT"),
        }?;
        Ok(self)
    }

    /// Set the client name. It must be the first call on startup.
    pub async fn set_client_name(&mut self, client_name: ClientName) -> ClientResult<&mut Self> {
        self.send(Request::SetName(client_name)).await
    }

    /// Initiate communitation to send text to speak
    pub async fn speak(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::Speak).await
    }

    /// Speak a char
    pub async fn speak_char(&mut self, ch: char) -> ClientResult<&mut Self> {
        self.send(Request::SpeakChar(ch)).await
    }

    /// Speak a symbolic key name
    pub async fn speak_key(&mut self, key_name: KeyName) -> ClientResult<&mut Self> {
        self.send(Request::SpeakKey(key_name)).await
    }

    /// Stop current message
    pub async fn stop(&mut self, scope: MessageScope) -> ClientResult<&mut Self> {
        self.send(Request::Stop(scope)).await
    }

    /// Cancel current message
    pub async fn cancel(&mut self, scope: MessageScope) -> ClientResult<&mut Self> {
        self.send(Request::Cancel(scope)).await
    }

    /// Pause current message
    pub async fn pause(&mut self, scope: MessageScope) -> ClientResult<&mut Self> {
        self.send(Request::Pause(scope)).await
    }

    /// Resume current message
    pub async fn resume(&mut self, scope: MessageScope) -> ClientResult<&mut Self> {
        self.send(Request::Resume(scope)).await
    }

    /// Set message priority
    pub async fn set_priority(&mut self, prio: Priority) -> ClientResult<&mut Self> {
        self.send(Request::SetPriority(prio)).await
    }

    /// Set debug mode. Return the log location
    pub async fn set_debug(&mut self, value: bool) -> ClientResult<&mut Self> {
        self.send(Request::SetDebug(value)).await
    }

    /// Set output module
    pub async fn set_output_module(
        &mut self,
        scope: ClientScope,
        value: &str,
    ) -> ClientResult<&mut Self> {
        self.send(Request::SetOutputModule(scope, value.to_string())).await
    }

    /// Get the current output module
    pub async fn get_output_module(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::GetOutputModule).await
    }

    /// List the available output modules
    pub async fn list_output_modules(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::ListOutputModules).await
    }

    /// Set language code
    pub async fn set_language(&mut self, scope: ClientScope, value: &str) -> ClientResult<&mut Self> {
        self.send(Request::SetLanguage(scope, value.to_string())).await
    }

    /// Get the current language
    pub async fn get_language(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::GetLanguage).await
    }

    /// Set SSML mode (Speech Synthesis Markup Language)
    pub async fn set_ssml_mode(&mut self, mode: bool) -> ClientResult<&mut Self> {
        self.send(Request::SetSsmlMode(mode)).await
    }

    /// Set punctuation mode
    pub async fn set_punctuation_mode(
        &mut self,
        scope: ClientScope,
        mode: PunctuationMode,
    ) -> ClientResult<&mut Self> {
        self.send(Request::SetPunctuationMode(scope, mode)).await
    }

    /// Set spelling on or off
    pub async fn set_spelling(&mut self, scope: ClientScope, value: bool) -> ClientResult<&mut Self> {
        self.send(Request::SetSpelling(scope, value)).await
    }

    /// Set capital letters recognition mode
    pub async fn set_capital_letter_recogn(
        &mut self,
        scope: ClientScope,
        mode: CapitalLettersRecognitionMode,
    ) -> ClientResult<&mut Self> {
        self.send(Request::SetCapitalLettersRecognitionMode(scope, mode)).await
    }

    /// Set the voice type (MALE1, FEMALE1, …)
    pub async fn set_voice_type(&mut self, scope: ClientScope, value: &str) -> ClientResult<&mut Self> {
        self.send(Request::SetVoiceType(scope, value.to_string())).await
    }

    /// Get the current pre-defined voice
    pub async fn get_voice_type(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::GetVoiceType).await
    }

    /// List the available symbolic voice names
    pub async fn list_voice_types(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::ListVoiceTypes).await
    }

    /// Set the voice
    pub async fn set_synthesis_voice(
        &mut self,
        scope: ClientScope,
        value: &str,
    ) -> ClientResult<&mut Self> {
        self.send(Request::SetSynthesisVoice(scope, value.to_string())).await
    }

    /// Lists the available voices for the current synthesizer
    pub async fn list_synthesis_voices(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::ListSynthesisVoices).await
    }

    /// Set the rate of speech. n is an integer value within the range from -100 to 100, lower values meaning slower speech.
    pub async fn set_rate(&mut self, scope: ClientScope, value: i8) -> ClientResult<&mut Self> {
        self.send(Request::SetRate(scope, value)).await
    }

    /// Get the current rate of speech.
    pub async fn get_rate(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::GetRate).await
    }

    /// Set the pitch of speech. n is an integer value within the range from -100 to 100.
    pub async fn set_pitch(&mut self, scope: ClientScope, value: i8) -> ClientResult<&mut Self> {
        self.send(Request::SetPitch(scope, value)).await
    }

    /// Get the current pitch value.
    pub async fn get_pitch(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::GetPitch).await
    }

    /// Set the volume of speech. n is an integer value within the range from -100 to 100.
    pub async fn set_volume(&mut self, scope: ClientScope, value: i8) -> ClientResult<&mut Self> {
        self.send(Request::SetVolume(scope, value)).await
    }

    /// Get the current volume.
    pub async fn get_volume(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::GetVolume).await
    }

    /// Set the number of (more or less) sentences that should be repeated after a previously paused text is resumed.
    pub async fn set_pause_context(&mut self, scope: ClientScope, value: u32) -> ClientResult<&mut Self> {
        self.send(Request::SetPauseContext(scope, value)).await
    }

    /// Enable notification events
    pub async fn set_notification(
        &mut self,
        ntype: NotificationType,
        value: bool,
    ) -> ClientResult<&mut Self> {
        self.send(Request::SetNotification(ntype, value)).await
    }

    /// Open a block
    pub async fn block_begin(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::Begin).await
    }

    /// End a block
    pub async fn block_end(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::End).await
    }

    /// Enable or disable history of received messages.
    pub async fn set_history(&mut self, scope: ClientScope, value: bool) -> ClientResult<&mut Self> {
        self.send(Request::SetHistory(scope, value)).await
    }

    /// Get clients in history.
    pub async fn history_get_clients(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::HistoryGetClients).await
    }

    /// Get client id in the history.
    pub async fn history_get_client_id(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::HistoryGetClientId).await
    }

    /// Get last message said.
    pub async fn history_get_last(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::HistoryGetLastMsgId).await
    }

    /// Get a range of client messages.
    pub async fn history_get_client_messages(
        &mut self,
        scope: ClientScope,
        start: u32,
        number: u32,
    ) -> ClientResult<&mut Self> {
        self.send(Request::HistoryGetClientMsgs(scope, start, number)).await
    }

    /// Get the id of the last message sent by the client.
    pub async fn history_get_last_message_id(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::HistoryGetLastMsgId).await
    }

    /// Return the text of an history message.
    pub async fn history_get_message(&mut self, msg_id: MessageId) -> ClientResult<&mut Self> {
        self.send(Request::HistoryGetMsg(msg_id)).await
    }

    /// Get the id of the message the history cursor is pointing to.
    pub async fn history_get_cursor(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::HistoryCursorGet).await
    }

    /// Set the history cursor position.
    pub async fn history_set_cursor(
        &mut self,
        scope: ClientScope,
        pos: HistoryPosition,
    ) -> ClientResult<&mut Self> {
        self.send(Request::HistoryCursorSet(scope, pos)).await
    }

    /// Move the cursor position backward or forward.
    pub async fn history_move_cursor(&mut self, direction: CursorDirection) -> ClientResult<&mut Self> {
        self.send(Request::HistoryCursorMove(direction)).await
    }

    /// Speak the message from history.
    pub async fn history_speak(&mut self, msg_id: MessageId) -> ClientResult<&mut Self> {
        self.send(Request::HistorySpeak(msg_id)).await
    }

    /// Sort messages in history.
    pub async fn history_sort(
        &mut self,
        direction: SortDirection,
        key: SortKey,
    ) -> ClientResult<&mut Self> {
        self.send(Request::HistorySort(direction, key)).await
    }

    /// Set the maximum length of short versions of history messages.
    pub async fn history_set_short_message_length(&mut self, length: u32) -> ClientResult<&mut Self> {
        self.send(Request::HistorySetShortMsgLength(length)).await
    }

    /// Set the ordering of the message types, from the minimum to the maximum.
    pub async fn history_set_ordering(&mut self, ordering: Vec<Ordering>) -> ClientResult<&mut Self> {
        self.send(Request::HistorySetMsgTypeOrdering(ordering)).await
    }

    /// Search in message history.
    pub async fn history_search(
        &mut self,
        scope: ClientScope,
        condition: &str,
    ) -> ClientResult<&mut Self> {
        self.send(Request::HistorySearch(scope, condition.to_string())).await
    }

    /// Close the connection
    pub async fn quit(&mut self) -> ClientResult<&mut Self> {
        self.send(Request::Quit).await
    }

    /// Check status of answer, discard lines.
    pub async fn check_status(&mut self, expected_code: ReturnCode) -> ClientResult<&mut Self> {
        self.receive_answer(None).await.and_then(|status| {
            if status.code == expected_code {
                Ok(self)
            } else {
                Err(ClientError::UnexpectedStatus(status.code))
            }
        })
    }

    /// Receive lines
    pub async fn receive_lines(&mut self, expected_code: ReturnCode) -> ClientResult<Vec<String>> {
        let mut lines = Vec::new();
        let status = self.receive_answer(Some(&mut lines)).await?;
        if status.code == expected_code {
            Ok(lines)
        } else {
            Err(ClientError::UnexpectedStatus(status.code))
        }
    }

    /// Receive a single string
    pub async fn receive_string(&mut self, expected_code: ReturnCode) -> ClientResult<String> {
        self.receive_lines(expected_code)
            .await.and_then(|lines| parse_single_value(&lines))
    }

    /// Receive signed 8-bit integer
    pub async fn receive_i8(&mut self) -> ClientResult<u8> {
        self.receive_string(OK_GET).await.and_then(|s| {
            s.parse()
                .map_err(|_| ClientError::invalid_data("invalid signed integer"))
        })
    }

    /// Receive unsigned 8-bit integer
    pub async fn receive_u8(&mut self) -> ClientResult<u8> {
        self.receive_string(OK_GET).await.and_then(|s| {
            s.parse()
                .map_err(|_| ClientError::invalid_data("invalid unsigned 8-bit integer"))
        })
    }

    /// Receive cursor pos
    pub async fn receive_cursor_pos(&mut self) -> ClientResult<u16> {
        self.receive_string(OK_CUR_POS_RET).await.and_then(|s| {
            s.parse()
                .map_err(|_| ClientError::invalid_data("invalid unsigned 16-bit integer"))
        })
    }

    /// Receive message id
    pub async fn receive_message_id(&mut self) -> ClientResult<MessageId> {
        let mut lines = Vec::new();
        match self.receive_answer(Some(&mut lines)).await?.code {
            OK_MESSAGE_QUEUED | OK_LAST_MSG => Ok(parse_single_integer(&lines)?),
            _ => Err(ClientError::invalid_data("not a message id")),
        }
    }

    /// Receive client id
    pub async fn receive_client_id(&mut self) -> ClientResult<ClientId> {
        self.receive_string(OK_CLIENT_ID_SENT).await.and_then(|s| {
            s.parse()
                .map_err(|_| ClientError::invalid_data("invalid client id"))
        })
    }

    /// Receive a list of synthesis voices
    pub async fn receive_synthesis_voices(&mut self) -> ClientResult<Vec<SynthesisVoice>> {
        self.receive_lines(OK_VOICES_LIST_SENT)
            .await.and_then(|lines| parse_typed_lines::<SynthesisVoice>(&lines))
    }

    /// Receive a notification
    pub async fn receive_event(&mut self) -> ClientResult<Event> {
        let mut lines = Vec::new();
        self.receive_answer(Some(&mut lines)).await.and_then(|status| {
            if lines.len() < 2 {
                Err(ClientError::unexpected_eof("event truncated"))
            } else {
                let message = &lines[0];
                let client = &lines[1];
                match status.code {
                    700 => {
                        if lines.len() != 3 {
                            Err(ClientError::unexpected_eof("index markevent truncated"))
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
                    _ => Err(ClientError::invalid_data("wrong status code for event")),
                }
            }
        })
    }

    /// Receive a list of client status from history.
    pub async fn receive_history_clients(&mut self) -> ClientResult<Vec<HistoryClientStatus>> {
        self.receive_lines(OK_CLIENTS_LIST_SENT)
            .await.and_then(|lines| parse_typed_lines::<HistoryClientStatus>(&lines))
    }

    /// Check the result of `set_client_name`.
    pub async fn check_client_name_set(&mut self) -> ClientResult<&mut Self> {
        self.check_status(OK_CLIENT_NAME_SET).await
    }

    /// Check if server accept data.
    pub async fn check_receiving_data(&mut self) -> ClientResult<&mut Self> {
        self.check_status(OK_RECEIVING_DATA).await
    }
}
