/*!
 * Module for error handling inside the compiler.
 */
use std::io;

#[derive(Debug)]
pub enum RSLoxError {
    Undefined1,
    Undefined2,
    IOError(io::Error),
}

impl From<io::Error> for RSLoxError {
    fn from(err: io::Error) -> Self {
        RSLoxError::IOError(err)
    }
}
