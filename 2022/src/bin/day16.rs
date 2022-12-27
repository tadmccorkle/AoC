use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc2022;

type ValveId = u32;

type Connection = (ValveId, u32);

#[derive(Debug)]
struct Valve {
    id: ValveId,
    flow_rate: u32,
    connections: Vec<Connection>,
}

#[derive(Debug)]
struct ParseValveError;

impl Valve {
    fn id_from(s: &str) -> Option<ValveId> {
        let mut chars = s.chars();
        let id = ((chars.next()? as u32) << 8) + chars.next()? as u32;

        Some(id)
    }
}

impl FromStr for Valve {
    type Err = ParseValveError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = Self::id_from(s.get(6..8).ok_or(ParseValveError)?).ok_or(ParseValveError)?;
        let flow_rate = s
            .split(&['=', ';'][..])
            .nth(1)
            .ok_or(ParseValveError)?
            .parse()
            .map_err(|_| ParseValveError)?;
        let connections: Option<Vec<_>> = s
            .split(&[' ', ','][..])
            .skip(9)
            .filter(|c| !c.is_empty())
            .map(|c| Valve::id_from(c))
            .collect();
        let connections = connections
            .ok_or(ParseValveError)?
            .iter()
            .map(|&c| (c, 1))
            .collect();

        Ok(Self {
            id,
            flow_rate,
            connections,
        })
    }
}

// a collection of all weighted connections from a specific node
// weights correspond to the number of edges (i.e. the distance) a node is from the provided node
// uses Dijkstra's algorithm to determine distances
fn connections_from(valve_id: ValveId, valves: &HashMap<ValveId, Valve>) -> Vec<Connection> {
    let mut distance_from_start: HashMap<ValveId, u32> = valves
        .keys()
        .map(|&id| (id, if id == valve_id { 0 } else { u32::MAX }))
        .collect();
    let mut unvisited_valves: HashSet<_> = valves.keys().map(|&id| id).collect();

    while !unvisited_valves.is_empty() {
        let &current_valve_id = unvisited_valves
            .iter()
            .min_by_key(|id| distance_from_start.get(id).unwrap())
            .unwrap();

        let &current_distance = distance_from_start.get(&current_valve_id).unwrap();
        if current_distance == u32::MAX {
            break;
        }

        let next_distance = current_distance + 1;
        let current_valve = valves.get(&current_valve_id).unwrap();
        for &(traversable, _) in current_valve.connections.iter() {
            let &distance_to_traversable = distance_from_start.get(&traversable).unwrap();
            if next_distance < distance_to_traversable {
                distance_from_start
                    .entry(traversable)
                    .and_modify(|d| *d = next_distance);
            }
        }

        unvisited_valves.remove(&current_valve_id);
    }

    distance_from_start
        .iter()
        .map(|(&id, &distance)| (id, distance))
        .collect()
}

fn parse_valves(input: &str) -> HashMap<ValveId, Valve> {
    let valves: Result<HashMap<_, _>, _> = input
        .lines()
        .map(|line| match line.parse::<Valve>() {
            Ok(valve) => Ok((valve.id, (valve))),
            Err(e) => Err(e),
        })
        .collect();

    valves.unwrap()
}

fn connect_valves(valves: &mut HashMap<ValveId, Valve>) {
    let all_connections: Vec<_> = valves
        .keys()
        .map(|&id| (id, connections_from(id, &valves)))
        .collect();

    for (valve_id, connections) in all_connections {
        for &connection in connections.iter().filter(|&&(_, distance)| distance > 1) {
            valves
                .entry(valve_id)
                .and_modify(|v| v.connections.push(connection));
        }
    }
}

fn max_pressure_release(
    valves: &HashMap<ValveId, Valve>,
    current_valve_id: ValveId,
    unreleased_valves: &HashSet<ValveId>,
    time_remaining: i32,
) -> u32 {
    if time_remaining <= 1 {
        return 0;
    }

    let mut unreleased_valves = unreleased_valves.clone();
    unreleased_valves.remove(&current_valve_id);

    let current_valve = valves.get(&current_valve_id).unwrap();
    let time_after_current_valve = if current_valve.flow_rate > 0 {
        time_remaining - 1
    } else {
        time_remaining
    };
    let total_current_valve_release = current_valve.flow_rate * (time_after_current_valve as u32);

    if unreleased_valves.is_empty() {
        return total_current_valve_release;
    }

    let total_valve_connections_release = current_valve
        .connections
        .iter()
        .filter(|&(id, _)| unreleased_valves.contains(id))
        .map(|&(id, distance)| {
            max_pressure_release(
                valves,
                id,
                &unreleased_valves,
                time_after_current_valve - distance as i32,
            )
        })
        .max()
        .unwrap();

    total_current_valve_release + total_valve_connections_release
}

fn part1(input: &str) -> u32 {
    let mut valves = parse_valves(input);
    connect_valves(&mut valves);

    let initial_valve = Valve::id_from("AA").unwrap();
    let unreleased_valves: HashSet<_> = valves
        .iter()
        .filter(|(_, valve)| valve.flow_rate > 0)
        .map(|(&id, _)| id)
        .collect();
    let time_remaining = 30;

    max_pressure_release(&valves, initial_valve, &unreleased_valves, time_remaining)
}

fn part2(input: &str) -> u32 {
    let mut valves = parse_valves(input);
    connect_valves(&mut valves);

    let initial_valve_id = Valve::id_from("AA").unwrap();
    let unreleased_valves: Vec<_> = valves
        .iter()
        .filter(|(_, valve)| valve.flow_rate > 0)
        .map(|(&id, _)| id)
        .collect();

    let mut unique_subset_pairs = Vec::new();
    let unreleased_valve_count = unreleased_valves.len();
    for i in 0..(1 << unreleased_valve_count) {
        if i & 1 == 1 {
            continue;
        }

        let mut subset1 = HashSet::new();
        let mut subset2 = HashSet::new();
        for j in 0..unreleased_valve_count {
            if i & (1 << j) == 0 {
                subset1.insert(unreleased_valves[j]);
            } else {
                subset2.insert(unreleased_valves[j]);
            }
        }
        unique_subset_pairs.push((subset1, subset2));
    }

    unique_subset_pairs
        .iter()
        .map(|(a, b)| {
            max_pressure_release(&valves, initial_valve_id, a, 26)
                + max_pressure_release(&valves, initial_valve_id, b, 26)
        })
        .max()
        .unwrap()
}

fn main() {
    let input = aoc2022::read_input(16);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 1651);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 1707);
    }
}
