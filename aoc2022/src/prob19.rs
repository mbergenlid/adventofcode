use nom::bytes::complete::tag;
use nom::character::complete;
use nom::sequence::{preceded, separated_pair, terminated};
use std::cmp::{max, Ordering};
use std::os::macos::raw::stat;
use std::str::FromStr;

#[derive(Debug)]
struct Blueprint<const M: u32> {
    index: u32,
    ore_robot: u32,
    clay_robot: u32,
    obsidian_robot: (u32, u32),
    geode_robot: (u32, u32),
}

#[derive(Clone, Hash, Eq, PartialEq, Copy, Debug)]
struct State {
    minute: u32,
    total_ore: u32,
    total_clay: u32,
    total_obsidian: u32,
    total_geode: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

impl State {
    fn advance(&self, minutes: u32) -> Self {
        State {
            minute: self.minute + minutes,
            total_ore: self.total_ore + self.ore_robots * minutes,
            total_clay: self.total_clay + self.clay_robots * minutes,
            total_obsidian: self.total_obsidian + self.obsidian_robots * minutes,
            total_geode: self.total_geode + self.geode_robots * minutes,
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
        }
    }

    fn buy_ore<const M: u32>(mut self, blueprint: &Blueprint<M>) -> Self {
        self.ore_robots += 1;
        self.total_ore -= blueprint.ore_robot;
        self
    }

    fn buy_clay<const M: u32>(mut self, blueprint: &Blueprint<M>) -> Self {
        self.clay_robots += 1;
        self.total_ore -= blueprint.clay_robot;
        self
    }

    fn buy_obsidian<const M: u32>(mut self, blueprint: &Blueprint<M>) -> Self {
        self.obsidian_robots += 1;
        self.total_ore -= blueprint.obsidian_robot.0;
        self.total_clay -= blueprint.obsidian_robot.1;
        self
    }

    fn buy_geode<const M: u32>(mut self, blueprint: &Blueprint<M>) -> Self {
        self.geode_robots += 1;
        self.total_ore -= blueprint.geode_robot.0;
        self.total_obsidian -= blueprint.geode_robot.1;
        self
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_geode
            .cmp(&other.total_geode)
            .then_with(|| self.total_obsidian.cmp(&other.total_obsidian))
            .then_with(|| self.total_clay.cmp(&other.total_clay))
            .then_with(|| self.total_ore.cmp(&other.total_ore))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl <const M: u32> FromStr for Blueprint<M> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, index) = terminated::<_, _, _, (), _, _>(
            preceded(tag("Blueprint "), complete::u32),
            tag(": "),
        )(s)
        .unwrap();
        let (s, ore_cost) = terminated::<_, _, _, (), _, _>(
            preceded(tag("Each ore robot costs "), complete::u32),
            tag(" ore. "),
        )(s)
        .unwrap();
        let (s, clay_cost) = terminated::<_, _, _, (), _, _>(
            preceded(tag("Each clay robot costs "), complete::u32),
            tag(" ore. "),
        )(s)
        .unwrap();
        let (s, obsidian_cost) = preceded::<_, _, _, (), _, _>(
            tag("Each obsidian robot costs "),
            separated_pair(
                terminated(complete::u32, tag(" ore")),
                tag(" and "),
                terminated(complete::u32, tag(" clay. ")),
            ),
        )(s)
        .unwrap();
        let (_, geode_cost) = preceded::<_, _, _, (), _, _>(
            tag("Each geode robot costs "),
            separated_pair(
                terminated(complete::u32, tag(" ore")),
                tag(" and "),
                terminated(complete::u32, tag(" obsidian.")),
            ),
        )(s)
        .unwrap();

        Ok(Blueprint {
            index,
            ore_robot: ore_cost,
            clay_robot: clay_cost,
            obsidian_robot: obsidian_cost,
            geode_robot: geode_cost,
        })
    }
}

impl<const M: u32> Blueprint<M> {

