use itertools::Itertools;
use std::collections::HashMap;
use std::panic::panic_any;
use std::hash::Hash;

pub fn solve_part_1(input: &str) -> usize {
    let mut template = input.lines().next().unwrap().to_owned();

    let patterns: HashMap<(char, char), _> = input
        .lines()
        .skip(2)
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(key, value)| {
            (
                key.chars().next_tuple().unwrap(),
                value.chars().next().unwrap(),
            )
        })
        .collect();

    let mut cache = HashMap::new();
    let mut res = template
        .chars()
        .zip(template.chars().skip(1))
        .map(|pair| {
            let mut res = solve(&mut cache, &patterns, pair, 10);
            // println!("({:?}) -> {:?}", pair, res);
            res
        })
        .fold(HashMap::new(), |acc, next| merge(next, acc));

    for c in template.chars() {
        update(&mut res, &(c, 1));
    }
    println!("{:?}", res);

    if let itertools::MinMaxResult::MinMax(&min, &max) = res.values().minmax() {
        (max - min) as usize
    } else {
        panic!();
    }
}

type Cache = HashMap<(char, char, u32), HashMap<char, usize>>;

fn solve(
    cache: &mut HashMap<(char, char, u32), HashMap<char, usize>>,
    patterns: &HashMap<(char, char), char>,
    pair: (char, char),
    steps: u32,
) -> HashMap<char, usize> {
    if let Some(entry) = cache.get(&(pair.0, pair.1, steps)) {
        return entry.clone()
    }
    if steps == 1 {
        if let Some(&c) = patterns.get(&pair) {
            update_cache(cache, (pair.0, pair.1, steps), vec![(c, 1)].into_iter().collect())
        } else {
            HashMap::new()
        }
    } else {
        if let Some(&c) = patterns.get(&pair) {
            let res1 = solve(cache, patterns, (pair.0, c), steps - 1);
            let mut res2 = solve(cache, patterns, (c, pair.1), steps - 1);
            if let Some(prev_count) = res2.get_mut(&c) {
                *prev_count += 1;
            } else {
                res2.insert(c, 1);
            }
            update_cache(cache, (pair.0, pair.1, steps), merge(res1, res2))
        } else {
            HashMap::new()
        }
    }
}

fn update_cache(cache: &mut Cache, key: (char, char, u32), map: HashMap<char, usize>) -> HashMap<char, usize> {
    cache.insert(key, map.clone());
    map
}

fn update(map: &mut HashMap<char, usize>, pair: &(char, usize)) {
    if let Some(prev_count) = map.get_mut(&pair.0) {
        *prev_count += pair.1;
    } else {
        map.insert(pair.0, pair.1);
    }
}

fn merge(map1: HashMap<char, usize>, mut map2: HashMap<char, usize>) -> HashMap<char, usize> {
    for (c, count) in map1 {
        if let Some(prev_count) = map2.get_mut(&c) {
            *prev_count += count;
        } else {
            map2.insert(c, count);
        }
    }
    map2
}

pub fn solve_part_2(input: &str) -> usize {
    let mut template = input.lines().next().unwrap().to_owned();

    let patterns: HashMap<(char, char), _> = input
        .lines()
        .skip(2)
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(key, value)| {
            (
                key.chars().next_tuple().unwrap(),
                value.chars().next().unwrap(),
            )
        })
        .collect();

    let mut cache = HashMap::new();
    let mut res = template
        .chars()
        .zip(template.chars().skip(1))
        .map(|pair| {
            let mut res = solve(&mut cache, &patterns, pair, 40);
            // println!("({:?}) -> {:?}", pair, res);
            res
        })
        .fold(HashMap::new(), |acc, next| merge(next, acc));

    for c in template.chars() {
        update(&mut res, &(c, 1));
    }
    println!("{:?}", res);

    if let itertools::MinMaxResult::MinMax(&min, &max) = res.values().minmax() {
        (max - min) as usize
    } else {
        panic!();
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 1588);
    }

    #[test]
    fn test_2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 2188189693529);
    }

    const TESTCASE: &'static str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
}
