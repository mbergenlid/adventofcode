use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = grid.len();
    let mut weight = 0;
    for col in 0..grid[0].len() {
        let mut top_most: Option<usize> = None;
        for row in 0..grid.len() {
            match &grid[row][col] {
                'O' => {
                    if let Some(top) = top_most {
                        weight += height - top;
                        top_most = Some(top + 1);
                    } else {
                        weight += height - row;
                        top_most = Some(row + 1);
                    }
                }
                '.' => {
                    if top_most.is_none() {
                        top_most = Some(row);
                    }
                }
                '#' => top_most = None,
                _ => unreachable!(),
            }
        }
    }
    weight
}

pub fn solve_part_2(input: &str) -> usize {
    let mut grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut all = Vec::new();
    all.push(grid.clone());
    for i in 0..1000000000 {
        //print_grid(&grid);
        cycle(&mut grid);

        if let Some((pos, _)) = all.iter().find_position(|&g| *g == grid) {
            let extra_cycles = (1_000_000_000 - pos) % (i + 1 - pos);
            for _ in 0..extra_cycles {
                cycle(&mut grid)
            }

            let mut total_load = 0;
            for row in 0..grid.len() {
                for col in 0..grid[0].len() {
                    if grid[row][col] == 'O' {
                        total_load += grid.len() - row;
                    }
                }
            }
            return total_load;
        }
        all.push(grid.clone());
    }

    todo!()
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
}

fn cycle(grid: &mut Vec<Vec<char>>) {
    for col in 0..grid[0].len() {
        let mut top_most: Option<usize> = None;
        for row in 0..grid.len() {
            match &grid[row][col] {
                'O' => {
                    if let Some(top) = top_most {
                        if top != row {
                            grid[top][col] = 'O';
                            grid[row][col] = '.';
                        }
                        top_most = Some(top + 1);
                    } else {
                        top_most = Some(row + 1);
                    }
                }
                '.' => {
                    if top_most.is_none() {
                        top_most = Some(row);
                    }
                }
                '#' => top_most = None,
                _ => unreachable!(),
            }
        }
    }

    let width = grid[0].len();
    for row in 0..grid.len() {
        let mut left_most: Option<usize> = None;
        for col in 0..grid[0].len() {
            match &grid[row][col] {
                'O' => {
                    if let Some(left) = left_most {
                        if left < width && left != col {
                            grid[row][left] = 'O';
                            grid[row][col] = '.';
                        }
                        left_most = Some(left + 1);
                    } else {
                        left_most = Some(col + 1);
                    }
                }
                '.' => {
                    if left_most.is_none() {
                        left_most = Some(col);
                    }
                }
                '#' => left_most = None,
                _ => unreachable!(),
            }
        }
    }

    for col in 0..grid[0].len() {
        let mut bottom_most: Option<usize> = None;
        for row in (0..grid.len()).rev() {
            match &grid[row][col] {
                'O' => {
                    if let Some(bottom) = bottom_most {
                        if bottom != row {
                            grid[bottom][col] = 'O';
                            grid[row][col] = '.';
                        }
                        bottom_most = Some(bottom.wrapping_sub(1));
                    } else {
                        bottom_most = Some(row.wrapping_sub(1));
                    }
                }
                '.' => {
                    if bottom_most.is_none() {
                        bottom_most = Some(row);
                    }
                }
                '#' => bottom_most = None,
                _ => unreachable!(),
            }
        }
    }

    for row in 0..grid.len() {
        let mut right_most: Option<usize> = None;
        for col in (0..grid[0].len()).rev() {
            match &grid[row][col] {
                'O' => {
                    if let Some(right) = right_most {
                        if right < width && right != col {
                            grid[row][right] = 'O';
                            grid[row][col] = '.';
                        }
                        right_most = Some(right.wrapping_sub(1));
                    } else {
                        right_most = Some(col - 1);
                    }
                }
                '.' => {
                    if right_most.is_none() {
                        right_most = Some(col);
                    }
                }
                '#' => right_most = None,
                _ => unreachable!(),
            }
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 136);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT), 64);
    }

    const TEST_INPUT: &'static str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
}
