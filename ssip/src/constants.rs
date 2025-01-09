// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use crate::ReturnCode;

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
pub const OK_CLIENTS_LIST_SENT: ReturnCode = 240;

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

/// Event: INDEX MARK
pub const EVENT_INDEX_MARK: ReturnCode = 700;

/// Event: BEGIN
pub const EVENT_BEGIN: ReturnCode = 701;

/// Event: END
pub const EVENT_END: ReturnCode = 702;

/// Event: CANCELED
pub const EVENT_CANCELED: ReturnCode = 703;

/// Event: PAUSED
pub const EVENT_PAUSED: ReturnCode = 704;

/// Event: RESUMED
pub const EVENT_RESUMED: ReturnCode = 705;
