use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    str::FromStr,
};

use aoc2022;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Cube {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCubeError;

impl FromStr for Cube {
    type Err = ParseCubeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split(',').map(|c| c.parse().map_err(|_| ParseCubeError));
        let x = components.next().ok_or(ParseCubeError)??;
        let y = components.next().ok_or(ParseCubeError)??;
        let z = components.next().ok_or(ParseCubeError)??;

        Ok(Self { x, y, z })
    }
}

enum CubeComponentGrouping {
    XY,
    XZ,
    YZ,
}

fn group_by(
    cubes: &Vec<Cube>,
    grouping: CubeComponentGrouping,
) -> HashMap<(i32, i32), BinaryHeap<i32>> {
    let mut grouped = HashMap::new();
    for cube in cubes {
        let (group, component) = match grouping {
            CubeComponentGrouping::XY => ((cube.x, cube.y), cube.z),
            CubeComponentGrouping::XZ => ((cube.x, cube.z), cube.y),
            CubeComponentGrouping::YZ => ((cube.y, cube.z), cube.x),
        };
        grouped
            .entry(group)
            .and_modify(|z_values: &mut BinaryHeap<_>| z_values.push(component))
            .or_insert(BinaryHeap::from([component]));
    }

    grouped
}

fn count_overlapping(grouped_components: HashMap<(i32, i32), BinaryHeap<i32>>) -> i32 {
    grouped_components
        .into_values()
        .map(|mut components| {
            let mut overlapping = 0;
            let mut prev = components.pop().unwrap_or(0);
            while let Some(z) = components.pop() {
                if z + 1 == prev {
                    overlapping += 1;
                }
                prev = z;
            }
            overlapping
        })
        .sum()
}

fn part1(input: &str) -> i32 {
    let cubes: Vec<Cube> = input.lines().map(|line| line.parse().unwrap()).collect();
    let same_xy = group_by(&cubes, CubeComponentGrouping::XY);
    let same_xz = group_by(&cubes, CubeComponentGrouping::XZ);
    let same_yz = group_by(&cubes, CubeComponentGrouping::YZ);

    let overlapping_face_count =
        count_overlapping(same_xy) + count_overlapping(same_xz) + count_overlapping(same_yz);

    let cube_count = cubes.len() as i32;

    (cube_count * 6) - (overlapping_face_count * 2)
}

fn get_internal_cubes(cubes: &Vec<Cube>) -> Vec<Cube> {
    let ((min_x, max_x), (min_y, max_y), (min_z, max_z)) = cubes.iter().fold(
        (
            (i32::MAX, i32::MIN), // min_x, max_x
            (i32::MAX, i32::MIN), // min_y, max_y
            (i32::MAX, i32::MIN), // min_z, max_z
        ),
        |acc, cube| {
            (
                (cube.x.min(acc.0 .0), cube.x.max(acc.0 .1)),
                (cube.y.min(acc.1 .0), cube.y.max(acc.1 .1)),
                (cube.z.min(acc.2 .0), cube.z.max(acc.2 .1)),
            )
        },
    );

    // space that contains all cubes
    let container: HashSet<_> = ((min_x - 1)..=(max_x + 1))
        .flat_map(|x| {
            ((min_y - 1)..=(max_y + 1))
                .flat_map(move |y| ((min_z - 1)..=(max_z + 1)).map(move |z| (x, y, z)))
        })
        .collect();

    let cubes = cubes.iter().map(|cube| (cube.x, cube.y, cube.z)).collect();

    // If all known cubes are removed from the container space, only "external"
    // cubes will be traversable from a container boundary, meaning that only
    // "internal" cubes will remain after traversal.
    let mut unvisited: HashSet<_> = container.difference(&cubes).collect();

    let start = (min_x, min_y, min_z);
    unvisited.remove(&start);

    let mut to_visit = Vec::from([start]);
    while let Some(current) = to_visit.pop() {
        let neighbors = [
            (current.0 - 1, current.1, current.2),
            (current.0 + 1, current.1, current.2),
            (current.0, current.1 - 1, current.2),
            (current.0, current.1 + 1, current.2),
            (current.0, current.1, current.2 - 1),
            (current.0, current.1, current.2 + 1),
        ];

        for neighbor in neighbors {
            if unvisited.remove(&neighbor) {
                to_visit.push(neighbor);
            }
        }
    }

    unvisited
        .iter()
        .map(|&&(x, y, z)| Cube { x, y, z })
        .collect()
}

fn part2(input: &str) -> i32 {
    let cubes: Vec<Cube> = input.lines().map(|line| line.parse().unwrap()).collect();
    let same_xy = group_by(&cubes, CubeComponentGrouping::XY);
    let same_xz = group_by(&cubes, CubeComponentGrouping::XZ);
    let same_yz = group_by(&cubes, CubeComponentGrouping::YZ);
    let overlapping_face_count =
        count_overlapping(same_xy) + count_overlapping(same_xz) + count_overlapping(same_yz);
    let cube_count = cubes.len() as i32;
    let total_surface_area = (cube_count * 6) - (overlapping_face_count * 2);

    let internal_cubes = get_internal_cubes(&cubes);
    let same_xy = group_by(&internal_cubes, CubeComponentGrouping::XY);
    let same_xz = group_by(&internal_cubes, CubeComponentGrouping::XZ);
    let same_yz = group_by(&internal_cubes, CubeComponentGrouping::YZ);
    let overlapping_face_count =
        count_overlapping(same_xy) + count_overlapping(same_xz) + count_overlapping(same_yz);
    let cube_count = internal_cubes.len() as i32;
    let internal_surface_area = (cube_count * 6) - (overlapping_face_count * 2);

    total_surface_area - internal_surface_area
}

fn main() {
    let input = aoc2022::read_input(18);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 64);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 58);
    }
}
