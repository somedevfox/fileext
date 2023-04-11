use std::io;

#[derive(Debug)]
pub enum Error {
    ExecutableDoesntExist,
    Io(io::Error),
}
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
