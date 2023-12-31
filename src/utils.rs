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

pub fn string_to_iter<'a, T>(
    string: &'a str,
    split_pattern: &'a str,
) -> impl Iterator<Item = T> + 'a
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    string
        .split(split_pattern)
        .map(|c| c.parse().expect(&format!("Could not parse string: {}", c)))
}

pub fn string_to_array<T>(string: &str, split_pattern: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    string_to_iter(string, split_pattern).collect()
}
