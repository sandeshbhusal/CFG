use std::collections::HashMap;

use crate::{pda, productionrule::ProductionRule, token::Token};

#[derive(Debug, Default, Clone)]
pub struct CFG {
    pub productions: HashMap<Token, Vec<ProductionRule>>,
    pub alphabet: Vec<Token>,
    pub variables: Vec<Token>,
}

impl CFG {
    pub fn trace_string(&self, input: &str, budget: isize) -> bool {
        let mut pda = pda::PDA::with_cfg(&self, budget);
        pda.trace(input)
    }

    pub fn longest_derivation_length(&self) -> usize {
        self.productions
            .iter()
            .map(|(_, k)| k.len())
            .max()
            .unwrap_or(0)
    }
}
