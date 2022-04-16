// Copyright (c) 2021-2022 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(not(feature = "async-mio"))]
use ssip_client::{client::Source, *};
#[cfg(not(feature = "async-mio"))]
use std::{
    io::{self, Read, Write},
    net::TcpStream,
    os::unix::net::UnixStream,
};

#[cfg(not(feature = "async-mio"))]
mod server;

/// Create a server on a Unix socket and run the client
///
/// The communication is an array of (["question", ...], "response")
#[cfg(not(feature = "async-mio"))]
fn test_unix_client<F>(
    communication: &'static [(&'static str, &'static str)],
    process: F,
) -> ClientResult<()>
where
    F: FnMut(&mut Client<UnixStream>) -> io::Result<()>,
{
    let socket_dir = tempfile::tempdir()?;
    let socket_path = socket_dir.path().join("test_client.socket");
    assert!(!socket_path.exists());
    let mut process_wrapper = std::panic::AssertUnwindSafe(process);
    let handle = server::run_unix(&socket_path, communication)?;
    let mut client = ssip_client::fifo::Builder::new()
        .path(&socket_path)
        .build()?;
    client
        .set_client_name(ClientName::new("test", "test"))?
        .check_client_name_set()?;
    process_wrapper(&mut client)?;
    handle.join().unwrap().unwrap();
    socket_dir.close()?;
    Ok(())
}

/// Create a server on a inet socket and run the client
///
/// The communication is an array of (["question", ...], "response")
#[cfg(not(feature = "async-mio"))]
fn test_tcp_client<F>(
    communication: &'static [(&'static str, &'static str)],
    process: F,
) -> ClientResult<()>
where
    F: FnMut(&mut Client<TcpStream>) -> io::Result<()>,
{
    let mut process_wrapper = std::panic::AssertUnwindSafe(process);
    let addr = "127.0.0.1:9999";
    let handle = server::run_tcp(addr, communication)?;
    let mut client = ssip_client::tcp::Builder::new(addr)?.build()?;
    client
        .set_client_name(ClientName::new("test", "test"))?
        .check_client_name_set()?;
    process_wrapper(&mut client)?;
    handle.join().unwrap().unwrap();
    Ok(())
}

#[cfg(not(feature = "async-mio"))]
const SET_CLIENT_COMMUNICATION: (&str, &str) = (
    "SET self CLIENT_NAME test:test:main\r\n",
    "208 OK CLIENT NAME SET\r\n",
);

#[test]
#[cfg(not(feature = "async-mio"))]
fn connect_and_quit() -> ClientResult<()> {
    const COMMUNICATION: [(&'static str, &'static str); 2] = [
        SET_CLIENT_COMMUNICATION,
        ("QUIT\r\n", "231 HAPPY HACKING\r\n"),
    ];
    fn process<S: Read + Write + Source>(client: &mut Client<S>) -> io::Result<()> {
        client.quit().unwrap().check_status(OK_BYE).unwrap();
        Ok(())
    }
    test_unix_client(&COMMUNICATION, process)?;
    test_tcp_client(&COMMUNICATION, process)?;
    Ok(())
}

#[test]
#[cfg(not(feature = "async-mio"))]
fn say_one_line() -> ClientResult<()> {
    test_unix_client(
        &[
            SET_CLIENT_COMMUNICATION,
            ("SPEAK\r\n", "230 OK RECEIVING DATA\r\n"),
            (
                "Hello, world\r\n.\r\n",
                "225-21\r\n225 OK MESSAGE QUEUED\r\n",
            ),
        ],
        |client| {
            assert_eq!(
                21,
                client
                    .speak()
                    .unwrap()
                    .check_status(OK_RECEIVING_DATA)
                    .unwrap()
                    .send_line("Hello, world")
                    .unwrap()
                    .receive_message_id()
                    .unwrap()
            );
            Ok(())
        },
    )
}

macro_rules! test_setter {
    ($setter:ident, $question:expr, $answer:expr, $code:expr, $($arg:tt)*) => {
        #[test]
        #[cfg(not(feature = "async-mio"))]
        fn $setter() -> ClientResult<()> {
            test_unix_client(
                &[SET_CLIENT_COMMUNICATION, ($question, $answer)],
                |client| {
                    client.$setter($($arg)*).unwrap().check_status($code).unwrap();
                    Ok(())
                },
            )
        }
    };
}

macro_rules! test_getter {
    ($getter:ident, $get_args:tt, $receive:ident, $recv_arg:tt, $question:expr, $answer:expr, $value:expr) => {
        #[test]
        #[cfg(not(feature = "async-mio"))]
        fn $getter() -> ClientResult<()> {
            test_unix_client(
                &[SET_CLIENT_COMMUNICATION, ($question, $answer)],
                |client| {
                    let value = client.$getter $get_args.unwrap().$receive $recv_arg.unwrap();
                    assert_eq!($value, value);
                    Ok(())
                },
            )
        }
    };
    ($getter:ident, $receive:ident, $arg:tt, $question:expr, $answer:expr, $value:expr) => {
        test_getter!($getter, (), $receive, $arg, $question, $answer, $value);
    };
    ($getter:ident, $question:expr, $answer:expr, $value:expr) => {
        test_getter!($getter, receive_string, (OK_GET), $question, $answer, $value);
    };
}

