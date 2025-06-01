// Copyright (c) 2022 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(feature = "async-mio")]
use mio::{Events, Poll, Token};
#[cfg(feature = "async-mio")]
use std::{
    io::{Read, Write},
    slice::Iter,
    time::Duration,
};

#[cfg(feature = "async-mio")]
use ssip_client_async::{client::Source, *};

#[cfg(feature = "async-mio")]
mod server;

#[cfg(feature = "async-mio")]
struct State<'a, 'b> {
    pub done: bool,
    pub countdown: usize,
    pub writable: bool,
    pub start_get: bool,
    pub iter_requests: Iter<'a, Request>,
    pub iter_answers: Iter<'b, &'static str>,
}

#[cfg(feature = "async-mio")]
impl<'a, 'b> State<'a, 'b> {
    fn new(iter_requests: Iter<'a, Request>, iter_answers: Iter<'b, &'static str>) -> Self {
        State {
            done: false,
            countdown: 50,
            writable: false,
            start_get: false,
            iter_requests,
            iter_answers,
        }
    }

    fn terminated(&self) -> bool {
        self.done || self.countdown == 0
    }

    fn must_send(&self) -> bool {
        self.writable && self.countdown > 0
    }

    fn next_request(&mut self) -> Option<&Request> {
        if self.start_get {
            self.iter_requests.next()
        } else {
            None
        }
    }

    fn assert_answer(&mut self, val: &str) {
        match self.iter_answers.next() {
            Some(expected_val) => assert_eq!(expected_val, &val),
            None => panic!("no more answers"),
        }
    }
}

#[cfg(feature = "async-mio")]
fn basic_async_client_communication<S: Read + Write + Source>(
    client: &mut QueuedClient<S>,
) -> ClientResult<usize> {
    let get_requests = [Request::GetOutputModule, Request::GetRate];
    let get_answers = ["espeak", "10"];
    let mut state = State::new(get_requests.iter(), get_answers.iter());

    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    let input_token = Token(0);
    let output_token = Token(1);
    let timeout = Duration::new(0, 500 * 1000 * 1000 /* 500 ms */);
    client.register(&poll, input_token, output_token).unwrap();
    client.push(Request::SetName(ClientName::new("test", "test")));
    while !state.terminated() {
        if !state.writable || !client.has_next() {
            poll.poll(&mut events, Some(timeout))?;
        }
        state.countdown -= 1;
        for event in &events {
            let token = event.token();
            if token == input_token {
                match dbg!(client.receive_next()?) {
                    Response::ClientNameSet => {
                        client.push(Request::SetLanguage(ClientScope::Current, "en".to_string()))
                    }
                    Response::LanguageSet => client.push(Request::Stop(MessageScope::Last)),
                    Response::Stopped => state.start_get = true,
                    Response::Get(val) => state.assert_answer(&val),
                    result => panic!("Unexpected response: {result:?}"),
                }
                if let Some(request) = state.next_request() {
                    client.push(request.clone());
                } else if state.start_get {
                    state.done = true; // No more get request
                }
            } else if token == output_token {
                state.writable = true;
            }
        }
        if state.must_send() {
            match client.send_next() {
                Ok(_) => (),
                Err(ClientError::NotReady) => state.writable = false,
                Err(err) => return Err(err),
            }
        }
    }
    Ok(state.countdown)
}

#[cfg(feature = "async-mio")]
const BASIC_COMMUNICATION: [(&str, &str); 5] = [
    (
        "SET self CLIENT_NAME test:test:main\r\n",
        "208 OK CLIENT NAME SET\r\n",
    ),
    ("SET self LANGUAGE en\r\n", "201 OK LANGUAGE SET\r\n"),
    ("STOP self\r\n", "210 OK STOPPED\r\n"),
    (
        "GET OUTPUT_MODULE\r\n",
        "251-espeak\r\n251 OK GET RETURNED\r\n",
    ),
    ("GET RATE\r\n", "251-10\r\n251 OK GET RETURNED\r\n"),
];

#[test]
#[cfg(all(unix, feature = "async-mio"))]
fn basic_async_unix_communication() -> ClientResult<()> {
    let socket_dir = tempfile::tempdir()?;
    let socket_path = socket_dir.path().join("basic_async_communication.socket");
    assert!(!socket_path.exists());
    let handle = server::run_unix(&socket_path, &BASIC_COMMUNICATION)?;
    let mut client = QueuedClient::new(fifo::Builder::new().path(&socket_path).build()?);
    let countdown = basic_async_client_communication(&mut client)?;
    handle.join().unwrap().unwrap();
    socket_dir.close()?;
    assert!(countdown > 0);
    Ok(())
}

#[test]
#[cfg(feature = "async-mio")]
fn basic_async_tcp_communication() -> ClientResult<()> {
    let addr = "127.0.0.1:9999";
    let handle = server::run_tcp(addr, &BASIC_COMMUNICATION)?;
    let mut client = QueuedClient::new(tcp::Builder::new(addr.parse().unwrap()).build()?);
    let countdown = basic_async_client_communication(&mut client)?;
    handle.join().unwrap().unwrap();
    assert!(countdown > 0);
    Ok(())
}
