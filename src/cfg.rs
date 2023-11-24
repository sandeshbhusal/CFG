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
        // in the production rules. If it does, then replace it with its alternatives
        // in the rule.
        // E.g. If A -> ! | ab, B -> aA, new rule is B -> a | aab
        for epsilon_variable in epsilon_variables {
            for (variable, production_rules) in rules.iter() {
                // Prevent a loop like A -> ! | aA.
                if variable == &epsilon_variable {
                    return;
                }

                for rule in production_rules {
                    if rule.contains(&epsilon_variable) {
                        log::info!("The variable {} contains an epsilon variable {} in rule {}", variable, epsilon_variable, rule);

                        rule.replace_variable(
                            epsilon_variable.to_owned(),
                            rules.get(&epsilon_variable).unwrap().to_owned(),
                        );
                    }
                }
            }
        }

        // Done.
        dbg!(&rules);
    }
}

#[test]
fn quick_test() {
    let mut cfg = r#"
        0
        S->Acd
        S->kAd
        S->!
        A->c
        A->d
        A->!
        A->aA
    "#
    .parse::<CFG>()
    .unwrap();

    cfg.remove_empty_productions();
}