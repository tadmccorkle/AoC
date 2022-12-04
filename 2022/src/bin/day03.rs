use aoc2022;

fn part1(input: &str) -> u32 {
    input.lines().fold(0, |acc, l| {
        let cpts = l.split_at(l.len() / 2);
        let item = cpts.0.chars().find(|&c| cpts.1.contains(c)).unwrap();
        let priority = if item.is_lowercase() {
            item as u8 - 96
        } else {
            item as u8 - 38
        };
        acc + priority as u32
    })
}

fn part2(input: &str) -> u32 {
    let mut priority_total = 0;
    let mut lines = input.lines();
    while let (Some(r1), Some(r2), Some(r3)) = (lines.next(), lines.next(), lines.next()) {
        let item = r1
            .chars()
            .find(|&c| r2.contains(c) && r3.contains(c))
            .unwrap();
        let priority = if item.is_lowercase() {
            item as u8 - 96
        } else {
            item as u8 - 38
        };
        priority_total += priority as u32;
    }

    priority_total
}

fn main() {
    let input = aoc2022::read_input(3);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 157);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 70);
    }
}
