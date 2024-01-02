use std::collections::HashMap;
use std::time::Instant;

use crate::utils;

fn get_key_map(lines: &[String]) -> HashMap<String, (String, String)> {
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in lines.iter().skip(2) {
        let (key, value_pair) = match line.split(" = ").collect::<Vec<&str>>()[..] {
            [a, b] => (a, b),
            _ => panic!("Malformed line: {:?}", line),
        };

        let (left, right) = match value_pair[1..(value_pair.len() - 1)]
            .split(", ")
            .collect::<Vec<&str>>()[..]
        {
            [a, b] => (a, b),
            _ => panic!("Malformed value_pair: {:?}", value_pair),
        };

        map.insert(key.to_owned(), (left.to_owned(), right.to_owned()));
    }

    map
}

fn step_key(key: &mut String, step: &char, map: &HashMap<String, (String, String)>) {
    let (left, right) = map
        .get(key)
        .unwrap_or_else(|| panic!("Map does not contain key {:?}!", key));
    match step {
        'L' => *key = left.to_owned(),
        'R' => *key = right.to_owned(),
        _ => panic!("Malformed step: {:?}", step),
    }
}

fn count_steps(filename: &str) -> u64 {
    let lines: Vec<String> = utils::read_lines(filename).collect();
    let steps: Vec<char> = lines[0].chars().collect();

    let map = get_key_map(&lines);

    let mut key = "AAA".to_owned();

    let mut step_count = 0;
    for step in steps.into_iter().cycle() {
        step_count += 1;
        step_key(&mut key, &step, &map);
        if key == "ZZZ" {
            break;
        }
    }
    step_count
}

fn get_least_common_multiple(numbers: &[u64]) -> u64 {
    // recursively solve for each set of two values
    if numbers.len() == 1 {
        return numbers[0];
    }
    let mut a = numbers[0];
    let mut b = get_least_common_multiple(&numbers[1..]);

    // Make sure b >= a
    if a > b {
        (a, b) = (b, a);
    }

    // Keep start values
    let a_start = a;
    let b_start = b;

    // Search for lcm
    loop {
        a += a_start * ((b - a) / a_start);
        if a == b {
            return a;
        }
        b += b_start;
    }
}

fn count_multi_steps(filename: &str) -> u64 {
    let lines: Vec<String> = utils::read_lines(filename).collect();
    let steps: Vec<char> = lines[0].chars().collect();

    let map = get_key_map(&lines);

    let mut keys: Vec<String> = vec![];
    for key in map.keys() {
        if key.as_bytes()[2] == b'A' {
            keys.push(key.to_owned());
        }
    }

    let mut key_cycles: Vec<u64> = vec![];
    for key in keys.iter_mut() {
        let mut key_step_count = 0;
        for step in steps.iter().cycle() {
            key_step_count += 1;
            step_key(key, step, &map);
            if key.as_bytes()[2] == b'Z' {
                break;
            }
        }
        key_cycles.push(key_step_count);
    }

    get_least_common_multiple(&key_cycles)
}

fn test() {
    assert_eq!(count_steps("src/d8/test_input_p1.dat"), 6);
    assert_eq!(count_multi_steps("src/d8/test_input_p2.dat"), 6);
}

pub fn test_final() {
    assert_eq!(count_steps("src/d8/full_input.dat"), 16897);
    assert_eq!(count_multi_steps("src/d8/full_input.dat"), 16563603485021);
}

pub fn main() {
    test();

    let mut now = Instant::now();
    let sum_p1 = count_steps("src/d8/full_input.dat");
    println!("Part one result: {} (took {:?})", sum_p1, now.elapsed());

    now = Instant::now();
    let sum_p2 = count_multi_steps("src/d8/full_input.dat");
    println!("Part two result: {} (took {:?})", sum_p2, now.elapsed());
}
