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

    /// Replace a variable in the production rule with available terminals to generate a list of production rules.
    pub fn replace_variable(
        &self,
        epsilon_variable: Token,
        terminals: Vec<ProductionRule>,
    ) -> Vec<Self> {
        log::debug!(
            "Going to replace variable {} in rule {} ",
            epsilon_variable,
            self
        );
        let mut new_rules = vec![];

        let mut this_rule = self.clone();
        this_rule.remove_variable(&epsilon_variable);
        new_rules.push(this_rule);

        let mut stack = vec![];
        stack.push(self.clone());
        while let Some(rule) = stack.pop() {
            dbg!("Working on rule:", &rule);
            // Check if rule does not contain the variable, if so, it can be sent back.
            // else, it needs further processing and replacement.
            if let ProductionRule::Sequence(rule) = rule {
                if rule.contains(&epsilon_variable) {
                    // Process this rule.
                    // Replace the first occurrence with "terminals" vector.
                    // E.g. for A -> kAdA, it splits the rule as k + A + dA, replaces the first "A"
                    // and pushes it onto the stack.
                    let mut split = rule.split(|r| r == &epsilon_variable);
                    let left = split.next().unwrap();
                    let right = split.next().unwrap();

                    // Generate more rules with the sequence.
                    for terminal in terminals.clone() {
                        if let ProductionRule::Sequence(seq) = terminal {
                            let mut new_sequence = vec![];
                            new_sequence.extend(left.to_vec());
                            new_sequence.extend(seq);
                            new_sequence.extend(right.to_vec());

                            // TODO: Check if this works. So far, I'm just adding it to the stack.
                            // and not to the final rules list. This might be a blunder.
                            // test with:
                            /*
                                A -> c | d | !
                                B -> kAlA

                                Then, B should have:
                                B -> kl | kAl | klA
                                A should have:
                                A -> c | d
                            */
                            stack.push(ProductionRule::Sequence(new_sequence));
                        } else {
                            panic!("The epsilon must be wiped out.")
                        }
                    }
                } else {
                    // Retain this rule.
                    new_rules.push(ProductionRule::Sequence(rule));
                }
            } else {
                panic!("Cannot be called for an epsilon rule. BUG.");
            }
        }

        new_rules
    }
}
