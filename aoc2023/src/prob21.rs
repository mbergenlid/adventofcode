use std::{collections::VecDeque, fmt::Debug, str::FromStr};

pub fn solve_part_1(input: &str) -> usize {
    let grid = input.parse::<Grid<64>>().unwrap();
    solve_1(grid)
}

fn solve_1<const N: usize>(mut grid: Grid<N>) -> usize {
    let (row, col) = grid.start;

    grid.solve(row, col, 0);
    grid.evens()
}

pub fn solve_part_2(input: &str) -> usize {
    solve(input.parse::<Grid<26501365>>().unwrap())
}

fn solve<const N: usize>(template: Grid<N>) -> usize {
    let (start_row, start_col) = template.start;
    let mut start_grid = template.clone();
    start_grid.solve(start_row, start_col, 0);

    let mut total = start_grid.odds_starting_at(0);
    {
        {
            let mut right = template.clone();
            right.solve(start_row, 0, 0);
            let max = right.max();
            let even_count = right.odds_starting_at(0);
            let odd_count = right.odds_starting_at(1);
            let something = (template.width - 1 - start_col) + max;
            let cols = if something > N {
                0
            } else {
                (N - something) / template.width
            };
            total += (cols / 2 + cols % 2) * even_count * 4;
            total += (cols / 2) * odd_count * 4;

            for col in cols.. {
                let this_tile =
                    right.odds_starting_at(col * template.width + (template.width - start_col));
                total += this_tile;

                if this_tile == 0 {
                    break;
                }
            }
            let mut left = template.clone();
            left.solve(start_row, template.width - 1, 0);
            for col in cols.. {
                let this_tile = left.odds_starting_at(col * template.width + start_col + 1);
                total += this_tile;

                if this_tile == 0 {
                    break;
                }
            }
            let mut up = template.clone();
            up.solve(template.height - 1, start_col, 0);
            for row in cols.. {
                let this_tile = up.odds_starting_at(row * template.height + start_row + 1);
                total += this_tile;

                if this_tile == 0 {
                    break;
                }
            }
            let mut down = template.clone();
            down.solve(0, start_col, 0);
            for row in cols.. {
                let this_tile =
                    down.odds_starting_at(row * template.height + (template.height - start_row));
                total += this_tile;

                if this_tile == 0 {
                    break;
                }
            }
        }
    }

    // Down
    {
        let mut right = template.clone();
        right.solve(0, 0, 0);
        let max_right = right.max();
        let mut left = template.clone();
        left.solve(0, template.width - 1, 0);
        let even_count_left = left.odds_starting_at(0);
        let odd_count_left = left.odds_starting_at(1);
        let max_left = left.max();
        {
            let something = 0 * template.height
                + (template.height - 1 - start_row)
                + 0 * template.width
                + (template.width - 1 - start_col)
                + max_right;
            let cols = if something > N {
                0
            } else {
                (N - something) / template.width
            };
            if cols % 2 == 0 {
                let number_of_odd_lines = cols;
                let total_filled = cols * (cols + 1);
                total += ((total_filled - number_of_odd_lines) / 2) * even_count_left;
                total += ((total_filled - number_of_odd_lines) / 2) * odd_count_left;
                total += number_of_odd_lines * odd_count_left
            } else {
                let number_of_odd_lines = (cols / 2 + 1) * 2;
                let total_filled = (cols) * (cols + 1);
                total += ((total_filled - number_of_odd_lines) / 2) * even_count_left;
                total += ((total_filled - number_of_odd_lines) / 2) * odd_count_left;
                total += number_of_odd_lines * even_count_left
            }
        }
        {
            let something = 0 * template.height
                + (template.height - 1 - start_row)
                + 0 * template.width
                + (template.width - 1 - start_col)
                + max_right;
            let col = if something > N {
                0
            } else {
                (N - something) / template.width
            };
            for col in col.. {
                let this_tile = right.odds_starting_at(
                    0 * template.height
                        + (template.height - 1 - start_row)
                        + col * template.width
                        + (template.width - 1 - start_col)
                        + 2,
                );
                total += this_tile * (col + 1);

                if this_tile == 0 {
                    break;
                }
            }
        }
        {
            let something = 0 * template.height
                + (template.height - 1 - start_row)
                + 0 * template.width
                + (template.width - 1 - start_col)
                + max_left;
            let col = if something > N {
                0
            } else {
                (N - something) / template.width
            };
            for col in col.. {
                let this_tile = left.odds_starting_at(
                    0 * template.height
                        + (template.height - 1 - start_row)
                        + col * template.width
                        + start_col
                        + 2,
                );
                total += this_tile * (col + 1);

                if this_tile == 0 {
                    break;
                }
            }
        }
    }
    // Up
    {
        let mut right = template.clone();
        right.solve(template.height - 1, 0, 0);
        let max_right = right.max();
        let mut left = template.clone();
        left.solve(template.height - 1, template.width - 1, 0);
        let even_count_left = left.odds_starting_at(0);
        let odd_count_left = left.odds_starting_at(1);
        let something = 0 * template.height
            + start_row
            + 0 * template.width
            + (template.width - 1 - start_col)
            + max_right;
        let cols = if something > N {
            0
        } else {
            (N - something) / template.width
        };
        if cols % 2 == 0 {
            let number_of_odd_lines = cols;
            let total_filled = cols * (cols + 1);
            total += ((total_filled - number_of_odd_lines) / 2) * even_count_left;
            total += ((total_filled - number_of_odd_lines) / 2) * odd_count_left;
            total += number_of_odd_lines * odd_count_left
        } else {
            let number_of_odd_lines = (cols / 2 + 1) * 2;
            let total_filled = (cols) * (cols + 1);
            total += ((total_filled - number_of_odd_lines) / 2) * even_count_left;
            total += ((total_filled - number_of_odd_lines) / 2) * odd_count_left;
            total += number_of_odd_lines * even_count_left
        }
        for col in cols.. {
            let this_tile = right.odds_starting_at(
                0 * template.height
                    + start_row
                    + col * template.width
                    + (template.width - 1 - start_col)
                    + 2,
            );
            total += this_tile * (col + 1);

            if this_tile == 0 {
                break;
            }
        }
        for col in cols.. {
            let this_tile = left.odds_starting_at(
                0 * template.height + start_row + col * template.width + start_col + 2,
            );
            total += this_tile * (col + 1);

            if this_tile == 0 {
                break;
            }
        }
    }

    total
}

