use std::fs;

fn parse_p1() -> String {
    fs::read_to_string("inputs/day01.txt").expect("Failed to read input")
}

pub fn main() {
    let input = parse_p1();
    println!("Input for Day 1: {}", input);
}
