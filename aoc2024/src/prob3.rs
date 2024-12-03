use regex::Regex;



pub fn solve_part_1(input: &str) -> usize {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();


    let mut sum = 0;
    for m in re.captures_iter(input) {
        let text = m.get(0).expect("error").as_str();
        sum += mul(text);

    }
    sum
}

pub fn solve_part_2(input: &str) -> usize {
    let re = Regex::new(r"(mul\(\d{1,3},\d{1,3}\))").unwrap();


    let mut source = input;
    let mut result = 0;
    loop {
        let next = source.find("don't()").unwrap_or(source.len());

        for c in re.captures_iter(&source[0..next]) {
            let text = c.get(0).expect("Error").as_str();
            result += mul(text);
        }

        if next == source.len() {
            break;
        }

        source = &source[next..];

        if let Some(next_do) = source.find("do()") {
            source = &source[next_do..];
        } else {
            break;
        }
    }

    result
}

fn mul(text: &str) -> usize {
    let comma_index = text.find(",").unwrap();
    let left = text["mul(".len()..comma_index].parse::<usize>().expect("Not a number");
    let right = text[(comma_index+1)..text.len()-1].parse::<usize>().expect("Not a number");
    left*right
}

#[cfg(test)]
mod test {
    use crate::prob3::{solve_part_1, solve_part_2};


    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(INPUT), 161);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(INPUT_2), 48);
    }

    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
}
