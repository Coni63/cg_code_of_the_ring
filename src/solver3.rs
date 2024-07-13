use crate::state::{get_char_idx, State};

fn distance_v(from_char: &u8, to_char: &u8) -> (char, u8) {
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

fn distance_h(from_col: usize, to_col: usize) -> (char, u8) {
    let asc = if from_col < to_col {
        to_col - from_col
    } else {
        30 - from_col + to_col
    };
    let dsc = 30 - asc;

    if asc < dsc {
        ('>', asc as u8)
    } else {
        ('<', dsc as u8)
    }
}

fn generate_string(c1: char, n1: u8, c2: char, n2: u8) -> String {
    let mut result = String::new();

    result.push_str(&c1.to_string().repeat(n1 as usize));
    result.push_str(&c2.to_string().repeat(n2 as usize));

    result
}

pub fn convert_to_brainfuck(s: &str) -> String {
    let tokens: Vec<u8> = s.chars().map(get_char_idx).collect();

    let mut state = State::new();

    for token in tokens.iter() {
        let mut best_action = String::new();
        let mut length = u8::MAX;
        let current_x = state.get_rune_idx();
        for x in 0..30 {
            let rune = state.get_rune(x);
            let (action_h, iteration_h) = distance_h(current_x, x);
            let (action_v, iteration_v) = distance_v(&rune, token);

            if iteration_h + iteration_v < length {
                length = iteration_h + iteration_v;
                best_action = generate_string(action_h, iteration_h, action_v, iteration_v);
            }
        }

        for action in best_action.chars() {
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
    fn test_distance_v() {
        assert_eq!(distance_v(&1, &26), ('-', 2));
        assert_eq!(distance_v(&1, &4), ('+', 3));
        assert_eq!(distance_v(&22, &4), ('+', 9));
        assert_eq!(distance_v(&22, &18), ('-', 4));
    }

    #[test]
    fn test_distance_h() {
        assert_eq!(distance_h(1, 26), ('<', 5));
        assert_eq!(distance_h(1, 4), ('>', 3));
        assert_eq!(distance_h(28, 4), ('>', 6));
        assert_eq!(distance_h(22, 18), ('<', 4));
    }

    #[test]
    fn test_generate_string() {
        assert_eq!(generate_string('>', 3, '+', 2), ">>>++");
        assert_eq!(generate_string('<', 3, '-', 2), "<<<--");
    }
}
