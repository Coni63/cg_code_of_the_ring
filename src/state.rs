use std::fmt::Debug;

pub struct State {
    runes: [u8; 30],
    rune_idx: usize,
    char_idx: usize,
    output: String,
}

impl State {
    pub fn new() -> Self {
        State {
            runes: [0; 30],
            rune_idx: 0,
            char_idx: 0,
            output: String::new(),
        }
    }

    pub fn get_output(&self) -> &str {
        &self.output
    }

    pub fn get_rune(&self, idx: usize) -> u8 {
        self.runes[idx]
    }

    pub fn get_current_rune(&self) -> u8 {
        self.runes[self.rune_idx]
    }

    pub fn get_rune_idx(&self) -> usize {
        self.rune_idx
    }

    pub fn play(&mut self, action: char) {
        match action {
            '>' => {
                self.rune_idx = self.rune_idx.wrapping_add(1).wrapping_rem(30);
            }
            '<' => {
                self.rune_idx = self.rune_idx.wrapping_add(29).wrapping_rem(30);
            }
            '.' => {
                self.char_idx += 1;
            }
            '+' => {
                self.runes[self.rune_idx] =
                    self.runes[self.rune_idx].wrapping_add(1).wrapping_rem(27);
            }
            '-' => {
                self.runes[self.rune_idx] =
                    self.runes[self.rune_idx].wrapping_add(26).wrapping_rem(27);
            }
            _ => (),
        }
        self.output.push(action);
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        State {
            runes: self.runes,
            rune_idx: self.rune_idx,
            char_idx: self.char_idx,
            output: self.output.clone(),
        }
    }
}

impl Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output: String = self.runes.iter().map(get_char_by_idx).collect();
        let position: String = self
            .runes
            .iter()
            .enumerate()
            .map(|(i, _)| if i == self.rune_idx { '^' } else { ' ' })
            .collect();
        write!(
            f,
            "Runes:\n{}\n{}\nRune index: {}\tChar index: {}",
            output, position, self.rune_idx, self.char_idx
        )
    }
}

pub fn get_char_idx(c: char) -> u8 {
    match c {
        ' ' => 0,
        'A'..='Z' => c as u8 - 64,
        _ => 0,
    }
}

pub fn get_char_by_idx(idx: &u8) -> char {
    match idx {
        0 => '_',
        i => (i + 64) as char,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_char_idx() {
        assert_eq!(get_char_idx(' '), 0);
        assert_eq!(get_char_idx('A'), 1);
        assert_eq!(get_char_idx('Z'), 26);
        assert_eq!(get_char_idx('a'), 0);
    }

    #[test]
    fn test_get_char_by_idx() {
        assert_eq!(get_char_by_idx(&0), '_');
        assert_eq!(get_char_by_idx(&1), 'A');
        assert_eq!(get_char_by_idx(&26), 'Z');
        assert_eq!(get_char_by_idx(&27), '[');
    }
}
