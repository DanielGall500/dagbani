use std::collections::HashMap;

// transition: (state, input) -> next state
// outputs: (state, input) -> output
pub struct FST {
    transitions: HashMap<(String, String), String>,
    outputs: HashMap<(String, String), String>,
}

impl FST {
    pub fn new() -> FST {
        FST {
            transitions: HashMap::new(),
            outputs: HashMap::new(),
        }
    }

    pub fn add_transition(
        &mut self,
        from_state: &str,
        to_state: &str,
        input_symbol: &str,
        output_symbol: &str,
    ) {
        self.transitions
            .insert((from_state.to_string(), input_symbol.to_string()), to_state.to_string());
        self.outputs
            .insert((from_state.to_string(), input_symbol.to_string()), output_symbol.to_string());
    }

    pub fn process(&self, word: Vec<&str>) -> Result<String, String> {
        let mut state = "start".to_string();
        let mut output = String::new();

        // iterate over each character in the string
        for symbol in word {
            // probably dont need to_string?
            let symbol_str = symbol.to_string();

            // ensure there is an available transition
            if let Some(next_state) = self.transitions.get(&(state.clone(), symbol_str.clone())) {
                // append output to result
                if let Some(output_symbol) = self.outputs.get(&(state.clone(), symbol_str.clone())) {
                    output.push_str(output_symbol);
                }

                // move to next state
                state = next_state.clone();
            } else {
                return Err(format!(
                    "Invalid transition from state '{}' with input '{}'",
                    state, symbol_str
                ));
            }
        }

        Ok(output)
    }
}
