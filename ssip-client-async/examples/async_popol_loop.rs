#[cfg(all(unix, not(feature = "async-mio")))]
use std::{
    collections::VecDeque,
    io::{self, Write},
};

#[cfg(all(unix, not(feature = "async-mio")))]
use ssip_client::{fifo, ClientError, ClientName, ClientResult, QueuedClient, Request, Response};

#[cfg(all(unix, not(feature = "async-mio")))]
fn main() -> ClientResult<()> {
    #[derive(Clone, Eq, PartialEq)]
    enum SourceKey {
        Stdin,
        SpeechIn,
        SpeechOut,
    }

    let mut sources = popol::Sources::with_capacity(2);
    let mut events = popol::Events::with_capacity(4);

    let stdin = io::stdin();
    let mut ssip_client = QueuedClient::new(fifo::Builder::new().nonblocking().build()?);

    sources.register(SourceKey::Stdin, &stdin, popol::interest::READ);
    sources.register(
        SourceKey::SpeechIn,
        ssip_client.input_source(),
        popol::interest::READ,
    );
    sources.register(
        SourceKey::SpeechOut,
        ssip_client.output_source(),
        popol::interest::WRITE,
    );

    // Loop for events
    let mut speech_writable = false;
    let mut send_requests = VecDeque::with_capacity(4);
    ssip_client.push(Request::SetName(ClientName::new("joe", "async")));

    fn prompt() -> io::Result<()> {
        let mut stdout = io::stdout();
        write!(stdout, "> ")?;
        stdout.flush()
    }

    println!("Enter an empty line to quit.");
    prompt()?;
    loop {
        sources.wait(&mut events)?;
        for (key, _event) in events.iter() {
            match key {
                SourceKey::Stdin => {
                    let mut text = String::new();
                    stdin.read_line(&mut text)?;
                    text = text.trim_end().to_string();
                    match text.len() {
                        0 => return Ok(()),
                        1 => {
                            if let Some(ch) = text.chars().next() {
                                println!("sending char: {}", ch);
                                ssip_client.push(Request::SpeakChar(ch))
                            }
                        }
                        _ => {
                            println!("sending line: {}", text);
                            send_requests.push_back(Request::SendLine(text.to_owned()));
                            ssip_client.push(Request::Speak);
                        }
                    }
                    prompt()?;
                }
                SourceKey::SpeechIn => match ssip_client.receive_next() {
                    Err(ClientError::Io(err)) => return Err(ClientError::from(err)),
                    Err(ClientError::Ssip(err)) => eprintln!("SSIP error: {:?}", err),
                    Err(_) => panic!("internal error"),
                    Ok(result) => match result {
                        Response::MessageQueued | Response::ClientNameSet => (),
                        Response::ReceivingData => {
                            ssip_client.push(send_requests.pop_front().unwrap())
                        }
                        _ => panic!("Unexpected response: {:?}", result),
                    },
                },
                SourceKey::SpeechOut => {
                    speech_writable = true;
                }
            }
        }
        if speech_writable {
            if sources.len() >= 3 {
                sources.unregister(&SourceKey::SpeechOut);
            }

            loop {
                match ssip_client.send_next() {
                    Err(ClientError::NotReady) => speech_writable = false,
                    Err(ClientError::Io(err)) => return Err(ClientError::from(err)),
                    Err(_) => panic!("internal error"),
                    Ok(true) => (),
                    Ok(false) => break,
                }
            }
        }
        if !speech_writable && ssip_client.has_next() {
            sources.register(
                SourceKey::SpeechOut,
                ssip_client.output_source(),
                popol::interest::WRITE,
            );
        }
    }
}

#[cfg(all(unix, feature = "async-mio"))]
fn main() {
    println!("asynchronous client not implemented");
}

#[cfg(not(unix))]
fn main() {
    println!("example only available on unix.");
}
