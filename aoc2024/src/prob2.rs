use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    input.lines().filter(|line| is_safe(parse(line))).count()
}

fn parse(line: &str) -> Vec<usize> {
    line.split(" ")
        .map(|a| {
            a.parse::<usize>()
                .unwrap_or_else(|_| panic!("Not a number {}", a))
        })
        .collect::<Vec<_>>()
}

fn is_safe<T>(numbers: T) -> bool
where
    T: IntoIterator<Item = usize> + Clone,
{
    let (first, second) = numbers
        .clone()
        .into_iter()
        .take(2)
        .collect_tuple::<(_, _)>()
        .expect("Iterator not enought elements");
    let asc = first < second;
    let clone = numbers.clone();
    for (first, second) in numbers.into_iter().zip(clone.into_iter().skip(1)) {
        if asc {
            if !(first < second && second - first <= 3) {
                return false;
            }
        } else if !(first > second && first - second <= 3) {
            return false;
        }
    }

    true
}

pub fn solve_part_2(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let numbers = parse(line);

            if is_safe(numbers.clone()) {
                return true;
            }
            for i in 0..numbers.len() {
                let mut copy = numbers.clone();
                copy.remove(i);
                if is_safe(copy) {
                    return true;
                }
            }
            false
        })
        .count()
}

#[cfg(test)]
mod test {
    use crate::prob2::{solve_part_1, solve_part_2};

    #[test]
    fn test_part1() {
        assert_eq!(solve_part_1(INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part_2(INPUT), 4);
        assert_eq!(solve_part_2("8 9 7 6 5 4"), 1);
    }

    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
}
