use regex::Regex;
use std::collections::{HashSet, VecDeque};

pub fn solve_part_1() -> usize {
    BagRules::from(include_str!("../inputs/prob7")).bags_that_can_contain("shiny gold")
}

pub fn solve_part_2() -> u32 {
    BagRules::from(include_str!("../inputs/prob7")).bags_required_inside("shiny gold")
}

lazy_static! {
    static ref OUTER_REGEX: Regex = Regex::new(r"(.+) bags contain").unwrap();
    static ref INNER_REGEX: Regex = Regex::new(r"(\d+) (.+?) bag").unwrap();
}

struct BagRules<'a> {
    rules: Vec<BagRule<'a>>,
}

impl<'a> From<&'a str> for BagRules<'a> {
    fn from(s: &'a str) -> Self {
        BagRules {
            rules: s.lines().map(|line| BagRule::from(line)).collect(),
        }
    }
}

impl<'a> BagRules<'a> {
    fn bags_that_can_contain(&self, bag: &str) -> usize {
        let mut queue = VecDeque::new();
        queue.push_back(bag);
        let mut visited = HashSet::new();
        let mut count = 0;
        while let Some(bag) = queue.pop_front() {
            if visited.contains(bag) {
                continue;
            }
            visited.insert(bag);
            count += 1;

            for bag_that_can_contain_this in self
                .rules
                .iter()
                .filter(|&rule| !visited.contains(&rule.bag))
                .filter(|&rule| rule.must_contain.iter().any(|(_, content)| *content == bag))
            {
                queue.push_back(&bag_that_can_contain_this.bag)
            }
        }

        count - 1
    }

    fn bags_required_inside(&self, bag: &str) -> u32 {
        let mut result = 0;
        if let Some(rule) = self.rules.iter().find(|&rule| rule.bag == bag) {
            for (amount, bag) in rule.must_contain.iter() {
                result = result + amount * (1 + self.bags_required_inside(bag));
            }
        }
        result
    }
}

#[derive(PartialEq, Debug)]
struct BagRule<'a> {
    bag: &'a str,
    must_contain: Vec<(u32, &'a str)>,
}

impl<'a> From<&'a str> for BagRule<'a> {
    fn from(s: &'a str) -> Self {
        let captures = OUTER_REGEX.captures(s).unwrap();
        let content = INNER_REGEX
            .captures_iter(s)
            .map(|c| (c[1].parse::<u32>().unwrap(), c.get(2).unwrap().as_str()))
            .collect();
        BagRule {
            bag: captures.get(1).unwrap().as_str(),
            must_contain: content,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prob7::{BagRule, BagRules};

    #[test]
    fn test_deserialize() {
        assert_eq!(
            BagRule::from("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            BagRule {
                bag: "light red",
                must_contain: vec![(1, "bright white"), (2, "muted yellow")]
            }
        )
    }

    #[test]
    fn test_part_1() {
        let rules = BagRules {
            rules: "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."
                .lines()
                .map(|line| BagRule::from(line))
                .collect::<Vec<_>>(),
        };

        assert_eq!(rules.bags_that_can_contain("shiny gold"), 4);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            BagRules::from(
                "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags."
            )
            .bags_required_inside("shiny gold"),
            126
        );
    }
}
