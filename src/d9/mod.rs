use std::time::Instant;

use crate::utils;

fn plot_layers(layers: &[Vec<i32>]) {
    let depth = layers.len();
    for (i, layer) in layers.iter().enumerate() {
        println!(
            "{}",
            layer
                .iter()
                .map(|v| v.to_string())
                .take(1 + depth - i)
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
    println!();
}

fn get_next_sum(filename: &str) -> i32 {
    let mut next_sum: i32 = 0;

    for line in utils::read_lines(filename) {
        let mut numbers = utils::string_to_array::<i32>(&line, " ", 0);
        numbers.reverse();

        let mut layers: Vec<Vec<i32>> = vec![numbers];
        let mut depth = 0;

        loop {
            let next_difference = layers[depth][0] - layers[depth][1];
            if next_difference == 0 {
                // plot_layers(&layers);
                next_sum += layers.iter().map(|vec| vec[0]).sum::<i32>();
                break;
            } else {
                depth += 1;
                layers.push(vec![next_difference]);

                for i in 0..depth {
                    let layer_difference = layers[i][depth - i] - layers[i][depth - i + 1];
                    layers[i + 1].push(layer_difference);
                }
            }
        }
    }

    next_sum
}

fn test() {
    assert_eq!(get_next_sum("src/d9/test_input.dat"), 114);
    // assert_eq!(get_next_sum("src/d9/test_input.dat"), 5905);
}

pub fn test_final() {
    // assert_eq!(get_next_sum("src/d9/full_input.dat"), 251136060);
    // assert_eq!(get_next_sum("src/d9/full_input.dat"), 249400220);
}

pub fn main() {
    test();

    let mut now = Instant::now();
    let sum_p1 = get_next_sum("src/d9/full_input.dat");
    println!("Part one result: {} (took {:?})", sum_p1, now.elapsed());

    // now = Instant::now();
    // let sum_p2 = get_next_sum("src/d9/full_input.dat");
    // println!("Part two result: {} (took {:?})", sum_p2, now.elapsed());
}
