use mio::{Events, Poll, Token};

use ssip_client::{ClientName, ClientResult};

fn main() -> ClientResult<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    let mut client = ssip_client::new_default_fifo_client()?;
    let token = Token(0);
    client.register(&poll, token)?;

    poll.poll(&mut events, None)?;
    let mut is_opened = false;
    while !is_opened {
        for event in &events {
            if event.token() == token && event.is_writable() {
                println!("opening client");
                match client.open(ClientName::new("joe", "hello")) {
                    Ok(()) => {
                        is_opened = true;
                        break;
                    }
                    Err(err) if err.kind() == io::ErrorKing::WouldBlock => {}
                    Err(err) => panic!("Error opening client: {:?}", err),
                }
                break;
            }
        }
    }

    poll.poll(&mut events, None)?;
    for event in &events {
        if event.token() == token && event.is_writable() {
            println!("sending message");
            let msg_id = client.say_line("hello")?;
            println!("message: {}", msg_id);

            break;
        }
    }

    poll.poll(&mut events, None)?;
    for event in &events {
        if event.token() == token && event.is_writable() {
            println!("quitting");
            client.quit()?;
            break;
        }
    }
    Ok(())
}
