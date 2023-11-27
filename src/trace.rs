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

        // Check if the stack top corresponds to the given pop
        if pop != StackAlphabet::Epsilon {
            if copy.pda.get_stack_top() == pop {
                if let Some(StackAlphabet::Symbol(stack_top_symbol)) = copy.pda.stack.pop() {
                    if stack_top_symbol
                        .chars()
                        .next()
                        .expect("Expected something on stack")
                        .is_ascii_uppercase()
                    {
                        log::debug!("pop {}", pop);
                        log::debug!("Expanded a variable.");
                        copy.bound = self.bound - 1;
                    }
                }
            } else {
                log::trace!(
                    "Need stack top as {} but found stack top as {}. Returning FALSE.",
                    pop,
                    copy.pda.get_stack_top()
                );
                return false;
            }
        }

        if push != StackAlphabet::EOF && push != StackAlphabet::Epsilon {
            log::debug!("push {}", push);
            copy.pda.stack.push(push);
        }

        if read != StackAlphabet::Epsilon {
            if copy.input_string.len() == 0 {
                log::trace!("No input string to read. Returning false");
                // Cannot read anything from the input, but we wanted to read something.
                return false;
            } else {
                let start_char = copy
                    .input_string
                    .chars()
                    .next()
                    .expect("Expected something")
                    .to_string();

                if let StackAlphabet::Symbol(want_read) = read {
                    if start_char == want_read {
                        // lop off the start char in the input.
                        copy.input_string = &copy.input_string[1..];
                    }
                } else {
                    log::trace!("Wanted to read something, checked it. It was not epsilon, but no chars found?");
                    return false;
                }
            }
        } else {
            log::trace!("Read epsilon.");
        }

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
