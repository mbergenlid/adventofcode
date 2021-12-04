
pub fn solve_part_1(input: &str) -> usize {
    let length = input.lines().next().unwrap().len();

    let mut ones = vec![0; length];
    for line in input.lines() {
        for (i, d) in line.chars().enumerate() {
            if d == '1' {
                ones[i] += 1;
            }
        }
    }

    let total_numbers = input.lines().count();
    let gamma_rate: usize = ones
        .iter()
        .map(|&i| if i > (total_numbers / 2) { 1 } else { 0 })
        .fold(0, |acc, i| acc * 2 + i);

    let epsilon_rate: usize = !gamma_rate & 0xFFF;
    gamma_rate * epsilon_rate
}

pub fn solve_part_2(input_str: &str) -> usize {
    oxygen_generator_rating(input_str) * co2_scrubber_rating(input_str)
}

fn oxygen_generator_rating(input_str: &str) -> usize {
    let length = input_str.lines().next().unwrap().len();
    let mut input: Vec<_> = input_str.lines().collect();

    let mut ones:usize = input.iter().map(|l| l.chars().nth(0).unwrap()).filter(|&c| c == '1').count();
    let mut total_numbers = input.len();
    let mut index = 0;
    while input.len() > 1 {
        input.retain(|n| n.chars().nth(index).unwrap().to_digit(2).unwrap() == (if ones*2 >= total_numbers { 1 } else { 0 }));
        index += 1;
        if index >= length {
            break;
        }
        ones = input.iter().map(|l| l.chars().nth(index).unwrap()).filter(|&c| c == '1').count();
        total_numbers = input.len();
    }

    usize::from_str_radix(input[0], 2).unwrap()
}

fn co2_scrubber_rating(input_str: &str) -> usize {
    let length = input_str.lines().next().unwrap().len();
    let mut input: Vec<_> = input_str.lines().collect();

    let mut ones:usize = input.iter().map(|l| l.chars().nth(0).unwrap()).filter(|&c| c == '1').count();
    let mut total_numbers = input.len();
    let mut index = 0;
    while input.len() > 1 {
        input.retain(|n| n.chars().nth(index).unwrap().to_digit(2).unwrap() == (if ones*2 < total_numbers { 1 } else { 0 }));
        index += 1;
        if index >= length {
            break;
        }
        ones = input.iter().map(|l| l.chars().nth(index).unwrap()).filter(|&c| c == '1').count();
        total_numbers = input.len();
    }

    usize::from_str_radix(input[0], 2).unwrap()
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        let res = super::solve_part_1(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
        );

        assert_eq!(res, 198);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(
            "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010",
        );

        assert_eq!(res, 230);
    }
}
