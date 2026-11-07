use crate::{custom_macros::boxed, formula::Formula, lexer::lexer_tokens::LexerToken};
use anyhow::{Result, anyhow};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("[!] Consuming '{0}' in position {1} but expected '{2}'")]
    InvalidTokenConsumption(LexerToken, usize, LexerToken),

    #[error("[!] Parser position out of bound '{0}'")]
    ParserPositionOutOfBound(usize),

    #[error("[!] Invalid token '{0}' in position {1}, expected '{2}'")]
    InvalidToken(LexerToken, usize, LexerToken),
}

pub struct Parser<'a> {
    _tokens: &'a [LexerToken],
    _position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [LexerToken]) -> Self {
        Self {
            _tokens: tokens,
            _position: 0,
        }
    }

    fn _peek(&mut self) -> Option<&LexerToken> {
        self._tokens.get(self._position)
    }

    fn _consume(&mut self, token: LexerToken) -> Result<()> {
        if token != self._tokens[self._position] {
            return Err(anyhow!(ParsingError::InvalidTokenConsumption(
                self._tokens[self._position],
                self._position,
                token,
            )));
        }

        self._position += 1;
        Ok(())
    }

    pub fn parse(&mut self) -> Result<Box<Formula>> {
        match self._peek() {
            Some(LexerToken::Begin) => self._consume(LexerToken::Begin)?,
            Some(token) => {
                return Err(anyhow!(ParsingError::InvalidToken(
                    *token,
                    self._position,
                    LexerToken::Begin
                )));
            }
            None => {
                return Err(anyhow!(ParsingError::ParserPositionOutOfBound(
                    self._position
                )));
            }
        };

        let formula = self._parse_biconditional()?;

        match self._peek() {
            Some(LexerToken::End) => self._consume(LexerToken::End)?,
            Some(token) => {
                return Err(anyhow!(ParsingError::InvalidToken(
                    *token,
                    self._position,
                    LexerToken::End
                )));
            }
            None => {
                return Err(anyhow!(ParsingError::ParserPositionOutOfBound(
                    self._position
                )));
            }
        };

        match self._peek() {
            Some(LexerToken::EoF) => self._consume(LexerToken::EoF)?,
            Some(token) => {
                return Err(anyhow!(ParsingError::InvalidToken(
                    *token,
                    self._position,
                    LexerToken::EoF
                )));
            }
            None => {
                return Err(anyhow!(ParsingError::ParserPositionOutOfBound(
                    self._position
                )));
            }
        };

        Ok(formula)
    }

    fn _parse_biconditional(&mut self) -> Result<Box<Formula>> {
        let left = self._parse_implication()?;

        match self._peek() {
            Some(LexerToken::Biconditional) => {
                self._consume(LexerToken::Biconditional)?;

                let not_left = boxed!(Formula::Not(left.clone()));
                let right = self._parse_biconditional()?;
                let not_right = boxed!(Formula::Not(right.clone()));

                let left_implies_right = boxed!(Formula::Or(vec![not_left, right]));
                let right_implies_left = boxed!(Formula::Or(vec![not_right, left]));

                Ok(boxed!(Formula::And(vec![
                    left_implies_right,
                    right_implies_left,
                ])))
            }
            Some(_) => Ok(left),
            None => {
                return Err(anyhow!(ParsingError::ParserPositionOutOfBound(
                    self._position
                )));
            }
        }
    }

    fn _parse_implication(&mut self) -> Result<Box<Formula>> {
        let left = self._parse_or()?;

        match self._peek() {
            Some(LexerToken::Implication) => {
                self._consume(LexerToken::Implication)?;

                let not_left = boxed!(Formula::Not(left));
                let right = self._parse_implication()?;

                Ok(boxed!(Formula::Or(vec![not_left, right])))
            }
            Some(_) => Ok(left),
            None => {
                return Err(anyhow!(ParsingError::ParserPositionOutOfBound(
                    self._position
                )));
            }
        }
    }

    fn _parse_or(&mut self) -> Result<Box<Formula>> {
        let left = self._parse_and()?;

        match self._peek() {
            Some(LexerToken::Or) => {
                self._consume(LexerToken::Or)?;

                let right = self._parse_or()?;

                Ok(boxed!(Formula::Or(vec![left, right])))
            }
            Some(_) => Ok(left),
            None => {
                return Err(anyhow!(ParsingError::ParserPositionOutOfBound(
                    self._position
                )));
            }
        }
    }

    fn _parse_and(&mut self) -> Result<Box<Formula>> {
        let left = self._parse_unary_operators()?;

        match self._peek() {
            Some(LexerToken::And) => {
                self._consume(LexerToken::And)?;

                let right = self._parse_and()?;

                Ok(boxed!(Formula::And(vec![left, right])))
            }
            Some(_) => Ok(left),
            None => {
                return Err(anyhow!(ParsingError::ParserPositionOutOfBound(
                    self._position
                )));
            }
        }
    }

    fn _parse_unary_operators(&mut self) -> Result<Box<Formula>> {
        match self._peek() {
            Some(LexerToken::Not) => {
                self._consume(LexerToken::Not)?;

                let subformula = self._parse_unary_operators()?;

                Ok(boxed!(Formula::Not(subformula)))
            }
            Some(LexerToken::Box(id)) => {
                let agent_id = *id;
                self._consume(LexerToken::Box(agent_id))?;

                let subformula = self._parse_unary_operators()?;

                Ok(boxed!(Formula::Knowledge(agent_id, subformula)))
            }
            Some(LexerToken::Diamond(id)) => {
                let agent_id = *id;
                self._consume(LexerToken::Diamond(agent_id))?;

                let subformula = self._parse_unary_operators()?;
                let neg_subformula = boxed!(Formula::Not(subformula));

                Ok(boxed!(Formula::Not(boxed!(Formula::Knowledge(
                    agent_id,
                    neg_subformula,
                )))))
            }
            Some(LexerToken::DistributedKnowledge) => {
                self._consume(LexerToken::DistributedKnowledge)?;

                let subformula = self._parse_unary_operators()?;

                Ok(boxed!(Formula::DistributedKnowledge(subformula)))
            }
            Some(LexerToken::CommonKnowledge) => {
                self._consume(LexerToken::CommonKnowledge)?;

                let subformula = self._parse_unary_operators()?;

                Ok(boxed!(Formula::CommonKnowledge(subformula)))
            }
            Some(_) => self._parse_primitives(),
            None => {
                return Err(anyhow!(ParsingError::ParserPositionOutOfBound(
                    self._position
                )));
            }
        }
    }

    fn _parse_primitives(&mut self) -> Result<Box<Formula>> {
        match self._peek() {
            Some(LexerToken::True) => {
                self._consume(LexerToken::True)?;
                Ok(boxed!(Formula::True))
            }
            Some(LexerToken::False) => {
                self._consume(LexerToken::False)?;
                Ok(boxed!(Formula::False))
            }
            Some(LexerToken::Atom(id)) => {
                let temp_id = *id;
                self._consume(LexerToken::Atom(temp_id))?;
                Ok(boxed!(Formula::Atom(temp_id)))
            }
            Some(LexerToken::LeftParen) => {
                self._consume(LexerToken::LeftParen)?;

                let formula = self._parse_biconditional()?;

                match self._peek() {
                    Some(LexerToken::RightParen) => {
                        self._consume(LexerToken::RightParen)?;
                        Ok(formula)
                    }
                    Some(token) => Err(anyhow!("[!] Expected ')', got: {}", token)),
                    None => {
                        return Err(anyhow!(ParsingError::ParserPositionOutOfBound(
                            self._position
                        )));
                    }
                }
            }
            Some(token) => Err(anyhow!(
                "[!] Expected 'True', 'False', 'Atom' or '(', got: {}",
                token
            )),
            None => {
                return Err(anyhow!(ParsingError::ParserPositionOutOfBound(
                    self._position
                )));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{custom_macros::fml_vec, formula::Formula, lexer::Lexer};

    fn tokenize(fml_str: &str) -> Result<Vec<LexerToken>> {
        let mut lexer = Lexer::new();
        lexer.tokenize(fml_str)
    }

    fn parse_formula(fml_str: &str) -> Result<Box<Formula>> {
        let tokens = tokenize(fml_str)?;
        let mut parser = Parser::new(&tokens);
        parser.parse()
    }

    #[test]
    fn parse_primitive_tokens() -> Result<()> {
        // Test parsing true
        let result = parse_formula("begin true end")?;
        assert_eq!(boxed!(Formula::True), result);

        // Test parsing false
        let result = parse_formula("begin false end")?;
        assert_eq!(boxed!(Formula::False), result);

        // Test parsing an atom
        let result = parse_formula("begin p1 end")?;
        assert_eq!(boxed!(Formula::Atom(1)), result);

        Ok(())
    }

    #[test]
    fn parse_modal_op_tokens() -> Result<()> {
        // Test parsing necessity operator
        let result = parse_formula("begin [r1]p1 end")?;
        let expected = boxed!(Formula::Knowledge(1, boxed!(Formula::Atom(1))));
        assert_eq!(expected, result);

        // Test parsing possibility operator
        let result = parse_formula("begin <r2>p3 end")?;
        let expected = boxed!(Formula::Not(boxed!(Formula::Knowledge(
            2,
            boxed!(Formula::Not(boxed!(Formula::Atom(3))))
        ))));
        assert_eq!(expected, result);

        // Test parsing distributed knowledge operator
        let result = parse_formula("begin Dp1 end")?;
        let expected = boxed!(Formula::DistributedKnowledge(boxed!(Formula::Atom(1))));
        assert_eq!(expected, result);

        // Test parsing common knowledge operator
        let result = parse_formula("begin Cp1 end")?;
        let expected = boxed!(Formula::CommonKnowledge(boxed!(Formula::Atom(1))));
        assert_eq!(expected, result);

        // Test parsing recursive modal operator
        let result = parse_formula("begin [r1][r1]p1 end")?;
        let expected = boxed!(Formula::Knowledge(
            1,
            boxed!(Formula::Knowledge(1, boxed!(Formula::Atom(1))))
        ));
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parse_and_token() -> Result<()> {
        // Test parsing simple `and`
        let result = parse_formula("begin p1 & p2 end")?;
        let expected = boxed!(Formula::And(fml_vec![Formula::Atom(1), Formula::Atom(2)]));
        assert_eq!(expected, result);

        // Test parsing complex `and`
        let result = parse_formula("begin p1 & p2 & p3 end")?;
        let inner_and = Formula::And(fml_vec![Formula::Atom(2), Formula::Atom(3)]);
        let expected = boxed!(Formula::And(fml_vec![Formula::Atom(1), inner_and]));
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parse_or_token() -> Result<()> {
        // Test parsing simple `or`
        let result = parse_formula("begin p1 | p2 end")?;
        let expected = boxed!(Formula::Or(fml_vec![Formula::Atom(1), Formula::Atom(2)]));
        assert_eq!(expected, result);

        // Test parsing complex `or`
        let result = parse_formula("begin p1 | p2 | p3 end")?;
        let inner_and = Formula::Or(fml_vec![Formula::Atom(2), Formula::Atom(3)]);
        let expected = boxed!(Formula::Or(fml_vec![Formula::Atom(1), inner_and]));
        assert_eq!(expected, result);

        Ok(())
    }

    #[test]
    fn parse_implication_token() -> Result<()> {
        // Test parsing and simplification of implication
        let result = parse_formula("begin p1 -> p2 end")?;
        let expected = boxed!(Formula::Or(fml_vec![
            Formula::Not(boxed!(Formula::Atom(1))),
            Formula::Atom(2)
        ]));
        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn parse_biconditional_token() -> Result<()> {
        // Test parsing and simplification of biconditional
        let result = parse_formula("begin p1 <-> p2 end")?;

        let p1_implies_p2 = Formula::Or(fml_vec![
            Formula::Not(boxed!(Formula::Atom(1))),
            Formula::Atom(2)
        ]);
        let p2_implies_p1 = Formula::Or(fml_vec![
            Formula::Not(boxed!(Formula::Atom(2))),
            Formula::Atom(1)
        ]);
        let expected = boxed!(Formula::And(fml_vec!(p1_implies_p2, p2_implies_p1)));

        assert_eq!(expected, result);
        Ok(())
    }

    #[test]
    fn parse_sample_formula() -> Result<()> {
        // Test parsing of sample formula
        let result = parse_formula("begin [r1]p1 & [r2]p1 & ~DCp1 end")?;

        let inner_and = Formula::And(fml_vec![
            Formula::Knowledge(2, boxed!(Formula::Atom(1))),
            Formula::Not(boxed!(Formula::DistributedKnowledge(boxed!(
                Formula::CommonKnowledge(boxed!(Formula::Atom(1)))
            )))),
        ]);
        let expected = boxed!(Formula::And(fml_vec![
            Formula::Knowledge(1, boxed!(Formula::Atom(1))),
            inner_and
        ]));

        assert_eq!(expected, result);
        Ok(())
    }
}
