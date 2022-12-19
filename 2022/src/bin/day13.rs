use std::{cmp::Ordering, iter::Peekable, str::Chars};

use aoc2022;

#[derive(Eq)]
enum ListItem {
    Number(u32),
    List(Vec<ListItem>),
}

impl ListItem {
    fn list_cmp(left: &Vec<Self>, right: &Vec<Self>) -> Ordering {
        let (mut left_iter, mut right_iter) = (left.iter(), right.iter());
        let mut ordering = Ordering::Equal;
        while ordering == Ordering::Equal {
            ordering = match (left_iter.next(), right_iter.next()) {
                (Some(l), Some(r)) => l.partial_cmp(r).unwrap(),
                (None, None) => break,
                (l, _) => {
                    if l.is_some() {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                }
            };
        }

        ordering
    }
}

impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ListItem::List(self_list), ListItem::List(other_list)) => {
                ListItem::list_cmp(self_list, other_list)
            }
            (ListItem::List(_), ListItem::Number(n)) => {
                self.cmp(&ListItem::List(Vec::from([ListItem::Number(*n)])))
            }
            (ListItem::Number(n), ListItem::List(_)) => {
                ListItem::List(Vec::from([ListItem::Number(*n)])).cmp(other)
            }
            (ListItem::Number(self_number), ListItem::Number(other_number)) => {
                self_number.cmp(other_number)
            }
        }
    }
}

impl PartialOrd for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for ListItem {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn parse_number(initial: char, input: &mut Peekable<Chars>) -> ListItem {
    let mut number = String::from(initial);
    while let Some(value) = input.peek() {
        if value.is_digit(10) {
            number.push(input.next().unwrap());
        } else {
            break;
        }
    }

    ListItem::Number(number.parse().unwrap())
}

fn parse_list(input: &mut Peekable<Chars>) -> Vec<ListItem> {
    let mut list = Vec::new();
    while let Some(value) = input.next() {
        if value.is_digit(10) {
            list.push(parse_number(value, input));
        } else if value == '[' {
            list.push(ListItem::List(parse_list(input)));
        } else if value == ']' {
            break;
        }
    }

    list
}

fn parse(input: &str) -> Vec<ListItem> {
    parse_list(&mut input.chars().peekable())
}

fn part1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pair| {
            let (left, right) = pair.split_once('\n').unwrap();
            (parse(left), parse(right))
        })
        .enumerate()
        .filter(|(_, (left, right))| ListItem::list_cmp(left, right) != Ordering::Greater)
        .map(|(i, _)| i + 1)
        .sum()
}

fn get_dividers() -> [ListItem; 2] {
    [
        ListItem::List(Vec::from([ListItem::Number(2)])),
        ListItem::List(Vec::from([ListItem::Number(6)])),
    ]
}

fn part2(input: &str) -> usize {
    let mut packets: Vec<_> = input
        .split_whitespace()
        .map(|packet| parse(packet))
        .collect();

    for divider in get_dividers() {
        packets.push(Vec::from([divider]));
    }
    packets.sort();

    let dividers = get_dividers();
    packets
        .iter()
        .enumerate()
        .filter(|&(_, packet)| packet.len() == 1 && dividers.contains(packet.first().unwrap()))
        .map(|(i, _)| i + 1)
        .product()
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
        assert_eq!(part2(INPUT), 140);
    }
}
