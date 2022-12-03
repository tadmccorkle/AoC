use aoc2022;

fn part1(input: &str) -> u32 {
    let mut score = 0;
    for game in input.lines() {
        let shape_score = match game.split(' ').nth(1) {
            Some("X") => 1,
            Some("Y") => 2,
            Some("Z") => 3,
            _ => panic!("invalid input"),
        };
        let game_score = match game {
            "A X" => 3,
            "B Y" => 3,
            "C Z" => 3,
            "A Y" => 6,
            "B Z" => 6,
            "C X" => 6,
            "A Z" => 0,
            "B X" => 0,
            "C Y" => 0,
            _ => panic!("invalid input"),
        };
        score += shape_score + game_score;
    }

    score
}

fn part2(input: &str) -> u32 {
    let mut score = 0;
    for game in input.lines() {
        let game_score = match game.split(' ').nth(1) {
            Some("X") => 0,
            Some("Y") => 3,
            Some("Z") => 6,
            _ => panic!("invalid input"),
        };
        let shape_score = match game {
            "A X" => 3,
            "B X" => 1,
            "C X" => 2,
            "A Y" => 1,
            "B Y" => 2,
            "C Y" => 3,
            "A Z" => 2,
            "B Z" => 3,
            "C Z" => 1,
            _ => panic!("invalid input"),
        };
        score += shape_score + game_score;
    }

    score
}

fn main() {
    let input = aoc2022::read_input(2);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 12);
    }
}
