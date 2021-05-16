// ssip-client -- Speech Dispatcher client in Rust
// Copyright (c) 2021 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

pub type ReturnCode = u16;

pub const OK_LANGUAGE_SET: ReturnCode = 201; // OK LANGUAGE SET
pub const OK_PRIORITY_SET: ReturnCode = 202; // OK PRIORITY SET
pub const OK_RATE_SET: ReturnCode = 203; // OK RATE SET
pub const OK_PITCH_SET: ReturnCode = 204; // OK PITCH SET
pub const OK_PUNCTUATION_SET: ReturnCode = 205; // OK PUNCTUATION SET
pub const OK_CAP_LET_RECOGN_SET: ReturnCode = 206; // OK CAP LET RECOGNITION SET
pub const OK_SPELLING_SET: ReturnCode = 207; // OK SPELLING SET
pub const OK_CLIENT_NAME_SET: ReturnCode = 208; // OK CLIENT NAME SET
pub const OK_VOICE_TYPE_SET: ReturnCode = 209; // OK VOICE SET
pub const OK_SYNTHESIS_VOICE_SET: ReturnCode = 209; // OK VOICE SET
pub const OK_STOPPED: ReturnCode = 210; // OK STOPPED
pub const OK_PAUSED: ReturnCode = 211; // OK PAUSED
pub const OK_RESUMED: ReturnCode = 212; // OK RESUMED
pub const OK_CANCELED: ReturnCode = 213; // OK CANCELED
pub const OK_TABLE_SET: ReturnCode = 215; // OK TABLE SET
pub const OK_OUTPUT_MODULE_SET: ReturnCode = 216; // OK OUTPUT MODULE SET
pub const OK_PAUSE_CONTEXT_SET: ReturnCode = 217; // OK PAUSE CONTEXT SET
pub const OK_VOLUME_SET: ReturnCode = 218; // OK VOLUME SET
pub const OK_SSML_MODE_SET: ReturnCode = 219; // OK SSML MODE SET
pub const OK_NOTIFICATION_SET: ReturnCode = 220; // OK NOTIFICATION SET
pub const OK_CUR_SET_FIRST: ReturnCode = 220; // OK CURSOR SET FIRST
pub const OK_CUR_SET_LAST: ReturnCode = 221; // OK CURSOR SET LAST
pub const OK_CUR_SET_POS: ReturnCode = 222; // OK CURSOR SET TO POSITION
pub const OK_CUR_MOV_FOR: ReturnCode = 223; // OK CURSOR MOVED FORWARD
pub const OK_CUR_MOV_BACK: ReturnCode = 224; // OK CURSOR MOVED BACKWARD
pub const OK_MESSAGE_QUEUED: ReturnCode = 225; // OK MESSAGE QUEUED
pub const OK_SND_ICON_QUEUED: ReturnCode = 226; // OK SOUND ICON QUEUED
pub const OK_MSG_CANCELED: ReturnCode = 227; // OK MESSAGE CANCELED
pub const OK_RECEIVING_DATA: ReturnCode = 230; // OK RECEIVING DATA
pub const OK_BYE: ReturnCode = 231; // HAPPY HACKING
pub const OK_CLIENT_LIST_SENT: ReturnCode = 240; // OK CLIENTS LIST SENT
pub const OK_MSGS_LIST_SENT: ReturnCode = 241; // OK MSGS LIST SENT
pub const OK_LAST_MSG: ReturnCode = 242; // OK LAST MSG SAID
pub const OK_CUR_POS_RET: ReturnCode = 243; // OK CURSOR POSITION RETURNED
pub const OK_TABLE_LIST_SENT: ReturnCode = 244; // OK TABLE LIST SEND
pub const OK_CLIENT_ID_SENT: ReturnCode = 245; // OK CLIENT ID SENT
pub const OK_MSG_TEXT_SENT: ReturnCode = 246; // OK MESSAGE TEXT SENT
pub const OK_HELP_SENT: ReturnCode = 248; // OK HELP SENT
pub const OK_VOICES_LIST_SENT: ReturnCode = 249; // OK VOICE LIST SENT
pub const OK_OUTPUT_MODULES_LIST_SENT: ReturnCode = 250; // OK MODULE LIST SENT
pub const OK_GET: ReturnCode = 251; // OK GET RETURNED
pub const OK_INSIDE_BLOCK: ReturnCode = 260; // OK INSIDE BLOCK
pub const OK_OUTSIDE_BLOCK: ReturnCode = 261; // OK OUTSIDE BLOCK
pub const OK_DEBUG_SET: ReturnCode = 262; // OK DEBUGGING SET
pub const OK_PITCH_RANGE_SET: ReturnCode = 263; // OK PITCH RANGE SET
pub const OK_NOT_IMPLEMENTED: ReturnCode = 299; // OK BUT NOT IMPLEMENTED -- DOES NOTHING

