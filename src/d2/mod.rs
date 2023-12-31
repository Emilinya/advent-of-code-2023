use regex::Regex;
use std::cmp::max;
use std::collections::HashMap;
use std::time::Instant;

use crate::utils;

fn is_possible(line: &str, regex: &Regex, max_map: &HashMap<&str, u32>) -> bool {
    for count_and_color in regex
        .captures_iter(&line)
        .flat_map(|c| c.get(0).unwrap().as_str().split(", "))
        .map(|v| v.split(" ").collect::<Vec<&str>>())
    {
        match count_and_color[..] {
            [count_str, color] => {
                if count_str.parse::<u32>().unwrap() > max_map[color] {
                    return false;
                }
            }
            _ => panic!("Got malformed count_and_color: {:?}", count_and_color),
        }
    }

    return true;
}

fn get_min_possible(line: &str, regex: &Regex) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    for count_and_color in regex
        .captures_iter(&line)
        .flat_map(|c| c.get(0).unwrap().as_str().split(", "))
        .map(|v| v.split(" ").collect::<Vec<&str>>())
    {
        match count_and_color[..] {
            [count_str, color] => {
                let count: u32 = count_str.parse().unwrap();

                map.entry(color.to_owned())
                    .and_modify(|v| *v = max(count, *v))
                    .or_insert(count);
            }
            _ => panic!("Got malformed count_and_color: {:?}", count_and_color),
        }
    }

    return map;
}

fn count_possible(filename: &str) -> u32 {
    let mut max_map = HashMap::new();
    max_map.insert("red", 12);
    max_map.insert("green", 13);
    max_map.insert("blue", 14);

    let id_re = Regex::new(r"Game (\d+):").unwrap();
    let game_re = Regex::new(r"(?:\d+ \w+(?:, )?)+").unwrap();

    let mut sum: u32 = 0;
    for line in utils::read_lines(filename) {
        let game_match = id_re.captures(&line).unwrap().get(1).unwrap();
        let game_id: u32 = game_match.as_str().parse().unwrap();

        if is_possible(&line, &game_re, &max_map) {
            sum += game_id;
        }
    }

    sum
}

fn get_power_sum(filename: &str) -> u32 {
    let game_re = Regex::new(r"(?:\d+ \w+(?:, )?)+").unwrap();

    let mut sum: u32 = 0;
    for line in utils::read_lines(filename) {
        let min_map = get_min_possible(&line, &game_re);

        sum += min_map["red"] * min_map["green"] * min_map["blue"];
    }

    sum
}

fn test() {
    assert_eq!(count_possible("src/d2/test_input.dat"), 8);
    assert_eq!(get_power_sum("src/d2/test_input.dat"), 2286);
}

pub fn main() {
    test();

    let mut now = Instant::now();
    let sum_p1 = count_possible("src/d2/full_input.dat");
    println!("Part one result: {} (took {:?})", sum_p1, now.elapsed());

    now = Instant::now();
    let sum_p2 = get_power_sum("src/d2/full_input.dat");
    println!("Part two result: {} (took {:?})", sum_p2, now.elapsed());
}
