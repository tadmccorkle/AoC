use std::{collections::HashMap, str::Lines};

use aoc2022;

fn traverse_filesystem(input: &mut Lines, root_dir: &str) -> HashMap<String, u32> {
    let mut filesystem = HashMap::new();
    filesystem.insert(root_dir.to_string(), 0);

    while let Some(line) = input.next() {
        if line == "$ cd .." {
            break;
        } else if line.starts_with("$ cd") {
            let child_dir = format!("{}{}/", root_dir, line.split(' ').last().unwrap());
            let children = traverse_filesystem(input, &child_dir);
            let current_size = filesystem.get_mut(root_dir).unwrap();
            *current_size += children.get(&child_dir).unwrap();
            for (k, v) in children {
                filesystem.insert(k, v);
            }
        } else if line.starts_with(|c: char| c.is_digit(10)) {
            let file_size: u32 = line.split(' ').next().unwrap().parse().unwrap();
            let current_size = filesystem.get_mut(root_dir).unwrap();
            *current_size += file_size;
        }
    }

    filesystem
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let root_dir = lines.next().unwrap().split(' ').last().unwrap();
    let dirs = traverse_filesystem(&mut lines, root_dir);

    dirs.into_values()
        .fold(0, |acc, size| if size < 100_000 { acc + size } else { acc })
}

const MAX_SPACE: u32 = 70_000_000;
const REQUIRED_SPACE: u32 = 30_000_000;

fn part2(input: &str) -> u32 {
    let mut lines = input.lines();
    let root_dir = lines.next().unwrap().split(' ').last().unwrap();
    let dirs = traverse_filesystem(&mut lines, root_dir);
    let used_space = dirs.get(root_dir).unwrap();
    let free_space = MAX_SPACE - used_space;
    let min_delete_space = REQUIRED_SPACE - free_space;

    dirs.into_values()
        .filter(|&size| size >= min_delete_space)
        .min()
        .unwrap()
}

fn main() {
    let input = aoc2022::read_input(7);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 95437);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 24933642);
    }
}
