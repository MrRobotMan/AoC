use std::collections::HashMap;

use aoc::runner::{output, Runner};

#[derive(Default)]
pub struct AocDay {
    input: String,
    blueprints: HashMap<usize, Blueprint>,
}

impl AocDay {
    pub fn new<S: Into<String>>(input: S) -> Self {
        Self {
            input: input.into(),
            ..Default::default()
        }
    }
}

impl Runner for AocDay {
    fn name(&self) -> (usize, usize) {
        (2022, 19)
    }

    fn parse(&mut self) {
        self.blueprints = aoc::read_lines(&self.input)
            .iter()
            .map(|l| {
                let (name, blueprint) = l.split_once(": ").unwrap();
                (
                    name.split_ascii_whitespace()
                        .last()
                        .unwrap()
                        .parse()
                        .unwrap(),
                    blueprint.into(),
                )
            })
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.blueprints
                .iter()
                .fold(0, |acc, (id, bp)| acc + (id * bp.collect_geodes(24))),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let bp1 = self.blueprints[&1].collect_geodes(32);
        let bp2 = self.blueprints[&2].collect_geodes(32);
        let bp3 = self.blueprints[&3].collect_geodes(32);
        output(bp1 * bp2 * bp3)
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_robot: usize,               // ore
    clay_robot: usize,              // ore
    obsidian_robot: (usize, usize), // ore, clay
    geode_robot: (usize, usize),    // ore, obsidian
}

impl Blueprint {
    fn collect_geodes(&self, time: usize) -> usize {
        let mut best = 0;

        let max_resources = [
            // Max ore, clay & obsidian
            self.ore_robot.max(
                self.clay_robot
                    .max(self.obsidian_robot.0.max(self.geode_robot.0)),
            ),
            self.obsidian_robot.1,
            self.geode_robot.1,
        ];
        let mut seen = vec![State {
            robots: [1, 0, 0, 0],
            resources: [0; 4],
            time,
        }];

        while let Some(state) = seen.pop() {
            if state.time == 0 {
                best = best.max(state.resources[3]);
                continue;
            }

            let ore = state.resources[0];
            let clay = state.resources[1];
            let obsidian = state.resources[2];
            let geodes = state.resources[3];

            let maxed_robots = state.robots[0] >= max_resources[0]
                && state.robots[1] >= max_resources[1]
                && state.robots[2] >= max_resources[2];

            // Start building a robot
            if ore >= self.geode_robot.0 && obsidian >= self.geode_robot.1 {
                seen.push(state.tick(
                    Some(3),
                    [
                        ore - self.geode_robot.0,
                        clay,
                        obsidian - self.geode_robot.1,
                        geodes,
                    ],
                ));
                continue; // Only make geode robots if possible.
            };

            if ore >= self.obsidian_robot.0
                && clay >= self.obsidian_robot.1
                && state.robots[2] < max_resources[2]
            {
                seen.push(state.tick(
                    Some(2),
                    [
                        ore - self.obsidian_robot.0,
                        clay - self.obsidian_robot.1,
                        obsidian,
                        geodes,
                    ],
                ));
                continue; // Only make obsidian robots if possible.
            };

            if ore >= self.clay_robot && state.robots[1] < max_resources[1] {
                seen.push(state.tick(Some(1), [ore - self.clay_robot, clay, obsidian, geodes]));
            };

            if ore >= self.ore_robot && state.robots[0] < max_resources[0] {
                seen.push(state.tick(Some(0), [ore - self.ore_robot, clay, obsidian, geodes]));
            };

            if !maxed_robots {
                seen.push(state.tick(None, [ore, clay, obsidian, geodes]));
            };
        }
        best
    }
}

impl<T: AsRef<str>> From<T> for Blueprint {
    fn from(value: T) -> Self {
        let parts = value
            .as_ref()
            .split(". ")
            .map(|p| p.split_ascii_whitespace().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Self {
            ore_robot: parts[0][4].parse().unwrap(),
            clay_robot: parts[1][4].parse().unwrap(),
            obsidian_robot: (parts[2][4].parse().unwrap(), parts[2][7].parse().unwrap()),
            geode_robot: (parts[3][4].parse().unwrap(), parts[3][7].parse().unwrap()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct State {
    robots: [usize; 4],
    resources: [usize; 4],
    time: usize,
}

impl State {
    fn tick(&self, robot: Option<usize>, resources: [usize; 4]) -> Self {
        let mut new_state = *self;
        // Update to spent resources
        new_state.resources = resources;

        // Harvest resourses
        for (idx, bot) in new_state.robots.iter().enumerate() {
            new_state.resources[idx] += bot;
        }

        // Finish building
        if let Some(bot) = robot {
            new_state.robots[bot] += 1;
        };

        new_state.time -= 1;
        new_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_part1() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 33;
        let actual = day.part1()[0].parse().unwrap_or_default();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let mut day = AocDay {
            input: INPUT.into(),
            ..Default::default()
        };
        day.parse();
        let expected = 62;
        let actual = day.blueprints[&2].collect_geodes(32);
        assert_eq!(expected, actual);
    }
}
