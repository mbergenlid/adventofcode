
fn snafu_to_i64(s: &str) -> i64 {
    s.chars().map(|c| match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '-' => -1,
        '=' => -2,
        _ => panic!(),
    }).reduce(|accum, n| accum * 5 + n).unwrap()
}

//1747 = 1=-0-2
//2022 = 1=11-2

//3125 625 125 25 5 1

fn i64_to_snafu(n: i64) -> String {
    let mut number = n;
    let mut result = String::new();

    let mut x = 1;
    loop {
        let div = ((number as f64) / (x as f64)).round() as i64;
        if div >= -2 && div <= 2 {
            break;
        }
        x *= 5;
    }
    while x > 0 {
        let div = ((number as f64) / (x as f64)).round() as i64;

        if div <= 2 {
            if div >= 0 {
                result.push(char::from_digit(div as u32, 10).unwrap());
            } else if div == -1 {
                result.push('-');
            } else if div == -2 {
                result.push('=');
            } else {
                unreachable!();
            }
            number = number - div*x;
        }
        x /= 5;
    }

    result
}

pub fn solve_part_1(input: &str) -> String {
    let sum = input.lines().map(|line| snafu_to_i64(line)).sum::<i64>();
    i64_to_snafu(sum)
}

pub fn solve_part_2(_input: &str) -> usize {
    todo!()
}


#[cfg(test)]
mod test {
    use crate::prob25::{i64_to_snafu, snafu_to_i64, solve_part_1};

    #[test]
    fn test_snafu() {
        assert_eq!(snafu_to_i64("1=-0-2"), 1747);
        assert_eq!(i64_to_snafu(1747), "1=-0-2".to_string());
    }

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(INPUT), "2=-1=0".to_string());
        assert_eq!(solve_part_1(include_str!("../inputs/prob25")), "2=-1=0".to_string());
    }

    const INPUT: &'static str = r"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
}