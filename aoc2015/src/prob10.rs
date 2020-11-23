
pub fn solve_part_1() -> usize {
    let mut look_and_say = LookAndSay { value: "1321131112".into() };
    look_and_say.nth(40).unwrap().len()
}

pub fn solve_part_2() -> usize {
    let mut look_and_say = LookAndSay { value: "1321131112".into() };
    look_and_say.nth(50).unwrap().len()
}

struct LookAndSay {
    value: String,
}

impl LookAndSay {
    fn advance(&mut self) {
        let mut next_value = String::new();
        let mut previous_char: char = self.value.chars().next().unwrap();
        let mut current_count = 1;
        for c in self.value.chars().skip(1) {
            if c != previous_char {
                next_value.push((('0' as u8) + current_count) as char);
                next_value.push(previous_char);
                current_count = 1;
                previous_char = c;
            } else {
                current_count += 1;
            }
        }
        next_value.push((('0' as u8) + current_count) as char);
        next_value.push(previous_char);
        self.value = next_value;
    }
}
impl Iterator for LookAndSay {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance();
        Some(self.value.clone())
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        for _ in 0..n {
            self.advance()
        }
        Some(self.value.clone())
    }

}

#[cfg(test)]
mod test {
    use crate::prob10::LookAndSay;

    #[test]
    fn test_part_1() {
        let mut look_and_say = LookAndSay { value: "1211".to_string() };
        assert_eq!(look_and_say.next(), Some("111221".to_string()));
    }
}
