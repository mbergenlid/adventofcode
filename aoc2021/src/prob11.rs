#[derive(Clone, Eq, PartialEq, Debug)]
struct Position((usize, usize));

impl Position {
    fn neighbours(&self) -> Vec<Position> {
        let row_start = if self.0 .0 == 0 { 0 } else { self.0 .0 - 1 };
        let col_start = if self.0 .1 == 0 { 0 } else { self.0 .1 - 1 };
        (row_start..=(self.0 .0 + 1))
            .flat_map(|r| (col_start..=(self.0 .1 + 1)).map(move |c| Position((r, c))))
            .filter(|p| p != self)
            .filter(Position::in_bounds)
            .collect()
    }

    fn in_bounds(&self) -> bool {
        self.0 .0 < 10 && self.0 .1 < 10
    }
}

struct Grid {
    data: Vec<Vec<u32>>,
    total_flashes: usize,
    number_of_flashes_in_last_step: usize
}

impl Grid {
    fn new(input: &str) -> Grid {
        Grid {
            data: input
                .lines()
                .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
            total_flashes: 0,
            number_of_flashes_in_last_step: 0,
        }
    }

    fn step(&mut self) {
        let mut octopuses_flashing = Vec::new();
        let mut total_octopuses_flashing_in_this_step = Vec::new();
        for (row_index, row) in self.data.iter_mut().enumerate() {
            for (col_index, value) in row.iter_mut().enumerate() {
                *value += 1;
                if *value > 9 {
                    octopuses_flashing.push(Position((row_index, col_index)));
                    total_octopuses_flashing_in_this_step.push(Position((row_index, col_index)));
                }
            }
        }

        while !octopuses_flashing.is_empty() {
            let next = octopuses_flashing.pop().unwrap();
            for neighbour in next.neighbours() {
                if self.data[neighbour.0 .0][neighbour.0 .1] <= 9 {
                    *self.data
                        .get_mut(neighbour.0 .0)
                        .unwrap()
                        .get_mut(neighbour.0 .1)
                        .unwrap() += 1;
                    if self.data[neighbour.0 .0][neighbour.0 .1] > 9 {
                        octopuses_flashing.push(neighbour.clone());
                        total_octopuses_flashing_in_this_step.push(neighbour);
                    }
                }
            }
        }

        self.number_of_flashes_in_last_step = total_octopuses_flashing_in_this_step.len();
        self.total_flashes += total_octopuses_flashing_in_this_step.len();
        for p in total_octopuses_flashing_in_this_step {
            self.data[p.0 .0][p.0 .1] = 0;
        }
    }

}

pub fn solve_part_1(_input: &str) -> usize {
    let mut grid = Grid::new(_input);

    for _step in 0..100 {
        grid.step()
    }
    grid.total_flashes
}

pub fn solve_part_2(_input: &str) -> usize {
    let mut grid = Grid::new(_input);

    for step in 0.. {
        grid.step();
        if grid.number_of_flashes_in_last_step == 100 {
            return step + 1;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 1656);
    }

    #[test]
    fn test_2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 195);
    }

    const TESTCASE: &'static str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
}
