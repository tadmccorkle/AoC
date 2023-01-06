use std::{collections::HashMap, str::FromStr};

use aoc2022;

struct Blueprint {
    ore_cost: u32,
    clay_cost: u32,
    obsidian_cost_ore: u32,
    obsidian_cost_clay: u32,
    geode_cost_ore: u32,
    geode_cost_obsidian: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseBlueprintError;

impl FromStr for Blueprint {
    type Err = ParseBlueprintError;

    fn from_str<'a>(s: &'a str) -> Result<Self, Self::Err> {
        let mut robot_recipes = s.split(&[':', '.'][..]).skip(1);
        let mut next_recipe_or_err = || robot_recipes.next().ok_or(ParseBlueprintError);
        let nth_or_err = |input: &'a str, nth: usize| {
            input.trim().split(' ').nth(nth).ok_or(ParseBlueprintError)
        };
        let parse_or_err =
            |input: &str| -> Result<u32, _> { input.parse().map_err(|_| ParseBlueprintError) };

        let ore_recipe = next_recipe_or_err()?;
        let clay_recipe = next_recipe_or_err()?;
        let obsidian_recipe = next_recipe_or_err()?;
        let geode_recipe = next_recipe_or_err()?;

        let ore_cost = nth_or_err(ore_recipe, 4)?;
        let ore_cost = parse_or_err(ore_cost)?;
        let clay_cost = nth_or_err(clay_recipe, 4)?;
        let clay_cost = parse_or_err(clay_cost)?;
        let obsidian_cost_ore = nth_or_err(obsidian_recipe, 4)?;
        let obsidian_cost_ore = parse_or_err(obsidian_cost_ore)?;
        let obsidian_cost_clay = nth_or_err(obsidian_recipe, 7)?;
        let obsidian_cost_clay = parse_or_err(obsidian_cost_clay)?;
        let geode_cost_ore = nth_or_err(geode_recipe, 4)?;
        let geode_cost_ore = parse_or_err(geode_cost_ore)?;
        let geode_cost_obsidian = nth_or_err(geode_recipe, 7)?;
        let geode_cost_obsidian = parse_or_err(geode_cost_obsidian)?;

        Ok(Self {
            ore_cost,
            clay_cost,
            obsidian_cost_ore,
            obsidian_cost_clay,
            geode_cost_ore,
            geode_cost_obsidian,
        })
    }
}

enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

type Stockpile = (u32, u32, u32);
type Robots = (u32, u32, u32, u32);

fn build_and_collect(
    robot: Robot,
    blueprint: &Blueprint,
    stockpile: &Stockpile,
    robots: &Robots,
) -> (Stockpile, Robots) {
    let (mut ore, mut clay, mut obsidian) = collect(robots, stockpile);
    let (mut ore_robots, mut clay_robots, mut obsidian_robots, mut geode_robots) = robots;
    match robot {
        Robot::Ore => {
            ore -= blueprint.ore_cost;
            ore_robots += 1;
        }
        Robot::Clay => {
            ore -= blueprint.clay_cost;
            clay_robots += 1;
        }
        Robot::Obsidian => {
            ore -= blueprint.obsidian_cost_ore;
            clay -= blueprint.obsidian_cost_clay;
            obsidian_robots += 1;
        }
        Robot::Geode => {
            ore -= blueprint.geode_cost_ore;
            obsidian -= blueprint.geode_cost_obsidian;
            geode_robots += 1;
        }
    }

    (
        (ore, clay, obsidian),
        (ore_robots, clay_robots, obsidian_robots, geode_robots),
    )
}

fn collect(robots: &Robots, stockpile: &Stockpile) -> Stockpile {
    (
        stockpile.0 + robots.0,
        stockpile.1 + robots.1,
        stockpile.2 + robots.2,
    )
}

