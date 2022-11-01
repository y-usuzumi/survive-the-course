/*!
 * Various utility functions.
 */
use std::{fs, path::Path};

use crate::RSLoxResult;

pub(crate) fn read_file<P: AsRef<Path>>(path: &str) -> RSLoxResult<String> {
    Ok(fs::read_to_string(path)?)
}
