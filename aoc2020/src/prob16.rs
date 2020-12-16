use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> u32 {
    let mut sections = input.split("\n\n");
    let rules = sections
        .next()
        .expect("Invalid input")
        .lines()
        .map(|line| line.parse::<Rule>().unwrap())
        .collect::<Vec<_>>();

    let mut error_rate = 0;
    for ticket_str in sections.nth(1).expect("Invalid input").lines().skip(1) {
        for ticket_value in ticket_str
            .trim()
            .split(",")
            .map(|s| s.parse::<u32>().unwrap())
        {
            if !rules.iter().any(|rule| rule.validate(ticket_value)) {
                error_rate += ticket_value;
            }
        }
    }
    error_rate
}

fn is_valid_ticket(rules: &[Rule], ticket_str: &str) -> bool {
    for ticket_value in ticket_str
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
    {
        if !rules.iter().any(|rule| rule.validate(ticket_value)) {
            return false;
        }
    }
    return true;
}

pub fn solve_part_2(input: &str) -> u64 {
    let mut sections = input.split("\n\n");
    let rules = sections
        .next()
        .expect("Invalid input")
        .lines()
        .map(|line| line.parse::<Rule>().unwrap())
        .collect::<Vec<_>>();

    let my_ticket: Vec<_> = sections
        .next()
        .expect("Invalid input")
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut possible_assignments: Vec<HashSet<&String>> =
        vec![rules.iter().map(|rule| &rule.name).collect::<HashSet<_>>(); my_ticket.len()];

    for ticket_str in sections
        .next()
        .expect("Invalid input")
        .lines()
        .skip(1)
        .filter(|&ticket| is_valid_ticket(rules.as_slice(), ticket))
    {
        for (index, ticket_value) in ticket_str
            .split(",")
            .enumerate()
            .map(|(i, s)| (i, s.parse::<u32>().unwrap()))
        {
            let possible_fields: HashSet<_> = rules
                .iter()
                .filter(|&rule| rule.validate(ticket_value))
                .map(|r| &r.name)
                .filter(|&f| possible_assignments[index].contains(f))
                .collect();

            possible_assignments[index] = possible_fields;
        }
    }

    let mut actual_assignments: Vec<Option<&String>> = vec![None; my_ticket.len()];
    while let Some((i, x)) = possible_assignments
        .iter()
        .enumerate()
        .find(|(_, s)| s.len() == 1)
    {
        let &field = x.iter().next().unwrap();
        actual_assignments[i] = Some(field);
        for pa in possible_assignments.iter_mut() {
            pa.remove(field);
        }
    }

    actual_assignments
        .iter()
        .map(|n| n.unwrap())
        .enumerate()
        .filter(|(_, name)| name.starts_with("departure"))
        .map(|(i, _)| my_ticket[i] as u64)
        .product()
}

#[derive(Deserialize, Debug, Eq, PartialEq, Recap)]
#[recap(
    regex = r#"(?P<name>[^:]+): (?P<start1>\d+)-(?P<end1>\d+) or (?P<start2>\d+)-(?P<end2>\d+)"#
)]
struct Rule {
    name: String,
    start1: u32,
    end1: u32,
    start2: u32,
    end2: u32,
}

impl Rule {
    fn validate(&self, value: u32) -> bool {
        (value >= self.start1 && value <= self.end1) || (value >= self.start2 && value <= self.end2)
    }
}

#[cfg(test)]
mod test {
    use crate::prob16::{solve_part_1, solve_part_2, Rule};

    #[test]
    fn test_parse() {
        assert_eq!(
            "departure platform: 39-863 or 877-970"
                .parse::<Rule>()
                .unwrap(),
            Rule {
                name: "departure platform".to_string(),
                start1: 39,
                end1: 863,
                start2: 877,
                end2: 970
            }
        )
    }

    const TESTCASE_1: &'static str = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(TESTCASE_1), 71);
    }

    const TESTCASE_2: &'static str = "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TESTCASE_2), 1);
        assert_eq!(solve_part_2(include_str!("../inputs/prob16")), 855275529001);
    }
}
