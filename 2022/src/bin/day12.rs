use std::collections::HashMap;

use aoc2022;

type NodeId = (usize, usize);

struct LocationNode {
    id: NodeId,
    value: char,
    connections: Vec<NodeId>,
}

impl LocationNode {
    fn from(id: NodeId, value: char) -> Self {
        Self {
            id,
            value,
            connections: Vec::new(),
        }
    }

    fn height(&self) -> u8 {
        (if self.is_root() {
            'a'
        } else if self.is_end() {
            'z'
        } else {
            self.value
        }) as u8
    }

    fn is_root(&self) -> bool {
        self.value == 'S'
    }

    fn is_end(&self) -> bool {
        self.value == 'E'
    }

    fn can_traverse_to(&self, other: &LocationNode) -> bool {
        self.height() >= other.height() - 1
    }

    fn is_traversable_from(&self, other: &LocationNode) -> bool {
        other.height() >= self.height() - 1
    }
}

fn traverse_from(start_id: NodeId, nodes: &Vec<Vec<LocationNode>>) -> HashMap<NodeId, u32> {
    let mut distance_from_start: HashMap<NodeId, u32> = nodes
        .iter()
        .flatten()
        .map(|n| (n.id, if n.id == start_id { 0 } else { u32::MAX }))
        .collect();
    let mut unvisited: HashMap<NodeId, &LocationNode> =
        nodes.iter().flatten().map(|n| (n.id, n)).collect();

    while !unvisited.is_empty() {
        let (_, current) = unvisited
            .iter()
            .min_by_key(|&(id, _)| distance_from_start.get(id).unwrap())
            .unwrap();

        let &dist = distance_from_start.get(&current.id).unwrap();
        if dist == u32::MAX {
            break;
        }

        let dist = dist + 1;
        for &traversable in current.connections.iter() {
            let &dist_to_traversable = distance_from_start.get(&traversable).unwrap();
            if dist < dist_to_traversable {
                distance_from_start
                    .entry(traversable)
                    .and_modify(|d| *d = dist);
            }
        }

        unvisited.remove(&current.id);
    }

    distance_from_start
}

fn part1(input: &str) -> u32 {
    let mut nodes: Vec<Vec<LocationNode>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| LocationNode::from((row, col), c))
                .collect::<Vec<LocationNode>>()
        })
        .collect();

    for row in 0..nodes.len() {
        for col in 0..nodes[0].len() {
            if row > 0 && nodes[row][col].can_traverse_to(&nodes[row - 1][col]) {
                nodes[row][col].connections.push((row - 1, col));
            }
            if row < nodes.len() - 1 && nodes[row][col].can_traverse_to(&nodes[row + 1][col]) {
                nodes[row][col].connections.push((row + 1, col));
            }
            if col > 0 && nodes[row][col].can_traverse_to(&nodes[row][col - 1]) {
                nodes[row][col].connections.push((row, col - 1));
            }
            if col < nodes[0].len() - 1 && nodes[row][col].can_traverse_to(&nodes[row][col + 1]) {
                nodes[row][col].connections.push((row, col + 1));
            }
        }
    }

    let root_id = nodes.iter().flatten().find(|&n| n.is_root()).unwrap().id;
    let end_id = nodes.iter().flatten().find(|&n| n.is_end()).unwrap().id;
    let distance_from_start = traverse_from(root_id, &nodes);

    *distance_from_start.get(&end_id).unwrap()
}

fn part2(input: &str) -> u32 {
    let mut nodes: Vec<Vec<LocationNode>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| LocationNode::from((row, col), c))
                .collect::<Vec<LocationNode>>()
        })
        .collect();

    for row in 0..nodes.len() {
        for col in 0..nodes[0].len() {
            if row > 0 && nodes[row][col].is_traversable_from(&nodes[row - 1][col]) {
                nodes[row][col].connections.push((row - 1, col));
            }
            if row < nodes.len() - 1 && nodes[row][col].is_traversable_from(&nodes[row + 1][col]) {
                nodes[row][col].connections.push((row + 1, col));
            }
            if col > 0 && nodes[row][col].is_traversable_from(&nodes[row][col - 1]) {
                nodes[row][col].connections.push((row, col - 1));
            }
            if col < nodes[0].len() - 1 && nodes[row][col].is_traversable_from(&nodes[row][col + 1])
            {
                nodes[row][col].connections.push((row, col + 1));
            }
        }
    }

    let end_id = nodes.iter().flatten().find(|&n| n.is_end()).unwrap().id;
    let distance_from_start = traverse_from(end_id, &nodes);

    let nodes: HashMap<NodeId, &LocationNode> = nodes.iter().flatten().map(|n| (n.id, n)).collect();
    let (_, min_steps) = distance_from_start
        .iter()
        .filter(|(id, _)| nodes.get(id).unwrap().value == 'a')
        .min_by_key(|&(_, distance)| distance)
        .unwrap();

    *min_steps
}

fn main() {
    let input = aoc2022::read_input(12);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 31);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 29);
    }
}
