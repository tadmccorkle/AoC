use std::{env, fmt::Display, fs};

mod days;
use days::*;

fn read_input(day: u8) -> String {
    let file_path = env::current_dir()
        .unwrap()
        .join("input")
        .join(format!("day{:02}.txt", day));

    fs::read_to_string(file_path).expect("could not open input file for specified day")
}

type Part<T> = fn(&str) -> T;

fn print_results<T: Display>(input: &str, p1: Part<T>, p2: Part<T>) {
    let p1 = p1(input);
    let p2 = p2(input);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

pub fn run(day: u8) {
    println!("AoC 2022 Day {}", day);

    let input = read_input(day);

    match day {
        1 => print_results(&input, day01::part1, day01::part2),
        _ => panic!("invalid day"),
    };
}
