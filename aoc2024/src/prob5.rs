use aoc_lib::graph::{algorithms::topological_sort, AdjacencyListGraph};
use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let (rules, updates) = input
        .split("\n\n")
        .collect_tuple::<(_, _)>()
        .expect("Not a tuple");

    let rules = rules
        .lines()
        .map(|line| {
            line.split('|')
                .map(|n| n.parse::<usize>().expect("Not a number"))
                .collect_tuple::<(_, _)>()
                .expect("Can not happen")
        })
        .collect::<Vec<_>>();

    updates
        .lines()
        .map(|update| {
            update
                .split(",")
                .map(|n| n.parse::<usize>().expect("Not a number in update"))
                .collect_vec()
        })
        .filter(|update| is_valid(update, &rules))
        .map(|update| {
            update
                .iter()
                .copied()
                .skip(update.len() / 2)
                .next()
                .expect("Can't be empty")
        })
        .sum::<usize>()
}

fn is_valid(update: &Vec<usize>, rules: &Vec<(usize, usize)>) -> bool {
    for (i, x) in update.iter().enumerate() {
        for other in &update[i + 1..] {
            if rules.iter().any(|(r1, r2)| r1 == other && r2 == x) {
                return false;
            }
        }
    }
    true
}

pub fn solve_part_2(input: &str) -> usize {
    let (rules, updates) = input
        .split("\n\n")
        .collect_tuple::<(_, _)>()
        .expect("Not a tuple");

    let rules = rules
        .lines()
        .map(|line| {
            line.split('|')
                .map(|n| n.parse::<usize>().expect("Not a number"))
                .collect_tuple::<(_, _)>()
                .expect("Can not happen")
        })
        .collect::<Vec<_>>();

    updates
        .lines()
        .map(|update| {
            update
                .split(",")
                .map(|n| n.parse::<usize>().expect("Not a number in update"))
                .collect_vec()
        })
        .filter(|update| !is_valid(update, &rules))
        .map(|update| {
            let graph = AdjacencyListGraph::new(
                rules
                    .iter()
                    .filter(|(a, b)| update.iter().contains(a) && update.iter().contains(b))
                    .copied()
                    .collect_vec(),
            );

            topological_sort(graph)
        })
        .map(|update| {
            update
                .iter()
                .copied()
                .skip(update.len() / 2)
                .next()
                .expect("Can't be empty")
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT), 143);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve_part_2(INPUT), 123);
    }

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
}
