use crate::prob21::Expression::{Number, Polynomial};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete;
use nom::character::complete::alpha1;
use nom::combinator::map;
use nom::error::VerboseError;
use nom::sequence::separated_pair;
use num::integer::gcd;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Deref, Div, Mul, Sub};

enum Expression {
    Number(i64),

    Polynomial(Poly),
}

#[derive(Copy, Clone)]
struct Fraction {
    nominator: i64,
    denominator: i64,
}

impl Default for Fraction {
    fn default() -> Self {
        Fraction {
            nominator: 0,
            denominator: 1,
        }
    }
}

impl Add<i64> for Fraction {
    type Output = Self;

    fn add(self, rhs: i64) -> Self::Output {
        let nominator = self.nominator + rhs * self.denominator;

        let gcd = gcd(nominator, self.denominator);
        Fraction {
            nominator: nominator / gcd,
            denominator: self.denominator / gcd,
        }
    }
}

impl Sub<i64> for Fraction {
    type Output = Self;

    fn sub(self, rhs: i64) -> Self::Output {
        self + (-rhs)
    }
}

impl Sub<Fraction> for i64 {
    type Output = Fraction;

    fn sub(self, rhs: Fraction) -> Self::Output {
        let nominator = self * rhs.denominator - rhs.nominator;

        let gcd = gcd(nominator, rhs.denominator);
        Fraction {
            nominator: nominator / gcd,
            denominator: rhs.denominator / gcd,
        }
    }
}

impl Mul<&i64> for Fraction {
    type Output = Self;

    fn mul(self, rhs: &i64) -> Self::Output {
        let nominator = self.nominator * rhs;
        let gcd = gcd(nominator, self.denominator);

        Fraction {
            nominator: nominator / gcd,
            denominator: self.denominator / gcd,
        }
    }
}

impl Div<&i64> for Fraction {
    type Output = Self;

    fn div(self, rhs: &i64) -> Self::Output {
        let denominator = self.denominator * rhs;
        let gcd = gcd(denominator, self.nominator);

        Fraction {
            nominator: self.nominator / gcd,
            denominator: denominator / gcd,
        }
    }
}

struct Poly {
    coefficient: Fraction,
    term: Fraction,
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number(n) => write!(f, "{}", n),
            Polynomial(p) => write!(
                f,
                "{}x/{} + {}/{}",
                p.coefficient.nominator,
                p.coefficient.denominator,
                p.term.nominator,
                p.term.denominator
            ),
        }
    }
}

impl Add for Expression {
    type Output = Expression;

    fn add(self, rhs: Self) -> Self::Output {
        let res = match (&self, &rhs) {
            (Number(n1), Number(n2)) => Number(n1 + n2),
            (Number(n1), Polynomial(p)) => Polynomial(Poly {
                term: p.term + *n1,
                ..*p
            }),
            (Polynomial(p), Number(n1)) => Polynomial(Poly {
                term: p.term + *n1,
                ..*p
            }),
            (Polynomial(_p1), Polynomial(_p2)) => todo!(),
        };
        res
    }
}

impl Sub for Expression {
    type Output = Expression;

    fn sub(self, rhs: Self) -> Self::Output {
        let res = match (&self, &rhs) {
            (Number(n1), Number(n2)) => Number(n1 - n2),
            (Number(n1), Polynomial(p)) => Polynomial(Poly {
                term: *n1 - p.term,
                coefficient: p.coefficient * &-1,
            }),
            (Polynomial(p), Number(n1)) => Polynomial(Poly {
                term: p.term - *n1,
                ..*p
            }),
            (Polynomial(_p1), Polynomial(_p2)) => todo!(),
        };
        res
    }
}

impl Mul for Expression {
    type Output = Expression;

    //150 = ((4 + (2 * (x - 3))) / 4)
    //150 = ((4 + ((x * 2) - 6)) / 4)
    fn mul(self, rhs: Self) -> Self::Output {
        let res = match (&self, &rhs) {
            (Number(n1), Number(n2)) => Number(n1 * n2),
            (Number(n1), Polynomial(p)) => Polynomial(Poly {
                coefficient: p.coefficient * n1,
                term: p.term * n1,
                ..*p
            }),
            (Polynomial(p), Number(n1)) => Polynomial(Poly {
                coefficient: p.coefficient * n1,
                term: p.term * n1,
                ..*p
            }),
            (Polynomial(_p1), Polynomial(_p2)) => todo!(),
        };
        res
    }
}

impl Div for Expression {
    type Output = Expression;

