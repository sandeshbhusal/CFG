use std::str::FromStr;

use crate::cfg::{ProductionRule, Token, CFG};

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
                    .entry(Token::Variable(variable))
                    .or_default();

                if produces == "!" {
                    entry.push(ProductionRule::Empty);
                    return;
                }

                // Loop through the rhs and split it into terminals and variables.
                let mut accum: String = String::new();
                let mut sequence: Vec<Token> = Vec::new();

                for t in produces.chars() {
                    if t.is_ascii_uppercase() {
                        // Dump the accumulator.
                        if accum.len() > 0 {
                            let accumulated = Token::Terminal(accum.clone());
                            sequence.push(accumulated);
                        }
                        sequence.push(Token::Variable(t.to_string()));
                        accum.clear();
                    } else {
                        accum.push(t);
                    }
                }

                if !accum.is_empty() {
                    sequence.push(Token::Terminal(accum));
                }

                (entry).push(ProductionRule::Sequence(sequence));
            }
        });

        Ok(rval)
    }
}
