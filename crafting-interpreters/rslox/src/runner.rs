use crate::{lexer::Lexer, RSLoxResult};

pub struct Runner;

impl Runner {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, source: &str) -> RSLoxResult<()> {
        let lexer = Lexer::new(source);
        let tokens = lexer.tokenize();
        for token in tokens {
            println!("{:?}", token);
        }
        Ok(())
    }
}
