#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub enum Token {
    Terminal(String),
    Variable(String),
}

impl Token {
    pub(crate) fn is_variable(&self) -> bool {
        matches!(self, Token::Variable(_))
    }

    pub(crate) fn is_terminal(&self) -> bool {
        matches!(self, Token::Terminal(_))
    }
}