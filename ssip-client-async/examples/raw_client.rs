use smol_macros::main;
use std::io::{Write, self};
use ssip_client_async::{
    fifo::asynchronous_async_io::Builder,
    types::{ClientName, ClientResult, ClientScope, Request},
};

#[cfg(all(unix, feature = "async-io"))]
main! {
async fn main() -> ClientResult<()> {
    println!("Example:");
    let mut client = Builder::default().build().await?;
    println!("Client created.");
    client
        .set_client_name(ClientName::new("raw_client", "cmd"))
        .await?
        .check_client_name_set()
        .await?;
    println!("Client connected");

    let mut stdout = io::stdout();
    let mut stdin = io::stdin();
    loop {
        write!(stdout, "> ")?;
        stdout.flush()?;

        let mut text = String::new();
        stdin.read_line(&mut text)?;
        text = text.trim_end().to_string();
        let maybe_cmd = Request::from(text);
        if let Err(e) = maybe_cmd {
            println!("Invalid format: {e:?}");
            continue;
        };

    }

    client.quit().await?.receive().await?;
    Ok(())
}
}

#[cfg(all(unix, not(feature = "async-io")))]
fn main() {
    println!("see hello.rs for an example of a synchronous client.");
}

#[cfg(not(unix))]
fn main() {
    println!("example only available on unix.");
}
