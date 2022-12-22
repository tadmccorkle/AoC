use std::str::FromStr;

use aoc2022;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn distance_from(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for Point {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split(&['=', ','][..]);

        let x = components
            .nth(1)
            .ok_or(ParsePointError)?
            .parse()
            .map_err(|_| ParsePointError)?;
        let y = components
            .nth(1)
            .ok_or(ParsePointError)?
            .parse()
            .map_err(|_| ParsePointError)?;

        Ok(Point { x, y })
    }
}

fn taken_positions(input: &str, row: i64) -> i64 {
    let taken_spans = input
        .lines()
        // sensors and closest beacons
        .map(|l| {
            let (sensor, beacon) = l.split_once(':').unwrap();
            (sensor.parse::<Point>().unwrap(), beacon.parse().unwrap())
        })
        // sensors and max ranges
        .map(|(sensor, beacon)| (sensor, sensor.distance_from(&beacon)))
        // filter for sensors that can cover part of row
        .filter(|&(sensor, range)| sensor.y.abs_diff(row) <= range)
        // span covered by each sensor on row
        .map(|(sensor, range)| {
            let distance_from_row = sensor.y.abs_diff(row);
            let span_diff = (range - distance_from_row) as i64;
            (sensor.x - span_diff, sensor.x + span_diff)
        });

    let mut consolidated_spans = Vec::<(i64, i64)>::new();
    for span in taken_spans {
        let overlaps_existing_span = consolidated_spans
            .iter()
            .any(|&(start, end)| start <= span.1 && span.0 <= end);
        if overlaps_existing_span {
            let overlapping_span = consolidated_spans
                .iter()
                .filter(|&&(start, end)| start <= span.1 && span.0 <= end);
            let consolidated_span =
                overlapping_span.fold((span.0, span.1), |acc, s| (acc.0.min(s.0), acc.1.max(s.1)));
            consolidated_spans.retain(|&(start, end)| start > span.1 || span.0 > end);
            consolidated_spans.push(consolidated_span);
        } else {
            consolidated_spans.push(span);
        }
    }

    consolidated_spans
        .iter()
        .fold(0, |acc, (start, end)| acc + (end - start))
}

fn part1(input: &str) -> i64 {
    taken_positions(input, 2_000_000)
}

fn get_tuning_frequency(input: &str, max_coord: i64) -> i64 {
    let nearby_sensors = input
        .lines()
        // sensors and closest beacons
        .map(|l| {
            let (sensor, beacon) = l.split_once(':').unwrap();
            (sensor.parse::<Point>().unwrap(), beacon.parse().unwrap())
        })
        // sensors and max ranges
        .map(|(sensor, beacon)| (sensor, sensor.distance_from(&beacon)))
        // filter for sensors that can cover points between (0,0) - (max_coord, max_coord)
        .filter(|&(sensor, range)| {
            (sensor.y >= 0 && sensor.y <= max_coord && sensor.x >= 0 && sensor.x <= max_coord)
                || ((sensor.y.abs() as u64 <= range || sensor.y.abs_diff(max_coord) <= range)
                    && (sensor.x.abs() as u64 <= range || sensor.x.abs_diff(max_coord) <= range))
        });

    let mut distress_beacon = Point { x: 0, y: 0 };
    for row in 0..=max_coord {
        let mut consolidated_spans = Vec::<(i64, i64)>::new();
        for (sensor, range) in nearby_sensors.clone() {
            let distance_from_row = sensor.y.abs_diff(row);
            if distance_from_row > range {
                continue;
            }

            let span_diff = (range - distance_from_row) as i64;
            let span = (
                (sensor.x - span_diff).max(0),
                (sensor.x + span_diff).min(max_coord),
            );
            let overlaps_existing_span = consolidated_spans
                .iter()
                .any(|&(start, end)| start <= span.1 && span.0 <= end);
            if overlaps_existing_span {
                let overlapping_span = consolidated_spans
                    .iter()
                    .filter(|&&(start, end)| start <= span.1 && span.0 <= end);
                let consolidated_span = overlapping_span
                    .fold((span.0, span.1), |acc, s| (acc.0.min(s.0), acc.1.max(s.1)));
                consolidated_spans.retain(|&(start, end)| start > span.1 || span.0 > end);
                consolidated_spans.push(consolidated_span);
            } else {
                consolidated_spans.push(span);
            }
        }

        consolidated_spans.sort();
        let mut x = 0;
        for (start, end) in consolidated_spans {
            if start != x {
                distress_beacon = Point { x, y: row };
                break;
            }
            x = end + 1;
        }
    }

    (distress_beacon.x * 4_000_000) + distress_beacon.y
}

fn part2(input: &str) -> i64 {
    get_tuning_frequency(input, 4_000_000)
}

fn main() {
    let input = aoc2022::read_input(15);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part1_example() {
        assert_eq!(taken_positions(INPUT, 10), 26);
    }

    #[test]
    fn part2_example() {
        assert_eq!(get_tuning_frequency(INPUT, 20), 56000011);
    }
}
