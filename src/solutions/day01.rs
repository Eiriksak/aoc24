use std::collections::HashMap;
use std::fs;

// Makes sense for column-oriented?
struct Data {
    c1: Vec<u32>,
    c2: Vec<u32>,
}
// Read the file into vecs to be sorted
fn parse_data() -> Data {
    let input = fs::read_to_string("inputs/day01.txt").expect("Failed to read input");
    let (mut c1, mut c2) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        c1.push(
            parts
                .next()
                .unwrap()
                .parse::<u32>()
                .expect("Invalid number"),
        );
        c2.push(
            parts
                .next()
                .unwrap()
                .parse::<u32>()
                .expect("Invalid number"),
        );
    }

    Data { c1, c2 }
}

fn value_counts(col: &Vec<u32>) -> HashMap<u32, u32> {
    let mut vc: HashMap<_, _> = HashMap::new();

    for &value in col {
        *vc.entry(value).or_insert(0) += 1;
    }

    vc
}

pub fn main() {
    let mut data = parse_data();

    data.c1.sort_unstable();
    data.c2.sort_unstable();

    let answ_p1: u32 = data
        .c1
        .iter()
        .zip(data.c2.iter())
        .map(|(a, b)| if a > b { a - b } else { b - a }) // creative .abs() for u32
        .sum();
    println!("Part 1 answer: {answ_p1}");

    let vc = value_counts(&data.c2);
    let answ_p2: u32 = data.c1.iter().map(|&a| a * *vc.get(&a).unwrap_or(&0)).sum();
    println!("Part 2 answer: {answ_p2}");
}
