use anyhow::Result;

use crate::lexer::{Lexer, LexerToken};

mod formula;
mod lexer;

pub fn solve() -> Result<()> {
    let mut lexer = Lexer::new();
    let tokens: Vec<LexerToken> = lexer.tokenize("begin [r1]p1 & [r2]p1 & ~DCp1 end")?;

    println!("Tokens: {:?}", tokens);

    Ok(())
}
