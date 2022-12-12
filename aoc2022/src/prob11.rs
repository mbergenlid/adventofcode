use std::ops::Deref;
use itertools::Itertools;

type Item = u128;

struct Monkey {
    items: Vec<Item>,
    operation: Box<dyn Fn(Item) -> Item>,
    test: Test,
    inspections: usize,
}

struct Test {
    condition: Box<dyn Fn(Item) -> bool>,
    if_true: usize,
    if_false: usize,
}

fn calculate_monkey_business(rounds: usize, worry_divider: Item) -> usize {
    let mut monkeys = monkeys();

    let modulo = 2*7*3*17*11*19*5*13;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let mut pass = Vec::new();
            {
                let current_monkey = &mut monkeys[i];

                while let Some(mut worry_level) = current_monkey.items.pop() {
                    worry_level = current_monkey.operation.deref()(worry_level);
                    worry_level = worry_level / worry_divider;
                    worry_level = worry_level % modulo;

                    let next_monkey = if current_monkey.test.condition.deref()(worry_level) {
                        current_monkey.test.if_true
                    } else {
                        current_monkey.test.if_false
                    };

                    pass.push((next_monkey, worry_level));
                    current_monkey.inspections += 1;
                }
            }
            for (next_monkey, worry_level) in pass {
                monkeys[next_monkey].items.push(worry_level);
            }
        }

    }

    monkeys.iter().map(|m| m.inspections).sorted().rev().take(2).product()
}

pub fn solve_part_1(_input: &str) -> usize {
    calculate_monkey_business(20, 3)
}

pub fn solve_part_2(_input: &str) -> usize {
    calculate_monkey_business(10_000, 1)
}

fn monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![80],
            operation: Box::new(|old| old * 5),
            test: Test {
                condition: Box::new(|item| item % 2 == 0),
                if_true: 4,
                if_false: 3,
            },

            inspections: 0,
        },
        Monkey {
            items: vec![75, 83, 74],
            operation: Box::new(|old| old + 7),
            test: Test {
                condition: Box::new(|item| item % 7 == 0),
                if_true: 5,
                if_false: 6,
            },
            inspections: 0,
        },
        Monkey {
            items: vec![86, 67, 61, 96, 52, 63, 73],
            operation: Box::new(|old| old + 5),
            test: Test {
                condition: Box::new(|item| item % 3 == 0),
                if_true: 7,
                if_false: 0,
            },
            inspections: 0,
        },
        Monkey {
            items: vec![85, 83, 55, 85, 57, 70, 85, 52],
            operation: Box::new(|old| old + 8),
            test: Test {
                condition: Box::new(|item| item % 17 == 0),
                if_true: 1,
                if_false: 5,
            },
            inspections: 0,
        },
        Monkey {
            items: vec![67, 75, 91, 72, 89],
            operation: Box::new(|old| old + 4),
            test: Test {
                condition: Box::new(|item| item % 11 == 0),
                if_true: 3,
                if_false: 1,
            },
            inspections: 0,
        },
        Monkey {
            items: vec![66, 64, 68, 92, 68, 77],
            operation: Box::new(|old| old * 2),
            test: Test {
                condition: Box::new(|item| item % 19 == 0),
                if_true: 6,
                if_false: 2,
            },
            inspections: 0,
        },
        Monkey {
            items: vec![97, 94, 79, 88],
            operation: Box::new(|old| { old * old}),
            test: Test {
                condition: Box::new(|item| item % 5 == 0),
                if_true: 2,
                if_false: 7,
            },
            inspections: 0,
        },
        Monkey {
            items: vec![77, 85],
            operation: Box::new(|old| old + 6),
            test: Test {
                condition: Box::new(|item| item % 13 == 0),
                if_true: 4,
                if_false: 0,
            },
            inspections: 0,
        },
    ]
}
