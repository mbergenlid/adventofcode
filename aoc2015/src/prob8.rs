use std::fs::File;
use std::io::Read;

pub fn solve_part_1() -> usize {
    let mut file = File::open("src/prob8_input").expect("Input not found");
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    return content
        .split("\n")
        .filter(|&s| !s.is_empty())
        .map(|s| s.len() - characters_in_memory(s))
        .sum();
}

pub fn solve_part_2() -> usize {
    let mut file = File::open("src/prob8_input").expect("Input not found");
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content
        .split("\n")
        .filter(|&s| !s.is_empty())
        .map(|s| encode(s).len() + 2 - s.len())
        .sum()
}

fn characters_in_memory(s: &str) -> usize {
    let mut iter = s.chars().enumerate();
    assert_eq!(iter.next(), Some((0, '"')));
    let mut count = 0;
    while let Some((i, c)) = iter.next() {
        match c {
            '\\' => {
                let (_, escape) = iter
                    .next()
                    .expect(&format!("Unclosed escape in {} at {}", i, s));
                if escape == '\\' || escape == '"' {
                    count += 1;
                } else if escape == 'x' {
                    count += 1;
                    iter.nth(1);
                } else {
                    panic!("Illegal escape {} in {}", escape, s);
                }
            }
            '"' => assert_eq!(i, s.len() - 1),
            _ => count += 1,
        }
    }
    count
}

fn encode(s: &str) -> String {
    let mut result = String::new();

    for c in s.chars() {
        match c {
            '"' => {
                result.push('\\');
                result.push('"');
            }
            '\\' => {
                result.push('\\');
                result.push('\\');
            }
            _ => result.push(c),
        }
    }

    result
}

#[cfg(test)]
mod test {
    use crate::prob8::{characters_in_memory, encode};

    #[test]
    fn test_part_1() {
        assert_eq!(characters_in_memory("\"abc\""), 3);
        assert_eq!(characters_in_memory("\"aaa\\\"aaa\""), 7);
        assert_eq!(characters_in_memory("\"\\x27\""), 1);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(encode(r#""""#).as_str(), r#"\"\""#);
        assert_eq!(encode(r#""abc""#).as_str(), r#"\"abc\""#);
        assert_eq!(encode(r#""\x27""#).as_str(), r#"\"\\x27\""#);

        assert_eq!(encode(r#""""#).len() + 2, 6)
    }
}
