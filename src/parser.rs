use crate::{formula::Formula, lexer::lexer_tokens::LexerToken};
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

                let not_left = Box::new(Formula::Not(left.clone()));
                let right = self._parse_biconditional()?;
                let not_right = Box::new(Formula::Not(right.clone()));

                let left_implies_right = Box::new(Formula::Or(vec![not_left, right]));
                let right_implies_left = Box::new(Formula::Or(vec![not_right, left]));

                Ok(Box::new(Formula::Or(vec![
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

                let not_left = Box::new(Formula::Not(left));
                let right = self._parse_implication()?;

                Ok(Box::new(Formula::Or(vec![not_left, right])))
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

                Ok(Box::new(Formula::Or(vec![left, right])))
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

                Ok(Box::new(Formula::And(vec![left, right])))
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

                Ok(Box::new(Formula::Not(subformula)))
            }
            Some(LexerToken::Box(id)) => {
                let agent_id = *id;
                self._consume(LexerToken::Box(agent_id))?;

                let subformula = self._parse_unary_operators()?;

                Ok(Box::new(Formula::Knowledge(agent_id, subformula)))
            }
            Some(LexerToken::Diamond(id)) => {
                let agent_id = *id;
                self._consume(LexerToken::Diamond(agent_id))?;

                let subformula = self._parse_unary_operators()?;
                let neg_subformula = Box::new(Formula::Not(subformula));

                Ok(Box::new(Formula::Not(Box::new(Formula::Knowledge(
                    agent_id,
                    neg_subformula,
                )))))
            }
            Some(LexerToken::DistributedKnowledge) => {
                self._consume(LexerToken::DistributedKnowledge)?;

                let subformula = self._parse_unary_operators()?;

                Ok(Box::new(Formula::DistributedKnowledge(subformula)))
            }
            Some(LexerToken::CommonKnowledge) => {
                self._consume(LexerToken::CommonKnowledge)?;

                let subformula = self._parse_unary_operators()?;

                Ok(Box::new(Formula::CommonKnowledge(subformula)))
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
                Ok(Box::new(Formula::True))
            }
            Some(LexerToken::False) => {
                self._consume(LexerToken::False)?;
                Ok(Box::new(Formula::False))
            }
            Some(LexerToken::Atom(id)) => {
                let temp_id = *id;
                self._consume(LexerToken::Atom(temp_id))?;
                Ok(Box::new(Formula::Atom(temp_id)))
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
