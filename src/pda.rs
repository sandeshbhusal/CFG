use std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Write},
};

use crate::{cfg::CFG, token::Token};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub(crate) enum StackAlphabet {
    #[default]
    Epsilon,
    Symbol(Token),
    EOF,
}

impl Display for StackAlphabet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackAlphabet::Epsilon => f.write_char('e'),
            StackAlphabet::Symbol(s) => f.write_str(&s.get_inner()),
            StackAlphabet::EOF => f.write_char('$'),
        }
    }
}

pub(crate) type PDAState = usize;

#[derive(Debug, Default, Clone)]
pub struct PDA {
    pub(crate) stack: Vec<StackAlphabet>,
    pub(crate) states: HashSet<PDAState>,
    pub(crate) start_state: PDAState,
    pub(crate) final_state: PDAState,
    pub(crate) table: HashMap<
        PDAState,
        HashMap<(StackAlphabet, StackAlphabet), Vec<(Vec<StackAlphabet>, PDAState)>>,
    >,
}

impl From<CFG> for PDA {
    fn from(cfg: CFG) -> Self {
        let mut pda = PDA::default();

        pda.stack.push(StackAlphabet::EOF);
        pda.stack
            .push(StackAlphabet::Symbol(Token::Variable("S".into())));

        let default_state = 0;
        pda.states.insert(default_state);

        let variables = cfg.variables.clone();
        let terminals = cfg.alphabet.clone();

        for terminal in terminals {
            pda.add_transition(
                default_state,
                StackAlphabet::Symbol(terminal.clone()),
                StackAlphabet::Symbol(terminal.clone()),
                vec![],
                default_state,
            );
        }

        for variable in variables {
            if let Some(rules) = cfg.productions.get(&variable) {
                for rule in rules {
                    match rule {
                        crate::productionrule::ProductionRule::Sequence(sequence) => {
                            let sequence = sequence
                                .iter()
                                .map(|k| StackAlphabet::Symbol(k.clone()))
                                .collect();

                            pda.add_transition(
                                default_state,
                                StackAlphabet::Epsilon,
                                StackAlphabet::Symbol(variable.clone()),
                                sequence,
                                default_state,
                            );
                        }
                        crate::productionrule::ProductionRule::Empty => {
                            pda.add_transition(
                                default_state,
                                StackAlphabet::Epsilon,
                                StackAlphabet::Symbol(variable.clone()),
                                vec![],
                                default_state,
                            );
                        }
                    }
                }
            } else {
                log::warn!(
                    "The variable {} does not have any production rules!",
                    variable
                );
            }
        }

        pda.start_state = 0;
        let final_state = pda.gen_new_state_id();
        pda.add_transition(
            default_state,
            StackAlphabet::Epsilon,
            StackAlphabet::EOF,
            vec![],
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
        read: StackAlphabet,
        pop: StackAlphabet,
        push: Vec<StackAlphabet>,
        to_state: PDAState,
    ) {
        // Get the table for the start state.
        let transitions = self.table.entry(from_state).or_default();
        let transitions_entry = transitions.entry((read, pop)).or_default();
        transitions_entry.push((push, to_state));
    }

    fn gen_new_state_id(&mut self) -> PDAState {
        let new_state_id = self.states.len();
        self.states.insert(new_state_id);

        new_state_id
    }

    pub(crate) fn get_stack_top(&self) -> StackAlphabet {
        return self.stack[self.stack.len() - 1].clone();
    }
}

impl Display for PDA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = {
            f.write_str("Pushdown Automata Description:\n")?;
            f.write_fmt(format_args!("States: {:?}\n", self.states))?;
            f.write_fmt(format_args!("Start state: {:?}\n", self.start_state))?;
            f.write_fmt(format_args!("Final state: {:?}\n\n", self.final_state))?;

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
