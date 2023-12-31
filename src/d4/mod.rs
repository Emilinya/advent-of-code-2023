use regex::{Captures, Regex};
use std::{collections::HashSet, time::Instant};

use crate::utils;

fn capture_to_set(index: usize, capture: &Captures, line: &str) -> HashSet<u32> {
    HashSet::from_iter(
        capture
            .get(index)
            .expect(&format!(
                "capture {} does not exist for line {}!",
                index, line
            ))
            .as_str()
            .split(" ")
            .filter(|s| *s != "")
            .map(|s| s.parse().expect(&format!("got non-int: {}!", s))),
    )
}

fn get_winning_count(line: &str, regex: &Regex) -> u32 {
    let captures = regex
        .captures(&line)
        .expect(&format!("got no captures for line {}!", line));

    let winners = capture_to_set(1, &captures, &line);
    let numbers = capture_to_set(2, &captures, &line);

    numbers.intersection(&winners).count() as u32
}

fn get_point_sum(filename: &str) -> u32 {
    let number_regex = Regex::new(r"Card +\d+: +((?:\d+ *)+) \| +((?:\d+ *)+)")
        .expect("number_regex could not be parsed!");

    utils::read_lines(filename)
        .filter_map(|line| match get_winning_count(&line, &number_regex) {
            0 => None,
            c => Some(2_u32.pow(c - 1)),
        })
        .sum()
}

fn get_card_count(filename: &str) -> u32 {
    let number_regex = Regex::new(r"Card +\d+: +((?:\d+ *)+) \| +((?:\d+ *)+)")
        .expect("number_regex could not be parsed!");

    let lines: Vec<String> = utils::read_lines(filename).collect();
    let mut card_numbers = vec![1_u32; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let winning_count = get_winning_count(&line, &number_regex);

        for j in 1..(winning_count as usize + 1) {
            card_numbers[i + j] += card_numbers[i]
        }
    }

    card_numbers.iter().sum()
}

fn test() {
    assert_eq!(get_point_sum("src/d4/test_input.dat"), 13);
    assert_eq!(get_card_count("src/d4/test_input.dat"), 30);
}

pub fn main() {
    test();

    let mut now = Instant::now();
    let sum_p1 = get_point_sum("src/d4/full_input.dat");
    println!("Part one result: {} (took {:?})", sum_p1, now.elapsed());

    now = Instant::now();
    let sum_p2 = get_card_count("src/d4/full_input.dat");
    println!("Part two result: {} (took {:?})", sum_p2, now.elapsed());
}
