use core::num;
use std::{collections::HashSet, str::FromStr};

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        let card = line.parse::<Card>().unwrap();
        let intersection = card
            .drawn_numbers
            .intersection(&card.winning_numbers)
            .count() as u32;

        if intersection > 0 {
            result += 2_usize.pow(intersection - 1);
        }
    }
    result
}

pub fn solve_part_2(input: &str) -> usize {
    let cards = input
        .lines()
        .map(|line| line.parse::<Card>().unwrap())
        .collect::<Vec<_>>();
    let mut card_scores = vec![0_usize; cards.len()];
    for i in (0..cards.len()).rev() {
        let card = &cards[i];
        let winners = card.winning_numbers();

        let mut score = 0;
        for copy in i + 1..=(i + winners) {
            score += card_scores[copy];
        }
        card_scores[i] = 1 + score;
    }

    card_scores.into_iter().sum::<usize>()
}

struct Card {
    winning_numbers: HashSet<usize>,
    drawn_numbers: HashSet<usize>,
}

impl Card {
    fn winning_numbers(&self) -> usize {
        self.drawn_numbers
            .intersection(&self.winning_numbers)
            .count()
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon_index = s.find(":").unwrap();

        let (winners, drawn) = s[colon_index + 1..].split("|").collect_tuple().unwrap();
        Ok(Card {
            winning_numbers: winners
                .split_whitespace()
                .map(|n| n.parse::<usize>().expect(&format!("Not a number '{}'", n)))
                .collect(),
            drawn_numbers: drawn
                .split_whitespace()
                .map(|n| n.parse::<usize>().expect(&format!("Not a number '{}'", n)))
                .collect(),
        })
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 13);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT), 30);
    }

    const TEST_INPUT: &'static str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
}