fn max_geodes(
    blueprint: &Blueprint,
    robots: Robots,
    stockpile: Stockpile,
    time_remaining: u32,
    cache: &mut HashMap<(Robots, Stockpile, u32), u32>,
) -> u32 {
    if let Some(&max) = cache.get(&(robots, stockpile, time_remaining)) {
        return max;
    }

    let (ore_robots, clay_robots, obsidian_robots, geode_robots) = robots;
    if time_remaining == 1 {
        return geode_robots;
    }

    let (ore, clay, obsidian) = stockpile;

    let mut max = 0;
    let mut max_geodes_for =
        |robots, stockpile| max_geodes(blueprint, robots, stockpile, time_remaining - 1, cache);

    // Additional checks can probably be done to improve performance. Some I've tried reduced
    // runtime by ~50%, but, while they worked for my input set, they are not guaranteed to work
    // for all input sets. This implementation should always work, but it isn't very fast.

    // Don't build a robot if there isn't enough time for it to pay off.
    // Don't build a resource robot if all other robot requirements for that resource have been met.

    let need_ore_robots = time_remaining > 3
        && (ore_robots < blueprint.clay_cost
            || ore_robots < blueprint.obsidian_cost_ore
            || ore_robots < blueprint.geode_cost_ore);
    if need_ore_robots && ore >= blueprint.ore_cost {
        let (stockpile, robots) = build_and_collect(Robot::Ore, blueprint, &stockpile, &robots);
        max = max.max(max_geodes_for(robots, stockpile));
    }

    let need_clay_robots = time_remaining > 4 && clay_robots < blueprint.obsidian_cost_clay;
    if need_clay_robots && ore >= blueprint.clay_cost {
        let (stockpile, robots) = build_and_collect(Robot::Clay, blueprint, &stockpile, &robots);
        max = max.max(max_geodes_for(robots, stockpile));
    }

    let need_obsidian_robots =
        time_remaining > 3 && obsidian_robots < blueprint.geode_cost_obsidian;
    if need_obsidian_robots
        && clay >= blueprint.obsidian_cost_clay
        && ore >= blueprint.obsidian_cost_ore
    {
        let (stockpile, robots) =
            build_and_collect(Robot::Obsidian, blueprint, &stockpile, &robots);
        max = max.max(max_geodes_for(robots, stockpile));
    }

    if obsidian >= blueprint.geode_cost_obsidian && ore >= blueprint.geode_cost_ore {
        // if a geode robot can be built, check what would happen if no robots were built (i.e. we
        // only collect this turn) only if other types of robots are still needed
        if (need_ore_robots && blueprint.ore_cost < ore + ((time_remaining - 4) * ore_robots))
            || (need_clay_robots && blueprint.clay_cost < ore + ((time_remaining - 5) * ore_robots))
            || (need_obsidian_robots
                && blueprint.obsidian_cost_ore < ore + ((time_remaining - 4) * ore_robots)
                && blueprint.obsidian_cost_clay < clay + ((time_remaining - 4) * clay_robots))
        {
            max = max.max(max_geodes_for(robots, collect(&robots, &stockpile)));
        }

        let (stockpile, robots) = build_and_collect(Robot::Geode, blueprint, &stockpile, &robots);
        max = max.max(max_geodes_for(robots, stockpile));
    } else {
        // if a geode robot cannot be built, always determine what would happen if we only collect
        // this turn
        max = max.max(max_geodes_for(robots, collect(&robots, &stockpile)));
    }

    max += geode_robots;
    cache.insert((robots, stockpile, time_remaining), max);
    max
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| line.parse::<Blueprint>().unwrap())
        .map(|blueprint| max_geodes(&blueprint, (1, 0, 0, 0), (0, 0, 0), 24, &mut HashMap::new()))
        .enumerate()
        .map(|(i, geodes)| (i as u32 + 1) * geodes)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .take(3)
        .map(|line| line.parse::<Blueprint>().unwrap())
        .map(|blueprint| max_geodes(&blueprint, (1, 0, 0, 0), (0, 0, 0), 32, &mut HashMap::new()))
        .product()
}

fn main() {
    let input = aoc2022::read_input(19);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
    Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 33);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 3472);
    }
}
