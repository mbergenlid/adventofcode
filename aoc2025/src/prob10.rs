use std::{collections::VecDeque, fmt::Debug, i64, str::FromStr};

pub fn solve_part_1(input: &str) -> usize {
    let lights = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<IndicatorLights>().unwrap())
        .collect::<Vec<_>>();

    lights.iter().map(|l| l.solve()).sum::<u32>() as usize
}

struct IndicatorLights {
    target: u32,
    buttons: Vec<u32>,
}

impl IndicatorLights {
    fn solve(&self) -> u32 {
        let mut queue = VecDeque::new();
        queue.push_back((0_u32, 0_u32));

        while let Some((steps, state)) = queue.pop_front() {
            for button in &self.buttons {
                let new_state = state ^ button;
                //println!("{:04b} + {:04b} => {:04b}", state, button, new_state);
                if new_state == self.target {
                    return steps + 1;
                }
                queue.push_back((steps + 1, new_state));
            }
        }

        unreachable!()
    }
}

impl FromStr for IndicatorLights {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" ");
        let target_str = parts.next().unwrap();
        let mut target = 0;
        let mut shift = 0;
        for c in target_str.chars().skip(1).take(target_str.len() - 2) {
            if c == '#' {
                target |= 1 << shift;
            }
            shift += 1;
        }

        let mut buttons = Vec::new();
        for b in parts.take_while(|p| p.starts_with("(")) {
            let mut button = 0;
            for l in b[1..b.len() - 1].split(",") {
                button |= 1 << l.parse::<u8>().unwrap();
            }
            buttons.push(button);
        }

        Ok(Self { target, buttons })
    }
}

pub fn solve_part_2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| {
            let m: Matrix = line.parse().unwrap();
            m.solve()
                .unwrap_or_else(|| panic!("Could not find solution {line}"))
        })
        .sum::<i64>() as usize
}

#[derive(Clone)]
struct Matrix {
    data: Vec<Vec<i64>>,
}

impl Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            f.write_str("[ ")?;
            for c in row {
                f.write_fmt(format_args!("{:>4} ", c))?;
            }
            f.write_str("]")?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl FromStr for Matrix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ");

        let mut buttons = Vec::new();
        let mut joltage = Vec::new();
        for b in parts.skip(1) {
            if b.starts_with("(") {
                let mut button = Vec::new();
                for l in b[1..b.len() - 1].split(",") {
                    button.push(l.parse::<u8>().unwrap());
                }
                buttons.push(button);
            } else {
                joltage = b[1..b.len() - 1]
                    .split(",")
                    .map(|j| j.parse::<i64>().unwrap())
                    .collect::<Vec<_>>();
            }
        }

        let mut matrix = Vec::with_capacity(joltage.len());
        for row in 0..joltage.len() {
            let mut row_v = Vec::with_capacity(buttons.len() + 1);
            for col in 0..buttons.len() {
                let value = if buttons[col].contains(&(row as u8)) {
                    1
                } else {
                    0
                };
                row_v.push(value);
            }
            row_v.push(joltage[row]);
            matrix.push(row_v);
        }

        Ok(Self { data: matrix })
    }
}

impl Matrix {
    fn guass_elimination(&mut self) {
        let m = self.data.len();
        let n = self.data[0].len();

        let mut h = 0;
        let mut k = 0;

        while h < m && k < n {
            let i_max: usize = (h..m).max_by_key(|i| self.data[*i][k].abs()).unwrap_or(0);
            if self.data[i_max][k] == 0 {
                k = k + 1;
            } else {
                self.data.swap(h, i_max);

                for i in h + 1..m {
                    if self.data[i][k] == 0 {
                        continue;
                    }
                    if self.data[i][k] == -self.data[h][k] {
                        self.multiply_row(i, -1);
                    } else if self.data[i][k] != self.data[h][k] {
                        let tmp = self.data[h][k];
                        self.multiply_row(h, self.data[i][k]);
                        self.multiply_row(i, tmp);
                    }

                    let f = 1_i64;

                    assert!(
                        f * self.data[h][k] == self.data[i][k],
                        "{}, {}",
                        self.data[h][k],
                        self.data[i][k]
                    );
                    self.data[i][k] = 0;
                    for j in (k + 1)..n {
                        self.data[i][j] = self.data[i][j] - self.data[h][j];
                    }
                }
                h = h + 1;
                k = k + 1;
            }
        }
    }

    fn multiply_row(&mut self, row: usize, c: i64) {
        for cell in self.data[row].iter_mut() {
            *cell = *cell * c;
        }
    }

    fn solve(&self) -> Option<i64> {
        let mut reduced = self.clone();
        reduced.guass_elimination();
        let cols = self.data[0].len() - 1;
        let rows = self.data.len();
        let mut res = vec![0_i64; cols];
        self.back_substitute(&reduced.data, &mut res, cols - 1, rows - 1, cols - 1)
    }

    fn back_substitute(
        &self,
        reduced: &Vec<Vec<i64>>,
        res: &mut [i64],
        index: usize,
        row: usize,
        col: usize,
    ) -> Option<i64> {
        let Some(pivot_col) = reduced[row][0..=col].iter().position(|&n| n != 0) else {
            return self.back_substitute(reduced, res, index, row - 1, col);
        };

        let mut current_row_sum: i64 = 0;
        for i in index + 1..res.len() {
            current_row_sum += res[i] * reduced[row][i];
        }

        let left = *reduced[row].last().unwrap() - current_row_sum;
        if pivot_col == col {
            let current = left / reduced[row][col];
            if current * reduced[row][col] != left {
                return None;
            }
            if current < 0 {
                return None;
            }
            res[index] = current;
            if index == 0 {
                assert!(self.verify_solution(res), "{self:?}\n{res:?}");
                return Some(res.iter().sum::<i64>());
            } else {
                return self.back_substitute(reduced, res, index - 1, row - 1, col - 1);
            }
        } else {
            let mut min = i64::MAX;
            //let current_max = left / reduced[row][col] + 1;
            for x in 0..200 {
                res[index] = x;
                if let Some(presses) = self.back_substitute(reduced, res, index - 1, row, col - 1) {
                    min = min.min(presses);
                };
            }
            if min == i64::MAX {
                return None;
            }
            return Some(min);
        }
    }

    fn verify_solution(&self, res: &[i64]) -> bool {
        for row in &self.data {
            let mut sum = 0;
            for (c, r) in row.iter().zip(res) {
                sum += c * r;
            }
            if sum != *row.last().unwrap() {
                println!("Failed on line {row:?}, expected {sum}",);
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod test {

    use crate::prob10::{solve_part_1, solve_part_2, IndicatorLights, Matrix};

    #[test]
    fn test_parse() {
        assert_eq!(
            "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
                .parse::<IndicatorLights>()
                .unwrap()
                .target,
            0b0110
        );
        assert_eq!(
            "[###.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"
                .parse::<IndicatorLights>()
                .unwrap()
                .target,
            0b0111
        );
        assert_eq!(
            "[.##.] (3) (1,3) (2) (2,3) {3,5,4,7}"
                .parse::<IndicatorLights>()
                .unwrap()
                .buttons,
            vec![0b1000, 0b1010, 0b100, 0b1100],
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), 7);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT), 33);
    }

    const INPUT: &'static str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
}
