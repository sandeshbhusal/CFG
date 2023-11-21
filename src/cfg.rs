#![allow(unused)]

use std::collections::HashMap;

use crate::{productionrule::ProductionRule, token::Token};

#[derive(Debug, Default)]
pub struct CFG {
    pub productions: HashMap<Token, Vec<ProductionRule>>,
    pub dependency_graph: HashMap<Token, Vec<Token>>,
}

impl CFG {
    pub fn variables_with_empty_productions(&self) -> Vec<Token> {
        // Find all variables of form A -> e | ..
        let variables_with_epsilon_productions = self
            .productions
            .iter()
            .filter_map(|(variable, productions)| {
                productions
                    .contains(&ProductionRule::Empty)
                    .then_some(variable.to_owned())
            })
            .collect();

        // Find all variables which contain all variables that contain the set above.
        // For e.g. if A -> e | .. and B -> e | .. and C -> AB, this grabs "C".
        todo!("Left to be implemented.");

        variables_with_epsilon_productions
    }

    pub fn remove_empty_productions(&mut self) {
        let epsilon_variables = self.variables_with_empty_productions().clone();

        // Delete all epsilon-containing productions (except start state 'S').
        epsilon_variables.iter().for_each(|var| {
            if var == &Token::Variable("S".into()) {
                return;
            }

            let entries = self.productions.get_mut(var).expect("Should not happen");
            *entries = entries
                .clone()
                .iter()
                .filter_map(|t| t.is_empty().then_some(t.to_owned()))
                .collect::<_>();
        });

        // todo!("Remaining to be implemented.");
        epsilon_variables.iter().for_each(|variable| {
            // For every production in the productions for this CFG, check if it contains this variable.
            // If it does, it needs to be replaced with something better ;)
            for (production_variable, production_rule) in self.productions.iter_mut() {
                if production_variable == variable {
                    continue;
                }

                // let mut new_rules = vec![];

                for rule in production_rule.iter_mut() {
                    if rule.contains(variable) {}
                }
            }
        });
    }
}
