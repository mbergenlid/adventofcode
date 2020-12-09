use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
}

pub fn solve_part_1(input: &str) -> usize {
    let mut total = 0;
    for line in input.split("\n").filter(|line| !line.is_empty()) {
        let captures = REGEX
            .captures(line)
            .expect(&format!("Not a valid line {}", line));
        let count = count_occurrences(&captures[4], captures[3].chars().next().unwrap());
        if count >= captures[1].parse::<usize>().unwrap()
            && count <= captures[2].parse::<usize>().unwrap()
        {
            total += 1;
        }
    }
    total
}

pub fn solve_part_2(input: &str) -> usize {

    let mut total = 0;
    for line in input.split("\n").filter(|line| !line.is_empty()) {
        let captures = REGEX
            .captures(line)
            .expect(&format!("Not a valid line {}", line));
        let index1 = captures[1].parse::<usize>().unwrap();
        let index2 = captures[2].parse::<usize>().unwrap();
        let password = &captures[4];
        let mut occurrences = 0;
        if password[index1 - 1..index1] == captures[3] {
            occurrences += 1;
        }

        if password[index2 - 1..index2] == captures[3] {
            occurrences += 1;
        }
        if occurrences == 1 {
            total += 1;
        }
    }
    total
}

fn count_occurrences(s: &str, c: char) -> usize {
    s.chars().filter(|&character| character == c).count()
}
