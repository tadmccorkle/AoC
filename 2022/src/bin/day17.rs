use std::{collections::HashMap, iter};

use aoc2022;

const CHAMBER_WIDTH: usize = 7;

type Chamber = Vec<Vec<bool>>;

enum Shift {
    Left,
    Right,
}

#[derive(Debug)]
enum Shape {
    Horizontal(usize, usize),
    Plus(usize, usize),
    Corner(usize, usize),
    Vertical(usize, usize),
    Square(usize, usize),
}

impl Shape {
    fn add(index: usize, chamber: &mut Chamber) -> Self {
        let current_chamber_height = chamber.len() - 1;
        let rock_tower_height = current_chamber_height
            - chamber
                .iter()
                .rev()
                .position(|r| r.contains(&true))
                .unwrap_or(0);
        let (shape, required_chamber_height) = match index % 5 {
            0 => (
                Shape::Horizontal(2, rock_tower_height + 4),
                rock_tower_height + 4,
            ),
            1 => (Shape::Plus(2, rock_tower_height + 5), rock_tower_height + 6),
            2 => (
                Shape::Corner(2, rock_tower_height + 4),
                rock_tower_height + 6,
            ),
            3 => (
                Shape::Vertical(2, rock_tower_height + 4),
                rock_tower_height + 7,
            ),
            4 => (
                Shape::Square(2, rock_tower_height + 4),
                rock_tower_height + 5,
            ),
            _ => panic!("cannot add shape at index: {index}"),
        };

        if required_chamber_height > current_chamber_height {
            chamber.extend(
                iter::repeat(vec![false; CHAMBER_WIDTH])
                    .take(required_chamber_height - current_chamber_height),
            );
        }

        shape
    }

    fn shift_left(self, chamber: &Chamber) -> Self {
        match self {
            Shape::Horizontal(x, y) => {
                if x > 0 && !chamber[y][x - 1] {
                    Shape::Horizontal(x - 1, y)
                } else {
                    self
                }
            }
            Shape::Plus(x, y) => {
                if x > 0 && !(chamber[y][x - 1] || chamber[y + 1][x] || chamber[y - 1][x]) {
                    Shape::Plus(x - 1, y)
                } else {
                    self
                }
            }
            Shape::Corner(x, y) => {
                if x > 0 && !(chamber[y][x - 1] || chamber[y + 1][x + 1] || chamber[y + 2][x + 1]) {
                    Shape::Corner(x - 1, y)
                } else {
                    self
                }
            }
            Shape::Vertical(x, y) => {
                if x > 0
                    && !(chamber[y][x - 1]
                        || chamber[y + 1][x - 1]
                        || chamber[y + 2][x - 1]
                        || chamber[y + 3][x - 1])
                {
                    Shape::Vertical(x - 1, y)
                } else {
                    self
                }
            }
            Shape::Square(x, y) => {
                if x > 0 && !(chamber[y][x - 1] || chamber[y + 1][x - 1]) {
                    Shape::Square(x - 1, y)
                } else {
                    self
                }
            }
        }
    }

    fn shift_right(self, chamber: &Chamber) -> Self {
        match self {
            Shape::Horizontal(x, y) => {
                if x + 4 < CHAMBER_WIDTH && !chamber[y][x + 4] {
                    Shape::Horizontal(x + 1, y)
                } else {
                    self
                }
            }
            Shape::Plus(x, y) => {
                if x + 3 < CHAMBER_WIDTH
                    && !(chamber[y][x + 3] || chamber[y + 1][x + 2] || chamber[y - 1][x + 2])
                {
                    Shape::Plus(x + 1, y)
                } else {
                    self
                }
            }
            Shape::Corner(x, y) => {
                if x + 3 < CHAMBER_WIDTH
                    && !(chamber[y][x + 3] || chamber[y + 1][x + 3] || chamber[y + 2][x + 3])
                {
                    Shape::Corner(x + 1, y)
                } else {
                    self
                }
            }
            Shape::Vertical(x, y) => {
                if x + 1 < CHAMBER_WIDTH
                    && !(chamber[y][x + 1]
                        || chamber[y + 1][x + 1]
                        || chamber[y + 2][x + 1]
                        || chamber[y + 3][x + 1])
                {
                    Shape::Vertical(x + 1, y)
                } else {
                    self
                }
            }
            Shape::Square(x, y) => {
                if x + 2 < CHAMBER_WIDTH && !(chamber[y][x + 2] || chamber[y + 1][x + 2]) {
                    Shape::Square(x + 1, y)
                } else {
                    self
                }
            }
        }
    }

