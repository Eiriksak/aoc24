use regex::Regex;
use std::fs;

// Sum up mul(x, y) instructions from a text
fn mul_sum(text: &str) -> u64 {
    let re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();

    re.captures_iter(&text)
        .map(|cap| {
            let x: u64 = cap[1].parse().expect("Invalid number for X");
            let y: u64 = cap[2].parse().expect("Invalid number for Y");
            x * y
        })
        .sum()
}

// Custom parser: removes subsets of string when state is considered disabled
fn clean_text(input: &str) -> String {
    let mut enabled = true;
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if enabled {
            result.push(c);
        }
        if chars.clone().take(7).collect::<String>() == "don't()" {
            enabled = false;
            chars.nth(6); // skip ahead
        }
        if chars.clone().take(4).collect::<String>() == "do()" {
            enabled = true;
            chars.nth(3); // skip ahead
        }
    }
    result
}

pub fn main() {
    let data = fs::read_to_string("inputs/day03.txt").expect("Failed to read input");

    let res1: u64 = mul_sum(&data);
    println!("Part 1 {:#?}", res1);

    let cleaned_text = clean_text(&data);
    let res2: u64 = mul_sum(&cleaned_text);
    println!("Part 2 {:#?}", &res2);
}
