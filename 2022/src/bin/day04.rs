use aoc2022;

fn part1(input: &str) -> u32 {
    input.lines().fold(0, |acc, l| {
        let pairs = l.split_once(',').unwrap();
        if let (Some(l), Some(r)) = (pairs.0.split_once('-'), pairs.1.split_once('-')) {
            let (l_min, l_max): (u32, u32) = (l.0.parse().unwrap(), l.1.parse().unwrap());
            let (r_min, r_max) = (r.0.parse().unwrap(), r.1.parse().unwrap());
            if (l_min >= r_min && l_max <= r_max) || (l_min <= r_min && l_max >= r_max) {
                return acc + 1;
            }
        }

        acc
    })
}

fn part2(input: &str) -> u32 {
    input.lines().fold(0, |acc, l| {
        let pairs = l.split_once(',').unwrap();
        if let (Some(l), Some(r)) = (pairs.0.split_once('-'), pairs.1.split_once('-')) {
            let (l_min, l_max): (i32, i32) = (l.0.parse().unwrap(), l.1.parse().unwrap());
            let (r_min, r_max): (i32, i32) = (r.0.parse().unwrap(), r.1.parse().unwrap());
            if (l_min <= r_max) && (r_min <= l_max) {
                return acc + 1;
            }
        }

        acc
    })
}

fn main() {
    let input = aoc2022::read_input(4);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 2);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 4);
    }
}
