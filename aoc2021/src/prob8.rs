use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split("|")
                .nth(1)
                .unwrap()
                .trim()
                .split_whitespace()
                .filter(|s| {
                    s.trim().len() == 2
                        || s.trim().len() == 4
                        || s.trim().len() == 3
                        || s.trim().len() == 7
                })
                .map(|s| s)
                .count()
        })
        .sum()
}

const DIGITS: [[bool; 7]; 10] = [
    [true, true, true, false, true, true, true],
    [false, false, true, false, false, true, false],
    [true, false, true, true, true, false, true],
    [true, false, true, true, false, true, true],
    [false, true, true, true, false, true, false],
    [true, true, false, true, false, true, true],
    [true, true, false, true, true, true, true],
    [true, false, true, false, false, true, false],
    [true; 7],
    [true, true, true, true, false, true, true],
];

pub fn solve_part_2(input: &str) -> usize {
    use itertools::Itertools;

    let all_digit_combinations: Vec<_> = "abcdefg"
        .chars()
        .permutations(7)
        .map(|permutation| {
            // println!("{:?}", permutation);
            let digits: Vec<_> = DIGITS
                .iter()
                .map(|d| {
                    let mut set: HashSet<char> = HashSet::new();
                    for (i, &p) in permutation.iter().enumerate() {
                        if d[i] {
                            set.insert(p);
                        }
                    }
                    set
                })
                .collect();
            // println!("{:?}", digits);
            digits
        })
        .collect();

    let mut result = 0;
    for line in input.lines() {
        let signals = line.split("|").nth(0).unwrap().trim();
        let signals: Vec<_> = signals
            .split_whitespace()
            .map(|s| s.trim().chars().collect::<HashSet<_>>())
            .collect();
        // println!("Signals: {:?}", signals);
        let combination = all_digit_combinations
            .iter()
            .find(|&digits| match_pattern(digits, &signals))
            .expect("No combinations found");
        // println!("{:?}", combination);

        let mut display_sum = 0;
        for d_value in line
            .split("|")
            .nth(1)
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|display_digit| {
                let display_digit_set = display_digit.trim().chars().collect::<HashSet<_>>();
                let display_value = combination
                    .iter()
                    .enumerate()
                    .find(|(_, combination)| **combination == display_digit_set)
                    .map(|(i, _)| i)
                    .expect("Blaaha");
                display_value
            })
        {
            display_sum = display_sum * 10 + d_value;
        }
        result += display_sum;
    }
    result
}

fn match_pattern(digit_patterns: &Vec<HashSet<char>>, signals: &Vec<HashSet<char>>) -> bool {
    signals.iter().all(|signal| digit_patterns.contains(signal))
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {
        let res = super::solve_part_1(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        );

        assert_eq!(res, 26);
    }

    #[test]
    fn test_2() {
        let res = super::solve_part_2(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        assert_eq!(res, 5353);
    }

    #[test]
    fn test_2_1() {
        let res = super::solve_part_2(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        );
        assert_eq!(res, 61229);
    }
}
