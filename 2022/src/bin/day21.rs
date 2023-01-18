use std::{collections::HashMap, str::FromStr};

use aoc2022;

enum Operation {
    Add(String, String),
    Subtract(String, String),
    Multiply(String, String),
    Divide(String, String),
}

#[derive(Debug)]
struct ParseOperationErr;

impl FromStr for Operation {
    type Err = ParseOperationErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut comps = s.split(' ');
        let lhs = comps.next().ok_or(ParseOperationErr)?.to_string();
        let op = comps.next().ok_or(ParseOperationErr)?;
        let rhs = comps.next().ok_or(ParseOperationErr)?.to_string();

        match op {
            "+" => Ok(Self::Add(lhs, rhs)),
            "-" => Ok(Self::Subtract(lhs, rhs)),
            "*" => Ok(Self::Multiply(lhs, rhs)),
            "/" => Ok(Self::Divide(lhs, rhs)),
            _ => Err(ParseOperationErr),
        }
    }
}

enum Job {
    Number(i64),
    Result(Operation),
}

#[derive(Debug)]
struct ParseJobErr;

impl FromStr for Job {
    type Err = ParseJobErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(char::is_whitespace) {
            let operation = s.parse().map_err(|_| ParseJobErr)?;
            Ok(Job::Result(operation))
        } else {
            let number = s.parse().map_err(|_| ParseJobErr)?;
            Ok(Job::Number(number))
        }
    }
}

fn execute_job(key: &str, monkeys: &HashMap<&str, Job>) -> i64 {
    match monkeys.get(key) {
        Some(job) => match job {
            Job::Number(number) => *number,
            Job::Result(operation) => match operation {
                Operation::Add(lhs, rhs) => execute_job(lhs, monkeys) + execute_job(rhs, monkeys),
                Operation::Subtract(lhs, rhs) => {
                    execute_job(lhs, monkeys) - execute_job(rhs, monkeys)
                }
                Operation::Multiply(lhs, rhs) => {
                    execute_job(lhs, monkeys) * execute_job(rhs, monkeys)
                }
                Operation::Divide(lhs, rhs) => {
                    execute_job(lhs, monkeys) / execute_job(rhs, monkeys)
                }
            },
        },
        None => panic!("Monkey {} does not have a job!", key),
    }
}

fn part1(input: &str) -> i64 {
    let monkeys: HashMap<_, Job> = input
        .lines()
        .map(|line| {
            let (name, job) = line.split_once(": ").unwrap();
            (name, job.parse().unwrap())
        })
        .collect();

    execute_job("root", &monkeys)
}

fn find_subkey<'a>(
    subkey: &str,
    key: &'a str,
    monkeys: &'a HashMap<&str, Job>,
) -> Option<Vec<&'a str>> {
    if key == subkey {
        return Some(Vec::from([key]));
    }

    if let Some(Job::Result(operation)) = monkeys.get(key) {
        let (lhs, rhs) = match operation {
            Operation::Add(lhs, rhs) => (lhs, rhs),
            Operation::Subtract(lhs, rhs) => (lhs, rhs),
            Operation::Multiply(lhs, rhs) => (lhs, rhs),
            Operation::Divide(lhs, rhs) => (lhs, rhs),
        };

        for hs in [lhs, rhs] {
            if let Some(mut path) = find_subkey(subkey, hs, monkeys) {
                path.push(key);
                return Some(path);
            }
        }
    }

    None
}

fn subkey_value_for_result(
    subkey: &str,
    result: i64,
    key: &str,
    path: &Vec<&str>,
    monkeys: &HashMap<&str, Job>,
) -> i64 {
    let (to_solve, subresult) = match monkeys.get(key) {
        Some(Job::Result(operation)) => match operation {
            Operation::Add(lhs, rhs) => {
                if path.contains(&&lhs[..]) {
                    (lhs, result - execute_job(rhs, monkeys))
                } else {
                    (rhs, result - execute_job(lhs, monkeys))
                }
            }
            Operation::Subtract(lhs, rhs) => {
                if path.contains(&&lhs[..]) {
                    (lhs, result + execute_job(rhs, monkeys))
                } else {
                    (rhs, execute_job(lhs, monkeys) - result)
                }
            }
            Operation::Multiply(lhs, rhs) => {
                if path.contains(&&lhs[..]) {
                    (lhs, result / execute_job(rhs, monkeys))
                } else {
                    (rhs, result / execute_job(lhs, monkeys))
                }
            }
            Operation::Divide(lhs, rhs) => {
                if path.contains(&&lhs[..]) {
                    (lhs, result * execute_job(rhs, monkeys))
                } else {
                    (rhs, execute_job(lhs, monkeys) / result)
                }
            }
        },
        _ => panic!("could not solve for key {}", key),
    };

    if to_solve != subkey {
        subkey_value_for_result(subkey, subresult, to_solve, path, monkeys)
    } else {
        subresult
    }
}

fn part2(input: &str) -> i64 {
    let monkeys: HashMap<_, Job> = input
        .lines()
        .map(|line| {
            let (name, job) = line.split_once(": ").unwrap();
            (name, job.parse().unwrap())
        })
        .collect();

    // traverse down using vec of subkeys with current desired value
    let (lhs, rhs) = match monkeys.get("root") {
        Some(job) => match job {
            Job::Result(operation) => match operation {
                Operation::Add(lhs, rhs) => (lhs, rhs),
                Operation::Subtract(lhs, rhs) => (lhs, rhs),
                Operation::Multiply(lhs, rhs) => (lhs, rhs),
                Operation::Divide(lhs, rhs) => (lhs, rhs),
            },
            _ => panic!("invalid root"),
        },
        _ => panic!("could not get root"),
    };

    let (path, to_solve, result) = if let Some(path) = find_subkey("humn", lhs, &monkeys) {
        (path, lhs, execute_job(rhs, &monkeys))
    } else if let Some(path) = find_subkey("humn", rhs, &monkeys) {
        (path, rhs, execute_job(lhs, &monkeys))
    } else {
        panic!("could not find humn")
    };

    subkey_value_for_result("humn", result, to_solve, &path, &monkeys)
}

fn main() {
    let input = aoc2022::read_input(21);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 152);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 301);
    }
}
