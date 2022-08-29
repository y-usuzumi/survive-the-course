use std::io;

#[derive(Debug)]
pub enum RSLoxError {
    IOError(io::Error),
}

impl From<io::Error> for RSLoxError {
    fn from(err: io::Error) -> Self {
        RSLoxError::IOError(err)
    }
}
