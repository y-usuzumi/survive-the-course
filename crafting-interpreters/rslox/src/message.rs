/*!
 * Module for human-readable output.
 */

use crate::globals::set_has_error;

pub(crate) fn error(line: i32, message: &str) {
    report(line, "", message);
}

pub(crate) fn report(line: i32, loc: &str, message: &str) {
    println!("[line {}] Error {}: {}", line, loc, message);
    set_has_error();
}
