pub fn solve_part_1() -> u32 {
    fill(0, input().as_slice(), 150).len() as u32
}

pub fn solve_part_2() -> u32 {
    let combinations = fill(0, input().as_slice(), 150);
    let min = combinations.iter().min().unwrap();
    combinations.iter().filter(|&v| v == min).count() as u32
}

fn fill(containers_used: u32, containers: &[u32], eggnog: u32) -> Vec<u32> {
    let mut combinations = Vec::new();
    for (i, &c) in containers.iter().enumerate() {
        if c < eggnog {
            let new_containers = containers.iter().skip(i + 1).cloned().collect::<Vec<_>>();
            combinations.append(&mut fill(
                containers_used + 1,
                new_containers.as_slice(),
                eggnog - c,
            ));
        } else if c == eggnog {
            combinations.push(containers_used + 1);
        }
    }
    combinations
}

fn input() -> Vec<u32> {
    vec![
        33, 14, 18, 20, 45, 35, 16, 35, 1, 13, 18, 13, 50, 44, 48, 6, 24, 41, 30, 42,
    ]
}
