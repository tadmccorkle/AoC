use std::{collections::HashSet, hash::Hash, str::FromStr};

use aoc2022;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(input: &str) -> Result<Direction, Self::Err> {
        match input {
            "U" => Ok(Direction::Up),
            "R" => Ok(Direction::Right),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn shift(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
        };
    }

    fn follow(&mut self, other: Point) {
        let x_diff = other.x - self.x;
        let y_diff = other.y - self.y;

        if i32::abs(x_diff) == 2 || i32::abs(y_diff) == 2 {
            self.x += x_diff.signum();
            self.y += y_diff.signum();
        }
    }
}

fn part1(input: &str) -> usize {
    let mut points = HashSet::new();
    let mut head_position = Point::from(0, 0);
    let mut tail_position = Point::from(0, 0);
    for (d, count) in input.lines().map(|l| l.split_once(' ').unwrap()) {
        for _ in 0..count.parse().unwrap() {
            head_position.shift(Direction::from_str(d).unwrap());
            tail_position.follow(head_position);
            points.insert(tail_position);
        }
    }
    points.len()
}

fn part2(input: &str) -> usize {
    let mut points = HashSet::new();
    let mut positions = vec![Point::from(0, 0); 10];
    for (d, count) in input.lines().map(|l| l.split_once(' ').unwrap()) {
        for _ in 0..count.parse().unwrap() {
            positions[0].shift(Direction::from_str(d).unwrap());
            for i in 1..positions.len() {
                let head = positions[i - 1];
                positions[i].follow(head);
            }
            points.insert(positions[positions.len() - 1]);
        }
    }
    points.len()
}

fn main() {
    let input = aoc2022::read_input(9);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT_1), 13);
    }

    const INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT_1), 1);
        assert_eq!(part2(INPUT_2), 36);
    }
}
