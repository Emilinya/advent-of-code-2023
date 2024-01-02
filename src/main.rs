use std::env;

mod utils;

macro_rules! create_main {
    ($($number: literal: $file: ident),*) => {
        $(mod $file;)*
        
        fn main() {
            let args: Vec<String> = env::args().collect();
            if args.len() != 2 {
                panic!("Expected a command line argument 'day'");
            }

            if let Ok(day) = args[1].parse() {
                match day {
                    $($number => $file::main(),)*
                    other => panic!("Unknown day: {}", other),
                }
            } else {
                panic!("Could not parse day as int: {}", args[1]);
            }
        }
    };
}

create_main!(1: d1, 2: d2, 3: d3, 4: d4, 5: d5, 6: d6);
