pub fn solve_part_1(input: &str) -> usize {
    part1::solve(input)
}

pub fn solve_part_2(input: &str) -> usize {
    part2::solve(input)
}

mod part1 {
    use std::{
        cmp::Ordering,
        convert::{TryFrom, TryInto},
        str::FromStr,
    };

    use itertools::Itertools;

    pub fn solve(input: &str) -> usize {
        let mut hands_and_bids = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .collect_tuple::<(&str, &str)>()
                    .unwrap()
            })
            .map(|(hand, bid)| (hand.parse::<Hand>().unwrap(), bid.parse::<usize>().unwrap()))
            .collect::<Vec<_>>();

        hands_and_bids.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));
        hands_and_bids
            .into_iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum()
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum Card {
        Two = 0,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        T,
        J,
        Q,
        K,
        A,
    }

    impl TryFrom<char> for Card {
        type Error = String;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'A' => Ok(Card::A),
                'K' => Ok(Card::K),
                'Q' => Ok(Card::Q),
                'J' => Ok(Card::J),
                'T' => Ok(Card::T),
                '9' => Ok(Card::Nine),
                '8' => Ok(Card::Eight),
                '7' => Ok(Card::Seven),
                '6' => Ok(Card::Six),
                '5' => Ok(Card::Five),
                '4' => Ok(Card::Four),
                '3' => Ok(Card::Three),
                '2' => Ok(Card::Two),
                _ => Err(format!("Invalid card {}", value)),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    enum Hand {
        FiveOfAKind([Card; 5]),
        FourOfAKind([Card; 5]),
        FullHouse([Card; 5]),
        ThreeOfAKind([Card; 5]),
        TwoPair([Card; 5]),
        OnePair([Card; 5]),
        HighCard([Card; 5]),
    }

    impl Hand {
        fn rank(&self) -> u8 {
            match self {
                Hand::FiveOfAKind(_) => 7,
                Hand::FourOfAKind(_) => 6,
                Hand::FullHouse(_) => 5,
                Hand::ThreeOfAKind(_) => 4,
                Hand::TwoPair(_) => 3,
                Hand::OnePair(_) => 2,
                Hand::HighCard(_) => 1,
            }
        }

        fn cards(&self) -> &[Card] {
            match self {
                Hand::FiveOfAKind(c) => c,
                Hand::FourOfAKind(c) => c,
                Hand::FullHouse(c) => c,
                Hand::ThreeOfAKind(c) => c,
                Hand::TwoPair(c) => c,
                Hand::OnePair(c) => c,
                Hand::HighCard(c) => c,
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let self_rank = self.rank();
            let other_rank = other.rank();

            let ord = self_rank.cmp(&other_rank);
            match ord {
                Ordering::Equal => self.cards().cmp(other.cards()),
                _ => ord,
            }
        }
    }

    impl FromStr for Hand {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut buckets = [0_u32; 13];
            let mut cards = [Card::A; 5];
            for (i, c) in s.chars().enumerate() {
                let card: Card = c.try_into().unwrap();
                buckets[card as usize] += 1;
                cards[i] = card;
            }
            buckets.sort();
            let max = buckets[12];
            let second_max = buckets[11];

            if max == 5 {
                Ok(Hand::FiveOfAKind(cards))
            } else if max == 4 {
                Ok(Hand::FourOfAKind(cards))
            } else if max == 3 {
                if second_max == 2 {
                    Ok(Hand::FullHouse(cards))
                } else {
                    Ok(Hand::ThreeOfAKind(cards))
                }
            } else if max == 2 {
                if second_max == 2 {
                    Ok(Hand::TwoPair(cards))
                } else {
                    Ok(Hand::OnePair(cards))
                }
            } else {
                Ok(Hand::HighCard(cards))
            }
        }
    }
}

mod part2 {
    use std::{
        cmp::Ordering,
        convert::{TryFrom, TryInto},
        str::FromStr,
    };

    use itertools::Itertools;

