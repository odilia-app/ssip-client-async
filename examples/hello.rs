use ssip_client::ClientResult;

fn main() -> ClientResult<()> {
    let mut client = ssip_client::new_unix_client("joe", "hello", "main")?;
    let msg_id = client.speak1("hello")?;
    println!("message: {}", msg_id);
    client.quit()?;
    Ok(())
}
