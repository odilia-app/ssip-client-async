#[cfg(not(feature = "async-mio"))]
use ssip_client::{
    ClientName, ClientResult, FifoBuilder, SynthesisVoice, OK_OUTPUT_MODULES_LIST_SENT,
    OK_VOICES_LIST_SENT,
};

#[cfg(not(feature = "async-mio"))]
fn main() -> ClientResult<()> {
    fn voice_to_string(voice: &SynthesisVoice) -> String {
        match &voice.language {
            Some(language) => match &voice.dialect {
                Some(dialect) => format!("{} [{}_{}]", voice.name, language, dialect),
                None => format!("{} [{}]", voice.name, language),
            },
            None => format!("{}", voice.name),
        }
    }

    fn print_list(title: &str, values: &[String]) {
        println!("{}:", title);
        for val in values {
            println!("- {}", val);
        }
    }

    let mut client = FifoBuilder::new().build()?;
    client
        .set_client_name(ClientName::new("joe", "list"))?
        .check_client_name_set()?;

    const OUTPUT_MODULE_TITLE: &str = "output modules";
    let modules = client
        .list_output_modules()?
        .receive_lines(OK_OUTPUT_MODULES_LIST_SENT)?;
    print_list(OUTPUT_MODULE_TITLE, &modules);

    const VOICE_TYPES_TITLE: &str = "voice types";
    let vtypes = client
        .list_voice_types()?
        .receive_lines(OK_VOICES_LIST_SENT)?;
    print_list(VOICE_TYPES_TITLE, &vtypes);

    const SYNTHESIS_VOICES_TITLE: &str = "synthesis voices";
    let voices = client.list_synthesis_voices()?.receive_synthesis_voices()?;
    print_list(
        SYNTHESIS_VOICES_TITLE,
        &voices
            .iter()
            .map(|ref v| voice_to_string(v))
            .collect::<Vec<String>>(),
    );

    client.quit().unwrap();
    Ok(())
}

#[cfg(feature = "async-mio")]
fn main() {
    println!("asynchronous client not implemented");
}
