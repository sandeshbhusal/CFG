use crate::token::Token;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum ProductionRule {
    Sequence(Vec<Token>),
    #[default]
    Empty,
}

impl std::fmt::Display for ProductionRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductionRule::Sequence(seq) => f.write_str(format!("{:?}", seq).as_str()),
            ProductionRule::Empty => f.write_str("-> e"),
        }
    }
}