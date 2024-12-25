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

    let mut count_p1 = 0;
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
                    count_p1 += 1
                }
            }
        }
    }

    println!("Part1 {}", count_p1);

    // M.S   [-1,-1],[-1,0],[-1,1]
    // .A.   [0,-1],[0,0],[0,1]
    // M.S   [1,-1],[1,0]],[1,1]

    let mut count_p2 = 0;
    let ms = b"MS";
    let sm = b"SM";

    for i in 0..rows {
        for j in 0..cols {
            if i < 1 || i + 1 >= rows || j < 1 || j + 1 >= cols || matrix[i][j] != b"A"[0] {
                continue;
            }
            let diag1 = [matrix[i - 1][j - 1], matrix[i + 1][j + 1]]; // up left, down right
            let diag2 = [matrix[i + 1][j - 1], matrix[i - 1][j + 1]]; // down left, up right

            if (&diag1 == ms || &diag1 == sm) && (&diag2 == ms || &diag2 == sm) {
                count_p2 += 1;
            }
        }
    }

    println!("Part2 {}", count_p2);
}
