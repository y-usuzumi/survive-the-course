use std::{fs, io, path::Path};

use crate::RSLoxResult;

pub fn read_file<P: AsRef<Path>>(path: &str) -> RSLoxResult<String> {
    Ok(fs::read_to_string(path)?)
}
