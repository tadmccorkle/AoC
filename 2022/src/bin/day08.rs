use aoc2022;

struct NumberGrid {
    cells: Vec<Vec<u32>>,
    width: usize,
    height: usize,
}

impl NumberGrid {
    fn from(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut cells = vec![vec![0; width]; height];
        for (row, line) in input.lines().enumerate() {
            for (col, num) in line.chars().enumerate() {
                cells[row][col] = num.to_digit(10).unwrap();
            }
        }

        Self {
            cells,
            width,
            height,
        }
    }
}

fn part1(input: &str) -> u32 {
    let grid = NumberGrid::from(input);
    let (width, height, cells) = (grid.width, grid.height, grid.cells);

    let mut visible = vec![vec![true; width]; height];
    for row in 1..(height - 1) {
        for col in 1..(width - 1) {
            visible[row][col] = (0..row).all(|i| cells[row][col] > cells[i][col])
                || ((row + 1)..height).all(|i| cells[row][col] > cells[i][col])
                || (0..col).all(|i| cells[row][col] > cells[row][i])
                || ((col + 1)..width).all(|i| cells[row][col] > cells[row][i]);
        }
    }

    visible.iter().fold(0, |acc, row| {
        acc + row.iter().fold(
            0,
            |inner_acc, col| if *col { inner_acc + 1 } else { inner_acc },
        )
    })
}

fn score(row: usize, col: usize, grid: &Vec<Vec<u32>>) -> u32 {
    let mut score_top = 0;
    let mut score_right = 0;
    let mut score_bottom = 0;
    let mut score_left = 0;

    for i in (0..row).rev() {
        score_top += 1;
        if grid[row][col] <= grid[i][col] {
            break;
        }
    }
    for i in (row + 1)..grid.len() {
        score_bottom += 1;
        if grid[row][col] <= grid[i][col] {
            break;
        }
    }
    for i in (0..col).rev() {
        score_left += 1;
        if grid[row][col] <= grid[row][i] {
            break;
        }
    }
    for i in (col + 1)..grid[0].len() {
        score_right += 1;
        if grid[row][col] <= grid[row][i] {
            break;
        }
    }

    score_top * score_right * score_bottom * score_left
}

fn part2(input: &str) -> u32 {
    let grid = NumberGrid::from(input);
    let (width, height, cells) = (grid.width, grid.height, grid.cells);

    let mut scores = vec![vec![0; width]; height];
    for row in 1..(height - 1) {
        for col in 1..(width - 1) {
            scores[row][col] = score(row, col, &cells);
        }
    }

    scores.into_iter().flatten().max().unwrap()
}

fn main() {
    let input = aoc2022::read_input(8);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373\n25512\n65332\n33549\n35390";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 21);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 8);
    }
}
