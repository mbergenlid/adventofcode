lalrpop_mod!(pub prob18grammar1);
lalrpop_mod!(pub prob18grammar2);
use prob18grammar1::ExprParser as Part1Parser;
use prob18grammar2::ExprParser as Part2Parser;

pub fn solve_part_1(input: &str) -> i64 {
    let parser = Part1Parser::new();
    input.lines().map(|line| parser.parse(line).unwrap()).sum()
}

pub fn solve_part_2(input: &str) -> i64 {
    let parser = Part2Parser::new();
    input.lines().map(|line| parser.parse(line).unwrap()).sum()
}

#[cfg(test)]
mod test {

    use super::prob18grammar1::ExprParser;
    use crate::prob18::{solve_part_1, solve_part_2};


    #[test]
    fn test_part_1() {
        assert_eq!(ExprParser::new().parse("2 * 3 + (4 * 5)").unwrap(), 26);
        assert_eq!(
            ExprParser::new()
                .parse("5 + (8 * 3 + 9 + 3 * 4 * 3)")
                .unwrap(),
            437
        );
        assert_eq!(
            ExprParser::new()
                .parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
                .unwrap(),
            12240
        );
        assert_eq!(
            ExprParser::new()
                .parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
                .unwrap(),
            13632
        );

        assert_eq!(solve_part_1(include_str!("../inputs/prob18")), 4491283311856);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(super::prob18grammar2::ExprParser::new().parse("1 + (2 * 3) + (4 * (5 + 6))").unwrap(), 51);
        assert_eq!(solve_part_2(include_str!("../inputs/prob18")), 68852578641904);
    }
}
