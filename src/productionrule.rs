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

impl ProductionRule {
    // Returns if the productionRule contains the given component (can be terminal/variable)
    pub fn contains(&self, input: &Token) -> bool {
        match self {
            ProductionRule::Sequence(rules) => rules.contains(input),
            ProductionRule::Empty => false,
        }
    }

    // Check if a production rule is empty.
    pub fn is_empty(&self) -> bool {
        matches!(self, ProductionRule::Empty)
    }

    // Extract variables from a given rule.
    pub fn get_variables(&self) -> Vec<Token> {
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

    // Remove a variable from this rule.
    pub fn remove_variable(&mut self, variable: &Token) {
        match self {
            ProductionRule::Sequence(s) => {
                s.retain(|t| t != variable);
            }
            ProductionRule::Empty => todo!("Cannot be done for an empty sequence."),
        }
    }

    /// Replaces a variable in the given rule, with epsilon
    pub fn replace_nullable_variable(&self, epsilon_variable: Token) -> Vec<Self> {
        let mut new_rules = vec![];

        log::debug!(
            "Going to replace variable {} in rule {} ",
            epsilon_variable,
            self
        );
        new_rules.push(self.clone());

        let mut stack = vec![self.clone()];

        while let Some(rule) = stack.pop() {
            if rule.contains(&epsilon_variable) {
                if let ProductionRule::Sequence(rule) = rule {
                    // Generate a new rule excluding the variable and push it to stack + list.
                    let mut split = rule.split(|t| t == &epsilon_variable);
                    let mut first = split.next().expect("Cannot happen. BUG.").to_vec();
                    let second = split.next().expect("Cannot happen. BUG.").to_vec();
                    first.extend(second);

                    if first.len() > 0 {
                        println!("Generated a new; from: {} to: {:?}", self.clone(), first.clone());
                        let new_rule = ProductionRule::Sequence(first);
                        stack.push(new_rule.clone());
                        new_rules.push(new_rule);
                    }
                } else {
                    panic!("Cannot happen. BUG.")
                }
            }
        }

        new_rules
    }
}
