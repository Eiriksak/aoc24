use regex::Regex;
use std::fs;

fn parse_data() -> Vec<i32> {
    let input = fs::read_to_string("inputs/day03.txt").expect("Failed to read input");
    let re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();

    let res: Vec<i32> = re
        .captures_iter(&input)
        .map(|cap| {
            let x: i32 = cap[1].parse().expect("Invalid number for X");
            let y: i32 = cap[2].parse().expect("Invalid number for Y");
            x * y
        })
        .collect();
    res
}

pub fn main() {
    let data = parse_data();

    let res1: i32 = data.iter().sum();
    println!("Part 1 {:#?}", res1);
}
