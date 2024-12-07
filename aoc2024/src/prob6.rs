use std::collections::HashSet;

use aoc_lib::grid::{Grid, Pos};

pub fn solve_part_1(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Invalid input");

    return positions_visited(&grid).len();
}

pub fn solve_part_2(input: &str) -> usize {
    let mut grid = input.parse::<Grid>().expect("Invalid input");

    let start_pos = grid
        .iter()
        .find(|point| point.value == '^')
        .expect("No start pos found");
    let visited = positions_visited(&grid);

    let mut result = 0;
    for pos in visited {
        if pos == start_pos.pos {
            continue;
        }

        grid.insert(pos, '#');
        if is_cycle(&grid, start_pos.pos) {
            result += 1;
        }
        grid.insert(pos, '.');
    }

    result
}

fn is_cycle(grid: &Grid, start_pos: Pos) -> bool {
    let directions = [Grid::up, Grid::right, Grid::down, Grid::left];
    let mut current_dir = 0;
    let mut visted = HashSet::new();
    let mut current_pos = start_pos;
    loop {
        let mut iter = (directions[current_dir])(&grid, current_pos).peekable();

        if let Some(peek) = iter.peek() {
            if peek.value == '#' {
                //turn
                println!("ooops");
            }
        }

        let mut hit_wall = false;
        while let Some(p) = iter.next() {
            if visted.contains(&(p.pos, current_dir)) {
                return true;
            }

            visted.insert((p.pos, current_dir));
            if let Some(peek) = iter.peek() {
                if peek.value == '#' {
                    current_dir = (current_dir + 1) % 4;
                    current_pos = p.pos;
                    hit_wall = true;
                    break;
                }
            }
        }
        if !hit_wall {
            //Outside..
            return false;
        }
    }
}

fn positions_visited(grid: &Grid) -> HashSet<Pos> {
    let start_pos = grid
        .iter()
        .find(|point| point.value == '^')
        .expect("No start pos found");

    let directions = [Grid::up, Grid::right, Grid::down, Grid::left];
    let mut current_dir = 0;
    let mut visted = HashSet::new();
    let mut current_pos = start_pos.pos;
    loop {
        let mut iter = (directions[current_dir])(&grid, current_pos).peekable();

        if let Some(peek) = iter.peek() {
            if peek.value == '#' {
                //turn
            }
        }

        let mut hit_wall = false;
        while let Some(p) = iter.next() {
            visted.insert(p.pos);
            if let Some(peek) = iter.peek() {
                if peek.value == '#' {
                    current_dir = (current_dir + 1) % 4;
                    current_pos = p.pos;
                    hit_wall = true;
                    break;
                }
            }
        }
        if !hit_wall {
            //Outside..
            return visted;
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT), 41);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve_part_2(INPUT), 6);
    }

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
}
