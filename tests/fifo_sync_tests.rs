// Copyright (c) 2021-2022 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(not(feature = "metal-io"))]
use ssip_client::*;
#[cfg(not(feature = "metal-io"))]
use std::{io, os::unix::net::UnixStream};

#[cfg(not(feature = "metal-io"))]
mod server;

#[cfg(not(feature = "metal-io"))]
use server::Server;

/// Create a server and run the client
///
/// The communication is an array of (["question", ...], "response")
#[cfg(not(feature = "metal-io"))]
fn test_client<F>(
    communication: &'static [(&'static str, &'static str)],
    process: F,
) -> io::Result<()>
where
    F: FnMut(&mut Client<UnixStream>) -> io::Result<()>,
{
    let socket_path = Server::temporary_path();
    assert!(!socket_path.exists());
    let server_path = socket_path.clone();
    let mut process_wrapper = std::panic::AssertUnwindSafe(process);
    let result = std::panic::catch_unwind(move || {
        let handle = Server::run(&server_path, communication);
        let mut client = ssip_client::new_fifo_client(&server_path, None).unwrap();
        client
            .set_client_name(ClientName::new("test", "test"))
            .unwrap()
            .check_client_name_set()
            .unwrap();
        process_wrapper(&mut client).unwrap();
        handle.join().unwrap()
    });
    std::fs::remove_file(socket_path)?;
    result.unwrap().unwrap();
    Ok(())
}

#[cfg(not(feature = "metal-io"))]
const SET_CLIENT_COMMUNICATION: (&str, &str) = (
    "SET self CLIENT_NAME test:test:main\r\n",
    "208 OK CLIENT NAME SET\r\n",
);

#[test]
#[cfg(not(feature = "metal-io"))]
fn connect_and_quit() -> io::Result<()> {
    test_client(
        &[
            SET_CLIENT_COMMUNICATION,
            ("QUIT\r\n", "231 HAPPY HACKING\r\n"),
        ],
        |client| {
            client.quit().unwrap().check_status(OK_BYE).unwrap();
            Ok(())
        },
    )
}

#[test]
#[cfg(not(feature = "metal-io"))]
fn say_one_line() -> io::Result<()> {
    test_client(
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
                "21",
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
        #[cfg(not(feature = "metal-io"))]
        fn $setter() -> io::Result<()> {
            test_client(
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
    ($getter:ident, $receive:ident, $question:expr, $answer:expr, $value:expr) => {
        #[test]
        #[cfg(not(feature = "metal-io"))]
        fn $getter() -> io::Result<()> {
            test_client(
                &[SET_CLIENT_COMMUNICATION, ($question, $answer)],
                |client| {
                    let value = client.$getter().unwrap().$receive(251).unwrap();
                    assert_eq!($value, value);
                    Ok(())
                },
            )
        }
    };
    ($getter:ident, $question:expr, $answer:expr, $value:expr) => {
        test_getter!($getter, receive_string, $question, $answer, $value);
    };
}

macro_rules! test_list {
    ($getter:ident, $question:expr, $answer:expr, $code:expr, $values:expr) => {
        #[test]
        #[cfg(not(feature = "metal-io"))]
        fn $getter() -> io::Result<()> {
            test_client(
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
#[cfg(not(feature = "metal-io"))]
fn set_debug() -> io::Result<()> {
    test_client(
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
    receive_u8,
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
    receive_u8,
    "GET VOLUME\r\n",
    "251-100\r\n251 OK GET RETURNED\r\n",
    100
);

test_getter!(
    get_pitch,
    receive_u8,
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
#[cfg(not(feature = "metal-io"))]
fn list_synthesis_voices() -> io::Result<()> {
    test_client(
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
#[cfg(not(feature = "metal-io"))]
fn receive_notification() -> io::Result<()> {
    test_client(
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
                "21",
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
            match client.receive_event() {
                Ok(Event {
                    ntype: EventType::Begin,
                    message: _,
                    client: _,
                }) => Ok(()),
                Ok(_) => panic!("wrong event"),
                Err(_) => panic!("error on event"),
            }
        },
    )
}
