use crate::prob19::Rule::{CharRule, ListRule};
use std::collections::HashMap;
use std::str::{Chars, FromStr};

pub fn solve_part_1(input: &str) -> usize {
    solve(input, |map, line| matches(map, line))
}

fn solve<F>(input: &str, matcher: F) -> usize where F: Fn(&HashMap<u32, Rule>, &str) -> bool {
    let mut sections = input.split("\n\n");
    let rule_map = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let index = line.find(":").unwrap();
            (
                line[0..index].parse::<u32>().unwrap(),
                line[index + 1..].parse::<Rule>().unwrap(),
            )
        })
        .collect::<HashMap<_, _>>();

    sections
        .next()
        .unwrap()
        .lines()
        .filter(|&line| {
            matcher(&rule_map, line)
        })
        .count()
}

fn matches_part_2(rule_map: &HashMap<u32, Rule>, chars: &str) -> bool {
    let mut chars_iterator = chars.chars();
    let sub_match = sub_matches(42, rule_map, &mut chars_iterator);
    if sub_match {
        let mut match_42_count = 0;
        let mut temp_chars_iteratorg = chars_iterator.as_str().chars();
        while sub_matches(42, rule_map, &mut temp_chars_iterator) {
            chars_iterator = temp_chars_iterator.as_str().chars();
            match_42_count += 1;
        }
        //Try to match rule 31 at most as many times as match_42_count
        let mut match_31_count = 0;
        temp_chars_iterator = chars_iterator.as_str().chars();
        while sub_matches(31, rule_map, &mut temp_chars_iterator) {
            chars_iterator = temp_chars_iterator.as_str().chars();
            match_31_count += 1;
        }
        if match_31_count > 0 && match_31_count <= match_42_count {
            return chars_iterator.count() == 0;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn matches(rule_map: &HashMap<u32, Rule>, chars: &str) -> bool {
    let mut iter = chars.chars();
    sub_matches(0, rule_map, &mut iter) && iter.count() == 0
}

pub fn solve_part_2(input: &str) -> usize {
    solve(input, |map, line| matches_part_2(map, line))
}

fn sub_matches(rule: u32, rule_map: &HashMap<u32, Rule>, chars: &mut Chars) -> bool {
    let rule = rule_map.get(&rule).unwrap();
    match rule {
        CharRule(c) => {
            let next_char = chars.next();
            next_char == Some(*c)
        }
        ListRule(l) => {
            if let Some((_, cs)) = l.iter().map(|rules| {
                let mut cloned_chars = chars.as_str().chars();
                let mut m = true;
                for r in rules.iter() {
                    let sub_match = sub_matches(*r, rule_map, &mut cloned_chars);
                    if !sub_match {
                        m = false;
                        break;
                    }
                }
                (m, cloned_chars)
            }).filter(|(m, _)| *m).take(1).next() {
                *chars = cs;
                true
            } else {
                false
            }
        },
    }
}

#[derive(Debug)]
enum Rule {
    CharRule(char),
    ListRule(Vec<Vec<u32>>),
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(i) = s.find("\"") {
            Ok(CharRule(s.chars().nth(i + 1).unwrap()))
        } else {
            Ok(ListRule(
                s.split("|")
                    .map(|l| {
                        l.trim()
                            .split(" ")
                            .map(|n| {
                                n.parse::<u32>().unwrap()
                            })
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>(),
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prob19::{solve_part_1, solve_part_2};

    const TESTCASE_1: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb"#;

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(TESTCASE_1), 2);
        assert_eq!(solve_part_1(include_str!("../inputs/prob19")), 134);
    }

    const TESTCASE_2: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31 | 42 11 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42 8 | 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;


    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TESTCASE_2), 12);
        assert_eq!(solve_part_2(include_str!("../inputs/prob19")), 377);
    }
}
