use std::time::Instant;

use crate::utils;

fn get_win_count(time: u64, record: u64) -> u64 {
    // simple math gives this as the range of values that beat the record.
    // The "- 1e-10" is there in case win_range is an integer. In that case,
    // we would get times where the distance traveled is equal to the record,
    // but we want ones that are strictly larger.
    let win_range = ((time.pow(2) - 4 * record) as f64).sqrt() - 1e-10;

    // we must floor and ceil as we are only operating on ints
    let min_t = ((time as f64 - win_range) / 2.0).ceil() as u64;
    let max_t = ((time as f64 + win_range) / 2.0).floor() as u64;

    // we add 1 because min_t = max_t gives one way to win
    max_t - min_t + 1
}

fn get_win_prod(filename: &str) -> u64 {
    let lines: Vec<String> = utils::read_lines(filename).collect();

    utils::string_to_iter(&lines[0], " ", 1)
        .zip(utils::string_to_iter(&lines[1], " ", 1))
        .map(|(time, record)| get_win_count(time, record))
        .product()
}

fn unkernel(line: &str) -> u64 {
    let number_str = utils::string_to_iter(line, " ", 1)
        .collect::<Vec<String>>()
        .join("");
    number_str
        .parse()
        .unwrap_or_else(|_| panic!("Could not parse string: {:?}", number_str))
}

fn get_long_win_count(filename: &str) -> u64 {
    let lines: Vec<String> = utils::read_lines(filename).collect();

    let true_time = unkernel(&lines[0]);
    let true_record = unkernel(&lines[1]);

    get_win_count(true_time, true_record)
}

fn test() {
    assert_eq!(get_win_prod("src/d6/test_input.dat"), 288);
    assert_eq!(get_long_win_count("src/d6/test_input.dat"), 71503);
}

pub fn test_final() {
    assert_eq!(get_win_prod("src/d6/full_input.dat"), 114400);
    assert_eq!(get_long_win_count("src/d6/full_input.dat"), 21039729);
}

pub fn main() {
    test();

    let mut now = Instant::now();
    let sum_p1 = get_win_prod("src/d6/full_input.dat");
    println!("Part one result: {} (took {:?})", sum_p1, now.elapsed());

    now = Instant::now();
    let sum_p2 = get_long_win_count("src/d6/full_input.dat");
    println!("Part two result: {} (took {:?})", sum_p2, now.elapsed());
}
