use std::{ops::RangeInclusive, str::FromStr};

use aoc2022;

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(ParsePointError)?;

        let x = x.parse().map_err(|_| ParsePointError)?;
        let y = y.parse().map_err(|_| ParsePointError)?;

        Ok(Point { x, y })
    }
}

fn range(v1: usize, v2: usize) -> RangeInclusive<usize> {
    if v1 < v2 {
        v1..=v2
    } else {
        v2..=v1
    }
}

fn build_cave(input: &str, width: usize, width_offset: usize, height: usize) -> Vec<Vec<bool>> {
    let mut cave = vec![vec![false; width]; height];

    let lines = input
        .lines()
        .map(|l| l.split(" -> ").map(|s| s.parse::<Point>().unwrap()));
    for line in lines {
        for (start, end) in line.clone().zip(line.skip(1)) {
            for i in range(start.x, end.x) {
                cave[start.y][i - width_offset] = true;
            }
            for i in range(start.y, end.y) {
                cave[i][start.x - width_offset] = true;
            }
        }
    }

    cave
}

fn part1(input: &str) -> u32 {
    let all_points = input
        .split(&['\n', ' ', '-', '>'][..])
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Point>().unwrap())
        .collect::<Vec<_>>();
    let min_x = all_points.iter().min_by_key(|p| p.x).unwrap().x;
    let max_x = all_points.iter().max_by_key(|p| p.x).unwrap().x;
    let max_y = all_points.iter().max_by_key(|p| p.y).unwrap().y;

    let mut cave = build_cave(input, max_x - min_x + 2, min_x - 1, max_y + 1);

    let sand_source = (500 - min_x + 1, 0);
    let mut units = 0;
    loop {
        let (mut x, mut y) = sand_source;
        loop {
            if cave[y][x] {
                if cave[y][x - 1] {
                    if cave[y][x + 1] {
                        cave[y - 1][x] = true;
                        units += 1;
                        break;
                    } else {
                        x += 1;
                    }
                } else {
                    x -= 1;
                }
            } else {
                y += 1;

                if y > max_y {
                    break;
                }
            }
        }

        if y > max_y {
            break;
        }
    }

    units
}

fn part2(input: &str) -> u32 {
    let max_y = input
        .split(&['\n', ' ', '-', '>'][..])
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Point>().unwrap())
        .max_by_key(|p| p.y)
        .unwrap()
        .y;

    let mut cave = build_cave(input, 1002, 0, max_y + 3);

    for i in 0..1002 {
        cave[max_y + 2][i] = true;
    }

    let mut units = 0;
    loop {
        let (mut x, mut y) = (500, 0);
        loop {
            if cave[y][x] {
                if cave[y][x - 1] {
                    if cave[y][x + 1] {
                        cave[y - 1][x] = true;
                        units += 1;
                        break;
                    } else {
                        x += 1;
                    }
                } else {
                    x -= 1;
                }
            } else {
                y += 1;
            }
        }

        if x == 500 && y - 1 == 0 {
            break;
        }
    }

    units
}

fn main() {
    let input = aoc2022::read_input(14);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 24);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 93);
    }
}
