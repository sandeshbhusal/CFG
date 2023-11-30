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
}

impl<'a> PDAConfiguration<'a> {
    pub fn with_pda(pda: PDA, input_string: &'a str, bound: usize) -> Self {
        Self {
            pda: pda.clone(),
            state: pda.start_state,
            input: input_string,
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
        let mut copy = PDAConfiguration::with_pda(self.pda.clone(), self.input, self.bound);

        match pop {
            // On terminal: check stack top + begin of input string
            // and pop the input string front and the stack top.
            StackAlphabet::Symbol(Token::Terminal(_))
                if copy.pda.get_stack_top() == pop
                    && copy.input.len() > 0
                    && read
                        == StackAlphabet::Symbol(Token::Terminal(copy.input[..1].to_string())) =>
            {
                copy.pda.stack.pop();
                copy.input = &copy.input[1..];
            }

            // On variable: pop the variable and decrement bound,
            // as we used up a unit of production budget.
            StackAlphabet::Symbol(Token::Variable(_)) if copy.pda.get_stack_top() == pop => {
                copy.pda.stack.pop();
                copy.bound -= 1;
            }

            // On EOF pop, we simply pop it.
            StackAlphabet::EOF => {
                copy.pda.stack.pop();
            }

            // On epsilon, do nothing.
            StackAlphabet::Epsilon => {}
            
            // If pop does not match any condition, return false early.
            _ => {
                return false;
            }
        };

        // Push if there's anything to push.
        if let StackAlphabet::Symbol(_) = push.clone() {
            copy.pda.stack.push(push);
        }

        copy.state = current_state;
        return copy.trace();
    }

    pub fn trace(&mut self) -> bool {
        // Check bound, base case.
        if self.bound <= 0 {
            // Check input should be EOF.
            if self.input.len() == 0
                && self
                    .pda
                    .ep_reachable(self.state)
                    .contains(&self.pda.final_state)
            {
                log::trace!("Input string finished. In final state (or reachable)");
                true
            } else {
                log::trace!("Input string finished. Still not in final state.");
                false
            }
        } else {
            // Trace this input.
            // Check what are the reachable states from the current state
            if let Some(transitions_here) = self.pda.table.get(&self.state) {
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
                    .ep_reachable(self.state)
                    .contains(&self.pda.final_state)
                    && self.input.len() == 0
            }
        }
    }
}
