use std::fs;

fn init_matrix() -> Vec<Vec<u8>> {
    let content = fs::read_to_string("inputs/day04.txt").expect("Failed to read input");
    content.lines().map(|line| line.bytes().collect()).collect()
}

// Init dx, dy to scan against
fn init_directions() -> Vec<(isize, isize)> {
    let mut directions = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx != 0 || dy != 0 {
                directions.push((dx, dy));
            }
        }
    }
    directions
}

pub fn main() {
    let matrix = init_matrix();
    let directions = init_directions();
    let rows = matrix.len();
    let cols = matrix[0].len();
    let xmas = b"XMAS";

    let mut count = 0;
    for i in 0..rows {
        for j in 0..cols {
            // For every item in matrix, scan all directions for word match
            for &(dx, dy) in &directions {
                let mut valid = true;
                for (k, &ch) in xmas.iter().enumerate() {
                    let ii = i as isize + k as isize * dx; // x dir
                    let jj = j as isize + k as isize * dy; // y dir
                    if ii < 0
                        || jj < 0
                        || ii >= rows as isize
                        || jj >= cols as isize
                        || matrix[ii as usize][jj as usize] != ch
                    {
                        // outside bounds
                        valid = false;
                        break;
                    }
                }
                if valid {
                    count += 1
                }
            }
        }
    }

    println!("{}", count);
}