pub const ERR_NO_CLIENT: ReturnCode = 401; // ERR NO CLIENT
pub const ERR_NO_SUCH_CLIENT: ReturnCode = 402; // ERR NO SUCH CLIENT
pub const ERR_NO_MESSAGE: ReturnCode = 403; // ERR NO MESSAGE
pub const ERR_POS_LOW: ReturnCode = 404; // ERR POSITION TOO LOW
pub const ERR_POS_HIGH: ReturnCode = 405; // ERR POSITION TOO HIGH
pub const ERR_ID_NOT_EXIST: ReturnCode = 406; // ERR ID DOESNT EXIST
pub const ERR_UNKNOWN_ICON: ReturnCode = 407; // ERR UNKNOWN ICON
pub const ERR_UNKNOWN_PRIORITY: ReturnCode = 408; // ERR UNKNOWN PRIORITY
pub const ERR_RATE_TOO_HIGH: ReturnCode = 409; // ERR RATE TOO HIGH
pub const ERR_RATE_TOO_LOW: ReturnCode = 410; // ERR RATE TOO LOW
pub const ERR_PITCH_TOO_HIGH: ReturnCode = 411; // ERR PITCH TOO HIGH
pub const ERR_PITCH_TOO_LOW: ReturnCode = 412; // ERR PITCH TOO LOW
pub const ERR_VOLUME_TOO_HIGH: ReturnCode = 413; // ERR VOLUME TOO HIGH
pub const ERR_VOLUME_TOO_LOW: ReturnCode = 414; // ERR VOLUME TOO LOW

pub const ERR_PITCH_RANGE_TOO_HIGH: ReturnCode = 415; // ERR PITCH RANGE TOO HIGH
pub const ERR_PITCH_RANGE_TOO_LOW: ReturnCode = 416; // ERR PITCH RANGE TOO LOW

pub const ERR_INTERNAL: ReturnCode = 300; // ERR INTERNAL
pub const ERR_COULDNT_SET_PRIORITY: ReturnCode = 301; // ERR COULDNT SET PRIORITY
pub const ERR_COULDNT_SET_LANGUAGE: ReturnCode = 302; // ERR COULDNT SET LANGUAGE
pub const ERR_COULDNT_SET_RATE: ReturnCode = 303; // ERR COULDNT SET RATE
pub const ERR_COULDNT_SET_PITCH: ReturnCode = 304; // ERR COULDNT SET PITCH
pub const ERR_COULDNT_SET_PUNCTUATION: ReturnCode = 305; // ERR COULDNT SET PUNCT MODE
pub const ERR_COULDNT_SET_CAP_LET_RECOG: ReturnCode = 306; // ERR COULDNT SET CAP LET RECOGNITION
pub const ERR_COULDNT_SET_SPELLING: ReturnCode = 308; // ERR COULDNT SET SPELLING
pub const ERR_COULDNT_SET_VOICE: ReturnCode = 309; // ERR COULDNT SET VOICE
pub const ERR_COULDNT_SET_TABLE: ReturnCode = 310; // ERR COULDNT SET TABLE
pub const ERR_COULDNT_SET_CLIENT_NAME: ReturnCode = 311; // ERR COULDNT SET CLIENT_NAME
pub const ERR_COULDNT_SET_OUTPUT_MODULE: ReturnCode = 312; // ERR COULDNT SET OUTPUT MODULE
pub const ERR_COULDNT_SET_PAUSE_CONTEXT: ReturnCode = 313; // ERR COULDNT SET PAUSE CONTEXT
pub const ERR_COULDNT_SET_VOLUME: ReturnCode = 314; // ERR COULDNT SET VOLUME
pub const ERR_COULDNT_SET_SSML_MODE: ReturnCode = 315; // ERR COULDNT SET SSML MODE
pub const ERR_COULDNT_SET_NOTIFICATION: ReturnCode = 316; // ERR COULDNT SET NOTIFICATION
pub const ERR_COULDNT_SET_DEBUG: ReturnCode = 317; // ERR COULDNT SET DEBUGGING
pub const ERR_NO_SND_ICONS: ReturnCode = 320; // ERR NO SOUND ICONS
pub const ERR_CANT_REPORT_VOICES: ReturnCode = 321; // ERR MODULE CANT REPORT VOICES
pub const ERR_NO_OUTPUT_MODULE: ReturnCode = 321; // ERR NO OUTPUT MODULE LOADED
pub const ERR_ALREADY_INSIDE_BLOCK: ReturnCode = 330; // ERR ALREADY INSIDE BLOCK
pub const ERR_ALREADY_OUTSIDE_BLOCK: ReturnCode = 331; // ERR ALREADY OUTSIDE BLOCK
pub const ERR_NOT_ALLOWED_INSIDE_BLOCK: ReturnCode = 332; // ERR NOT ALLOWED INSIDE BLOCK
pub const ERR_COULDNT_SET_PITCH_RANGE: ReturnCode = 340; // ERR COULDNT SET PITCH RANGE
pub const ERR_NOT_IMPLEMENTED: ReturnCode = 380; // ERR NOT YET IMPLEMENTED

pub const ERR_INVALID_COMMAND: ReturnCode = 500; // ERR INVALID COMMAND
pub const ERR_INVALID_ENCODING: ReturnCode = 501; // ERR INVALID ENCODING
pub const ERR_MISSING_PARAMETER: ReturnCode = 510; // ERR MISSING PARAMETER
pub const ERR_NOT_A_NUMBER: ReturnCode = 511; // ERR PARAMETER NOT A NUMBER
pub const ERR_NOT_A_STRING: ReturnCode = 512; // ERR PARAMETER NOT A STRING
pub const ERR_PARAMETER_NOT_ON_OFF: ReturnCode = 513; // ERR PARAMETER NOT ON OR OFF
pub const ERR_PARAMETER_INVALID: ReturnCode = 514; // ERR PARAMETER INVALID
