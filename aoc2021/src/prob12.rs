use itertools::Itertools;
use std::collections::HashMap;
use std::path::Path;

pub fn solve_part_1(input: &str) -> usize {
    let graph = Graph(
        input
            .lines()
            .map(|line| line.split_once("-").unwrap())
            .flat_map(|(a1, a2)| vec![(a1, a2), (a2, a1)])
            .into_group_map(),
    );

    graph.count_paths(&mut vec!["start"])
}

pub fn solve_part_2(input: &str) -> usize {
    let graph = Graph(
        input
            .lines()
            .map(|line| line.split_once("-").unwrap())
            .flat_map(|(a1, a2)| vec![(a1, a2), (a2, a1)])
            .into_group_map(),
    );

    graph.count_paths_2(&mut Paths {
        path: vec!["start"],
        has_visited_two_small_caves: false,
    })
}

struct Graph<'a>(HashMap<&'a str, Vec<&'a str>>);

struct Paths<'a> {
    path: Vec<&'a str>,
    has_visited_two_small_caves: bool,
}

impl<'a> Graph<'a> {
    fn count_paths(&self, start: &mut Vec<&'a str>) -> usize {
        let &current = start.last().unwrap();
        let mut path_count = 0;
        for &n in self.0.get(current).unwrap() {
            match n {
                "start" => {}
                "end" => path_count += 1,
                s if s.chars().all(|c| c.is_lowercase()) && start.contains(&s) => {}
                _ => {
                    start.push(n);
                    path_count += self.count_paths(start);
                    start.pop();
                }
            }
        }
        path_count
    }

    fn count_paths_2(&self, start: &mut Paths<'a>) -> usize {
        let &current = start.path.last().unwrap();
        let mut path_count = 0;
        for &n in self.0.get(current).unwrap() {
            match n {
                "start" => {}
                "end" => path_count += 1,
                s if s.chars().all(|c| c.is_lowercase()) && start.path.contains(&s) => {
                    if !start.has_visited_two_small_caves {
                        start.has_visited_two_small_caves = true;
                        start.path.push(n);
                        path_count += self.count_paths_2(start);
                        start.path.pop();
                        start.has_visited_two_small_caves = false;
                    }
                }
                _ => {
                    start.path.push(n);
                    path_count += self.count_paths_2(start);
                    start.path.pop();
                }
            }
        }
        path_count
    }
}


#[cfg(test)]
mod test {

    #[test]
    fn test_1() {
        let res = super::solve_part_1(TESTCASE_1);
        assert_eq!(res, 10);

        let res = super::solve_part_1(TESTCASE_2);
        assert_eq!(res, 19);

        let res = super::solve_part_1(TESTCASE_3);
        assert_eq!(res, 226);
    }

    #[test]
    fn test_2() {
        let res = super::solve_part_2(TESTCASE_1);
        assert_eq!(res, 36);

        let res = super::solve_part_2(TESTCASE_2);
        assert_eq!(res, 103);

        let res = super::solve_part_2(TESTCASE_3);
        assert_eq!(res, 3509);
    }

    const TESTCASE_1: &'static str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    const TESTCASE_2: &'static str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    const TESTCASE_3: &'static str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
}
