/*!
 * The rslox Lox compiler.
 */
pub mod color;
pub mod error;
pub mod globals;
pub mod lexer;
pub mod message;
pub mod runner;
pub mod token;
pub mod util;

use std::io::{stdin, stdout, BufRead, Write};

use color::{colorize, Color};
use error::RSLoxError;
use globals::init_globals;

use crate::runner::Runner;

pub type RSLoxResult<R> = Result<R, RSLoxError>;

pub fn init_rslox() {
    init_globals();
}

pub fn run_file(script: &str) -> RSLoxResult<()> {
    let content = util::read_file::<&str>(script)?;
    let colorized_log = colorize(&format!("Running script {}", script), Color::LightGreen);
    println!("{}", colorized_log);
    println!("{}", content);
    Ok(())
}

fn print_prompt() {
    print!("> ");
    if let Err(err) = stdout().flush() {
        panic!("{}", err);
    }
}

pub fn run_prompt() -> RSLoxResult<()> {
    println!("REPL!");
    print_prompt();
    for line in stdin().lock().lines() {
        match line {
            Ok(s) => {
                let result = Runner::new().run(&s)?;
                println!("{:?}", result);
                print_prompt();
            }
            // FIXME: better error handling
            Err(err) => panic!("{}", err),
        }
    }
    Ok(())
}
