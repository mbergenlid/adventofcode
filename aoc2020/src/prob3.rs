use crate::prob3::Tile::{Empty, Tree};

pub fn solve_part_1() -> usize {
    let slope = Slope::parse(INPUT);
    slope.count_trees_on_path(3, 1)
}

pub fn solve_part_2() -> usize {
    let slope = Slope::parse(INPUT);
    slope.count_trees_on_path(1, 1)
        * slope.count_trees_on_path(3, 1)
        * slope.count_trees_on_path(5, 1)
        * slope.count_trees_on_path(7, 1)
        * slope.count_trees_on_path(1, 2)
}

const INPUT: &'static str = include_str!("../inputs/prob3");

#[derive(Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Tree,
}
struct Slope {
    width: usize,
    height: usize,
    slope: Vec<Vec<Tile>>,
}

impl Slope {
    fn parse(s: &str) -> Slope {
        let mut slope = Vec::new();
        let mut width = 0;
        let mut height = 0;
        for line in s.lines() {
            height += 1;
            if line.len() > width {
                width = line.len();
            }
            slope.push(
                line.chars()
                    .map(|c| match c {
                        '.' => Empty,
                        '#' => Tree,
                        _ => panic!("Unknown Tile {}", c),
                    })
                    .collect(),
            );
        }
        Slope {
            width,
            height,
            slope,
        }
    }

    fn count_trees_on_path(&self, right: usize, down: usize) -> usize {
        let mut current_row = 0;
        let mut current_col = 0;
        let mut trees = 0;
        while current_row < self.height {
            if self.slope[current_row][current_col] == Tree {
                trees += 1;
            }
            current_row += down;
            current_col = (current_col + right) % self.width;
        }
        trees
    }
}

#[cfg(test)]
mod test {
    use crate::prob3::Slope;

    #[test]
    fn test_part_1() {
        let slope = Slope::parse(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        );
        assert_eq!(slope.count_trees_on_path(3, 1), 7);
    }
}
