use ssip_client::{ClientName, ClientResult};

fn main() -> ClientResult<()> {
    let mut client = ssip_client::new_default_fifo_client(&ClientName::new("joe", "hello"), None)?;
    let msg_id = client.say_line("hello")?;
    println!("message: {}", msg_id);
    client.quit()?;
    Ok(())
}
