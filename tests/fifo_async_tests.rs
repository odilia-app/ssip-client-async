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
use ssip_client::*;

#[cfg(feature = "async-mio")]
mod server;

#[cfg(feature = "async-mio")]
use server::Server;

#[cfg(feature = "async-mio")]
mod utils {

    use ssip_client::*;

    const MAX_RETRIES: u16 = 10;

    pub struct Controler {
        step: u16,
        retry: u16,
    }

    impl Controler {
        pub fn new() -> Controler {
            Controler {
                step: 0,
                retry: MAX_RETRIES,
            }
        }

        pub fn step(&self) -> u16 {
            self.step
        }

        pub fn check_result<V>(&mut self, result: ClientResult<V>) -> Option<V> {
            match result {
                Ok(value) => {
                    self.step += 1;
                    self.retry = MAX_RETRIES;
                    Some(value)
                }
                Err(ClientError::NotReady) if self.retry > 0 => {
                    self.retry -= 1;
                    None
                }
                Err(err) => panic!("{:?}", err),
            }
        }
    }
}

#[cfg(feature = "async-mio")]
use utils::Controler;

#[test]
#[cfg(feature = "async-mio")]
fn basic_async_communication() -> std::io::Result<()> {
    const COMMUNICATION: [(&str, &str); 1] = [(
        "SET self CLIENT_NAME test:test:main\r\n",
        "208 OK CLIENT NAME SET\r\n",
    )];

    let socket_path = Server::temporary_path();
    assert!(!socket_path.exists());
    let server_path = socket_path.clone();
    let result = std::panic::catch_unwind(move || -> std::io::Result<u16> {
        let handle = Server::run(&server_path, &COMMUNICATION);
        let mut poll = Poll::new()?;
        let mut events = Events::with_capacity(128);
        let mut client = fifo::Builder::new().path(&server_path).build().unwrap();
        let input_token = Token(0);
        let output_token = Token(1);
        client.register(&poll, input_token, output_token).unwrap();
        let mut controler = Controler::new();
        while controler.step() < 2 {
            poll.poll(&mut events, None)?;
            for event in &events {
                if event.token() == output_token && event.is_writable() && controler.step() == 0 {
                    controler.check_result(client.set_client_name(ClientName::new("test", "test")));
                } else if event.token() == input_token
                    && event.is_readable()
                    && controler.step() == 1
                {
                    controler.check_result(client.check_client_name_set());
                }
            }
        }
        handle.join().unwrap().unwrap();
        Ok(controler.step())
    });
    std::fs::remove_file(socket_path)?;
    assert_eq!(2, result.unwrap().unwrap());
    Ok(())
}
