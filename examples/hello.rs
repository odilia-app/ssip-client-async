use ssip_client::ClientResult;

fn main() -> ClientResult<()> {
    let mut client = ssip_client::new_default_fifo_client("joe", "hello", "main", None)?;
    let msg_id = client.say_line("hello")?;
    println!("message: {}", msg_id);
    client.quit()?;
    Ok(())
}
