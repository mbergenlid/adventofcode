use std::{collections::HashMap, ops::RangeInclusive, str::FromStr};

use itertools::Itertools;
use recap::Recap;
use serde_derive::Deserialize;

pub fn solve_part_1(input: &str) -> usize {
    let (workflows, objects) = input.split("\n\n").collect_tuple().unwrap();

    let workflows = workflows
        .lines()
        .map(|line| {
            if let Some(index) = line.find("{") {
                (
                    line[0..index].to_string(),
                    line[index + 1..line.len() - 1].parse::<Workflow>().unwrap(),
                )
            } else {
                panic!()
            }
        })
        .collect::<HashMap<_, _>>();

    let objects = objects
        .lines()
        .map(|line| line.parse::<Object>().unwrap())
        .collect::<Vec<_>>();

    let mut total_rating = 0;
    for object in objects {
        let mut workflow = workflows.get("in").unwrap();
        loop {
            match workflow.evaluate(&object) {
                Action::GotoWorkflow(name) => workflow = workflows.get(name).unwrap(),
                Action::Accept => {
                    total_rating += object.total_rating();
                    break;
                }
                Action::Reject => break,
            }
        }
    }
    total_rating
}

pub fn solve_part_2(input: &str) -> usize {
    let (workflows, _) = input.split("\n\n").collect_tuple().unwrap();

    let workflows = workflows
        .lines()
        .map(|line| {
            if let Some(index) = line.find("{") {
                (
                    line[0..index].to_string(),
                    line[index + 1..line.len() - 1].parse::<Workflow>().unwrap(),
                )
            } else {
                panic!()
            }
        })
        .collect::<HashMap<_, _>>();

    let workflows = Workflows(workflows);
    workflows.evaluate_part2(ObjectPart2::default())
}

struct Workflows(HashMap<String, Workflow>);

impl Workflows {
    fn evaluate_part2(&self, object: ObjectPart2) -> usize {
        let workflow = self.0.get("in").unwrap();

        self._evalutate(object, workflow)
    }

    fn _evalutate(&self, object: ObjectPart2, workflow: &Workflow) -> usize {
        let mut result = 0;
        for (o, action) in workflow.evaluate_part2(object) {
            match action {
                Action::GotoWorkflow(name) => {
                    result += self._evalutate(o, self.0.get(name).unwrap())
                }
                Action::Accept => result += o.count_combinations(),
                Action::Reject => {}
            }
        }
        result
    }
}

#[derive(Deserialize, Recap, Debug)]
#[recap(regex = r"\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}")]
struct Object {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Object {
    fn get(&self, var: char) -> Option<usize> {
        match var {
            'x' => Some(self.x),
            'm' => Some(self.m),
            'a' => Some(self.a),
            's' => Some(self.s),
            _ => None,
        }
    }

    fn total_rating(&self) -> usize {
        (self.x + self.m + self.a + self.s) as usize
    }
}

#[derive(Debug, Clone)]
struct ObjectPart2 {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
}

impl Default for ObjectPart2 {
    fn default() -> Self {
        Self {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        }
    }
}

impl ObjectPart2 {
    fn split(self, rule: &Rule) -> (Option<ObjectPart2>, Option<ObjectPart2>) {
        match rule {
            Rule::GreaterThan(var, value) => match var {
                'x' => {
                    if *value < *self.x.start() {
                        (Some(self), None)
                    } else if self.x.contains(value) {
                        (
                            Some(ObjectPart2 {
                                x: *value + 1..=*self.x.end(),
                                m: self.m.clone(),
                                a: self.a.clone(),
                                s: self.s.clone(),
                            }),
                            Some(ObjectPart2 {
                                x: *self.x.start()..=*value,
                                m: self.m,
                                a: self.a,
                                s: self.s,
                            }),
                        )
                    } else {
                        (None, Some(self))
                    }
                }
                'm' => {
                    if *value < *self.m.start() {
                        (Some(self), None)
                    } else if self.m.contains(value) {
                        (
                            Some(ObjectPart2 {
                                m: *value + 1..=*self.m.end(),
                                x: self.x.clone(),
                                a: self.a.clone(),
                                s: self.s.clone(),
                            }),
                            Some(ObjectPart2 {
                                m: *self.m.start()..=*value,
                                x: self.x,
                                a: self.a,
                                s: self.s,
                            }),
                        )
                    } else {
                        (None, Some(self))
                    }
                }
                'a' => {
                    if *value < *self.a.start() {
                        (Some(self), None)
                    } else if self.a.contains(value) {
                        (
                            Some(ObjectPart2 {
                                a: *value + 1..=*self.a.end(),
                                m: self.m.clone(),
                                x: self.x.clone(),
                                s: self.s.clone(),
                            }),
                            Some(ObjectPart2 {
                                a: *self.a.start()..=*value,
                                m: self.m,
                                x: self.x,
                                s: self.s,
                            }),
                        )
                    } else {
                        (None, Some(self))
                    }
                }
                's' => {
                    if *value < *self.s.start() {
                        (Some(self), None)
                    } else if self.s.contains(value) {
                        (
                            Some(ObjectPart2 {
                                s: *value + 1..=*self.s.end(),
                                m: self.m.clone(),
                                a: self.a.clone(),
                                x: self.x.clone(),
                            }),
                            Some(ObjectPart2 {
                                s: *self.s.start()..=*value,
                                m: self.m,
                                a: self.a,
                                x: self.x,
                            }),
                        )
                    } else {
                        (None, Some(self))
                    }
                }
                _ => unreachable!(),
            },
            Rule::LessThan(var, value) => match var {
                'x' => {
                    if *value < *self.x.start() {
                        (None, Some(self))
                    } else if self.x.contains(value) {
                        (
                            Some(ObjectPart2 {
                                x: *self.x.start()..=*value - 1,
                                m: self.m.clone(),
                                a: self.a.clone(),
                                s: self.s.clone(),
                            }),
                            Some(ObjectPart2 {
                                x: *value..=*self.x.end(),
                                m: self.m,
                                a: self.a,
                                s: self.s,
                            }),
                        )
                    } else {
                        (Some(self), None)
                    }
                }
                'm' => {
                    if *value < *self.m.start() {
                        (None, Some(self))
                    } else if self.m.contains(value) {
                        (
                            Some(ObjectPart2 {
                                m: *self.m.start()..=*value - 1,
                                x: self.x.clone(),
                                a: self.a.clone(),
                                s: self.s.clone(),
                            }),
                            Some(ObjectPart2 {
                                m: *value..=*self.m.end(),
                                x: self.x,
                                a: self.a,
                                s: self.s,
                            }),
                        )
                    } else {
                        (Some(self), None)
                    }
                }
                's' => {
                    if *value < *self.s.start() {
                        (None, Some(self))
                    } else if self.s.contains(value) {
                        (
                            Some(ObjectPart2 {
                                s: *self.s.start()..=*value - 1,
                                m: self.m.clone(),
                                a: self.a.clone(),
                                x: self.x.clone(),
                            }),
                            Some(ObjectPart2 {
                                s: *value..=*self.s.end(),
                                m: self.m,
                                a: self.a,
                                x: self.x,
                            }),
                        )
                    } else {
                        (Some(self), None)
                    }
                }
                'a' => {
                    if *value < *self.a.start() {
                        (None, Some(self))
                    } else if self.a.contains(value) {
                        (
                            Some(ObjectPart2 {
                                a: *self.a.start()..=*value - 1,
                                m: self.m.clone(),
                                x: self.x.clone(),
                                s: self.s.clone(),
                            }),
                            Some(ObjectPart2 {
                                a: *value..=*self.a.end(),
                                m: self.m,
                                x: self.x,
                                s: self.s,
                            }),
                        )
                    } else {
                        (Some(self), None)
                    }
                }
                _ => unreachable!(),
            },
            Rule::True => (Some(self), None),
        }
    }