    pub fn solve(input: &str) -> usize {
        let mut hands_and_bids = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .collect_tuple::<(&str, &str)>()
                    .unwrap()
            })
            .map(|(hand, bid)| (Hand::hand_part_2(hand), bid.parse::<usize>().unwrap()))
            .collect::<Vec<_>>();

        hands_and_bids.sort_by(|(h1, _), (h2, _)| h1.cmp(h2));
        hands_and_bids
            .into_iter()
            .enumerate()
            .map(|(i, (_, bid))| (i + 1) * bid)
            .sum()
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    enum Card {
        J = 0,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        T,
        Q,
        K,
        A,
    }

    impl TryFrom<char> for Card {
        type Error = String;

        fn try_from(value: char) -> Result<Self, Self::Error> {
            match value {
                'A' => Ok(Card::A),
                'K' => Ok(Card::K),
                'Q' => Ok(Card::Q),
                'J' => Ok(Card::J),
                'T' => Ok(Card::T),
                '9' => Ok(Card::Nine),
                '8' => Ok(Card::Eight),
                '7' => Ok(Card::Seven),
                '6' => Ok(Card::Six),
                '5' => Ok(Card::Five),
                '4' => Ok(Card::Four),
                '3' => Ok(Card::Three),
                '2' => Ok(Card::Two),
                _ => Err(format!("Invalid card {}", value)),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    enum Hand {
        FiveOfAKind([Card; 5]),
        FourOfAKind([Card; 5]),
        FullHouse([Card; 5]),
        ThreeOfAKind([Card; 5]),
        TwoPair([Card; 5]),
        OnePair([Card; 5]),
        HighCard([Card; 5]),
    }

    impl Hand {
        fn rank(&self) -> u8 {
            match self {
                Hand::FiveOfAKind(_) => 7,
                Hand::FourOfAKind(_) => 6,
                Hand::FullHouse(_) => 5,
                Hand::ThreeOfAKind(_) => 4,
                Hand::TwoPair(_) => 3,
                Hand::OnePair(_) => 2,
                Hand::HighCard(_) => 1,
            }
        }

        fn cards(&self) -> &[Card] {
            match self {
                Hand::FiveOfAKind(c) => c,
                Hand::FourOfAKind(c) => c,
                Hand::FullHouse(c) => c,
                Hand::ThreeOfAKind(c) => c,
                Hand::TwoPair(c) => c,
                Hand::OnePair(c) => c,
                Hand::HighCard(c) => c,
            }
        }

        fn hand_part_2(s: &str) -> Hand {
            if !s.contains("J") {
                s.parse().unwrap()
            } else {
                let mut original_cards = [Card::A; 5];
                for (i, c) in s.chars().enumerate() {
                    let card: Card = c.try_into().unwrap();
                    original_cards[i] = card;
                }
                let mut all_combos = Vec::new();
                for card in ["A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2"] {
                    all_combos.push(s.replace('J', card).parse::<Hand>().unwrap());
                }

                match all_combos.into_iter().max().unwrap() {
                    Hand::FiveOfAKind(_) => Hand::FiveOfAKind(original_cards),
                    Hand::FourOfAKind(_) => Hand::FourOfAKind(original_cards),
                    Hand::FullHouse(_) => Hand::FullHouse(original_cards),
                    Hand::ThreeOfAKind(_) => Hand::ThreeOfAKind(original_cards),
                    Hand::TwoPair(_) => Hand::TwoPair(original_cards),
                    Hand::OnePair(_) => Hand::OnePair(original_cards),
                    Hand::HighCard(_) => Hand::HighCard(original_cards),
                }
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let self_rank = self.rank();
            let other_rank = other.rank();

            let ord = self_rank.cmp(&other_rank);
            match ord {
                Ordering::Equal => self.cards().cmp(other.cards()),
                _ => ord,
            }
        }
    }

    impl FromStr for Hand {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut buckets = [0_u32; 13];
            let mut cards = [Card::A; 5];
            for (i, c) in s.chars().enumerate() {
                let card: Card = c.try_into().unwrap();
                buckets[card as usize] += 1;
                cards[i] = card;
            }
            buckets.sort();
            let max = buckets[12];
            let second_max = buckets[11];

            if max == 5 {
                Ok(Hand::FiveOfAKind(cards))
            } else if max == 4 {
                Ok(Hand::FourOfAKind(cards))
            } else if max == 3 {
                if second_max == 2 {
                    Ok(Hand::FullHouse(cards))
                } else {
                    Ok(Hand::ThreeOfAKind(cards))
                }
            } else if max == 2 {
                if second_max == 2 {
                    Ok(Hand::TwoPair(cards))
                } else {
                    Ok(Hand::OnePair(cards))
                }
            } else {
                Ok(Hand::HighCard(cards))
            }
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 6440);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT), 5905);
    }

    const TEST_INPUT: &'static str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
}
