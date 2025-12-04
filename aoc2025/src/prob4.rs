use aoc_lib::grid::{Direction, Grid, Point};

pub fn solve_part_1(input: &str) -> usize {
    let grid: Grid<char> = input.parse().unwrap();
    all_removable_rolls(&grid).count()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut grid: Grid<char> = input.parse().unwrap();

    let mut result = 0;

    loop {
        let removable = all_removable_rolls(&grid).collect::<Vec<_>>();
        result += removable.len();
        if removable.is_empty() {
            return result;
        }
        for point in removable {
            grid.insert(point.pos, '.');
        }
    }
}

fn all_removable_rolls<'a>(grid: &'a Grid<char>) -> impl Iterator<Item = Point<char>> + 'a {
    grid.iter().filter(|p| p.value == '@').filter(|point| {
        let mut roll_neighbours = 0;
        for dir in [
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
        ] {
            if grid
                .path(dir, point.pos)
                .nth(1)
                .map(|p| p.value == '@')
                .unwrap_or(false)
            {
                roll_neighbours += 1;
            }
        }
        roll_neighbours < 4
    })
}

#[cfg(test)]
mod test {
    use crate::prob4::{solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), 13);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT), 43);
    }

    const INPUT: &'static str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
}
