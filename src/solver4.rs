use crate::state::{get_char_idx, State};

fn distance_v(from_char: &u8, to_char: &u8) -> String {
    let asc = if from_char < to_char {
        to_char - from_char
    } else {
        27 - from_char + to_char
    };
    let dsc = 27 - asc;

    if asc < dsc {
        String::from("+").repeat(asc as usize)
    } else {
        String::from("-").repeat(dsc as usize)
    }
}

fn distance_h(from_col: usize, to_col: usize) -> String {
    let asc = if from_col < to_col {
        to_col - from_col
    } else {
        30 - from_col + to_col
    };
    let dsc = 30 - asc;

    if asc < dsc {
        String::from(">").repeat(asc)
    } else {
        String::from("<").repeat(dsc)
    }
}

fn reset_column(from_char: &u8) -> String {
    let d = distance_v(from_char, &0);
    if d.len() <= 3 {
        d
    } else {
        String::from("[+]")
    }
}

fn next_free_column(state: &State, from_col: usize) -> Option<String> {
    let mut closest: Option<String> = None;
    let mut best_dist = 999;
    for i in 0..30 {
        if state.get_rune(i) == 0 {
            let d = distance_h(from_col, i);
            if d.len() < best_dist {
                best_dist = d.len();
                closest = Some(d);
            }
        }
    }

    match closest {
        Some(d) => {
            if d.len() <= 3 {
                Some(d)
            } else {
                let dir = d.chars().next().unwrap();
                Some(format!("[{}]", dir))
            }
        }
        None => None,
    }
}

fn best_distance_v(from_char: &u8, to_char: &u8) -> String {
    let normal = distance_v(from_char, to_char);
    let mut reset = reset_column(from_char);
    reset.push_str(&distance_v(&0, to_char));

    if reset.len() < normal.len() {
        reset
    } else {
        normal
    }
}

fn is_next_free_column(state: &State, from_col: usize, to_col: usize) -> bool {
    for i in from_col..to_col {
        if state.get_rune(i) == 0 {
            return false;
        }
    }
    true
}

fn best_distance_h(state: &State, from_col: usize, to_col: usize) -> String {
    let normal = distance_h(from_col, to_col);

    if (normal.len() >= 3)
        && (state.get_rune(to_col) == 0)
        && is_next_free_column(state, from_col, to_col)
    {
        let dir = normal.chars().next().unwrap();
        format!("[{}]", dir)
    } else {
        normal
    }
}

pub fn convert_to_brainfuck(s: &str) -> String {
    let tokens: Vec<u8> = s.chars().map(get_char_idx).collect();

    let mut state = State::new();

    for token in tokens.iter() {
        let mut best_action = String::new();
        let mut length = usize::MAX;
        let current_x = state.get_rune_idx();
        for x in 0..30 {
            let rune = state.get_rune(x);
            let action_h = best_distance_h(&state, current_x, x);
            let action_v = best_distance_v(&rune, token);

            if action_h.len() + action_v.len() < length {
                length = action_h.len() + action_v.len();
                best_action.clear();
                best_action.push_str(&action_h);
                best_action.push_str(&action_v);
            }
        }

        best_action.push('.');
        state.play(&best_action);
    }

    state.get_output().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_v() {
        assert_eq!(distance_v(&1, &26), String::from("--"));
        assert_eq!(distance_v(&1, &4), String::from("+++"));
        assert_eq!(distance_v(&22, &4), String::from("+++++++++"));
        assert_eq!(distance_v(&22, &18), String::from("----"));
    }

    #[test]
    fn test_distance_h() {
        assert_eq!(distance_h(1, 26), String::from("<<<<<"));
        assert_eq!(distance_h(1, 4), String::from(">>>"));
        assert_eq!(distance_h(28, 4), String::from(">>>>>>"));
        assert_eq!(distance_h(22, 18), String::from("<<<<"));
    }
}
