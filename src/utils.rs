use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Flatten;
use std::path::Path;
use std::str::FromStr;

pub fn read_lines<T: AsRef<Path>>(filename: T) -> Flatten<std::io::Lines<BufReader<File>>> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines().flatten()
}

/// Creates an iterator that iterates over the parsed values in a string.
/// String is split using 'pattern', and the first 'skip' values are skipped
/// before parsing.
pub fn string_to_iter<'a, T>(
    string: &'a str,
    pattern: &'a str,
    skip: usize,
) -> impl Iterator<Item = T> + 'a
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    string
        .split(pattern)
        .filter(|c| *c != "")
        .skip(skip)
        .map(|c| {
            c.parse()
                .expect(&format!("Could not parse string: {:?}", c))
        })
}

/// Creates a vector that contains the parsed values in a string.
/// String is split using 'pattern', and the first 'skip' values are skipped
/// before parsing.
pub fn string_to_array<T>(string: &str, pattern: &str, skip: usize) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    string_to_iter(string, pattern, skip).collect()
}
