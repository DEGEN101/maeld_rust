use anyhow::Result;

use crate::{
    lexer::{Lexer, lexer_tokens::LexerToken},
    parser::Parser,
};

mod formula;
mod lexer;
mod parser;

pub fn solve() -> Result<()> {
    let mut lexer = Lexer::new();
    let tokens: Vec<LexerToken> = lexer.tokenize("begin [r1]p1 & [r2]p1 & ~DCp1 end")?;

    println!("Tokens: {:?}", tokens);

    let mut parser = Parser::new(&tokens);
    let formula = parser.parse()?;
    println!("Formula: {}", formula);

    Ok(())
}
