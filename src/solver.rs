use crate::state::{get_char_idx, State};

pub fn convert_to_brainfuck(s: &str) -> String {
    let tokens: Vec<u8> = s.chars().map(get_char_idx).collect();

    let mut state = State::new();

    for token in tokens.iter() {
        while state.get_current_rune() != *token {
            state.play('+');
        }
        state.play('.');
    }

    state.get_output().to_string()
}
