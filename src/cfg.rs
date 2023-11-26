#![allow(unused)]

use std::collections::HashMap;

use boilermates::boilermates;

use crate::{productionrule::ProductionRule, token::Token};

#[derive(Debug, Default)]
pub struct CFG {
    pub productions: HashMap<Token, Vec<ProductionRule>>,
    pub inverted_dep_graph: HashMap<Token, Vec<Token>>,
    pub alphabet: Vec<String>,
    pub variables: Vec<String>
}

impl CFG {
    /// Update the dependency graph for this CFG.
    pub(crate) fn update_dependency_graph(&mut self) {

    }

    /// Prune off the rules in the CFG with the following conditions:
    ///
    /// 1. All rules that are unreachble from other rules.
    /// 2. All rules that produce only Epsilon.
    /// 3. All rules that have a production with (2)
    pub fn prune_rules(&mut self) {}

    pub fn variables_with_empty_productions(&self) -> Vec<Token> {
        // Find all variables of form A -> e | ..
        self.productions
            .iter()
            .filter_map(|(variable, production_rules_list)| {
                // We allow "S" to have a null production.
                if variable == &Token::Variable("S".into()) {
                    None
                } else {
                    production_rules_list
                        .contains(&ProductionRule::Empty)
                        .then_some(variable.to_owned())
                }
            })
            .collect()
    }

    pub fn remove_empty_productions(&mut self) {
        env_logger::init();

        let mut epsilon_variables = self.variables_with_empty_productions().clone();
        let mut rules = self.productions.clone();

        // Delete all epsilon-containing productions (except start state 'S') from the rules
        // for the epsilon variables.
        epsilon_variables.iter().for_each(|variable| {
            let entries = rules.get_mut(variable).expect("Should not happen");
            *entries = entries
                .clone()
                .iter()
                .filter_map(|production_rule| {
                    (!production_rule.is_empty()).then_some(production_rule.to_owned())
                })
                .collect::<_>();
        });

        // Now for every variable in the epsilon_variables, check if it appears
        // in the production rules.
        // E.g. If A -> ! | ab, B -> aA, new rule is B -> a | aA
        let mut new_productions: HashMap<Token, Vec<ProductionRule>> = HashMap::default();
        for epsilon_variable in epsilon_variables {

            for (variable, production_rules) in rules.iter_mut() {
                // Prevent a loop like A -> ! | aA.
                if variable == &epsilon_variable {
                    return;
                }

                let mut new_rules = production_rules.clone();

                for rule in production_rules {
                    if rule.contains(&epsilon_variable) {
                        // println!("The variable {} contains an epsilon variable {} in rule {}", variable, epsilon_variable, rule);
                        new_rules
                            .extend(rule.replace_nullable_variable(epsilon_variable.to_owned()));
                    } else {
                        new_rules.push(rule.clone());
                    }
                }

                new_productions.insert(variable.to_owned(), new_rules);
            }
        }

        self.productions = new_productions;
    }
}
