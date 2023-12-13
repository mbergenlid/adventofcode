use std::str::FromStr;

pub fn solve_part_1(input: &str) -> usize {
    solve(input, 0)
}

pub fn solve_part_2(input: &str) -> usize {
    solve(input, 1)
}

fn solve(input: &str, diffs_allowed: usize) -> usize {
    let grids = input
        .split("\n\n")
        .map(|l| l.parse::<Grid>().unwrap())
        .collect::<Vec<_>>();

    let mut vertical_result = 0;
    let mut horizontal_result = 0;
    for grid in grids {
        for reflection_col in 0..grid.width() - 1 {
            if grid.is_almost_reflection_col(reflection_col, diffs_allowed) {
                vertical_result += reflection_col + 1;
            }
        }
        for reflection_row in 0..grid.height() - 1 {
            if grid.is_almost_reflection_row(reflection_row, diffs_allowed) {
                horizontal_result += reflection_row + 1;
            }
        }
    }
    horizontal_result * 100 + vertical_result
}

struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn is_almost_reflection_col(&self, reflection_col: usize, diffs_allowed: usize) -> bool {
        let mut diffs_allowed = diffs_allowed;
        for row in &self.data {
            for col in 0..=reflection_col {
                let original = row.get(col).unwrap();
                let reflection = row.get(reflection_col + reflection_col - col + 1);
                if let Some(reflection) = reflection {
                    if reflection != original {
                        if diffs_allowed == 0 {
                            return false;
                        }
                        diffs_allowed -= 1;
                    }
                }
            }
        }
        return diffs_allowed == 0;
    }

    fn is_almost_reflection_row(&self, reflection_row: usize, diffs_allowed: usize) -> bool {
        let mut diffs_allowed = diffs_allowed;
        for col in 0..self.data[0].len() {
            for row in 0..=reflection_row {
                let original = self.get(row, col).unwrap();
                let reflection = self.get(reflection_row + reflection_row - row + 1, col);
                if let Some(reflection) = reflection {
                    if reflection != original {
                        if diffs_allowed == 0 {
                            return false;
                        }
                        diffs_allowed -= 1;
                    }
                }
            }
        }
        return diffs_allowed == 0;
    }

    fn get(&self, row: usize, col: usize) -> Option<char> {
        let row = self.data.get(row)?;
        row.get(col).copied()
    }

    fn width(&self) -> usize {
        self.data[0].len()
    }
    fn height(&self) -> usize {
        self.data.len()
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        Ok(Grid { data })
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 405);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT), 400);
    }

    const TEST_INPUT: &'static str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
}
