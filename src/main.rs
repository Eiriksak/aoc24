use std::env;

mod solutions;

fn main() {
    println!("Running aoc2024!");

    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<u32>().expect("Pass arg as a number 1-25");

    match day {
        1 => solutions::day01::main(),
        // Add more days here as you implement them
        _ => eprintln!("Solution for Day {} is not implemented yet", day),
    }
}
