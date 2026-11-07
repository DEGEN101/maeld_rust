use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum Formula {
    True,
    False,
    Atom(usize),
    Not(Box<Formula>),
    Knowledge(usize, Box<Formula>),
    DistributedKnowledge(Box<Formula>),
    CommonKnowledge(Box<Formula>),
    And(Vec<Box<Formula>>),
    Or(Vec<Box<Formula>>),
}

impl Display for Formula {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Formula::True => write!(f, "true"),
            Formula::False => write!(f, "false"),
            Formula::Atom(id) => write!(f, "p{}", id),
            Formula::Not(subformula) => match subformula.as_ref() {
                Formula::And(_) | Formula::Or(_) => write!(f, "~({})", subformula),
                _ => write!(f, "~{}", subformula),
            },
            Formula::Knowledge(id, subformula) => match subformula.as_ref() {
                Formula::And(_) | Formula::Or(_) => write!(f, "[r{}]({})", id, subformula),
                _ => write!(f, "[r{}]{}", id, subformula),
            },
            Formula::DistributedKnowledge(subformula) => match subformula.as_ref() {
                Formula::And(_) | Formula::Or(_) => write!(f, "D({})", subformula),
                _ => write!(f, "D{}", subformula),
            },
            Formula::CommonKnowledge(subformula) => match subformula.as_ref() {
                Formula::And(_) | Formula::Or(_) => write!(f, "C({})", subformula),
                _ => write!(f, "C{}", subformula),
            },
            Formula::And(subformulas) => {
                let result = subformulas
                    .iter()
                    .map(|formula| match formula.as_ref() {
                        Formula::And(_) | Formula::Or(_) => format!("({})", formula),
                        _ => format!("{}", formula),
                    })
                    .collect::<Vec<String>>()
                    .join(" & ");

                write!(f, "{}", result)
            }
            Formula::Or(subformulas) => {
                let result = subformulas
                    .iter()
                    .map(|formula| match formula.as_ref() {
                        Formula::And(_) | Formula::Or(_) => format!("({})", formula),
                        _ => format!("{}", formula),
                    })
                    .collect::<Vec<String>>()
                    .join(" | ");

                write!(f, "{}", result)
            }
        }
    }
}
