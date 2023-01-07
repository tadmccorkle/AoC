use std::collections::VecDeque;

use aoc2022;

fn mix(numbers: &mut VecDeque<(usize, isize)>, times: usize) {
    let repeated_indices = (0..numbers.len()).cycle().take(numbers.len() * times);

    for i in repeated_indices {
        let mut position = numbers.iter().position(|n| n.0 == i).unwrap();
        let number = numbers[position].1;
        if number == 0 {
            continue;
        }

        // found out about rem_euclid after implementing the
        // more complex (and slower) swapping solution below
        // let value = numbers.remove(position).unwrap();
        // let position = (position as isize + number).rem_euclid(numbers.len() as isize);
        // numbers.insert(position as usize, value);

        let mut swap = isize::abs(number) as usize % (numbers.len() - 1);
        if number < 0 {
            if swap >= position {
                let value = numbers.remove(position).unwrap();
                numbers.push_back(value);
                swap -= position;
                position = numbers.len() - 1;
            }

            for _ in 0..swap {
                numbers.swap(position, position - 1);
                position -= 1;
            }
        } else {
            if swap + position >= numbers.len() - 1 {
                let value = numbers.remove(position).unwrap();
                numbers.push_front(value);
                swap -= numbers.len() - 1 - position;
                position = 0;
            }

            for _ in 0..swap {
                numbers.swap(position, position + 1);
                position += 1;
            }
        }
    }
}

fn add_coords(numbers: &VecDeque<(usize, isize)>) -> isize {
    let zero_position = numbers.iter().position(|n| n.1 == 0).unwrap();
    let _1 = numbers[(zero_position + 1000) % numbers.len()].1;
    let _2 = numbers[(zero_position + 2000) % numbers.len()].1;
    let _3 = numbers[(zero_position + 3000) % numbers.len()].1;

    _1 + _2 + _3
}

fn part1(input: &str) -> isize {
    let mut numbers: VecDeque<(usize, isize)> = input
        .lines()
        .enumerate()
        .map(|(i, line)| (i, line.parse().unwrap()))
        .collect();

    mix(&mut numbers, 1);
    add_coords(&numbers)
}

fn part2(input: &str) -> isize {
    const KEY: isize = 811589153;
    let mut numbers: VecDeque<_> = input
        .lines()
        .enumerate()
        .map(|(i, line)| (i, line.parse::<isize>().unwrap() * KEY))
        .collect();

    mix(&mut numbers, 10);
    add_coords(&numbers)
}

fn main() {
    let input = aoc2022::read_input(20);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1\n2\n-3\n3\n-2\n0\n4";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 3);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 1623178306);
    }
}
