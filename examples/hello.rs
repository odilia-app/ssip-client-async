use ssip_client::{new_default_fifo_client, ClientName, ClientResult};

fn main() -> ClientResult<()> {
    let mut client = new_default_fifo_client(None)?;
    client
        .set_client_name(ClientName::new("joe", "hello"))?
        .check_client_name_set()?;
    let msg_id = client.speak()?.send_line("hello")?.receive_message_id()?;
    println!("message: {}", msg_id);
    client.quit()?;
    Ok(())
}