#[derive(PartialEq, Eq, Clone)]
enum Tile {
    Wall,
    Step(usize),
}

impl Tile {
    fn steps(&self) -> Option<usize> {
        match self {
            Tile::Wall => None,
            Tile::Step(s) => Some(*s),
        }
    }
}

#[derive(Clone)]
struct Grid<const N: usize> {
    data: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    start: (usize, usize),
}

impl<const N: usize> Debug for Grid<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for col in row {
                match col {
                    Tile::Wall => write!(f, "#")?,
                    Tile::Step(s) => {
                        if *s <= N && s % 2 == 1 {
                            write!(f, "+")?
                        } else if *s == usize::MAX {
                            write!(f, "*")?
                        } else {
                            write!(f, ".")?
                        }
                    }
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<const N: usize> Grid<N> {
    fn solve(&mut self, row: usize, col: usize, step_count: usize) {
        let mut queue = VecDeque::new();
        queue.push_back((step_count, row, col));
        while let Some((steps, row, col)) = queue.pop_front() {
            if self.data[row][col] == Tile::Wall {
                continue;
            }
            if let Tile::Step(c) = &mut self.data[row][col] {
                if *c <= steps {
                    continue;
                } else {
                    *c = steps;
                }
            }
            //Up
            {
                if row > 0 {
                    queue.push_back((steps + 1, row - 1, col));
                }
            }
            //Right
            {
                if col < self.width - 1 {
                    queue.push_back((steps + 1, row, col + 1));
                }
            }
            //Down
            {
                if row < self.height - 1 {
                    queue.push_back((steps + 1, row + 1, col));
                }
            }
            //Left
            {
                if col > 0 {
                    queue.push_back((steps + 1, row, col - 1));
                }
            }
        }
    }

    fn evens(&self) -> usize {
        self.data
            .iter()
            .flat_map(|r| r.iter().filter_map(|c| c.steps()))
            .filter(|s| *s != usize::MAX && *s <= N)
            .filter(|s| *s % 2 == 0)
            .count()
    }

    fn odds_starting_at(&self, start: usize) -> usize {
        self.data
            .iter()
            .flat_map(|r| r.iter().filter_map(|c| c.steps()))
            .filter(|s| *s != usize::MAX && *s + start <= N)
            .filter(|s| (*s + start) % 2 == 1)
            .count()
    }

    fn max(&self) -> usize {
        self.data
            .iter()
            .flat_map(|r| r.iter().filter_map(|c| c.steps()))
            .filter(|s| *s != usize::MAX)
            .max()
            .unwrap()
    }
}

impl<const N: usize> FromStr for Grid<N> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;
        let data: Vec<Vec<_>> = s
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .map(|(col, c)| match c {
                        'S' => {
                            start = Some((row, col));
                            Tile::Step(usize::MAX)
                        }
                        '.' => Tile::Step(usize::MAX),
                        '#' => Tile::Wall,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect();
        let width = data[0].len();
        let height = data.len();

        Ok(Grid {
            data,
            width,
            height,
            start: start.unwrap(),
        })
    }
}

#[cfg(test)]
mod test {
    use crate::prob21::Grid;

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_1(TEST_INPUT.parse::<Grid<6>>().unwrap()), 16);
    }

    #[test]
    fn solve_part_2() {
        let input = include_str!("../inputs/prob21");
        //assert_eq!(super::solve(input.parse::<Grid<101>>().unwrap()), 9279);
        //assert_eq!(super::solve(input.parse::<Grid<201>>().unwrap()), 36525);
        //assert_eq!(super::solve(input.parse::<Grid<251>>().unwrap()), 56624);
        //assert_eq!(super::solve(input.parse::<Grid<501>>().unwrap()), 224670);

        assert_eq!(super::solve(input.parse::<Grid<1001>>().unwrap()), 895133);
        assert_eq!(super::solve(input.parse::<Grid<2001>>().unwrap()), 3569651);
        assert_eq!(super::solve(input.parse::<Grid<5001>>().unwrap()), 22283554);
        assert_eq!(super::solve(input.parse::<Grid<5005>>().unwrap()), 22318766);
    }

    #[test]
    fn real_test_part_2() {
        let input = include_str!("../inputs/prob21");
        assert_eq!(
            super::solve(input.parse::<Grid<26501365>>().unwrap()),
            625587097150084
        );
    }

    const TEST_INPUT: &'static str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
}
