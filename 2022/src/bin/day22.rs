use aoc2022;

#[derive(PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    Void,
}

impl Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Open,
            '#' => Self::Wall,
            ' ' => Self::Void,
            other => panic!("unexpected tile char {}", other),
        }
    }
}

type Board = Vec<Vec<Tile>>;

fn parse_board(input: &str) -> Board {
    let width = input.lines().map(|l| l.len()).max().unwrap();
    let mut board: Board = input
        .lines()
        .map(|l| l.chars().map(|c| Tile::from(c)).collect())
        .collect();

    for row in &mut board {
        if row.len() < width {
            for _ in 0..(width - row.len()) {
                row.push(Tile::Void);
            }
        }
    }

    board
}

#[allow(dead_code)]
fn print_board(board: &Board) {
    for r in board {
        println!(
            "{}",
            r.iter()
                .map(|t| match t {
                    Tile::Open => '.',
                    Tile::Wall => '#',
                    Tile::Void => ' ',
                })
                .collect::<String>()
        );
    }
}

fn part1(input: &str) -> i32 {
    let (board, moves) = input.split_once("\n\n").unwrap();

    let board = parse_board(board);
    let mut rotations = moves.chars().filter(|&c| c == 'L' || c == 'R');
    let mut moves = moves
        .trim()
        .split(|c| c == 'L' || c == 'R')
        .map(|n| n.parse::<u32>().unwrap());

    let mut change: (i32, i32) = (0, 1);
    let mut row = 0;
    let mut col = board[0]
        .iter()
        .position(|tile| matches!(tile, Tile::Open))
        .unwrap() as i32;

    while let Some(m) = moves.next() {
        for _ in 0..m {
            if change.0 != 0 {
                let mut new_row = row + change.0;
                if new_row < 0
                    || (change.0 < 0 && board[new_row as usize][col as usize] == Tile::Void)
                {
                    new_row = (board.len()
                        - 1
                        - board
                            .iter()
                            .rev()
                            .position(|row| row[col as usize] != Tile::Void)
                            .unwrap()) as i32;
                } else if new_row >= board.len() as i32
                    || (change.0 > 0 && board[new_row as usize][col as usize] == Tile::Void)
                {
                    new_row = board
                        .iter()
                        .position(|row| row[col as usize] != Tile::Void)
                        .unwrap() as i32;
                }

                if board[new_row as usize][col as usize] == Tile::Wall {
                    break;
                }

                row = new_row;
            } else {
                let mut new_col = col + change.1;
                if new_col < 0
                    || (change.1 < 0 && board[row as usize][new_col as usize] == Tile::Void)
                {
                    new_col = (board[0].len()
                        - 1
                        - board[row as usize]
                            .iter()
                            .rev()
                            .position(|tile| !matches!(tile, Tile::Void))
                            .unwrap()) as i32;
                } else if new_col >= board[0].len() as i32
                    || (change.1 > 0 && board[row as usize][new_col as usize] == Tile::Void)
                {
                    new_col = board[row as usize]
                        .iter()
                        .position(|tile| !matches!(tile, Tile::Void))
                        .unwrap() as i32;
                }

                if board[row as usize][new_col as usize] == Tile::Wall {
                    break;
                }

                col = new_col;
            }
        }

        change = match rotations.next() {
            Some('L') => (-change.1, change.0),
            Some('R') => (change.1, -change.0),
            _ => change,
        };
    }

    1000 * (row + 1)
        + 4 * (col + 1)
        + match change {
            (0, 1) => 0,
            (-1, 0) => 1,
            (0, -1) => 2,
            (1, 0) => 3,
            _ => panic!("unexpected change"),
        }
}

fn part2(input: &str) -> String {
    String::from("solution not yet implemented")
}

fn main() {
    let input = aoc2022::read_input(22);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 6032);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), "");
    }
}
