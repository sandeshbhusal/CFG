use crate::token::Token;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum ProductionRule {
    Sequence(Vec<Token>),
    #[default]
    Empty,
}

impl ProductionRule {
    // Returns if the productionRule contains the given component (can be terminal/variable)
    pub(crate) fn contains(&self, input: &Token) -> bool {
        match self {
            ProductionRule::Sequence(rules) => rules.contains(input),
            ProductionRule::Empty => false,
        }
    }

    // Check if a production rule is empty.
    pub(crate) fn is_empty(&self) -> bool {
        matches!(self, ProductionRule::Empty)
    }

    // Extract variables from a given rule.
    pub(crate) fn get_variables(&self) -> Vec<Token> {
        match self {
            ProductionRule::Sequence(sequence) => sequence
                .iter()
                .filter_map(|t| t.is_variable().then_some(t.to_owned()))
                .collect::<Vec<Token>>(),

            ProductionRule::Empty => {
                panic!("Cannot get variables from an empty production. BUG.")
            }
        }
    }

    /// Replace a variable in the production rule with available terminals terminals.
    pub(crate) fn replace_variable(&self, variable: Token, terminals: Vec<Token>) -> Vec<Self> {
        vec![]
    }
}
