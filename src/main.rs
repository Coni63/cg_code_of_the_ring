use std::io;

mod solver;
mod solver2;
mod state;

use solver::convert_to_brainfuck;
use solver2::convert_to_brainfuck2;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let magic_phrase = input_line.trim_matches('\n');

    let brainfuck = convert_to_brainfuck(magic_phrase);
    println!("{}", brainfuck);
}
