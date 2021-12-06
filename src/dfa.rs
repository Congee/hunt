use anyhow::Result;
use std::collections::HashMap;
use std::collections::HashSet;

struct State(String);
struct Symbol(String);

pub struct Dfa<State, Symbol> {
    states: Vec<State>,
    input_symbols: HashSet<Symbol>,
    transitions: HashMap<State, HashMap<Symbol, State>>,
    initial_state: State,
    final_states: HashSet<State>,
}

trait Automaton {
    type State;

    fn start(&self) -> Self::State;
    fn accepts(&self, state: &Self::State, input: Symbol) -> Self::State;
}

pub mod dfa {
    pub use super::Dfa;
}

#[derive(Hash, PartialEq, Eq, Clone)]
enum MyState {
    S0,
    S1,
    S2,
}

#[derive(Hash, PartialEq, Eq)]
enum MySymbol {
    Zero,
    One,
}

impl Dfa<MyState, MySymbol> {
    fn new() -> Self {
        use MyState::*;
        use MySymbol::*;

        Dfa {
            states: vec![S0, S1, S2],
            input_symbols: HashSet::from([Zero, One]),
            transitions: HashMap::from([
                (S0, HashMap::from([(Zero, S0), (One, S1)])),
                (S1, HashMap::from([(Zero, S0), (One, S2)])),
                (S2, HashMap::from([(Zero, S2), (One, S1)])),
            ]),
            initial_state: MyState::S0,
            final_states: HashSet::from([MyState::S1]),
        }
    }

    fn accepts(&self, input: &[MySymbol]) -> bool {
        let mut state = self.initial_state.clone();
        for symbol in input {
            state = self.transitions[&state][symbol].clone();
        }
        self.final_states.contains(&state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use MySymbol::*;

    #[test]
    fn test_dfa() {
        let dfa = Dfa::<MyState, MySymbol>::new();
        assert!(dfa.accepts(&[Zero, One]));
        assert!(!dfa.accepts(&[Zero, One, One]));
    }
}
