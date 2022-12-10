use aoc2022;

fn part1(input: &str) -> i32 {
    let mut signal_strength = 0;
    let mut x = 1;
    let mut cycle = 0;

    for line in input.lines() {
        cycle += 1;
        if (cycle + 20) % 40 == 0 {
            signal_strength += x * cycle;
        }

        if line.starts_with("addx") {
            cycle += 1;
            if (cycle + 20) % 40 == 0 {
                signal_strength += x * cycle;
            }

            x += line.split_once(' ').unwrap().1.parse::<i32>().unwrap();
        }
    }

    signal_strength
}

fn draw_pixel(crt: &mut String, cycle: i32, x: i32) {
    crt.push(if ((x - 1)..=(x + 1)).contains(&(cycle % 40)) {
        '#'
    } else {
        '.'
    });

    if crt.lines().last().unwrap().len() % 40 == 0 {
        crt.push('\n');
    }
}

fn part2(input: &str) -> String {
    let mut crt = String::from('\n');
    let mut x = 1;
    let mut cycle = -1;

    for line in input.lines() {
        cycle += 1;
        draw_pixel(&mut crt, cycle, x);

        if line.starts_with("addx") {
            cycle += 1;
            draw_pixel(&mut crt, cycle, x);

            x += line.split_once(' ').unwrap().1.parse::<i32>().unwrap();
        }
    }

    crt
}

fn main() {
    let input = aoc2022::read_input(10);
    aoc2022::print_results(&input, part1, part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 13140);
    }
}
