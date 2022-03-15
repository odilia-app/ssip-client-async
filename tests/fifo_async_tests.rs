// Copyright (c) 2022 Laurent Pelecq
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(feature = "metal-io")]
use mio::{Events, Poll, Token};
#[cfg(feature = "metal-io")]
use ssip_client::*;

#[cfg(feature = "metal-io")]
mod server;

#[cfg(feature = "metal-io")]
use server::Server;

#[cfg(feature = "metal-io")]
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

#[cfg(feature = "metal-io")]
use utils::Controler;

#[test]
#[cfg(feature = "metal-io")]
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
        let mut client = FifoBuilder::new().path(&server_path).build().unwrap();
        let token = Token(0);
        client.register(&poll, token).unwrap();
        let mut controler = Controler::new();
        use std::io::Write;
        let mut log_file = std::fs::File::create("/home/laurent/tmp/test_client.log")?;
        while controler.step() < 2 {
            poll.poll(&mut events, None)?;
            log_file.write_all(format!("Step: {}\n", controler.step()).as_bytes())?;
            for event in &events {
                if event.token() == token {
                    if event.is_writable() {
                        log_file.write_all(b"Event writable\n")?;
                        if controler.step() == 0 {
                            log_file.write_all(b"Send: set client name\n")?;
                            controler.check_result(
                                client.set_client_name(ClientName::new("test", "test")),
                            );
                        }
                    } else if event.is_readable() {
                        log_file.write_all(b"Event readable\n")?;
                        if controler.step() == 1 {
                            log_file.write_all(b"Receive: client name set\n")?;
                            controler.check_result(client.check_client_name_set());
                        }
                    }
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
