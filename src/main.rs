use std::env;

mod solutions;

fn main() {
    println!("Running aoc2024!");

    let args: Vec<String> = env::args().collect();
    let day = args[1].parse::<u8>().expect("Pass arg as a number 1-25");

    match day {
        1 => solutions::day01::main(),
        2 => solutions::day02::main(),
        _ => eprintln!("Solution for Day {} is not implemented yet", day),
    }
}
