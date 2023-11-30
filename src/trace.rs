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

    #[inline]
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
            StackAlphabet::Symbol(Token::Terminal(t))
                if copy.pda.get_stack_top() == pop
                    && copy.input.len() > 0
                    && read
                        == StackAlphabet::Symbol(Token::Terminal(copy.input[..1].to_string())) =>
            {
                log::trace!("pop {}", t);
                copy.pda.stack.pop();
                copy.input = &copy.input[1..];
            }

            // On variable: pop the variable and decrement bound,
            // as we used up a unit of production budget.
            StackAlphabet::Symbol(Token::Variable(v)) if copy.pda.get_stack_top() == pop => {
                log::trace!("pop {}", v);
                copy.pda.stack.pop();
                copy.bound -= 1;
            }

            // On EOF pop, we simply pop it.
            StackAlphabet::EOF if copy.pda.get_stack_top() == pop => {
                log::trace!("pop $");
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
        if let StackAlphabet::Symbol(s) = push.clone() {
            log::trace!("push {}", s);
            copy.pda.stack.push(push);
        }

        copy.state = current_state;
        return copy.trace();
    }

    pub fn trace(&mut self) -> bool {
        let check_accept = || self.input.len() == 0 && self.state == self.pda.final_state;

        // Check bound, base case.
        match self.bound <= 0 {
            true => check_accept(),
            false => self
                .pda
                .table
                .get(&self.state)
                .map(|state_transitions| {
                    state_transitions.iter().any(|((read, pop), actions)| {
                        actions.iter().any(|(push, next_state)| {
                            self.run_copy(
                                push.clone(),
                                pop.clone(),
                                read.clone(),
                                next_state.to_owned(),
                            )
                        })
                    })
                })
                .unwrap_or(check_accept()),
        }
    }
}