    fn count_combinations(&self) -> usize {
        ObjectPart2::size_of_range(&self.x)
            * ObjectPart2::size_of_range(&self.m)
            * ObjectPart2::size_of_range(&self.a)
            * ObjectPart2::size_of_range(&self.s)
    }

    fn size_of_range(r: &RangeInclusive<usize>) -> usize {
        r.end() - r.start() + 1
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<(Rule, Action)>,
}

impl Workflow {
    fn evaluate(&self, object: &Object) -> &Action {
        for (rule, action) in &self.rules {
            if rule.evaluate(object) {
                return action;
            }
        }
        unreachable!()
    }

    fn evaluate_part2(&self, mut object: ObjectPart2) -> Vec<(ObjectPart2, &Action)> {
        let mut result = Vec::new();
        for (rule, action) in &self.rules {
            let (left, right) = object.split(rule);
            if let Some(passed) = left {
                result.push((passed, action));
            }
            if let Some(not_passed) = right {
                object = not_passed;
            } else {
                break;
            }
        }
        result
    }
}

#[derive(Debug)]
enum Rule {
    GreaterThan(char, usize),
    LessThan(char, usize),
    True,
}
impl Rule {
    #[inline]
    fn evaluate(&self, object: &Object) -> bool {
        match self {
            Rule::GreaterThan(var, value) => object.get(*var).unwrap() > *value,
            Rule::LessThan(var, value) => object.get(*var).expect(&format!("{}", var)) < *value,
            Rule::True => true,
        }
    }
}

#[derive(Debug)]
enum Action {
    GotoWorkflow(String),
    Accept,
    Reject,
}

impl FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Workflow {
            rules: s
                .split(",")
                .map(|r| {
                    if let Some(index) = r.find(">") {
                        //Greater than
                        assert!(index == 1);
                        if let Some(colon) = r.find(":") {
                            (
                                Rule::GreaterThan(
                                    r.chars().nth(0).unwrap(),
                                    r[index + 1..colon].parse::<usize>().unwrap(),
                                ),
                                r[colon + 1..].parse::<Action>().unwrap(),
                            )
                        } else {
                            panic!();
                        }
                    } else if let Some(index) = r.find("<") {
                        //Less than
                        assert!(index == 1);
                        if let Some(colon) = r.find(":") {
                            (
                                Rule::LessThan(
                                    r.chars().nth(0).unwrap(),
                                    r[index + 1..colon].parse::<usize>().unwrap(),
                                ),
                                r[colon + 1..].parse::<Action>().unwrap(),
                            )
                        } else {
                            panic!();
                        }
                    } else {
                        (Rule::True, r.parse::<Action>().unwrap())
                    }
                })
                .collect(),
        })
    }
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Action::Accept),
            "R" => Ok(Action::Reject),
            _ => Ok(Action::GotoWorkflow(s.to_string())),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 19114);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT), 167409079868000);
    }

    const TEST_INPUT: &'static str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
}
