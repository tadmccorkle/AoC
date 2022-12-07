use aoc2022;

fn find_start(input: &str, marker_len: usize) -> usize {
    for i in 0..(input.len() - marker_len + 1) {
        if input[i..(i + marker_len)]
            .chars()
            .collect::<std::collections::HashSet<char>>()
            .len()
            == marker_len
        {
            return i + marker_len;
        }
    }
    panic!("could not find start");
}

fn part1(input: &str) -> usize {
    find_start(input, 4)
}

fn part2(input: &str) -> usize {
    find_start(input, 14)
}

fn main() {
    let input = aoc2022::read_input(6);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
