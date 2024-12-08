
use std::collections::HashSet;

use aoc_lib::grid::Grid;
use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let grid = input.parse::<Grid<char>>().expect("Invalid input");

    let antennas = grid.iter()
        .filter_map(|p| if p.value == '.' { None } else { Some(p) })
        .collect_vec();

    let mut visited = HashSet::new();
    for (index, a1) in antennas.iter().enumerate() {
        for a2 in antennas.iter().skip(index+1) {
            if a1.value == a2.value {
                let row_diff = a1.pos.row() as isize - a2.pos.row() as isize;
                let col_diff = a1.pos.col() as isize - a2.pos.col() as isize;

               let mut iter =  grid.step(a1.pos, row_diff, col_diff);
               if let Some(p) = iter.find(|p| p.pos != a1.pos && p.pos != a2.pos) {
                   visited.insert(p.pos);
               }

               let mut iter =  grid.step(a1.pos, -row_diff, -col_diff);
               if let Some(p) = iter.find(|p| p.pos != a1.pos && p.pos != a2.pos) {
                   visited.insert(p.pos);
               }

            }
        }
    }
    visited.len()
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = input.parse::<Grid<char>>().expect("Invalid input");

    let antennas = grid.iter()
        .filter_map(|p| if p.value == '.' { None } else { Some(p) })
        .collect_vec();

    let mut visited = HashSet::new();
    for (index, a1) in antennas.iter().enumerate() {
        for a2 in antennas.iter().skip(index+1) {
            if a1.value == a2.value {
                let row_diff = a1.pos.row() as isize - a2.pos.row() as isize;
                let col_diff = a1.pos.col() as isize - a2.pos.col() as isize;

               for p in grid.step(a1.pos, row_diff, col_diff) {
                   visited.insert(p.pos);
               }

               for p in grid.step(a1.pos, -row_diff, -col_diff) {
                   visited.insert(p.pos);
               }


            }
        }
    }
    visited.len()
}



#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT), 14);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve_part_2(INPUT), 34);
    }

    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
}
