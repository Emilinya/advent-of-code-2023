use std::env;

mod d1;
mod d2;
mod d3;
mod d4;
mod d5;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Expected a command line argument 'day'");
    }

    if let Ok(day) = args[1].parse() {
        match day {
            1 => d1::main(),
            2 => d2::main(),
            3 => d3::main(),
            4 => d4::main(),
            5 => d5::main(),
            other => panic!("Unknown day: {}", other),
        }
    } else {
        panic!("Could not parse day as int: {}", args[1]);
    }
}
