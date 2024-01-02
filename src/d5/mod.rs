use indicatif::{ProgressIterator, ProgressStyle};
use std::time::Instant;

use crate::utils;

struct WackyMapPart {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

struct WackyMap {
    sub_maps: Vec<WackyMapPart>,
}

struct MapPipe {
    maps: Vec<WackyMap>,
}

impl WackyMapPart {
    fn from_string(string: &str) -> Self {
        match utils::string_to_array(string, " ", 0)[..] {
            [a, b, c] => WackyMapPart {
                destination_range_start: a,
                source_range_start: b,
                range_length: c,
            },
            _ => panic!(
                "WackyMapPart::from_string got malformed string: {:?}",
                string
            ),
        }
    }
}

impl WackyMap {
    fn new() -> Self {
        WackyMap { sub_maps: vec![] }
    }

    fn map(&self, key: u64) -> u64 {
        for part in self.sub_maps.iter() {
            let start = part.source_range_start;
            let end = start + part.range_length;

            if (start..=end).contains(&key) {
                return key - start + part.destination_range_start;
            }
        }

        key
    }
}

impl MapPipe {
    fn new() -> Self {
        MapPipe { maps: vec![] }
    }

    fn map(&self, key: u64) -> u64 {
        let mut mapped_value = key;

        for map in self.maps.iter() {
            mapped_value = map.map(mapped_value);
        }

        mapped_value
    }
}

fn get_map_pipe(lines: &Vec<String>) -> MapPipe {
    let line_indecies: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter_map(|(i, line)| if *line == "" { Some(i + 1) } else { None })
        .collect();

    let mut map_pipe = MapPipe::new();
    for (map_i, line_i) in line_indecies.iter().enumerate() {
        let data_lines = if map_i >= line_indecies.len() - 1 {
            lines.len() - line_i - 1
        } else {
            line_indecies[map_i + 1] - line_i - 2
        };

        map_pipe.maps.push(WackyMap::new());
        for data_line in 1..(data_lines + 1) {
            map_pipe.maps[map_i]
                .sub_maps
                .push(WackyMapPart::from_string(&lines[line_i + data_line]));
        }
    }

    map_pipe
}

fn get_min_location(filename: &str) -> u64 {
    let lines: Vec<String> = utils::read_lines(filename).collect();

    let map_pipe = get_map_pipe(&lines);

    utils::string_to_iter(
        lines[0]
            .get(7..)
            .expect(&format!("Could not get 7.. from line 0: {}", lines[0])),
        " ",
        0,
    )
    .map(|seed| map_pipe.map(seed))
    .min()
    .expect("location list is empty!?")
}

fn get_true_min_location(filename: &str) -> u64 {
    let lines: Vec<String> = utils::read_lines(filename).collect();

    let map_pipe = get_map_pipe(&lines);

    utils::string_to_array(
        lines[0]
            .get(7..)
            .expect(&format!("Could not get 7.. from line 0: {}", lines[0])),
        " ",
        0,
    )
    .chunks(2)
    .progress_with_style(
        ProgressStyle::with_template("{wide_bar} {pos}/{len} [{elapsed} : {eta}] ").unwrap(),
    )
    .flat_map(|v| match v {
        [start, range] => (*start)..(start + range),
        _ => panic!("what? {:?}", v),
    })
    .map(|seed| map_pipe.map(seed))
    .min()
    .expect("location list is empty!?")
}

fn test() {
    assert_eq!(get_min_location("src/d5/test_input.dat"), 35);
    assert_eq!(get_true_min_location("src/d5/test_input.dat"), 46);
}

pub fn main() {
    test();

    let mut now = Instant::now();
    let sum_p1 = get_min_location("src/d5/full_input.dat");
    println!("Part one result: {} (took {:?})", sum_p1, now.elapsed());

    now = Instant::now();
    let sum_p2 = get_true_min_location("src/d5/full_input.dat");
    println!("Part two result: {} (took {:?})", sum_p2, now.elapsed());
}
