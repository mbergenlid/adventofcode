use std::collections::VecDeque;

fn parse_stacks<'a>(input: impl Iterator<Item = &'a str>) -> Vec<VecDeque<char>> {
    let mut res = vec![VecDeque::new(); 9];

    for line in input {
        for i in (1..).step_by(4).take(9) {
            match line.chars().nth(i) {
                Some(' ') => {}
                Some(c) => res[i / 4].push_back(c),
                None => {}
            };
        }
    }
    res
}

#[derive(Deserialize, Recap)]
#[recap(regex = r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)")]
struct Move {
    from: usize,
    to: usize,

    amount: u32,
}

pub fn solve_part_1(input: &str) -> usize {
    let mut stacks = parse_stacks(input.lines().take_while(|l| !l.starts_with(" 1 ")));

    let moves = input.lines().skip_while(|line| !line.is_empty()).skip(1);

    for m_str in moves {
        let m: Move = m_str.parse().unwrap();

        for _ in 0..m.amount {
            let x = stacks[m.from - 1]
                .pop_front()
                .expect("Unable to pop from emtpy stack");
            stacks[m.to - 1].push_front(x);
        }
    }

    let result: String = stacks.iter().map(|s| s.front().unwrap()).collect();
    println!("{}", result);
    0
}

pub fn solve_part_2(input: &str) -> usize {
    let mut stacks = parse_stacks(input.lines().take_while(|l| !l.starts_with(" 1 ")));

    let moves = input.lines().skip_while(|line| !line.is_empty()).skip(1);

    for m_str in moves {
        let m: Move = m_str.parse().unwrap();

        let mut to_push = Vec::new();
        for _ in 0..m.amount {
            let x = stacks[m.from - 1]
                .pop_front()
                .expect("Unable to pop from emtpy stack");
            to_push.push(x);
        }
        for x in to_push.into_iter().rev() {
            stacks[m.to - 1].push_front(x);
        }
    }

    let result: String = stacks.iter().map(|s| s.front().unwrap()).collect();
    println!("{}", result);
    0
}