    fn fall(self, chamber: &mut Chamber) -> (Self, bool) {
        match self {
            Shape::Horizontal(x, y) => {
                if !(chamber[y - 1][x]
                    || chamber[y - 1][x + 1]
                    || chamber[y - 1][x + 2]
                    || chamber[y - 1][x + 3])
                {
                    (Shape::Horizontal(x, y - 1), false)
                } else {
                    chamber[y][x] = true;
                    chamber[y][x + 1] = true;
                    chamber[y][x + 2] = true;
                    chamber[y][x + 3] = true;
                    (self, true)
                }
            }
            Shape::Plus(x, y) => {
                if !(chamber[y - 1][x] || chamber[y - 2][x + 1] || chamber[y - 1][x + 2]) {
                    (Shape::Plus(x, y - 1), false)
                } else {
                    chamber[y][x] = true;
                    chamber[y + 1][x + 1] = true;
                    chamber[y][x + 1] = true;
                    chamber[y - 1][x + 1] = true;
                    chamber[y][x + 2] = true;
                    (self, true)
                }
            }
            Shape::Corner(x, y) => {
                if !(chamber[y - 1][x] || chamber[y - 1][x + 1] || chamber[y - 1][x + 2]) {
                    (Shape::Corner(x, y - 1), false)
                } else {
                    chamber[y][x] = true;
                    chamber[y][x + 1] = true;
                    chamber[y][x + 2] = true;
                    chamber[y + 1][x + 2] = true;
                    chamber[y + 2][x + 2] = true;
                    (self, true)
                }
            }
            Shape::Vertical(x, y) => {
                if !chamber[y - 1][x] {
                    (Shape::Vertical(x, y - 1), false)
                } else {
                    chamber[y][x] = true;
                    chamber[y + 1][x] = true;
                    chamber[y + 2][x] = true;
                    chamber[y + 3][x] = true;
                    (self, true)
                }
            }
            Shape::Square(x, y) => {
                if !(chamber[y - 1][x] || chamber[y - 1][x + 1]) {
                    (Shape::Square(x, y - 1), false)
                } else {
                    chamber[y][x] = true;
                    chamber[y + 1][x] = true;
                    chamber[y][x + 1] = true;
                    chamber[y + 1][x + 1] = true;
                    (self, true)
                }
            }
        }
    }
}

#[allow(dead_code)]
fn print_chamber(chamber: &Chamber) {
    for row in chamber.iter().rev() {
        println!(
            "{}",
            row.iter()
                .map(|&c| if c { '#' } else { '.' })
                .collect::<String>()
        );
    }
}

fn tower_height(chamber: &Chamber) -> usize {
    (chamber.len() - 1)
        - chamber
            .iter()
            .rev()
            .position(|r| r.contains(&true))
            .unwrap_or(0)
}

fn part1(input: &str) -> usize {
    let mut chamber = vec![vec![false; CHAMBER_WIDTH]; 4];
    for i in 0..CHAMBER_WIDTH {
        chamber[0][i] = true;
    }

    let mut moves = input
        .chars()
        .map(|c| match c {
            '<' => Shift::Left,
            '>' => Shift::Right,
            x => panic!("unexpected shift character: {}", x),
        })
        .cycle();

    for i in 0..2022 {
        let mut shape = Shape::add(i, &mut chamber);

        loop {
            shape = match moves.next() {
                Some(Shift::Left) => shape.shift_left(&chamber),
                Some(Shift::Right) => shape.shift_right(&chamber),
                None => panic!("move iterator has unexpected operation"),
            };

            let fall_result = shape.fall(&mut chamber);
            if fall_result.1 {
                break;
            }
            shape = fall_result.0;
        }
    }

    tower_height(&chamber)
}


fn part2(input: &str) -> usize {
    const SIM_ITERATIONS: usize = 1000000000000;

    let mut chamber = vec![vec![false; CHAMBER_WIDTH]; 4];
    for i in 0..CHAMBER_WIDTH {
        chamber[0][i] = true;
    }

    let mut moves = input
        .chars()
        .enumerate()
        .map(|(i, c)| match c {
            '<' => (i, Shift::Left),
            '>' => (i, Shift::Right),
            x => panic!("unexpected shift character: {}", x),
        })
        .cycle();

    // Determine if cycle occurs by comparing the current shape type, the index of the move list,
    // and the shape of the top of the tower (all these values can be stored as `usize`). This 
    // won't find a cycle in some edge cases (e.g., if a column is never used).
    let mut move_history = HashMap::<(usize, usize, [usize; 7]), usize>::new();
    let mut tower_heights = HashMap::<usize, usize>::new();
    let mut cycle_start_index = 0;

    let mut mv_index = 0;
    for i in 0..SIM_ITERATIONS {
        let mut shape = Shape::add(i, &mut chamber);

        let mut tower_shape = [0, 0, 0, 0, 0, 0, 0];
        for (i, row) in chamber.iter().rev().enumerate() {
            if tower_shape.iter().all(|&c| c > 0) {
                break;
            }

            for (j, &c) in row.iter().enumerate() {
                if c && tower_shape[j] == 0 {
                    tower_shape[j] = i;
                }
            }
        }

        if let Some(csi) = move_history.insert((i % 5, mv_index, tower_shape), i) {
            cycle_start_index = csi;
            break;
        }

        mv_index = loop {
            let (i, mv) = moves.next().expect("move iterator failed unexpectedly");
            shape = match mv {
                Shift::Left => shape.shift_left(&chamber),
                Shift::Right => shape.shift_right(&chamber),
            };

            let fall_result = shape.fall(&mut chamber);
            if fall_result.1 {
                break i;
            }
            shape = fall_result.0;
        };

        tower_heights.insert(i, tower_height(&chamber));
    }

    if cycle_start_index > 0 {
        let base_height = tower_heights.get(&(cycle_start_index - 1)).unwrap();
        let cycle_length = move_history.len() - cycle_start_index;
        let cycle_height = tower_height(&chamber) - base_height;

        let sim_full_cycles = (SIM_ITERATIONS - cycle_start_index) / cycle_length;
        let sim_sub_cycle_length = (SIM_ITERATIONS - cycle_start_index) % cycle_length;
        let sim_sub_cycle_height = tower_heights
            .get(&(&cycle_start_index + sim_sub_cycle_length - 1))
            .unwrap()
            - base_height;

        base_height + (cycle_height * sim_full_cycles) + sim_sub_cycle_height
    } else {
        // worst case...we couldn't find a cycle
        tower_height(&chamber)
    }
}

fn main() {
    let input = aoc2022::read_input(17);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 3068);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 1514285714288);
    }
}