macro_rules! test_list {
    ($getter:ident, $question:expr, $answer:expr, $code:expr, $values:expr) => {
        #[test]
        #[cfg(not(feature = "async-mio"))]
        fn $getter() -> ClientResult<()> {
            test_unix_client(
                &[SET_CLIENT_COMMUNICATION, ($question, $answer)],
                |client| {
                    let values = client.$getter().unwrap().receive_lines($code).unwrap();
                    assert_eq!($values, values.as_slice());
                    Ok(())
                },
            )
        }
    };
}

test_setter!(
    set_priority,
    "SET self PRIORITY important\r\n",
    "202 OK PRIORITY SET\r\n",
    202,
    Priority::Important,
);

#[test]
#[cfg(not(feature = "async-mio"))]
fn set_debug() -> ClientResult<()> {
    test_unix_client(
        &[
            SET_CLIENT_COMMUNICATION,
            (
                "SET all DEBUG on\r\n",
                "262-/run/user/100/speech-dispatcher/log/debug\r\n262 OK DEBUGGING SET\r\n",
            ),
        ],
        |client| {
            let output = client
                .set_debug(true)
                .unwrap()
                .receive_string(OK_DEBUG_SET)
                .unwrap();
            assert_eq!("/run/user/100/speech-dispatcher/log/debug", output);
            Ok(())
        },
    )
}

test_setter!(
    set_output_module,
    "SET self OUTPUT_MODULE espeak-ng\r\n",
    "216 OK OUTPUT MODULE SET\r\n",
    216,
    ClientScope::Current,
    "espeak-ng",
);

test_getter!(
    get_output_module,
    "GET OUTPUT_MODULE\r\n",
    "251-espeak-ng\r\n251 OK GET RETURNED\r\n",
    "espeak-ng"
);

test_list!(
    list_output_modules,
    "LIST OUTPUT_MODULES\r\n",
    "250-espeak-ng\r\n250-festival\r\n250 OK MODULE LIST SENT\r\n",
    250,
    &["espeak-ng", "festival"]
);

test_setter!(
    set_language,
    "SET self LANGUAGE en\r\n",
    "201 OK LANGUAGE SET\r\n",
    201,
    ClientScope::Current,
    "en",
);

test_getter!(
    get_language,
    "GET LANGUAGE\r\n",
    "251-fr\r\n251 OK GET RETURNED\r\n",
    "fr"
);

test_setter!(
    set_rate,
    "SET self RATE 15\r\n",
    "203 OK RATE SET\r\n",
    203,
    ClientScope::Current,
    15,
);

test_getter!(
    get_rate,
    receive_i8,
    (),
    "GET RATE\r\n",
    "251-0\r\n251 OK GET RETURNED\r\n",
    0
);

test_setter!(
    set_volume,
    "SET self VOLUME 80\r\n",
    "218 OK VOLUME SET\r\n",
    218,
    ClientScope::Current,
    80,
);

test_getter!(
    get_volume,
    receive_i8,
    (),
    "GET VOLUME\r\n",
    "251-100\r\n251 OK GET RETURNED\r\n",
    100
);

test_getter!(
    get_pitch,
    receive_i8,
    (),
    "GET PITCH\r\n",
    "251-0\r\n251 OK GET RETURNED\r\n",
    0
);

test_setter!(
    set_pitch,
    "SET self PITCH 10\r\n",
    "204 OK PITCH SET\r\n",
    204,
    ClientScope::Current,
    10,
);

test_setter!(
    set_ssml_mode,
    "SET self SSML_MODE on\r\n",
    "219 OK SSML MODE SET\r\n",
    219,
    true
);

test_setter!(
    set_spelling,
    "SET self SPELLING on\r\n",
    "207 OK SPELLING SET\r\n'",
    207,
    ClientScope::Current,
    true
);

test_setter!(
    set_punctuation_mode,
    "SET self PUNCTUATION all\r\n",
    "205 OK PUNCTUATION SET\r\n",
    205,
    ClientScope::Current,
    PunctuationMode::All
);

test_setter!(
    set_capital_letter_recogn,
    "SET self CAP_LET_RECOGN spell\r\n",
    "206 OK CAP LET RECOGNITION SET\r\n",
    206,
    ClientScope::Current,
    CapitalLettersRecognitionMode::Spell
);

test_getter!(
    get_voice_type,
    "GET VOICE_TYPE\r\n",
    "251-MALE1\r\n251 OK GET RETURNED\r\n",
    "MALE1"
);

