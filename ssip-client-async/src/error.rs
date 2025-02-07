use ssip::Error as SsipError;

pub enum Error {
    Io(std::io::Error),
    Ssip(SsipError),
}
