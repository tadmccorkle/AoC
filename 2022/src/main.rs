use aoc2022;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: u8 = args[1].parse().unwrap();

    aoc2022::run(day);
}
