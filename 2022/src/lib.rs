use std::{env, fmt::Display, fs};

pub fn read_input(day: u8) -> String {
    let file_path = env::current_dir()
        .unwrap()
        .join("input")
        .join(format!("day{:02}.txt", day));

    fs::read_to_string(file_path).expect("could not open input file for specified day")
}

type Part<T> = fn(&str) -> T;

pub fn print_results<T: Display>(input: &str, p1: Part<T>, p2: Part<T>) {
    println!("Part 1: {}", p1(input));
    println!("Part 2: {}", p2(input));
}
