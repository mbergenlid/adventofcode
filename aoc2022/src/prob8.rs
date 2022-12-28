use std::cmp::max;

type Grid = Vec<Vec<u8>>;

fn read_tree_grid(input: &str) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();

        for digit in line.chars() {
            if digit == '\n' {
                continue;
            }
            row.push(
                digit
                    .to_digit(10)
                    .expect(&format!("Unable to parse digit {}", digit)) as u8,
            );
        }

        result.push(row)
    }

    result
}

pub fn solve_part_1(input: &str) -> usize {
    let grid = read_tree_grid(input);

    let mut count = 0;
    for row in 1..(grid.len() - 1) {
        for col in 1..(grid[0].len() - 1) {
            if tree_viewable(&grid, (row, col), |(r, c)| {
                if r == 0 {
                    None
                } else {
                    Some((r - 1, c))
                }
            }) {
                count += 1;
            } else if tree_viewable(&grid, (row, col), |(r, c)| {
                if r == grid.len() {
                    None
                } else {
                    Some((r + 1, c))
                }
            }) {
                count += 1;
            } else if tree_viewable(&grid, (row, col), |(r, c)| {
                if c == 0 {
                    None
                } else {
                    Some((r, c - 1))
                }
            }) {
                count += 1;
            } else if tree_viewable(&grid, (row, col), |(r, c)| {
                if c == grid.len() {
                    None
                } else {
                    Some((r, c + 1))
                }
            }) {
                count += 1;
            }
        }
    }

    count + grid.len() * 2 + (grid[0].len() - 2) * 2
}

fn tree_viewable<F>(grid: &Grid, pos: (usize, usize), next: F) -> bool
where
    F: Fn((usize, usize)) -> Option<(usize, usize)>,
{
    let mut prev_pos = pos;
    let original_height = grid[pos.0][pos.1];
    while let Some(next_pos) = next(prev_pos) {
        let next_height = grid.get(next_pos.0).and_then(|row| row.get(next_pos.1));

        match next_height {
            None => return true,
            Some(&height) => {
                if height >= original_height {
                    return false;
                }
            }
        }

        prev_pos = next_pos;
    }
    true
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = read_tree_grid(input);

    let mut max_scenic_score = 0;
    for row in 1..(grid.len() - 1) {
        for col in 1..(grid[0].len() - 1) {
            let mut scenic_score = 1;
            scenic_score *= viewing_distance(&grid, (row, col), |(r, c)| {
                if r == 0 {
                    None
                } else {
                    Some((r - 1, c))
                }
            });
            scenic_score *= viewing_distance(&grid, (row, col), |(r, c)| {
                if r == grid.len() {
                    None
                } else {
                    Some((r + 1, c))
                }
            });
            scenic_score *= viewing_distance(&grid, (row, col), |(r, c)| {
                if c == 0 {
                    None
                } else {
                    Some((r, c - 1))
                }
            });
            scenic_score *= viewing_distance(&grid, (row, col), |(r, c)| {
                if c == grid.len() {
                    None
                } else {
                    Some((r, c + 1))
                }
            });

            max_scenic_score = max(max_scenic_score, scenic_score);
        }
    }

    max_scenic_score
}

fn viewing_distance<F>(grid: &Grid, pos: (usize, usize), next: F) -> usize
where
    F: Fn((usize, usize)) -> Option<(usize, usize)>,
{
    let mut count = 0;
    let mut prev_pos = pos;
    let original_height = grid[pos.0][pos.1];
    while let Some(next_pos) = next(prev_pos) {
        let next_height = grid.get(next_pos.0).and_then(|row| row.get(next_pos.1));

        match next_height {
            None => return count,
            Some(&height) => {
                count += 1;
                if height >= original_height {
                    return count;
                }
            }
        }

        prev_pos = next_pos;
    }
    count
}

#[cfg(test)]
mod test {
    use crate::prob8::{solve_part_1, solve_part_2};

    #[test]
    fn test_part1() {
        let input = r"30373
25512
65332
33549
35390";

        println!("{}", solve_part_1(input));
    }

    #[test]
    fn test_part2() {
        let input = r"30373
25512
65332
33549
35390";

        println!("{}", solve_part_2(input));
    }
}
