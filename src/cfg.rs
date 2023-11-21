#![allow(unused)]

use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Hash, Clone)]
pub(crate) enum Production {
    Empty,
    Terminal(String),
    Variable(String),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct ProductionRule(Vec<Production>);
impl ProductionRule {
    // Returns if the productionRule contains the given component (can be terminal/variable)
    fn contains(&self, input: &Production) -> bool {
        return self.0.contains(input);
    }
}

#[derive(Debug, Default)]
pub struct CFG {
    productions: HashMap<Production, Vec<ProductionRule>>,
}

impl CFG {
    pub(crate) fn variables_with_empty_productions(&self) -> Vec<Production> {
        // Find all variables of form A -> e | ..
        let variables_with_epsilon_productions = self.productions
            .iter()
            .filter_map(|(variable, productions)| {
                if productions.iter().any(|t| t.contains(&Production::Empty)) {
                    Some(variable.to_owned())
                } else {
                    None
                }
            })
            .collect();
        
        // Find all variables which contain all variables that contain the set above.
        // For e.g. if A -> e | .. and B -> e | .. and C -> AB, this grabs "C".
        todo!("Left to be implemented.");

        variables_with_epsilon_productions
    }

    fn remove_empty_productions(&mut self) {
        let epsilon_variables = self.variables_with_empty_productions().clone();

        // Delete all epsilon-containing productions (except start state 'S').
        epsilon_variables.iter().for_each(|var| {
            if var == &Production::Variable("S".into()) {
                return;
            }

            let entries = self.productions.get_mut(var).expect("Should not happen");
            *entries = entries
                .clone()
                .iter()
                .filter_map(|t| {
                    if t.contains(&Production::Empty) {
                        None
                    } else {
                        Some(t.to_owned())
                    }
                })
                .collect::<_>();
        });

        todo!("Remaining to be implemented.");
        epsilon_variables.iter().for_each(|variable| {
            // For every production in the productions for this CFG, check if it contains this variable.
            // If it does, it needs to be replaced with something better ;)
            for (production_variable, production_rule) in self.productions.iter_mut() {
                if production_variable == variable {
                    continue;
                }

                // let mut new_rules = vec![];

                for rule in production_rule.iter_mut() {
                    if rule.contains(variable) {

                    }
                }
            }
        });
    }
}

#[cfg(test)]
mod test_cfg {
    use super::{Production, CFG};

    #[test]
    fn filter_empty_productions() {
        let cfg_string = r#"
            7
            S->XY
            X->AX
            X->!
            A->a
            Y->BY
            Y->!
            B->b
        "#;

        let my_cfg = cfg_string.parse::<CFG>().expect("Building a CFG Failed.");
        let empty_variables = &[
            Production::Variable("X".into()),
            Production::Variable("Y".into()),
        ];
        let mut variables = my_cfg.variables_with_empty_productions();
        variables.sort();

        assert!(variables.iter().eq(empty_variables.iter()));
    }

    #[test]
    fn remove_empty_productions() {
        let mut my_cfg = r#"
            7
            S->XY
            S->!
            X->AX
            X->!
            A->a
            Y->BY
            Y->!
            B->b        
        "#
        .parse::<CFG>()
        .expect("Building CFG Failed.");
        my_cfg.remove_empty_productions();
    }
}

impl FromStr for CFG {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rval: CFG = Default::default();
        // Parse the string into lines, and while it starts
        // with an uppercase symbol, generate a "production".
        s.lines().for_each(|line| {
            let line = line.trim();
            if line.starts_with(|t: char| t.is_ascii_uppercase()) {
                let mut split = line.split("->");
                let variable = split.next().expect("Variable expected.").to_owned();
                let produces = split.next().expect("Production expected.").to_owned();

                let entry = rval
                    .productions
                    .entry(Production::Variable(variable))
                    .or_default();

                if produces == "!" {
                    entry.push(ProductionRule(vec![Production::Empty]));
                    return;
                }

                // Loop through the rhs and split it into terminals and variables.
                let mut accum: String = String::new();
                let mut sequence: Vec<Production> = Vec::new();

                for t in produces.chars() {
                    if t.is_ascii_uppercase() {
                        // Dump the accumulator.
                        if accum.len() > 0 {
                            let accumulated = Production::Terminal(accum.clone());
                            sequence.push(accumulated);
                        }
                        sequence.push(Production::Variable(t.to_string()));
                        accum.clear();
                    } else {
                        accum.push(t);
                    }
                }

                if !accum.is_empty() {
                    sequence.push(Production::Terminal(accum));
                }

                (entry).push(ProductionRule(sequence));
            }
        });

        dbg!(&rval);

        Ok(rval)
    }
}
