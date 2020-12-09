
pub fn solve_part_1(input: &str) -> u64 {
    let xmas: Vec<_> = input.lines().map(|l| l.parse::<u64>().unwrap()).collect();

    for (index, &value) in xmas.iter().enumerate().skip(25) {
        if !two_numbers_sum_up_to(&xmas[index-25..index], value) {
            return value;
        }
    }
    panic!("Not found")
}

pub fn solve_part_2(input: &str) -> u64 {
    let target = solve_part_1(input);
    let xmas: Vec<_> = input.lines().map(|l| l.parse::<u64>().unwrap()).collect();

    let sequence = find_sequence(xmas.as_slice(), target).expect("Sequence not found");

    xmas[sequence.0..sequence.1].iter().max().unwrap() + xmas[sequence.0..sequence.1].iter().min().unwrap()
}

fn two_numbers_sum_up_to(slice: &[u64], value: u64) -> bool {
    for (index, x) in slice.iter().enumerate() {
        if slice.iter().skip(index).any(|y| x != y && x + y == value) {
            return true;
        }
    }
    false
}

fn find_sequence(xmas: &[u64], target: u64) -> Option<(usize, usize)> {
    let mut from_index = 0;
    let mut to_index = 1;
    let mut current_sum = xmas[0];

    loop {
        if current_sum < target {
            if to_index == xmas.len() {
                return None;
            }
            current_sum += xmas[to_index];
            to_index += 1;
        } else if current_sum > target {
            current_sum -= xmas[from_index];
            from_index += 1;
            if from_index == to_index {
                if to_index == xmas.len() {
                    return None;
                }
                current_sum += xmas[to_index];
                to_index += 1;
            }
        } else if current_sum == target {
            return Some((from_index, to_index));
        }
    }
}

