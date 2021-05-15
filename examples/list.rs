use ssip_client::ClientResult;

fn main() -> ClientResult<()> {
    let mut client = ssip_client::new_unix("joe", "list", "main")?;
    let status = client.quit()?;
    println!("status: {}", status);
    Ok(())
}
