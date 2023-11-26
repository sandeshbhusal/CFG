use crate::pda::{PDA, PDAState};

pub struct PDAInstance<'a> {
    pda: PDA,
    current_state: PDAState,
    input_string: &'a str,
    bound: usize
}

impl<'a> PDAInstance<'a> {
    pub fn with_pda(pda: PDA, input_string: &'a str, bound: usize) -> Self {
        Self {
            pda: pda.clone(),
            current_state: pda.start_state,
            input_string,
            bound
        }
    }

    pub fn trace(&mut self) -> bool {
        // Check bound, base case.
        if self.bound == 0 {
            // Check input should be EOF.
            if self.input_string.len() == 0 {
                true
            } else {
                false
            }
        } else {
            // Trace this input.
            // Check what are the reachable states from the current state
            // i.e. find epsilon closure of the current state.
            let reachable_states = self.pda.ep_closure(self.current_state);
            for state in reachable_states {
                // Check if this particular state contains a transition
                // that consumes a string, pops the stack, or both.
            }
            false
        }
    }
}
