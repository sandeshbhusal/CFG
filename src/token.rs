use std::fmt::Display;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub enum Token {
    Terminal(String),
    Variable(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Terminal(t) => {
                f.write_str(format!("({})", t).as_str()).unwrap();
            }
            Token::Variable(v) => {
                f.write_str(format!("({})", v).as_str()).unwrap();
            }
        }

        Ok(())
    }
}

impl Token {
    pub fn is_variable(&self) -> bool {
        matches!(self, Token::Variable(_))
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Token::Terminal(_))
    }
}
