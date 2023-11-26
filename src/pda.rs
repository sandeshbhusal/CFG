use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
};

use crate::{cfg::CFG, token::Token};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
enum StackAlphabet {
    #[default]
    Epsilon,
    Symbol(Alphabet),
    EOF,
}

impl Display for StackAlphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackAlphabet::Epsilon => f.write_char('e'),
            StackAlphabet::Symbol(s) => f.write_str(s),
            StackAlphabet::EOF => f.write_char('$'),
        }
    }
}

type Alphabet = String;
type PDAState = usize;

#[derive(Debug, Default)]
struct PDA {
    stack: Vec<StackAlphabet>,
    states: HashSet<PDAState>,
    start_state: PDAState,
    final_state: PDAState,
    table:
        HashMap<PDAState, HashMap<(StackAlphabet, StackAlphabet), Vec<(StackAlphabet, PDAState)>>>,
}

impl From<CFG> for PDA {
    fn from(cfg: CFG) -> Self {
        let mut pda = PDA::default();

        pda.stack.push(StackAlphabet::EOF);

        let default_state = 0;
        pda.states.insert(default_state);

        let variables = cfg.variables.clone();
        let terminals = cfg.alphabet.clone();

        for terminal in terminals {
            pda.add_transition(
                default_state,
                StackAlphabet::Symbol(terminal.clone()),
                StackAlphabet::Symbol(terminal.clone()),
                StackAlphabet::Epsilon,
                default_state,
            );
        }

        for variable in variables {
            // Read the variable off the stack, read nothing off the input string,
            // and read the transitions for the variable from the cfg.
            if let Some(rules) = cfg.productions.get(&Token::Variable(variable.clone())) {
                for rule in rules {
                    match rule {
                        crate::productionrule::ProductionRule::Sequence(sequence) => {
                            let mut current_state = pda.get_new_state_id();
                            pda.add_transition(
                                default_state,
                                StackAlphabet::Epsilon,
                                StackAlphabet::Symbol(variable.clone()),
                                StackAlphabet::Epsilon,
                                current_state,
                            );

                            let mut seq = sequence.clone();
                            seq.reverse();

                            for symb_index in 0..(seq.len() - 1) {
                                let symb = seq[symb_index].clone();
                                let next_state = pda.get_new_state_id();

                                pda.add_transition(
                                    current_state,
                                    StackAlphabet::Epsilon,
                                    StackAlphabet::Epsilon,
                                    StackAlphabet::Symbol(symb.get_inner()),
                                    next_state,
                                );

                                pda.states.insert(next_state);
                                current_state = next_state;
                            }

                            pda.add_transition(
                                current_state,
                                StackAlphabet::Epsilon,
                                StackAlphabet::Epsilon,
                                StackAlphabet::Symbol(seq[seq.len() - 1].get_inner()),
                                default_state,
                            )
                        }
                        crate::productionrule::ProductionRule::Empty => {
                            pda.add_transition(
                                default_state,
                                StackAlphabet::Epsilon,
                                StackAlphabet::Symbol(variable.clone()),
                                StackAlphabet::Epsilon,
                                default_state,
                            );
                        }
                    }
                }
            } else {
                println!(
                    "The variable {} does not have any production rules!",
                    variable
                );
            }
        }

        pda.start_state = 0;
        let final_state = pda.get_new_state_id();
        pda.add_transition(
            0,
            StackAlphabet::Epsilon,
            StackAlphabet::EOF,
            StackAlphabet::Epsilon,
            final_state,
        );

        pda.final_state = final_state;

        pda
    }
}

impl PDA {
    fn add_transition(
        &mut self,
        from_state: PDAState,
        read_string: StackAlphabet,
        stack_top: StackAlphabet,
        stack_write: StackAlphabet,
        to_state: PDAState,
    ) {
        // Get the table for the start state.
        let transitions = self.table.entry(from_state).or_default();
        let transitions_entry = transitions.entry((read_string, stack_top)).or_default();
        transitions_entry.push((stack_write, to_state));
    }

    fn get_new_state_id(&mut self) -> PDAState {
        let new_state_id = self.states.len();
        self.states.insert(new_state_id);

        new_state_id
    }
}

impl Display for PDA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = {
            f.write_str("Pushdown Automata Description:\n")?;
            f.write_fmt(format_args!("States: {:?}\n", self.states))?;
            f.write_fmt(format_args!("Start state: {:?}\n", self.start_state))?;
            f.write_fmt(format_args!("Final state: {:?}\n", self.final_state))?;

            for entry in self.table.clone() {
                f.write_fmt(format_args!("State: {}\n", entry.0))?;
                f.write_fmt(format_args!("------------------\n"))?;
                for entry in entry.1 {
                    f.write_fmt(format_args!(
                        "Read: {}\t Stack Top: {}\t Transition to: {:?}\n",
                        entry.0 .0, entry.0 .1, entry.1
                    ))?;
                }
                f.write_char('\n')?;
            }
        };

        Ok(())
    }
}
