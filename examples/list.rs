use ssip_client::ClientResult;

fn main() -> ClientResult<()> {
    let mut client = ssip_client::new_default_fifo_client("joe", "list", "main", None)?;
    let status = client.quit()?;
    println!("status: {}", status);
    Ok(())
}
