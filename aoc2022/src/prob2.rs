use crate::prob2::RockPaperScissors::{Paper, Rock, Scissors};
use crate::prob2::RoundResult::{Draw, Loss, Win};
use std::str::FromStr;

#[derive(Copy, Clone)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    fn from(v: &str) -> Option<RockPaperScissors> {
        match v {
            "A" | "X" => Some(RockPaperScissors::Rock),
            "B" | "Y" => Some(RockPaperScissors::Paper),
            "C" | "Z" => Some(RockPaperScissors::Scissors),
            _ => None,
        }
    }

    fn score(&self, opponent: &RockPaperScissors) -> usize {
        let res = self.result(opponent);
        match self {
            RockPaperScissors::Rock => 1 + res.points(),
            RockPaperScissors::Paper => 2 + res.points(),
            RockPaperScissors::Scissors => 3 + res.points(),
        }
    }

    fn result(&self, opponent: &RockPaperScissors) -> RoundResult {
        match self {
            RockPaperScissors::Rock => match opponent {
                RockPaperScissors::Rock => Draw,
                RockPaperScissors::Paper => Loss,
                RockPaperScissors::Scissors => Win,
            },
            RockPaperScissors::Paper => match opponent {
                RockPaperScissors::Rock => Win,
                RockPaperScissors::Paper => Draw,
                RockPaperScissors::Scissors => Loss,
            },
            RockPaperScissors::Scissors => match opponent {
                RockPaperScissors::Rock => Loss,
                RockPaperScissors::Paper => Win,
                RockPaperScissors::Scissors => Draw,
            },
        }
    }

    fn pick(&self, result: &RoundResult) -> RockPaperScissors {
        *[Paper, Rock, Scissors]
            .iter()
            .filter(|r| r.result(self) == *result)
            .next()
            .unwrap()
    }
}

#[derive(Eq, PartialEq)]
enum RoundResult {
    Win,
    Loss,
    Draw,
}

impl RoundResult {
    fn points(&self) -> usize {
        match self {
            Win => 6,
            Loss => 0,
            Draw => 3,
        }
    }
}

impl FromStr for RoundResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(()),
        }
    }
}

pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let opponent = RockPaperScissors::from(split.next().unwrap()).unwrap();
            let me = RockPaperScissors::from(split.next().unwrap()).unwrap();

            me.score(&opponent)
        })
        .sum()
}

pub fn solve_part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" ");
            let opponent = RockPaperScissors::from(split.next().unwrap()).unwrap();
            let expected_result = split.next().unwrap().parse::<RoundResult>().unwrap();

            let me = opponent.pick(&expected_result);

            me.score(&opponent)
        })
        .sum()
}
