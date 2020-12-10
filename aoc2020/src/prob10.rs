
pub fn solve_part_1(input: &str) -> u64 {
    let mut jolts: Vec<_> = input.lines().map(|s| s.parse::<u64>().unwrap()).collect();
    jolts.push(0);
    jolts.push(jolts.iter().max().unwrap() + 3);
    jolts.sort();

    let mut ones = 0;
    let mut threes = 0;
    for (&output, &input) in jolts.iter().zip(jolts.iter().skip(1)) {
        if input == output + 1 {
            ones += 1;
        } else if input == output + 3 {
            threes += 1;
        }
    }
    ones * threes
}

//                       4  4  2
//(0), 1, 2, 3, 4, 5, 6, 7, 8, 9, (10)
pub fn solve_part_2(input: &str) -> u64 {
    let mut jolts: Vec<_> = input.lines().map(|s| s.parse::<u64>().unwrap()).collect();
    jolts.push(0);
    jolts.push(jolts.iter().max().unwrap() + 3);
    jolts.sort();

    let mut combinations: Vec<(u64, u64)> = vec![(0, 0); jolts.len()];
    combinations[jolts.len()-1] = (1,0);

    for x in (1..combinations.len()-1).rev() {

        let (if_next_present, if_next_not_present) = combinations[x+1];
        let if_present = if_next_present + if_next_not_present;

        let mut if_not_present = 0;
        if jolts[x-1] + 3 >= jolts[x+1] {
            if_not_present += if_next_present;
            if jolts[x-1] + 3 >= jolts[x+2] {
                if_not_present += combinations[x+2].0
            }
        }

        combinations[x] = (if_present, if_not_present);
    }

    combinations[1].0 + combinations[1].1
}


#[cfg(test)]
mod test {
    use crate::prob10::{solve_part_2, solve_part_1};

    const TESTCASE: &'static str =
    "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(TESTCASE), 22*10);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            solve_part_2("16
10
15
5
1
11
7
19
6
12
4"), 8
        );
        assert_eq!(solve_part_2(TESTCASE), 19208);
        assert_eq!(solve_part_2(include_str!("../inputs/prob10")), 396857386627072);
    }
}
