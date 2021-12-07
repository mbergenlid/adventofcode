
struct BingoBoard {
    filled_columns: [u8; 5],
    filled_rows: [u8; 5],
    numbers: Vec<Option<u32>>,
    has_completed: bool,
}

impl BingoBoard {
    fn add_number(&mut self, number: u32) -> Option<u32> {
        if self.has_completed {
            return None;
        }
        if let Some((i, n)) = self
            .numbers
            .iter_mut()
            .enumerate()
            .find(|(_i, &mut n)| n.map(|i| i == number).unwrap_or(false))
        {
            n.take();
            self.filled_rows[i / 5] += 1;
            self.filled_columns[i % 5] += 1;

            if self.filled_rows[i / 5] == 5 {
                self.has_completed = true;
                return Some(self.numbers.iter().map(|n| n.unwrap_or(0)).sum::<u32>() * number);
            } else if self.filled_columns[i % 5] == 5 {
                self.has_completed = true;
                return Some(self.numbers.iter().map(|n| n.unwrap_or(0)).sum::<u32>() * number);
            }
        }

        None
    }

    fn from_lines<'a, T: Iterator<Item = &'a str>>(lines: &mut T) -> Option<BingoBoard> {
        let numbers: Vec<Option<u32>> = lines
            .take(5)
            .flat_map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse().expect(&format!("Parse error: {}", n)))
                    .map(Some)
            })
            .collect();
        if numbers.len() == 5 * 5 {
            Some(BingoBoard {
                filled_columns: [0; 5],
                filled_rows: [0; 5],
                numbers,
                has_completed: false
            })
        } else {
            None
        }
    }
}


pub fn solve_part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let drawn_numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    lines.next();
    let mut boards = Vec::new();
    while let Some(board) = BingoBoard::from_lines(&mut lines) {
        lines.next();
        boards.push(board);
    }

    for number in drawn_numbers {
        for board in boards.iter_mut() {
            if let Some(score) = board.add_number(number) {
                return score as usize;
            }
        }
    }
    panic!();
}

pub fn solve_part_2(input: &str) -> usize {
    let mut lines: Box<dyn Iterator<Item = &str>> = Box::new(input.lines());
    let drawn_numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    lines = Box::new(lines.skip(1));
    let mut boards = Vec::new();
    while let Some(board) = BingoBoard::from_lines(&mut lines) {
        lines = Box::new(lines.skip(1));
        boards.push(board);
    }

    let mut number_of_winners = 0;
    let number_of_boards = boards.len();
    for number in drawn_numbers {
        for board in boards.iter_mut() {
            if let Some(score) = board.add_number(number) {
                number_of_winners += 1;
                if number_of_boards == number_of_winners  {
                    return score as usize;
                }
            }
        }
    }
    panic!();
}

#[cfg(test)]
mod test {
    #[test]
    fn test1() {
        let res = super::solve_part_1(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
        );

        assert_eq!(res, 4512)
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
        );

        assert_eq!(res, 1924)
    }
}
