use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let mut circuit = input.parse::<Circuit>().unwrap();

    let mut low_count = 0;
    let mut high_count = 0;
    for _ in 0..1000 {
        let (low, high) = circuit.run(0);
        low_count += low;
        high_count += high;
    }
    dbg!(low_count) * dbg!(high_count)
}

pub fn solve_part_2(input: &str) -> usize {
    let mut circuit = input.parse::<Circuit>().unwrap();

    // !output = !con
    // !con =  b & a
    // !inv = !a => i mod 2 = 1
    // b    =
    // a    = i mod 2 = 0

    let mut msg_count = 0;
    for i in 0..1000 {
        let (low, high) = circuit.run(msg_count);
        msg_count += low + high;
    }

    if let ModuleType::FlipFlop(_, state_changes) =
        &circuit.modules.iter().find(|m| m.name == "kr").unwrap().tpe
    {
        for diff in state_changes
            .iter()
            .zip(state_changes.iter().skip(1))
            .map(|(x1, x2)| x2.1 - x1.1)
        {
            println!("{}", diff);
        }
    }
    unreachable!()
}

#[derive(Debug)]
struct Circuit {
    modules: Vec<Module>,
    broadcaster: usize,
    module_names: HashMap<String, usize>,
}

impl Circuit {
    fn run(&mut self, msg_count: usize) -> (usize, usize) {
        let broadcaster_index = self.broadcaster;
        let mut queue = VecDeque::new();
        queue.push_back(Message {
            from: usize::MAX,
            to: broadcaster_index,
            value: false,
            counter: msg_count,
        });
        let mut low_count = 0;
        let mut high_count = 0;
        while let Some(msg) = queue.pop_front() {
            let module = &mut self.modules[msg.to];
            for out in module.send(msg) {
                if out.value {
                    high_count += 1;
                } else {
                    low_count += 1;
                }
                if out.to < usize::MAX - 1 {
                    queue.push_back(out);
                }
            }
        }

        (low_count + 1, high_count)
    }

    fn run2(&mut self, index: i32) -> bool {
        let mut output_seen_false = false;
        let flip_flop_a = *self.module_names.get("a").unwrap();
        let flip_flop_b = *self.module_names.get("b").unwrap();
        let broadcaster_index = self.broadcaster;
        let mut queue = VecDeque::new();
        queue.push_back(Message {
            from: usize::MAX,
            to: broadcaster_index,
            value: false,
            counter: 0,
        });
        while let Some(msg) = queue.pop_front() {
            let module = &mut self.modules[msg.to];
            let to_send = module.send(msg);
            if let Some(msg) = to_send.iter().next() {
                if msg.from == flip_flop_a {
                    println!("{} a => {}", index, msg.value);
                }
            }
            if let Some(msg) = to_send.iter().next() {
                if msg.from == flip_flop_b {
                    println!("{} b => {}", index, msg.value);
                }
            }
            for out in to_send {
                if out.to < usize::MAX - 1 {
                    queue.push_back(out);
                } else if out.to == usize::MAX - 1 {
                    if !out.value {
                        output_seen_false = true;
                    }
                }
            }
        }

        return output_seen_false;
    }
}

struct Message {
    from: usize,
    to: usize,
    value: bool,
    counter: usize,
}

#[derive(Clone, Debug)]
struct Module {
    outputs: Vec<usize>,
    tpe: ModuleType,
    name: String,
}

impl Module {
    fn send(&mut self, msg: Message) -> Vec<Message> {
        let output = match &mut self.tpe {
            ModuleType::Broadcaster => msg.value,
            ModuleType::FlipFlop(state, state_changes) => {
                if msg.value {
                    return Vec::new();
                } else {
                    *state = !*state;
                    state_changes.push((*state, msg.counter));
                    *state
                }
            }
            ModuleType::Conjunction(inputs) => {
                inputs[msg.from] = msg.value;
                !inputs.into_iter().all(|input| *input)
            }
        };
        self.forward(msg.to, output, msg.counter)
    }

    fn forward(&self, from: usize, value: bool, start_counter: usize) -> Vec<Message> {
        self.outputs
            .iter()
            .enumerate()
            .map(|(i, o)| Message {
                from,
                to: *o,
                value,
                counter: start_counter + 1 + i,
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop(bool, Vec<(bool, usize)>),
    Conjunction(Vec<bool>),
}

impl FromStr for Circuit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut name_map = HashMap::<String, usize>::new();
        let mut broadcaster = 0;
        for (index, line) in s.lines().enumerate() {
            let name = line.split(" -> ").next().unwrap();
            if name == "broadcaster" {
                broadcaster = index;
                name_map.insert(name.to_string(), index);
            } else {
                name_map.insert(name[1..].to_string(), index);
            }
        }

        let name_map = dbg!(name_map);
        let mut modules = dbg!(vec![None; name_map.len()]);
        let mut input_map = vec![Vec::new(); name_map.len()];
        for line in s.lines() {
            let (name_and_type, outputs) = line.split(" -> ").collect_tuple().unwrap();
            let outputs = outputs
                .split(", ")
                .map(|o| {
                    *name_map.get(o).unwrap_or_else(|| {
                        if o == "rx" {
                            &(usize::MAX - 1)
                        } else {
                            &usize::MAX
                        }
                    })
                })
                .collect::<Vec<_>>();

            let my_index = index_of(&name_map, name_and_type);
            for output in outputs.iter().filter(|&o| *o < usize::MAX - 1) {
                input_map[*output].push(my_index);
            }
            if name_and_type == "broadcaster" {
                modules[my_index] = Some(Module {
                    tpe: ModuleType::Broadcaster,
                    outputs: outputs,
                    name: "broadcaster".to_string(),
                });
            } else if name_and_type.starts_with("%") {
                modules[my_index] = Some(Module {
                    outputs: outputs,
                    tpe: ModuleType::FlipFlop(false, Vec::new()),
                    name: name_and_type[1..].to_string(),
                });
            } else if name_and_type.starts_with("&") {
                modules[my_index] = Some(Module {
                    outputs: outputs,
                    tpe: ModuleType::Conjunction(vec![true; name_map.len()]),
                    name: name_and_type[1..].to_string(),
                });
            }
        }

        fn index_of(name_map: &HashMap<String, usize>, name: &str) -> usize {
            if name == "broadcaster" {
                *name_map.get(name).unwrap()
            } else if name.starts_with("%") {
                *name_map.get(&name[1..]).unwrap()
            } else if name.starts_with("&") {
                *name_map.get(&name[1..]).unwrap()
            } else {
                unreachable!()
            }
        }

        let modules = dbg!(modules);
        let mut result = Vec::new();

        for (index, module) in modules.into_iter().enumerate() {
            assert!(index == name_map.len() || module.is_some());
            if let Some(mut module) = module {
                if let ModuleType::Conjunction(x) = &mut module.tpe {
                    for out in &input_map[index] {
                        x[*out] = false;
                    }
                }
                result.push(module);
            }
        }

        Ok(Circuit {
            modules: result,
            broadcaster,
            module_names: name_map,
        })
    }
}

// !rx = !bn
// !bn = lcm(!pl, !mz, !lz, !zm)
// !pl = !qt
// !

// !rx = bn
// !bn = pl & mz & lz & zm
//
// pl
// kr

//
#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 32000000);
        assert_eq!(super::solve_part_1(TEST_INPUT_2), 11687500);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT_2), 0);
    }

    const TEST_INPUT: &'static str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const TEST_INPUT_2: &'static str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
}
