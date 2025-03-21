use std::io::{Read, Write};
// Trick to have common implementation for std and mio streams..
#[cfg(all(not(feature = "async-mio"), unix))]
pub use std::os::unix::io::AsRawFd as Source;

use crate::{fifo::Builder, Client};
use lingua::IsoCode639_3;
use ssip::ClientResult;

impl Builder {
    /// Initialize the language detection model with a list of languages to distinguish between
    /// Use the ISO 639-3 language codes to distinguish between languages
    pub fn with_language_detection(&mut self, languages: &Vec<IsoCode639_3>) -> &mut Self {
        self.language_detector_model = Some(
            lingua::LanguageDetectorBuilder::from_iso_codes_639_3(languages)
                // preload all language models into memory for faster client detection
                .with_preloaded_language_models()
                .build(),
        );
        self
    }
}

impl<S: Read + Write + Source> Client<S> {


    /// A wrapper over the `send_lines` method to send lines in multiple languages
    pub fn send_lines_multilingual(&mut self, lines: &[String]) -> ClientResult<()> {

        Ok(())
    }
}
