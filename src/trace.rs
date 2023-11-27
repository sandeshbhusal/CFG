use crate::pda::{PDAState, StackAlphabet, PDA};

#[derive(Debug, Clone)]
pub struct PDAInstance<'a> {
    pda: PDA,
    current_state: PDAState,
    input_string: &'a str,
    bound: usize,
}

impl<'a> PDAInstance<'a> {
    pub fn with_pda(pda: PDA, input_string: &'a str, bound: usize) -> Self {
        Self {
            pda: pda.clone(),
            current_state: pda.start_state,
            input_string,
            bound,
        }
    }

    pub(crate) fn run_copy(
        &self,
        push: StackAlphabet,
        pop: StackAlphabet,
        read: StackAlphabet,
        current_state: PDAState,
    ) -> bool {
        log::trace!("Running a copy");
        let mut copy = self.clone();

        match (read, pop.clone()) {
            (StackAlphabet::Epsilon, StackAlphabet::Epsilon) => {
                // If nothing to read or pop, just push if required.
                if matches!(push, StackAlphabet::Symbol(_)) {
                    log::debug!("push {}", push);
                    copy.pda.stack.push(push);
                }
            }

            (StackAlphabet::Epsilon, StackAlphabet::Symbol(k)) => {
                // If nothing to read, but something to pop, pop it first
                // if the stack top matches. Then proceed to push to stack if required (this condition can never happen).
                if copy.pda.get_stack_top() == StackAlphabet::Symbol(k) {
                    log::debug!("pop {}", &pop);
                    copy.pda.stack.pop();

                    if push != StackAlphabet::Epsilon {
                        panic!("This condition can never happen. pop + push");
                    }
                } else {
                    // Pop requested, but stack does not match required pop.
                    return false;
                }
            }

            (StackAlphabet::Epsilon, StackAlphabet::EOF) => {
                // If nothing to read, but we need to pop off the stacktop
                if copy.pda.get_stack_top() == StackAlphabet::EOF {
                    log::debug!("pop eof {}", &pop);
                    copy.pda.stack.pop();

                    if push != StackAlphabet::Epsilon {
                        panic!("This condition can never happen. pop + push");
                    }
                } else {
                    // Pop requested, but stack does not match required pop.
                    return false;
                }
            }

            (StackAlphabet::Symbol(read_string), StackAlphabet::Symbol(pop_string)) => {
                // If something to read, make sure that the stack top matches
                // and pop it as well for our special PDA.
                if copy.input_string.len() == 0 {
                    return false;
                }

                let input_first_char = copy.input_string.chars().next().expect("Expected some chars").to_string();

                if copy.pda.get_stack_top() == StackAlphabet::Symbol(pop_string) && input_first_char == read_string {
                    // Pop it.
                    copy.pda.stack.pop();
                    // Move input ptr to right.
                    copy.input_string = &copy.input_string[1..];
                } else {
                    // Pop requested, but stack top does not match.
                    return false;
                }
            }

            _ => panic!("Cannot happen."),
        };

        copy.current_state = current_state;
        return copy.trace();
    }

    pub fn trace(&mut self) -> bool {
        // Check bound, base case.
        if self.bound == 0
            || (self.input_string.len() == 0
                && self
                    .pda
                    .ep_reachable(self.current_state)
                    .contains(&self.pda.final_state))
        {
            // Check input should be EOF.
            if self.input_string.len() == 0 {
                log::trace!("Input string finished. In final state (or reachable)");
                true
            } else {
                log::trace!("Input string finished. Still not in final state.");
                false
            }
        } else {
            // Trace this input.
            // Check what are the reachable states from the current state
            if let Some(transitions_here) = self.pda.table.get(&self.current_state) {
                // follow each transition.
                for ((read_char, need_stack_top), possible_next) in transitions_here.iter() {
                    // We can trivially check this without reading string or popping from stack.
                    for (push_to_stack, next_state) in possible_next.iter() {
                        if self.run_copy(
                            push_to_stack.to_owned(),
                            need_stack_top.clone(),
                            read_char.clone(),
                            *next_state,
                        ) {
                            return true;
                        }
                    }
                }

                return false;
            } else {
                // No more transitions on this state. Check if final reachable, otherwise, return false.
                self.pda
                    .ep_reachable(self.current_state)
                    .contains(&self.pda.final_state)
                    && self.input_string.len() == 0
            }
        }
    }
}
