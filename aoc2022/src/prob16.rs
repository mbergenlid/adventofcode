use itertools::{Itertools, min};
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::os::macos::raw::stat;
use std::time::Instant;

#[derive(Clone)]
struct Valve {
    id: String,
    rate: u32,
    tunnels: Vec<String>,
    steps: HashMap<String, u32>,
}

impl Valve {
    fn parse(s: &str) -> Self {
        let valve_id: String = s.chars().skip("Valve ".len()).take(2).collect();
        let (index, flow_rate) = s
            .chars()
            .enumerate()
            .skip("Valve AA has flow rate=".len())
            .take_while(|(_, c)| c.is_numeric())
            .map(|(i, c)| (i, c.to_digit(10).unwrap()))
            .reduce(|(i1, d1), (i2, d2)| (max(i1, i2), d1 * 10 + d2))
            .unwrap();

        let tunnels = if s[(index + 1 + "; ".len())..].starts_with("tunnels lead to valves") {
            s[(index + 1 + "; tunnels lead to valves ".len())..]
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        } else {
            vec![s[(index + 1 + "; tunnel leads to valve ".len())..].to_string()]
        };

        Valve {
            id: valve_id,
            rate: flow_rate,
            tunnels,
            steps: HashMap::new(),
        }
    }

    fn steps_to(&self, destination: &str, all_valves: &Vec<Valve>) -> u32 {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((&self.id, 0));

        while let Some((next, steps)) = queue.pop_front() {
            if next == destination {
                return steps;
            }

            visited.insert(next);
            let valve = all_valves
                .iter()
                .find(|v| &v.id == next)
                .expect(&format!("Doesn't exist {}", next));

            for tunnel in valve.tunnels.iter() {
                if visited.contains(tunnel) {
                    continue;
                }

                queue.push_back((tunnel, steps + 1));
            }
        }

        panic!("Not found");
    }
}

fn  dynamic_programming(valves: &Vec<&Valve>, connections: &Vec<Vec<u32>>) -> Vec<Vec<Vec<u32>>> {
    //state[min][set] = max pressure when `min` minutes left
    let num_valves = valves.len();
    let mut state = vec![vec![vec![0; num_valves]; 1 << (num_valves)]; 30];

    //      1   2
    // 0  [ 0  [0]
    // 1  [ 0  []
    // 2  [

    for min_left in 2..=29 {
        for comb in 1..(1 << num_valves) {
            for valve in 0..num_valves {
                let mut max_pressure = 0;

                for n in 0..num_valves {
                    if valve != n && comb & (0b1 << n) > 0 {
                        let steps = connections[valve][n];
                        if steps + 1 < min_left {
                            let min_left_if_opening = min_left - (steps + 1);

                            max_pressure = max(
                                valves[n].rate * min_left_if_opening
                                    + state[(min_left_if_opening) as usize][comb & !(0b1 << n)]
                                        [n],
                                max_pressure,
                            );
                        }
                    }
                }
                state[(min_left) as usize][comb][valve] = max_pressure;
                // println!("Minutes left = {}, Valve = {}, Comb = {:#01b}, {:?}", min_left, valve, comb, state[min_left as usize][comb][valve]);
            }
        }
    }

    println!(
        "Ran {} iterations ({}*{}*{}*{})",
        30 * (1 << num_valves) * num_valves * num_valves,
        30,
        1 << num_valves,
        num_valves,
        num_valves
    );
    state
}

fn create_state_map(valves: &Vec<Valve>) -> Vec<Vec<u32>> {
    let valves_with_rate = valves.iter().filter(|v| v.rate > 0).collect::<Vec<_>>();

    let mut state = vec![vec![0; valves_with_rate.len()]; valves_with_rate.len()];
    for (v1_index, v1) in valves_with_rate.iter().enumerate() {
        for (v2_index, v2) in valves_with_rate.iter().enumerate() {
            if v1_index == v2_index {
                continue;
            }
            let steps = v1.steps_to(v2.id.as_str(), valves);

            state[v1_index][v2_index] = steps;
        }
    }

    state
}

pub fn solve_part_1(input: &str) -> usize {
    let valves: Vec<_> = input.lines().map(|line| Valve::parse(line)).collect();

    let start_valve = valves.iter().find(|v| v.id == "AA").unwrap();
    let valves_with_rate = valves.iter().filter(|v| v.rate > 0).collect::<Vec<_>>();
    let state = create_state_map(&valves);

    let result = dynamic_programming(&valves_with_rate, &state);

    let mut max_pressure = 0;
    for (i, valve) in valves_with_rate.iter().enumerate() {
        let steps = start_valve.steps_to(valve.id.as_str(), &valves) + 1;

        max_pressure = max(
            max_pressure,
            (30 - steps)*valve.rate + result[30-steps as usize][0b111111111111111 & !(0b1 << i)][i]
        );
    }
    println!("{:?}", result[28][0b111011]);

    max_pressure as usize
}

pub fn solve_part_2(input: &str) -> usize {
    let valves: Vec<_> = input.lines().map(|line| Valve::parse(line)).collect();

    let start_valve = valves.iter().find(|v| v.id == "AA").unwrap();
    let valves_with_rate = valves.iter().filter(|v| v.rate > 0).collect::<Vec<_>>();
    let state = create_state_map(&valves);
    let mut best = HashMap::new();

    let mut queue = VecDeque::new();
    for (index, valve) in valves_with_rate.iter().enumerate() {
        let steps = start_valve.steps_to(valve.id.as_str(), &valves);
        let minutes_left = 30 - (steps + 1);
        queue.push_back((index, minutes_left, minutes_left*valve.rate, 0b1 << index));
        best.insert(0b1 << index, minutes_left*valve.rate);
    }

    let mut states = 0;
    let mut skipped_because_unreachable = 0;
    while let Some((valve_index, minutes_left, best_pressure, visited)) = queue.pop_front() {
        states += 1;
        for (n_index, _) in valves_with_rate.iter().enumerate().filter(|(index, _)| visited & (0b1 << index) == 0) {
            let steps = state[valve_index][n_index];
            if steps + 1 < minutes_left {
                let minutes_left = minutes_left - (steps + 1);
                let pressure = best_pressure + minutes_left*valves_with_rate[n_index].rate;
                queue.push_back((n_index, minutes_left, pressure, visited | (0b1 << n_index)));
                if let Some(current_best) = best.get_mut(&(visited | (0b1 << n_index))) {
                    if pressure > *current_best {
                        *current_best = pressure;
                    }
                } else {
                    best.insert(visited | (0b1 << n_index), pressure);
                }
            } else {
                let mut non_visited = 0;
                let mut visited_mut = visited;
                while visited_mut > 0 {
                    if visited_mut % 2 == 0 {
                        non_visited += 1;
                    }
                    visited_mut = visited_mut /2;
                }
                skipped_because_unreachable = skipped_because_unreachable + (non_visited);
            }
        }
    }

    println!("Total states: {}, States seen {}, skipped {}", states + skipped_because_unreachable, states, skipped_because_unreachable);

    let mut best_pressure = 0;
    for (index, (key1, value1)) in best.iter().enumerate() {
        for (key2, value2) in best.iter().skip(index+1) {
            if key1 & key2 == 0 {
                best_pressure = max(best_pressure, value1+value2);
            }
        }
    }

    best_pressure as usize
}

#[cfg(test)]
mod test {
    use crate::prob16::solve_part_1;

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(INPUT), 1651);
    }

    const INPUT: &'static str = r"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
}
