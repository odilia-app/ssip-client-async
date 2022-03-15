use ssip_client::{new_default_fifo_client, ClientName, ClientResult, OK_CLIENT_NAME_SET};

fn main() -> ClientResult<()> {
    let mut client = new_default_fifo_client(None)?;
    client
        .open(ClientName::new("joe", "hello"))?
        .check_status(OK_CLIENT_NAME_SET)?;
    let msg_id = client.speak()?.send_line("hello")?.receive_message_id()?;
    println!("message: {}", msg_id);
    client.quit()?;
    Ok(())
}
