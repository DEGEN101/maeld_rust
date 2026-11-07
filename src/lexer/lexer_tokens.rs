use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LexerToken {
    Begin,
    End,
    True,
    False,
    Atom(usize),
    Not,
    Diamond(usize),
    Box(usize),
    DistributedKnowledge,
    CommonKnowledge,
    And,
    Or,
    Implication,
    Biconditional,
    LeftParen,
    RightParen,
    EoF,
}

impl Display for LexerToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerToken::Begin => write!(f, "[Begin]"),
            LexerToken::End => write!(f, "[End]"),
            LexerToken::True => write!(f, "[True]"),
            LexerToken::False => write!(f, "[False]"),
            LexerToken::Atom(id) => write!(f, "[Atom({})]", id),
            LexerToken::Not => write!(f, "[Not]"),
            LexerToken::Diamond(id) => write!(f, "[Diamond({})]", id),
            LexerToken::Box(id) => write!(f, "[Box(){}]", id),
            LexerToken::DistributedKnowledge => write!(f, "[DistributedKnowledge]"),
            LexerToken::CommonKnowledge => write!(f, "[CommonKnowledge]"),
            LexerToken::And => write!(f, "[And]"),
            LexerToken::Or => write!(f, "[Or]"),
            LexerToken::Implication => write!(f, "[Implication]"),
            LexerToken::Biconditional => write!(f, "[Biconditional]"),
            LexerToken::LeftParen => write!(f, "[LeftParen]"),
            LexerToken::RightParen => write!(f, "[RightParen]"),
            LexerToken::EoF => write!(f, "[EoF]"),
        }
    }
}
