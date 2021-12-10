use crate::prob10::Line::{Corrupted, InComplete};
use itertools::Itertools;
use std::collections::VecDeque;
use std::str::FromStr;

pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Line>().unwrap())
        .filter(Line::is_corrupted)
        .map(|line| line.score())
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let scores: Vec<_> = input
        .lines()
        .map(|line| line.parse::<Line>().unwrap())
        .filter(Line::is_incomplete)
        .map(|line| line.score())
        .sorted()
        .collect();

    scores[scores.len() / 2] as usize
}

type ChunksParser = VecDeque<char>;

trait Token {
    fn matching_token(&self, token: &Self) -> bool;
}

impl Token for char {
    fn matching_token(&self, token: &Self) -> bool {
        match self {
            ')' => *token == '(',
            ']' => *token == '[',
            '}' => *token == '{',
            '>' => *token == '<',
            _ => unreachable!(),
        }
    }
}

enum Line {
    Corrupted(char),
    InComplete(ChunksParser),
}

impl Line {
    fn score(&self) -> usize {
        match self {
            Corrupted(illegal_char) => match illegal_char {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                x => panic!("Unreacheable {}", x),
            },
            InComplete(line) => line
                .iter()
                .rev()
                .map(|c| match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                })
                .fold(0, |acc, n| acc * 5 + n),
        }
    }

    fn is_corrupted(&self) -> bool {
        match self {
            Corrupted(_) => true,
            InComplete(_) => false,
        }
    }

    fn is_incomplete(&self) -> bool {
        match self {
            Corrupted(_) => false,
            InComplete(_) => true,
        }
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut parser = ChunksParser::new();
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => parser.push_back(c),
                ')' | ']' | '}' | '>' => {
                    if parser.is_empty() {
                        return Err(format!("Mismatch line {} at {}", line, c));
                    } else {
                        let open = parser.pop_back().unwrap();
                        if !c.matching_token(&open) {
                            return Ok(Corrupted(c));
                        }
                    }
                }
                _ => panic!("Unexpected input {}", c),
            }
        }
        return Ok(InComplete(parser));
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {
        let res = super::solve_part_1(
            "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
        );

        assert_eq!(res, 26397);
    }

    #[test]
    fn test_2() {
        let res = super::solve_part_2(
            "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]",
        );

        assert_eq!(res, 288957);
    }
}
