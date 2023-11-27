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

        // Pop before push
        if let StackAlphabet::Symbol(s) = pop.clone() {
            if StackAlphabet::Symbol(s.clone()) == copy.pda.get_stack_top() {
                if s.chars().next().unwrap().is_ascii_uppercase() {
                    // Popping variable.
                    copy.pda.stack.pop();
                } else {
                    // Popping terminal.
                    // Cannot pop terminal without read as well
                    // All transitions with pop terminal should have read
                    // as well.
                    copy.pda.stack.pop();
                    if copy.input_string.len() == 0 {
                        return false;
                    }
                    if StackAlphabet::Symbol(copy.input_string.chars().next().unwrap().to_string())
                        == read
                    {
                        copy.input_string = &copy.input_string[1..];
                    } else {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }

        // Push if there's anything to push.
        if let StackAlphabet::Symbol(_) = push.clone() {
            copy.pda.stack.push(push);
        }

        // Pop epsilon if it should be popped.
        if StackAlphabet::EOF == pop {
            copy.pda.stack.pop();
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
                for ((read_char, pop_from_stack), possible_next) in transitions_here.iter() {
                    for (push_to_stack, next_state) in possible_next.iter() {
                        if self.run_copy(
                            push_to_stack.to_owned(),
                            pop_from_stack.clone(),
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
