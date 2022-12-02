use aoc2022;

fn part1(input: &str) -> u32 {
    // acc.0 is current sum, acc.1 is current maximum
    let result = input.lines().fold((0, 0), |acc, l| {
        if let Ok(n) = l.parse::<u32>() {
            (acc.0 + n, acc.1)
        } else if acc.0 > acc.1 {
            (0, acc.0)
        } else {
            (0, acc.1)
        }
    });

    // check to see if the final elf had the most calories
    if result.0 > result.1 {
        result.0
    } else {
        result.1
    }
}

fn part2(input: &str) -> u32 {
    let mut sums: Vec<u32> = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .fold(0, |acc, l| acc + l.parse::<u32>().unwrap())
        })
        .collect();
    sums.sort();
    sums.iter().rev().take(3).sum()
}

fn main() {
    let input = aoc2022::read_input(1);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 24000);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 45000);
    }
}
