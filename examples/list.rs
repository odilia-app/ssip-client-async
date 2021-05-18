use ssip_client::{ClientName, ClientResult};

fn main() -> ClientResult<()> {
    let mut client = ssip_client::new_default_fifo_client(&ClientName::new("joe", "list"), None)?;
    let status = client.quit()?;
    println!("status: {}", status);
    Ok(())
}