    fn solve(&self, state: State) -> u32 {
        if state.minute >= M {
            return state.total_geode;
        }

        let mut max_geode = 0;

        //bulld a geode robot next
        let minutes_until_geode_buildable = max(
            if state.total_ore >= self.geode_robot.0 {
                0
            } else {
                (self.geode_robot.0 - state.total_ore).div_ceil(state.ore_robots)
            },
            if state.total_obsidian >= self.geode_robot.1 {
                0
            } else {
                if state.obsidian_robots == 0 {
                    u32::MAX
                } else {
                    (self.geode_robot.1 - state.total_obsidian).div_ceil(state.obsidian_robots)
                }
            },
        );

        if minutes_until_geode_buildable < M - state.minute {
            let minutes_inc = minutes_until_geode_buildable + 1;
            max_geode = max(
                self.solve(state.advance(minutes_inc).buy_geode(self)),
                max_geode,
            );
        }

        //build an obsidian next
        let minutes_until_obsedian_buildable = max(
            if state.total_ore >= self.obsidian_robot.0 {
                0
            } else {
                (self.obsidian_robot.0 - state.total_ore).div_ceil(state.ore_robots)
            },
            if state.total_clay >= self.obsidian_robot.1 {
                0
            } else {
                if state.clay_robots == 0 {
                    u32::MAX
                } else {
                    (self.obsidian_robot.1 - state.total_clay).div_ceil(state.clay_robots)
                }
            },
        );
        if minutes_until_obsedian_buildable < M - state.minute
            && state.obsidian_robots < self.geode_robot.1
        {
            let minutes_inc = minutes_until_obsedian_buildable + 1;
            max_geode = max(
                self.solve(
                    state.advance(minutes_inc).buy_obsidian(self),
                ),
                max_geode,
            );
        }

        //build a clay next
        let minutes_until_clay_buildable = if state.total_ore >= self.clay_robot {
            0
        } else {
            (self.clay_robot - state.total_ore).div_ceil(state.ore_robots)
        };
        if minutes_until_clay_buildable < M - state.minute
            && state.clay_robots < self.obsidian_robot.1
        {
            let minutes_inc = minutes_until_clay_buildable + 1;
            max_geode = max(
                self.solve(state.advance(minutes_inc).buy_clay(self),
                ),
                max_geode,
            );
        }

        //build an ore next
        let minutes_until_ore_buildable = if state.total_ore >= self.ore_robot {
            0
        } else {
            (self.ore_robot - state.total_ore).div_ceil(state.ore_robots)
        };
        if minutes_until_ore_buildable < M - state.minute && state.ore_robots < 4 {
            let minutes_inc = minutes_until_ore_buildable + 1;
            max_geode = max(
                self.solve(state.advance(minutes_inc).buy_ore(self),
                ),
                max_geode,
            );
        }

        max(
            state.total_geode + state.geode_robots*(M-state.minute),
            max_geode
        )
    }
}

pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Blueprint<24>>().unwrap())
        // .map(|b| {
        //     println!("{:?}", b);
        //     b
        // })
        .map(|b| {
            let result = b.solve(
                State {
                    minute: 0,
                    total_ore: 0,
                    total_clay: 0,
                    total_obsidian: 0,
                    total_geode: 0,
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 0,
                },
            );
            result * b.index
        })
        .sum::<u32>() as usize
}

pub fn solve_part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Blueprint<32>>().unwrap())
        .take(3)
        .map(|b| {
            let result = b.solve(
                State {
                    minute: 0,
                    total_ore: 0,
                    total_clay: 0,
                    total_obsidian: 0,
                    total_geode: 0,
                    ore_robots: 1,
                    clay_robots: 0,
                    obsidian_robots: 0,
                    geode_robots: 0,
                },
            );
            result
        })
        .product::<u32>() as usize
}

#[cfg(test)]
mod test {
    use crate::prob19::{solve_part_1, solve_part_2};

    #[test]
    fn test_1() {
        // assert_eq!(solve_part_1(INPUT), 33);
        assert_eq!(solve_part_1("Blueprint 22: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 11 clay. Each geode robot costs 4 ore and 8 obsidian."), 66);
    }

    #[test]
    fn test_3() {
        assert_eq!(solve_part_2(INPUT), 3596);
    }



    const INPUT: &'static str = r"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
}
