use std::fs;

// Map each row into Box (more memory efficient) since size is fixed after init
fn parse_data() -> Vec<Box<[i16]>> {
    let input = fs::read_to_string("inputs/day02.txt").expect("Failed to read input");

    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i16>().expect("Invalid number"))
                .collect::<Vec<i16>>()
                .into_boxed_slice()
        })
        .collect()
}

fn check_safe(row: &[i16], max_diff: i16) -> i16 {
    let is_increasing = row[0] < row[1]; // determine trend

    // .all to stop immediate as it fails
    if row.windows(2).all(|pair| {
        let diff = pair[0] - pair[1];
        if is_increasing {
            diff < 0 && diff.abs() <= max_diff
        } else {
            diff > 0 && diff <= max_diff
        }
    }) {
        1
    } else {
        0
    }
}

fn check_safe2(row: &[i16], max_diff: i16) -> i16 {
    let is_increasing = row[0] < row[1]; // determine trend

    // .filter to loop all and .count to sum up fails
    let n_fails = row
        .windows(2)
        .filter(|pair| {
            let diff = pair[0] - pair[1];
            if is_increasing {
                !(diff < 0 && diff.abs() <= max_diff)
            } else {
                !(diff > 0 && diff <= max_diff)
            }
        })
        .count();

    if n_fails > 1 {
        0
    } else {
        1
    }
}

pub fn main() {
    let data = parse_data();
    //println!("{:#?}", data);

    let res1: i16 = data.iter().map(|row| check_safe(row, 3)).sum();
    println!("Part 1 {:#?}", res1);

    let res2: i16 = data.iter().map(|row| check_safe2(row, 3)).sum();
    println!("Part 2 {:#?}", res2);
}
