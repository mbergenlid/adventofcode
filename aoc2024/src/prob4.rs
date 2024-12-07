use aoc_lib::grid::Grid;

pub fn solve_part_1(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Invalid input");

    let mut result = 0;
    for point in grid.iter() {
        if point.value != 'X' {
            continue;
        }
        //Look left
        if grid.right(point.pos).values().starts_with("XMAS".chars()) {
            result += 1;
        }
        if grid.left(point.pos).values().starts_with("XMAS".chars()) {
            result += 1;
        }
        //Look up
        if grid.up(point.pos).values().starts_with("XMAS".chars()) {
            result += 1;
        }
        //Look down
        if grid.down(point.pos).values().starts_with("XMAS".chars()) {
            result += 1;
        }
        if grid.up_left(point.pos).values().starts_with("XMAS".chars()) {
            result += 1;
        }
        if grid
            .up_right(point.pos)
            .values()
            .starts_with("XMAS".chars())
        {
            result += 1;
        }
        if grid
            .down_left(point.pos)
            .values()
            .starts_with("XMAS".chars())
        {
            result += 1;
        }
        if grid
            .down_right(point.pos)
            .values()
            .starts_with("XMAS".chars())
        {
            result += 1;
        }
    }
    result
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Invalid input");

    let mut result = 0;
    for point in grid.iter() {
        if point.value != 'A' {
            continue;
        }

        let up_left = point.pos.up_left();
        let down_left = point.pos.down_left();
        if (grid.down_right(up_left).values().starts_with("MAS".chars())
            || grid.down_right(up_left).values().starts_with("SAM".chars()))
            && (grid.up_right(down_left).values().starts_with("MAS".chars())
                || grid.up_right(down_left).values().starts_with("SAM".chars()))
        {
            result += 1;
        }
    }
    result
}

impl<T> StartsWithIter for T
where
    T: Iterator,
    T::Item: Eq,
{
    fn starts_with(mut self, other: impl Iterator<Item = Self::Item>) -> bool {
        for x in other {
            if let Some(self_val) = self.next() {
                if x != self_val {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

trait StartsWithIter: Iterator {
    fn starts_with(self, other: impl Iterator<Item = Self::Item>) -> bool;
}

#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT), 18);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve_part_2(INPUT), 9);
    }

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
}