test_setter!(
    set_voice_type,
    "SET self VOICE_TYPE FEMALE1\r\n",
    "209 OK VOICE SET\r\n",
    209,
    ClientScope::Current,
    "FEMALE1"
);

test_list!(
    list_voice_types,
    "LIST VOICES\r\n",
    "249-MALE1\r\n249-MALE2\r\n249-FEMALE1\r\n249-FEMALE2\r\n249-CHILD_MALE\r\n249-CHILD_FEMALE\r\n249 OK VOICE LIST SENT\r\n",
    249,
    &[ "MALE1", "MALE2", "FEMALE1", "FEMALE2", "CHILD_MALE", "CHILD_FEMALE" ]
);

#[test]
#[cfg(not(feature = "async-mio"))]
fn list_synthesis_voices() -> ClientResult<()> {
    test_unix_client(
        &[
            SET_CLIENT_COMMUNICATION,
            (
                "LIST SYNTHESIS_VOICES\r\n",
                "249-Amharic\tam\tnone\r\n249-Greek+Auntie\tel\tAuntie\r\n249-Vietnamese (Southern)+shelby\tvi-VN-X-SOUTH\tshelby\r\n249 OK VOICE LIST SENT\r\n"
            ),
        ],
        |client| {
            let voices = client.list_synthesis_voices().unwrap().receive_synthesis_voices().unwrap();
            let expected_voices: [SynthesisVoice; 3] = [ SynthesisVoice::new("Amharic", Some("am"), None),
                                     SynthesisVoice::new("Greek+Auntie", Some("el"), Some("Auntie")),
                                     SynthesisVoice::new("Vietnamese (Southern)+shelby", Some("vi-VN-X-SOUTH"), Some("shelby")),
            ];
            assert_eq!(expected_voices.len(), voices.len());
            for (expected, found) in expected_voices.iter().zip(voices.iter()) {
                assert_eq!(*expected, *found);
            }
            Ok(())
        },
    )
}

#[test]
#[cfg(not(feature = "async-mio"))]
fn receive_notification() -> ClientResult<()> {
    test_unix_client(
        &[
            SET_CLIENT_COMMUNICATION,
            ("SPEAK\r\n", "230 OK RECEIVING DATA\r\n"),
            (
                "Hello, world\r\n.\r\n",
                "225-21\r\n225 OK MESSAGE QUEUED\r\n701-21\r\n701-test\r\n701 BEGIN\r\n",
            ),
        ],
        |client| {
            assert_eq!(
                21,
                client
                    .speak()
                    .unwrap()
                    .check_receiving_data()
                    .unwrap()
                    .send_line("Hello, world")
                    .unwrap()
                    .receive_message_id()
                    .unwrap()
            );
            match client.receive_event() {
                Ok(Event {
                    ntype: EventType::Begin,
                    ..
                }) => Ok(()),
                Ok(_) => panic!("wrong event"),
                Err(_) => panic!("error on event"),
            }
        },
    )
}

#[test]
#[cfg(not(feature = "async-mio"))]
fn history_clients_list() -> ClientResult<()> {
    test_unix_client(
        &[
            SET_CLIENT_COMMUNICATION,
            (
                "HISTORY GET CLIENT_LIST\r\n",
                "240-0 joe:speechd_client:main 0\r\n240-1 joe:speechd_client:status 0\r\n240-2 unknown:unknown:unknown 1\r\n240 OK CLIENTS LIST SENT\r\n"
            ),
        ],
        |client| {
            let statuses = client.history_get_clients().unwrap().receive_history_clients().unwrap();
            let expected_statuses: [HistoryClientStatus; 3] = [ HistoryClientStatus::new(0, "joe:speechd_client:main", false),
                                                                HistoryClientStatus::new(1, "joe:speechd_client:status", false),
                                                                HistoryClientStatus::new(2, "unknown:unknown:unknown", true),
            ];
            assert_eq!(expected_statuses.len(), statuses.len());
            for (expected, found) in expected_statuses.iter().zip(statuses.iter()) {
                assert_eq!(*expected, *found);
            }
            Ok(())
        },
    )
}

test_getter!(
    history_get_client_id,
    receive_client_id,
    (),
    "HISTORY GET CLIENT_ID\r\n",
    "245-123\r\n245 OK CLIENT ID SENT\r\n",
    123
);

test_getter!(
    history_get_last,
    receive_message_id,
    (),
    "HISTORY GET LAST\r\n",
    "242-123\r\n242 OK LAST MSG SAID\r\n",
    123
);

test_getter!(
    history_get_message,
    (123),
    receive_string,
    (OK_MSG_TEXT_SENT),
    "HISTORY GET MESSAGE 123\r\n",
    "246-Hello, world!\r\n246 OK MESSAGE SENT\r\n",
    "Hello, world!"
);

test_getter!(
    history_get_cursor,
    receive_cursor_pos,
    (),
    "HISTORY CURSOR GET",
    "243-42\r\n243 OK CURSOR POSITION RETURNED\r\n",
    42
);
