use std::collections::HashMap;
use std::fs;

fn valid_middle_pages(data: &str) -> i32 {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();
    let (rules, updates) = data.split_once("\n\n").unwrap();
    // list of valid pages for each page
    for line in rules.lines() {
        if let Some((l, r)) = line.split_once('|') {
            let key: i32 = l.trim().parse().unwrap();
            let value: i32 = r.trim().parse().unwrap();
            map.entry(key).or_insert_with(Vec::new).push(value);
        }
    }

    let mut res = 0;
    for line in updates.lines() {
        let values: Vec<i32> = line.split(',').map(|v| v.trim().parse().unwrap()).collect();
        let n = values.len();
        let mut valid = true;
        for (i, page) in values.iter().enumerate() {
            for next_page in i..n {
                if let Some(invalid_pages) = map.get(&values[next_page]) {
                    if invalid_pages.contains(page) {
                        valid = false;
                        //println!("Invalid pair found: {:?} in {:?}", page, values[next_page]);
                        break;
                    }
                }
            }
        }
        if valid {
            let middle = n / 2;
            //println!("{}", &middle);
            res += values[middle];
        }
    }
    res
}

pub fn main() {
    // SECTION 1
    // x|y -> x must be before y

    // SECTION 2
    // x,y,z -> update order

    let data = fs::read_to_string("inputs/day05.txt").expect("Failed to read input");
    let p1 = valid_middle_pages(&data);
    println!("Part1 {}", p1);
}
