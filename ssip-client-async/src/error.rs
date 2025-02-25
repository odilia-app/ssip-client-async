use ssip::Error as SsipError;

pub enum Error {
    Io(std::io::Error),
    Ssip(SsipError),
}

impl From<std::io::Error> for Error {
    fn from(ioe: std::io::Error) -> Error {
        Error::Io(ioe)
    }
}
impl From<SsipError> for Error {
    fn from(ssipe: SsipError) -> Error {
        Error::Ssip(ssipe)
    }
}
