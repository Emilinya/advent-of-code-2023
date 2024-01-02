use regex::Regex;
use std::{collections::HashMap, time::Instant};

use crate::utils;

#[derive(Debug)]
struct PartNumber {
    number: u32,
    row: usize,
    range: (usize, usize),
}

fn get_part_numbers(lines: &[String]) -> Vec<PartNumber> {
    let number_re = Regex::new(r"\d+").unwrap();
    lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| number_re.captures_iter(line).map(move |v| (row, v)))
        .map(|(row, c)| {
            let re_match = c.get(0).unwrap();

            PartNumber {
                number: re_match.as_str().parse().unwrap(),
                row,
                range: (re_match.start(), re_match.end() - 1),
            }
        })
        .collect()
}

fn cmp_sides(part_number: &PartNumber, lines: &[String], rows: usize, offset: usize) -> bool {
    if lines[part_number.row].as_bytes()[offset] != b'.' {
        return true;
    }

    if part_number.row != 0 && lines[part_number.row - 1].as_bytes()[offset] != b'.' {
        return true;
    }

    if part_number.row != rows - 1 && lines[part_number.row + 1].as_bytes()[offset] != b'.' {
        return true;
    }

    false
}

fn symbol_around(part_number: &PartNumber, lines: &Vec<String>) -> bool {
    let (left, right) = part_number.range;
    let columns = lines[0].len();
    let row = part_number.row;
    let rows = lines.len();

    let contains_symbol_at = |idx: usize| {
        !lines[idx]
            .get(left..(right + 1))
            .unwrap()
            .chars()
            .all(|c| c == '.')
    };

    if left != 0 && cmp_sides(part_number, lines, rows, left - 1) {
        return true;
    }

    if right < columns - 1 && cmp_sides(part_number, lines, rows, right + 1) {
        return true;
    }

    if part_number.row != 0 && contains_symbol_at(row - 1) {
        return true;
    }

    if part_number.row < rows - 1 && contains_symbol_at(row + 1) {
        return true;
    }

    false
}

fn get_part_sum(filename: &str) -> u32 {
    let lines: Vec<String> = utils::read_lines(filename).collect();

    let mut part_sum = 0;
    for part_number in get_part_numbers(&lines) {
        if symbol_around(&part_number, &lines) {
            part_sum += part_number.number;
            continue;
        }
    }
    part_sum
}

fn get_side_index(
    part_number: &PartNumber,
    lines: &[String],
    rows: usize,
    offset: usize,
) -> Option<(usize, usize)> {
    if lines[part_number.row].as_bytes()[offset] == b'*' {
        return Some((part_number.row, offset));
    }

    if part_number.row != 0 && lines[part_number.row - 1].as_bytes()[offset] == b'*' {
        return Some((part_number.row - 1, offset));
    }

    if part_number.row != rows - 1 && lines[part_number.row + 1].as_bytes()[offset] == b'*' {
        return Some((part_number.row + 1, offset));
    }

    None
}

fn get_gear_index(part_number: &PartNumber, lines: &Vec<String>) -> Option<(usize, usize)> {
    let (left, right) = part_number.range;
    let columns = lines[0].len();
    let row = part_number.row;
    let rows = lines.len();

    let get_gear_at = |row: usize| {
        lines[row]
            .get(left..(right + 1))
            .unwrap()
            .chars()
            .enumerate()
            .map(|(i, c)| (row, left + i, c))
            .find(|(_, _, c)| *c == '*')
    };

    if left != 0 {
        let index = get_side_index(part_number, lines, rows, left - 1);
        if index.is_some() {
            return index;
        }
    }

    if right < columns - 1 {
        let index = get_side_index(part_number, lines, rows, right + 1);
        if index.is_some() {
            return index;
        }
    }

    if part_number.row != 0 {
        if let Some(result) = get_gear_at(row - 1) {
            return Some((result.0, result.1));
        }
    }

    if part_number.row < rows - 1 {
        if let Some(result) = get_gear_at(row + 1) {
            return Some((result.0, result.1));
        }
    }

    None
}

fn get_ratio_sum(filename: &str) -> u32 {
    let lines: Vec<String> = utils::read_lines(filename).collect();

    let gear_re = Regex::new(r"\*").unwrap();
    let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for (row, c) in lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| gear_re.captures_iter(line).map(move |v| (row, v)))
    {
        let re_match = c.get(0).unwrap();

        gears.insert((row, re_match.start()), vec![]);
    }

    for part_number in get_part_numbers(&lines) {
        if let Some((row, column)) = get_gear_index(&part_number, &lines) {
            gears
                .entry((row, column))
                .and_modify(|v| v.push(part_number.number));
        }
    }

    let mut ratio_sum = 0;
    for numbers in gears.values() {
        if let [a, b] = numbers[..] {
            ratio_sum += a * b
        };
    }
    ratio_sum
}

fn test() {
    assert_eq!(get_part_sum("src/d3/test_input.dat"), 4361);
    assert_eq!(get_ratio_sum("src/d3/test_input.dat"), 467835);
}

pub fn test_final() {
    assert_eq!(get_part_sum("src/d3/full_input.dat"), 536576);
    assert_eq!(get_ratio_sum("src/d3/full_input.dat"), 75741499);
}

pub fn main() {
    test();

    let mut now = Instant::now();
    let sum_p1 = get_part_sum("src/d3/full_input.dat");
    println!("Part one result: {} (took {:?})", sum_p1, now.elapsed());

    now = Instant::now();
    let sum_p2 = get_ratio_sum("src/d3/full_input.dat");
    println!("Part two result: {} (took {:?})", sum_p2, now.elapsed());
}
