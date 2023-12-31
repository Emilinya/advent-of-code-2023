use std::time::Instant;

use crate::utils;

fn find_num<'a, T: Iterator<Item = &'a u8>>(bytes: T) -> Option<u8> {
    for byte in bytes {
        let num = *byte;
        if num >= b'0' && num <= b'9' {
            return Some(num - b'0');
        }
    }

    return None;
}

fn get_sum_p1(filename: &str) -> u32 {
    let mut calibration_sum: u32 = 0;
    for line in utils::read_lines(filename) {
        let bytes = line.as_bytes();

        let left_num = find_num(bytes.iter()).unwrap() as u32;
        let right_num = find_num(bytes.iter().rev()).unwrap() as u32;

        calibration_sum += left_num * 10 + right_num;
    }
    calibration_sum
}

macro_rules! test_string {
    ($numbers: tt, $small_vec: tt; $($number_string: literal: $number: literal),*) => {
        if let Ok(string) = String::from_utf8($small_vec.clone()) {
            $(
                if string == $number_string {
                    $numbers.push($number);
                    continue;
                }
            )*
        }
    };
}

fn get_sum_p2(filename: &str) -> u32 {
    let mut calibration_sum: u32 = 0;

    for line in utils::read_lines(filename) {
        let bytes: Vec<u8> = line.as_bytes().into();

        let mut has_itered = false;
        let mut idx = 0;

        let mut numbers: Vec<u8> = vec![];

        while idx + 1 < bytes.len() {
            if has_itered {
                idx += 1;
            } else {
                has_itered = true;
            }

            if bytes[idx] >= b'0' && bytes[idx] <= b'9' {
                numbers.push(bytes[idx] - b'0');
                continue;
            }

            let mut small_vec: Vec<u8> = vec![];

            if idx + 2 < bytes.len() {
                small_vec = [bytes[idx], bytes[idx + 1], bytes[idx + 2]].to_vec();
                test_string!(numbers, small_vec; "one": 1, "two": 2, "six": 6);
            }

            if idx + 3 < bytes.len() {
                small_vec.push(bytes[idx + 3]);
                test_string!(numbers, small_vec; "four": 4, "five": 5, "nine": 9);
            }

            if idx + 4 < bytes.len() {
                small_vec.push(bytes[idx + 4]);
                test_string!(numbers, small_vec; "three": 3, "seven": 7, "eight": 8);
            }
        }

        let left_num = numbers[0] as u32;
        let right_num = numbers[numbers.len() - 1] as u32;

        calibration_sum += left_num * 10 + right_num;
    }

    calibration_sum
}

fn test() {
    assert_eq!(get_sum_p1("src/d1/test_input_p1.dat"), 142);
    assert_eq!(get_sum_p2("src/d1/test_input_p2.dat"), 281);
}

pub fn main() {
    test();

    let mut now = Instant::now();
    let sum_p1 = get_sum_p1("src/d1/full_input.dat");
    println!("Part one result: {} (took {:?})", sum_p1, now.elapsed());

    now = Instant::now();
    let sum_p2 = get_sum_p2("src/d1/full_input.dat");
    println!("Part two result: {} (took {:?})", sum_p2, now.elapsed());
}
