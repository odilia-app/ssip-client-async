#[cfg(all(unix, not(feature = "async-mio")))]
use ssip_client_async::{fifo, ClientResult};

#[cfg(all(unix, not(feature = "async-mio")))]
fn main() -> ClientResult<()> {
    use ssip::ClientName;
    let english_or_russian = vec![lingua::IsoCode639_1::EN, lingua::IsoCode639_1::RU];
    let mut english_or_russian_client = fifo::Builder::new()
        // initialize the language detection model with a list of languages to distinguish between
        .with_automatic_detection_languages(&english_or_russian)
        .with_spawn()?
        .build()?;

    english_or_russian_client
        .set_client_name(ClientName::new("joe", "example_app"))?
        .check_client_name_set()?
        .send_lines_multilingual(&"Hello, my name is Joe. меня зовут джо".to_string())?;

    english_or_russian_client.quit()?.receive()?;

    let english_or_spanish = vec![lingua::IsoCode639_1::EN, lingua::IsoCode639_1::ES];
    let mut english_or_spanish_client = fifo::Builder::new()
        // initialize the language detection model with a list of languages to distinguish between
        .with_automatic_detection_languages(&english_or_spanish)
        .with_spawn()?
        .build()?;

    english_or_spanish_client
        .set_client_name(ClientName::new("joe", "example_app"))?
        .check_client_name_set()?
        .send_lines_multilingual(
            &"Hello, my name is Joe. Me llamo Joe y yo hablo espanol".to_string(),
        )?;

    english_or_spanish_client.quit()?.receive()?;

    Ok(())
}

#[cfg(all(unix, feature = "async-mio"))]
fn main() {
    println!("see async_mio_loop for an example of asynchronous client.");
}

#[cfg(not(unix))]
fn main() {
    println!("example only available on unix.");
}