    fn div(self, rhs: Self) -> Self::Output {
        let res = match (&self, &rhs) {
            (Number(n1), Number(n2)) => Number(n1 / n2),
            (Number(n1), Polynomial(p)) => Polynomial(Poly {
                coefficient: p.coefficient / n1,
                term: p.term / n1,
                ..*p
            }),
            (Polynomial(p), Number(n1)) => Polynomial(Poly {
                coefficient: p.coefficient / n1,
                term: p.term / n1,
                ..*p
            }),
            (Polynomial(_p1), Polynomial(_p2)) => todo!(),
        };
        res
    }
}

#[derive(Debug)]
enum Monkey {
    Number(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

#[derive(Debug)]
struct Monkeys {
    data: HashMap<String, RefCell<Monkey>>,
}

impl Monkeys {
    fn parse(s: &str) -> Self {
        Monkeys {
            data: s
                .lines()
                .map(Monkeys::parse_monkey)
                .map(|(s, m)| (s, RefCell::new(m)))
                .collect(),
        }
    }

    fn parse_monkey(s: &str) -> (String, Monkey) {
        let (s, name) = take_until::<_, _, ()>(": ")(s).unwrap();
        let (s, _) = tag::<_, _, ()>(": ")(s).unwrap();
        // println!("{} -- {}", s, name);
        let (_, res) = alt::<_, _, VerboseError<&str>, _>((
            map(complete::i64, |n| Monkey::Number(n)),
            map(
                separated_pair(alpha1::<&str, _>, tag(" + "), alpha1),
                |(s1, s2)| Monkey::Add(s1.to_string(), s2.to_string()),
            ),
            map(
                separated_pair(alpha1::<&str, _>, tag(" - "), alpha1),
                |(s1, s2)| Monkey::Sub(s1.to_string(), s2.to_string()),
            ),
            map(
                separated_pair(alpha1::<&str, _>, tag(" * "), alpha1),
                |(s1, s2)| Monkey::Mul(s1.to_string(), s2.to_string()),
            ),
            map(
                separated_pair(alpha1::<&str, _>, tag(" / "), alpha1),
                |(s1, s2)| Monkey::Div(s1.to_string(), s2.to_string()),
            ),
        ))(s)
        .unwrap();
        (name.to_string(), res)
    }

    fn solve_for(&self, monkey: &str) -> i64 {
        if let Some(m) = self.data.get(monkey) {
            // println!("Resolving {}: {:?}", monkey, m);
            m.replace_with(|m| {
                Monkey::Number(match m {
                    Monkey::Number(n) => *n,
                    Monkey::Add(m1, m2) => {
                        self.solve_for(m1.as_str()) + self.solve_for(m2.as_str())
                    }
                    Monkey::Sub(m1, m2) => {
                        self.solve_for(m1.as_str()) - self.solve_for(m2.as_str())
                    }
                    Monkey::Mul(m1, m2) => {
                        self.solve_for(m1.as_str()) * self.solve_for(m2.as_str())
                    }
                    Monkey::Div(m1, m2) => {
                        self.solve_for(m1.as_str()) / self.solve_for(m2.as_str())
                    }
                })
            });

            // println!("Resolved {} to {:?}", monkey, m);
            if let Monkey::Number(n) = m.borrow().deref() {
                return *n;
            } else {
                panic!("{:?} is not a number", m)
            }
        }
        unreachable!()
    }

    fn expression(&self, monkey: &str) -> Expression {
        if monkey == "humn" {
            Polynomial(Poly {
                coefficient: Fraction {
                    nominator: 1,
                    denominator: 1,
                },
                term: Fraction::default(),
            })
        } else if let Some(m) = self.data.get(monkey) {
            let res = match m.borrow().deref() {
                Monkey::Number(n) => Number(*n),
                Monkey::Add(m1, m2) => self.expression(m1.as_str()) + self.expression(m2.as_str()),
                Monkey::Sub(m1, m2) => self.expression(m1.as_str()) - self.expression(m2.as_str()),
                Monkey::Mul(m1, m2) => self.expression(m1.as_str()) * self.expression(m2.as_str()),
                Monkey::Div(m1, m2) => self.expression(m1.as_str()) / self.expression(m2.as_str()),
            };
            res
        } else {
            unreachable!()
        }
    }
}

pub fn solve_part_1(input: &str) -> i64 {
    let monkeys = Monkeys::parse(input);
    // println!("{:?}", monkeys);
    monkeys.solve_for("root")
}

pub fn solve_part_2(input: &str) -> String {
    let monkeys = Monkeys::parse(input);
    if let Some(m) = monkeys.data.get("root") {
        if let Monkey::Add(m1, m2) = m.borrow().deref() {
            let rhs = monkeys.expression(m1.as_str());
            let lhs = monkeys.expression(m2.as_str());

            return format!("{} = {}", lhs, rhs);
        }
    }

    todo!()
}

#[cfg(test)]
mod test {
    use crate::prob21::solve_part_1;

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(INPUT), 152);
    }

    const INPUT: &'static str = r"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
}
