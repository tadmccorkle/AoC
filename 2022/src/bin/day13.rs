use aoc2022;

fn compare(left: &str, right: &str) -> bool {
    let mut in_order = true;
    let mut l_chars = left.chars();
    let mut r_chars = right.chars();
    while let (Some(l), Some(r)) = (l_chars.next(), r_chars.next()) {

    }

    in_order
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .enumerate()
        .filter(|(_, pair)| {
            let (left, right) = pair.split_once('\n').unwrap();
            compare(left, right)
        })
        .map(|(i, _)| i)
        .sum()
}

fn part2(input: &str) -> String {
    String::from("solution not yet implemented")
}

fn main() {
    let input = aoc2022::read_input(13);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 13);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), "");
    }
}
