use log::debug;

use crate::{
    pda::{PDAState, StackAlphabet, PDA},
    token::Token,
};

#[derive(Debug, Clone, Default)]
pub struct PDAConfiguration<'a> {
    pda: PDA,
    state: PDAState,
    input: &'a str,
    bound: usize,
    produced: String,
}

impl<'a> PDAConfiguration<'a> {
    pub fn with_pda(pda: PDA, input_string: &'a str, bound: usize, produced: String) -> Self {
        Self {
            pda: pda.clone(),
            state: pda.start_state,
            input: input_string,
            bound,
            produced,
        }
    }

    pub fn trace(&mut self) -> bool {
        // To terminate, we MUST exhaust the entire input.
        // we also terminate when no match happens within "X" variable expansions.
        if self.bound <= 0 {
            // Check input should be EOF.
            if self.input.len() == 0 && self.state == self.pda.final_state {
                log::trace!("Input string finished. In final state (or reachable)");
                return true;
            } else {
                log::trace!("Input string finished. Still not in final state.");
                return false;
            }
        }

        // Otherwise, generate a new transition following all transitions here.
        if let Some(transitions) = self.pda.table.get(&self.state) {
            for ((read, pop), action) in transitions {
                for (stack_push, next_state) in action {
                    // dbg!(&read, &pop, &stack_push, &next_state);
                    // Generate a new trace from here.
                    let mut copy = self.clone();

                    // Do the popping first.
                    match pop {
                        crate::pda::StackAlphabet::Epsilon => {
                            // Nothing to do while popping epsilon.
                        }
                        crate::pda::StackAlphabet::Symbol(token) => {
                            match token {
                                crate::token::Token::Terminal(terminal) => {
                                    // To pop a terminal, we need to read the same terminal.
                                    // make sure readable string is present.
                                    if copy.input.len() == 0 {
                                        continue;
                                    }

                                    // Ensure the character is at TOS.
                                    if &copy.pda.get_stack_top() != pop {
                                        continue;
                                    }

                                    // make sure the read command and input's first chars match.
                                    if &StackAlphabet::Symbol(Token::Terminal(terminal.into()))
                                        != read
                                    {
                                        continue;
                                    }

                                    // then move the input one char to right, and pop the terminal.
                                    copy.input = &copy.input[1..];
                                    debug!("pop {}", terminal);
                                    copy.pda.stack.pop();
                                }
                                crate::token::Token::Variable(variable) => {
                                    // Ensure the variable is on the TOS.
                                    if &copy.pda.get_stack_top() != pop {
                                        continue;
                                    }

                                    // Variable is popped means bound decreases.
                                    copy.bound -= 1;
                                    // pop the variable out.
                                    debug!("pop {}", variable);
                                    copy.pda.stack.pop();
                                }
                            }
                        }
                        crate::pda::StackAlphabet::EOF => {
                            // Pop the EOF from the stack. Nothing to do here.
                            copy.pda.stack.pop();
                        }
                    }

                    for symbol in stack_push {
                        match symbol {
                            StackAlphabet::Symbol(t) => {
                                debug!("push {}", t);
                                copy.pda.stack.push(symbol.clone());
                            }
                            _ => {
                                panic!("Can never happen")
                            }
                        }
                    }

                    copy.state = next_state.clone();

                    if copy.trace() {
                        return true;
                    }
                }
            }
            return false;
        } else {
            // The PDA reached to final state without reading the entire string. This condition should return false.
            return false;
        }
    }
}
