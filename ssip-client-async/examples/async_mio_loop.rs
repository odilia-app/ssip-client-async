#[cfg(all(unix, feature = "async-mio"))]
use mio::{unix::SourceFd, Events, Interest, Poll, Token};
#[cfg(all(unix, feature = "async-mio"))]
use std::{
    collections::VecDeque,
    io::{self, Write},
    os::unix::io::AsRawFd,
};

#[cfg(all(unix, feature = "async-mio"))]
use ssip_client_async::{
    fifo, ClientError, ClientName, ClientResult, MioQueuedClient, Request, Response,
};

#[cfg(all(unix, feature = "async-mio"))]
fn main() -> ClientResult<()> {
    let stdin = io::stdin();

    // Poll instance
    let mut poll = Poll::new()?;

    // Register stdin
    let stdin_fd = stdin.as_raw_fd();
    let mut source_fd = SourceFd(&stdin_fd);
    let stdin_token = Token(0);
    poll.registry()
        .register(&mut source_fd, stdin_token, Interest::READABLE)?;

    // Register the SSIP client
    let mut ssip_client = MioQueuedClient::new(fifo::asynchronous_mio::Builder::new().build()?);
    let speech_input_token = Token(1);
    let speech_output_token = Token(2);
    ssip_client.register(&poll, speech_input_token, speech_output_token)?;

    // Loop for events
    let mut events = Events::with_capacity(16);
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
        if !speech_writable || !ssip_client.has_next() {
            poll.poll(&mut events, None)?;
        }
        for event in &events {
            let token = event.token();
            if token == stdin_token {
                let mut text = String::new();
                stdin.read_line(&mut text)?;
                text = text.trim_end().to_string();
                match text.len() {
                    0 => return Ok(()),
                    1 => {
                        if let Some(ch) = text.chars().next() {
                            println!("sending char: {ch}");
                            ssip_client.push(Request::SpeakChar(ch))
                        }
                    }
                    _ => {
                        println!("sending line: {text}");
                        send_requests.push_back(Request::SendLine(text.to_owned()));
                        ssip_client.push(Request::Speak);
                    }
                }
                prompt()?;
            } else if token == speech_input_token {
                match ssip_client.receive_next() {
                    Err(ClientError::Io(err)) => return Err(ClientError::from(err)),
                    Err(ClientError::Ssip(err)) => eprintln!("SSIP error: {err:?}"),
                    Err(_) => panic!("internal error"),
                    Ok(result) => match result {
                        Response::MessageQueued | Response::ClientNameSet => (),
                        Response::ReceivingData => {
                            ssip_client.push(send_requests.pop_front().unwrap())
                        }
                        _ => panic!("Unexpected response: {result:?}"),
                    },
                }
            } else if token == speech_output_token {
                speech_writable = true;
            }
        }
        if speech_writable {
            match ssip_client.send_next() {
                Err(ClientError::NotReady) => speech_writable = false,
                Err(ClientError::Io(err)) => return Err(ClientError::from(err)),
                Err(_) => panic!("internal error"),
                Ok(_) => (),
            }
        }
    }
}

#[cfg(all(unix, not(feature = "async-mio")))]
fn main() {
    println!("see hello for an example of synchronous client.");
}

#[cfg(not(unix))]
fn main() {
    println!("example only available on unix.");
}
