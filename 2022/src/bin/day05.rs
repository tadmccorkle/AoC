use std::collections::BTreeMap;

use aoc2022;

struct Move {
    count: u32,
    from: usize,
    to: usize,
}

fn build_stacks(stacks_input: &str) -> BTreeMap<usize, String> {
    stacks_input
        .lines()
        .rev()
        .skip(1)
        .fold(BTreeMap::new(), |mut map, line| {
            for (i, c) in line.chars().skip(1).step_by(4).enumerate() {
                if c.is_alphabetic() {
                    map.entry(i + 1)
                        .and_modify(|s| s.push(c))
                        .or_insert(String::from(c));
                }
            }
            map
        })
}

fn get_tops(stacks: BTreeMap<usize, String>) -> String {
    stacks.values().fold(String::new(), |mut acc, s| {
        if let Some(c) = s.chars().last() {
            acc.push(c);
            acc
        } else {
            acc
        }
    })
}

fn build_moves<'a>(moves_input: &'a str) -> impl Iterator<Item = Move> + 'a {
    moves_input.lines().map(|line| {
        let mut ops = line.split_whitespace().skip(1).step_by(2);
        let count = ops.next().unwrap().parse().unwrap();
        let from = ops.next().unwrap().parse().unwrap();
        let to = ops.next().unwrap().parse().unwrap();
        Move { count, from, to }
    })
}

fn part1(input: &str) -> String {
    let (stacks_input, moves_input) = input.split_once("\n\n").unwrap();

    let mut stacks = build_stacks(stacks_input);
    let moves = build_moves(moves_input);

    for m in moves {
        let from_stack = stacks.get_mut(&m.from).unwrap();

        let mut remaining = m.count;
        let pops: Vec<char> = std::iter::from_fn(move || {
            if 0 < remaining {
                remaining -= 1;
                from_stack.pop()
            } else {
                None
            }
        })
        .collect();

        let to_stack = stacks.get_mut(&m.to).unwrap();
        for pop in pops {
            to_stack.push(pop);
        }
    }

    get_tops(stacks)
}

fn part2(input: &str) -> String {
    let (stacks_input, moves_input) = input.split_once("\n\n").unwrap();

    let mut stacks = build_stacks(stacks_input);
    let moves = build_moves(moves_input);

    for m in moves {
        let from_stack = stacks.get_mut(&m.from).unwrap();

        let mut remaining = m.count;
        let pops: Vec<char> = std::iter::from_fn(move || {
            if 0 < remaining {
                remaining -= 1;
                from_stack.pop()
            } else {
                None
            }
        })
        .collect();

        let to_stack = stacks.get_mut(&m.to).unwrap();
        for pop in pops.iter().rev() {
            to_stack.push(*pop);
        }
    }

    get_tops(stacks)
}

fn main() {
    let input = aoc2022::read_input(5);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), String::from("CMZ"));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), String::from("MCD"));
    }
}
