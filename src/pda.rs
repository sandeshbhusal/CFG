use crate::{cfg::CFG, token::Token};

#[derive(Default, Debug, PartialEq, Clone)]
enum StackAlphabet {
    #[default]
    EOF,
    Token(Token),
}

#[derive(Debug, Clone)]
pub struct PDA<'a> {
    cfg: &'a CFG,
    stack: Vec<StackAlphabet>,
    budget: usize,
}

impl<'a> PDA<'a> {
    pub fn with_cfg(cfg: &'a CFG, budget: usize) -> Self {
        let mut rval = Self {
            cfg,
            budget,
            stack: vec![],
        };

        rval.stack.push(StackAlphabet::EOF);
        rval.stack
            .push(StackAlphabet::Token(Token::Variable("S".into())));
        rval
    }

    pub fn trace(&mut self, input: &str) -> bool {
        log::trace!("the input is '{}'", input);
        if self.budget == 0 {
            // Budget exhausted.
            return input.len() == 0 && self.stack.is_empty();
        } else {
            // Get the first symbol on the stack.
            let popped = self.stack.pop();
            match popped {
                Some(StackAlphabet::Token(Token::Terminal(t))) if input.len() > 0 => {
                    log::trace!("pop {}", t);
                    // if t matches input string initial char, then proceed to simulate.
                    if &input[..1] == t.as_str() {
                        let input = &input[1..];
                        self.trace(input)
                    } else {
                        log::debug!("Stack top {} and input string {} do not match", t, input);
                        false
                    }
                }
                Some(StackAlphabet::Token(Token::Variable(t))) => {
                    log::trace!("pop {}", t);
                    // if t matches a variable on top of stack, it is popped,
                    // and all possible expansions of the variable are tried one after another.
                    let productions = self
                        .cfg
                        .productions
                        .get(&Token::Variable(t.clone()))
                        .expect("Variable does not have productions?");

                    for production in productions {
                        log::debug!("Production rule: {:?}", production);
                        let mut copy = self.clone();
                        copy.budget -= 1;
                        match production {
                            crate::productionrule::ProductionRule::Sequence(seq) => {
                                let mut r = seq.clone();
                                r.reverse();
                                for symb in r {
                                    log::trace!("push {}", symb);
                                    copy.stack.push(StackAlphabet::Token(symb));
                                }
                            }
                            _ => {}
                        }

                        if copy.trace(input) {
                            return true;
                        }
                    }

                    log::trace!("No production rule for this variable can derive the string.");
                    false
                }
                Some(StackAlphabet::EOF) => {
                    // If popping a EOF, check if input is finished.
                    log::trace!("pop $");
                    input.len() == 0
                }
                _ => false,
            }
        }
    }
}
