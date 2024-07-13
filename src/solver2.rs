use crate::state::{get_char_idx, State};

fn distance(from_char: &u8, to_char: &u8) -> (char, u8) {
    let asc = if from_char < to_char {
        to_char - from_char
    } else {
        27 - from_char + to_char
    };
    let dsc = 27 - asc;

    if asc < dsc {
        ('+', asc)
    } else {
        ('-', dsc)
    }
}

pub fn convert_to_brainfuck(s: &str) -> String {
    let tokens: Vec<u8> = s.chars().map(get_char_idx).collect();

    let mut state = State::new();

    for token in tokens.iter() {
        let rune = state.get_current_rune();
        let (action, iteration) = distance(&rune, token);
        for _ in 0..iteration {
            state.play(action);
        }
        state.play('.');
    }

    state.get_output().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(distance(&1, &26), ('-', 2));
        assert_eq!(distance(&1, &4), ('+', 3));
        assert_eq!(distance(&22, &4), ('+', 9));
        assert_eq!(distance(&22, &18), ('-', 4));
    }
}
