pub mod lexer_tokens;

use lexer_tokens::LexerToken;

use anyhow::{Result, anyhow};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenizationError {
    #[error("[!] Invalid token '{0}' on line {1}, column {2}")]
    InvalidToken(char, usize, usize),
}

pub struct Lexer {
    line: usize,
    column: usize,
    position: usize,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            line: 0,
            column: 0,
            position: 0,
        }
    }

    fn _consume(&mut self, buffer: &[u8], pattern: &str) -> Result<()> {
        for byte in pattern.as_bytes().iter() {
            if buffer[self.position] != *byte {
                return Err(anyhow!(TokenizationError::InvalidToken(
                    *byte as char,
                    self.line,
                    self.column
                )));
            }
            self.position += 1;
            self.column += 1;
        }
        Ok(())
    }

    fn _is_exact_match(&self, buffer: &[u8], pattern: &str) -> bool {
        for (index, byte) in pattern.as_bytes().iter().enumerate() {
            if buffer[self.position + index] != *byte {
                return false;
            }
        }
        true
    }

    fn _parse_id(&mut self, buffer: &[u8]) -> Result<usize> {
        let mut pattern = String::new();
        let mut position_clone = self.position;

        while position_clone < buffer.len() && buffer[position_clone].is_ascii_digit() {
            pattern.push(buffer[position_clone] as char);
            position_clone += 1;
        }

        if position_clone == self.position {
            return Err(anyhow!(TokenizationError::InvalidToken(
                buffer[self.position] as char,
                self.line,
                self.column
            )));
        }

        self._consume(buffer, &pattern)?;

        Ok(pattern.parse::<usize>()?)
    }

    pub fn tokenize(&mut self, content: &str) -> Result<Vec<LexerToken>> {
        let buffer = content.as_bytes();
        let mut tokens: Vec<LexerToken> = Vec::new();

        while self.position < buffer.len() {
            let current = buffer[self.position] as char;

            match current {
                'b' => {
                    self._consume(buffer, "begin")?;
                    tokens.push(LexerToken::Begin);
                }
                'e' => {
                    self._consume(buffer, "end")?;
                    tokens.push(LexerToken::End);
                }
                't' => {
                    self._consume(buffer, "true")?;
                    tokens.push(LexerToken::True);
                }
                'f' => {
                    self._consume(buffer, "false")?;
                    tokens.push(LexerToken::False);
                }
                'p' => {
                    self._consume(buffer, "p")?;
                    let id = self._parse_id(buffer)?;
                    tokens.push(LexerToken::Atom(id));
                }
                '~' => {
                    self._consume(buffer, "~")?;
                    tokens.push(LexerToken::Not);
                }
                '[' => {
                    self._consume(buffer, "[r")?;
                    let id = self._parse_id(buffer)?;
                    self._consume(buffer, "]")?;
                    tokens.push(LexerToken::Box(id));
                }
                '-' => {
                    self._consume(buffer, "->")?;
                    tokens.push(LexerToken::Implication);
                }
                '<' => {
                    if self._is_exact_match(buffer, "<->") {
                        self._consume(buffer, "<->")?;
                        tokens.push(LexerToken::Biconditional);
                    } else if self._is_exact_match(buffer, "<r") {
                        self._consume(buffer, "<r")?;
                        let id = self._parse_id(buffer)?;
                        self._consume(buffer, ">")?;
                        tokens.push(LexerToken::Diamond(id));
                    } else {
                        return Err(anyhow!(TokenizationError::InvalidToken(
                            current,
                            self.line,
                            self.column
                        )));
                    }
                }
                'D' => {
                    self._consume(buffer, "D")?;
                    tokens.push(LexerToken::DistributedKnowledge);
                }
                'C' => {
                    self._consume(buffer, "C")?;
                    tokens.push(LexerToken::CommonKnowledge);
                }
                '&' => {
                    self._consume(buffer, "&")?;
                    tokens.push(LexerToken::And);
                }
                '|' => {
                    self._consume(buffer, "|")?;
                    tokens.push(LexerToken::Or);
                }
                '(' => {
                    self._consume(buffer, "(")?;
                    tokens.push(LexerToken::LeftParen);
                }
                ')' => {
                    self._consume(buffer, ")")?;
                    tokens.push(LexerToken::RightParen);
                }
                '\n' => {
                    self._consume(buffer, "\n")?;
                    self.line += 1;
                    self.column = 0;
                }
                ' ' | '\t' | '\r' => {
                    self.position += 1;
                    self.column += 1;
                }
                _ => {
                    return Err(anyhow!(TokenizationError::InvalidToken(
                        current,
                        self.line,
                        self.column
                    )));
                }
            };
        }

        tokens.push(LexerToken::EoF);
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_keywords() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("begin").unwrap();

        assert_eq!(vec![LexerToken::Begin, LexerToken::EoF], result);

        let mut lexer = Lexer::new();
        let result = lexer.tokenize("end").unwrap();

        assert_eq!(vec![LexerToken::End, LexerToken::EoF], result);
    }

    #[test]
    fn tokenize_constants() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("true").unwrap();

        assert_eq!(vec![LexerToken::True, LexerToken::EoF], result);

        let mut lexer = Lexer::new();
        let result = lexer.tokenize("false").unwrap();

        assert_eq!(vec![LexerToken::False, LexerToken::EoF], result);
    }

    #[test]
    fn tokenize_atom() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("p1").unwrap();

        assert_eq!(vec![LexerToken::Atom(1), LexerToken::EoF], result);
    }

    #[test]
    fn tokenize_not_atom() {
        let mut lexer = Lexer::new();
        let expected = vec![LexerToken::Not, LexerToken::Atom(3), LexerToken::EoF];
        let result = lexer.tokenize("~p3").unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn tokenize_necessity() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("[r1]").unwrap();

        assert_eq!(vec![LexerToken::Box(1), LexerToken::EoF], result);
    }

    #[test]
    fn tokenize_possibility() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("<r1>").unwrap();

        assert_eq!(vec![LexerToken::Diamond(1), LexerToken::EoF], result);
    }

    #[test]
    fn tokenize_knowledge_operators() {
        let mut lexer = Lexer::new();
        let result = lexer.tokenize("D").unwrap();

        assert_eq!(
            vec![LexerToken::DistributedKnowledge, LexerToken::EoF],
            result
        );

        let mut lexer = Lexer::new();
        let result = lexer.tokenize("C").unwrap();

        assert_eq!(vec![LexerToken::CommonKnowledge, LexerToken::EoF], result);
    }

    #[test]
    fn tokenize_parentheses() {
        let mut lexer = Lexer::new();
        let expected = vec![
            LexerToken::LeftParen,
            LexerToken::RightParen,
            LexerToken::EoF,
        ];
        let result = lexer.tokenize("()").unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn basic_tokenization() {
        let mut lexer = Lexer::new();
        let expected = vec![
            LexerToken::Begin,
            LexerToken::Atom(2),
            LexerToken::End,
            LexerToken::EoF,
        ];
        let result = lexer.tokenize("begin p2 end").unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn complex_tokenization() {
        let mut lexer = Lexer::new();
        let expected = vec![
            LexerToken::Begin,
            LexerToken::Box(1),
            LexerToken::LeftParen,
            LexerToken::Atom(1),
            LexerToken::Implication,
            LexerToken::Atom(2),
            LexerToken::RightParen,
            LexerToken::And,
            LexerToken::Box(2),
            LexerToken::Atom(1),
            LexerToken::And,
            LexerToken::Not,
            LexerToken::DistributedKnowledge,
            LexerToken::CommonKnowledge,
            LexerToken::Atom(1),
            LexerToken::End,
            LexerToken::EoF,
        ];
        let result = lexer
            .tokenize("begin [r1](p1 -> p2) & [r2]p1 & ~DCp1 end")
            .unwrap();

        assert_eq!(expected, result);
    }
}
