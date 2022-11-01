/*!
 * The rslox compiler executable.
 */
use std::{env::args, process::exit};

use rslox::{globals::globals, init_rslox, run_file, run_prompt, RSLoxResult};

const EXITCODE_INCORRECT_USAGE: i32 = 64;

fn main() -> RSLoxResult<()> {
    let mut full_args = args();
    let command_name = full_args.next().expect("Impossible!");
    let args: Vec<String> = full_args.collect();
    if args.len() > 1 {
        println!("Usage: {} [script]", command_name);
        exit(EXITCODE_INCORRECT_USAGE);
    }

    init_rslox();
    println!("has_error: {}", globals().read().unwrap().has_error);

    if args.len() == 1 {
        run_file(&args[0])
    } else {
        run_prompt()
    }
}
