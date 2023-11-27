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

    pub(crate) fn run_copy(
        &self,
        push: StackAlphabet,
        pop: StackAlphabet,
        read: StackAlphabet,
        current_state: PDAState,
    ) -> bool {
        let mut copy = PDAConfiguration::with_pda(
            self.pda.clone(),
            self.input,
            self.bound,
            self.produced.clone(),
        );

        // Pop before push
        if let StackAlphabet::Symbol(s) = pop.clone() {
            if StackAlphabet::Symbol(s.clone()) == copy.pda.get_stack_top() {
                match s {
                    crate::token::Token::Terminal(_) => {
                        // Pop the terminal from the input string as well as top of stack.
                        copy.pda.stack.pop();
                        // Make sure there is input string to be read here.
                        if copy.input.len() == 0 {
                            return false;
                        }
                        // Make sure we read what was required to be read.
                        if read == StackAlphabet::Symbol(Token::Terminal((&copy.input[..1]).into()))
                        {
                            copy.produced +=
                                copy.input.chars().next().unwrap().to_string().as_str();
                            copy.input = &copy.input[1..];
                        } else {
                            // Read string does not match the transition read symbol.
                            return false;
                        }
                    }
                    crate::token::Token::Variable(_) => {
                        // Pop the variable from the top of stack.
                        println!("{}", copy.produced.clone());
                        copy.pda.stack.pop();
                        // Copy derived a variable at this point. Bound is decremented.
                        copy.bound -= 1;
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

        copy.state = current_state;
        return copy.trace();
    }

    pub fn trace(&mut self) -> bool {
        // Check bound, base case.
        if self.bound <= 0
            || (self.input.len() == 0
                && self
                    .pda
                    .ep_reachable(self.state)
                    .contains(&self.pda.final_state))
        {
            // Check input should be EOF.
            if self.input.len() == 0 {
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
