use std::io;

mod state;

// mod solver;
// use solver::convert_to_brainfuck;

// mod solver2;
// use solver2::convert_to_brainfuck;

// mod solver3;
// use solver3::convert_to_brainfuck;

mod solver4;
use solver4::convert_to_brainfuck;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let magic_phrase = input_line.trim_matches('\n');
    eprintln!("{}", magic_phrase);

    let brainfuck = convert_to_brainfuck(magic_phrase);
    println!("{}", brainfuck);
}
