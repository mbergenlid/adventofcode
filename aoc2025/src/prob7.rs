use std::collections::{HashMap, HashSet};

use aoc_lib::grid::{Grid, Pos};

pub fn solve_part_1(input: &str) -> usize {
    let grid: Grid<char> = input.trim().parse().unwrap();
    let start = grid
        .iter()
        .find(|p| p.value == 'S')
        .expect("There should be a start pos");

    let mut visited = HashSet::new();
    part1::count_splits(&grid, &mut visited, grid.down(start.pos))
}

mod part1 {
    use std::collections::HashSet;

    use aoc_lib::grid::{Grid, Pos};

    pub fn count_splits(
        grid: &Grid<char>,
        visited: &mut HashSet<Pos>,
        path: aoc_lib::grid::PathIterator<'_, char>,
    ) -> usize {
        for p in path {
            if p.value == '^' {
                if !visited.contains(&p.pos) {
                    visited.insert(p.pos);
                    return 1
                        + count_splits(grid, visited, grid.down(p.pos.left()))
                        + count_splits(grid, visited, grid.down(p.pos.right()));
                } else {
                    return 0;
                }
            }
        }
        0
    }
}

pub fn solve_part_2(input: &str) -> usize {
    let grid: Grid<char> = input.trim().parse().unwrap();

    let mut cache: HashMap<Pos, usize> = HashMap::new();

    let splitters: Vec<_> = grid
        .iter()
        .filter(|p| p.value == '^')
        .map(|p| p.pos)
        .collect();
    for splitter in splitters.iter().rev() {
        let left = part2::count_timelines(&mut cache, grid.down(splitter.left()));
        let right = part2::count_timelines(&mut cache, grid.down(splitter.right()));

        cache.insert(*splitter, left + right);
    }

    *cache
        .get(splitters.first().expect("Should be at least one splitter"))
        .expect("All splitters should be assigned")
}

mod part2 {
    use std::collections::HashMap;

    use aoc_lib::grid::Pos;

    pub fn count_timelines(
        cache: &mut HashMap<Pos, usize>,
        path: aoc_lib::grid::PathIterator<'_, char>,
    ) -> usize {
        for p in path {
            if p.value == '^' {
                return *cache.get(&p.pos).expect("Should be set in reverse order");
            }
        }
        1
    }
}

#[cfg(test)]
mod test {
    use crate::prob7::{solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), 21);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT), 40);
    }

    const INPUT: &'static str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
}
