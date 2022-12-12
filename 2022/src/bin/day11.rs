use std::str::FromStr;

use aoc2022;

enum Operation {
    Add(u64),
    Mult(u64),
    Square,
}

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_by: u64,
    true_to: usize,
    false_to: usize,
    inspected: u64,
}

fn parse_last<T: FromStr>(s: &str, split_by: char) -> Result<T, T::Err> {
    s.split(split_by).last().unwrap().parse()
}

impl FromStr for Monkey {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = s.lines().skip(1);

        let items = data
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(|item| item.trim().parse().unwrap())
            .collect();

        let operation = data.next().unwrap();
        let operation = if operation.matches("old").count() == 2 {
            Operation::Square
        } else if operation.contains('+') {
            Operation::Add(parse_last(operation, ' ').unwrap())
        } else if operation.contains('*') {
            Operation::Mult(parse_last(operation, ' ').unwrap())
        } else {
            return Err(());
        };

        Ok(Self {
            items,
            operation,
            test_by: parse_last(data.next().unwrap(), ' ').unwrap(),
            true_to: parse_last(data.next().unwrap(), ' ').unwrap(),
            false_to: parse_last(data.next().unwrap(), ' ').unwrap(),
            inspected: 0,
        })
    }
}

fn part1(input: &str) -> u64 {
    let mut monkeys = input
        .split("\n\n")
        .map(|m| Monkey::from_str(m).unwrap())
        .collect::<Vec<Monkey>>();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let item = monkeys[i].items.drain(..1).next().unwrap();
                let worry_level = match monkeys[i].operation {
                    Operation::Add(i) => item + i,
                    Operation::Mult(i) => item * i,
                    Operation::Square => item * item,
                } / 3;
                let to = if worry_level % monkeys[i].test_by == 0 {
                    monkeys[i].true_to
                } else {
                    monkeys[i].false_to
                };
                monkeys[to].items.push(worry_level);
                monkeys[i].inspected += 1;
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspected);
    monkeys.reverse();

    monkeys[0].inspected * monkeys[1].inspected
}

fn part2(input: &str) -> u64 {
    let mut monkeys = input
        .split("\n\n")
        .map(|m| Monkey::from_str(m).unwrap())
        .collect::<Vec<Monkey>>();

    let max_worry_level: u64 = monkeys.iter().map(|m| m.test_by).product();

    for _ in 0..10_000 {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let item = monkeys[i].items.drain(..1).next().unwrap();
                let worry_level = match monkeys[i].operation {
                    Operation::Add(i) => item + i,
                    Operation::Mult(i) => item * i,
                    Operation::Square => item * item,
                } % max_worry_level;
                let to = if worry_level % monkeys[i].test_by == 0 {
                    monkeys[i].true_to
                } else {
                    monkeys[i].false_to
                };
                monkeys[to].items.push(worry_level);
                monkeys[i].inspected += 1;
            }
        }
    }

    monkeys.sort_by_key(|m| m.inspected);
    monkeys.reverse();

    monkeys[0].inspected * monkeys[1].inspected
}

fn main() {
    let input = aoc2022::read_input(11);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 10_605);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 2_713_310_158);
    }
}
