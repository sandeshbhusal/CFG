use std::{str::FromStr, collections::HashSet};

use crate::{cfg::CFG, productionrule::ProductionRule, token::Token};

impl FromStr for CFG {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rval: CFG = Default::default();

        let mut variables: HashSet<Token> = HashSet::default();
        let mut alphabet: HashSet<Token> = HashSet::default();

        // Parse the string into lines, and while it starts
        // with an uppercase symbol, generate a "production".
        s.lines().for_each(|line| {
            let line = line.trim();
            if line.starts_with(|t: char| t.is_ascii_uppercase()) {
                let mut split = line.split("->");
                let variable = split.next().expect("Variable expected.").to_owned();

                variables.insert(Token::Variable(variable.clone()));

                let produces = split.next().expect("Production expected.").to_owned();

                let entry = rval
                    .productions
                    .entry(Token::Variable(variable))
                    .or_default();

                if produces == "!" {
                    entry.push(ProductionRule::Empty);
                    return;
                }

                // Loop through the rhs and split it into terminals and variables.
                let mut sequence: Vec<Token> = Vec::new();

                for t in produces.chars() {
                    if t.is_ascii_uppercase() {
                        sequence.push(Token::Variable(t.to_string()));
                    } else {
                        alphabet.insert(Token::Terminal(t.to_string()));
                        sequence.push(Token::Terminal(t.to_string()));
                    }
                }

                (entry).push(ProductionRule::Sequence(sequence));
            }
        });

        rval.alphabet = alphabet.into_iter().collect::<Vec<_>>();
        rval.variables = variables.into_iter().collect::<Vec<_>>();

        Ok(rval)
    }
}
