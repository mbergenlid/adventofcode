use std::collections::{HashSet, VecDeque};

use aoc_lib::grid::{Grid, Point};

pub fn solve_part_1(input: &str) -> usize {
    let (result, _) = solve(input);
    result
}

pub fn solve_part_2(input: &str) -> usize {
    let (_, result) = solve(input);
    result
}

fn solve(input: &str) -> (usize, usize) {
    let grid = input.parse::<Grid<u32>>().expect("Invalid input");

    let trail_heads = grid.iter().filter(|p| p.value == 0).collect::<VecDeque<_>>();
    let mut unique_paths = 0;
    let mut unique_targets = 0;

    for head in trail_heads {
        let mut queue = VecDeque::new();
        let mut targets = HashSet::new();
        queue.push_back(head);

        while let Some(next) = queue.pop_front() {

            if next.value == 9 {
                unique_paths += 1;
                targets.insert(next.pos);
            }

            let up_pos = next.pos.up();
            if let Some(up) = grid.get(up_pos) {
                if *up == next.value + 1 {
                    queue.push_back(Point { value: *up, pos: up_pos });
                }
            }
            let down_pos = next.pos.down();
            if let Some(down) = grid.get(down_pos) {
                if *down == next.value + 1 {
                    queue.push_back(Point { value: *down, pos: down_pos });
                }
            }
            let left_pos = next.pos.left();
            if let Some(left) = grid.get(left_pos) {
                if *left == next.value + 1 {
                    queue.push_back(Point { value: *left, pos: left_pos });
                }
            }
            let right_pos = next.pos.right();
            if let Some(right) = grid.get(right_pos) {
                if *right == next.value + 1 {
                    queue.push_back(Point { value: *right, pos: right_pos });
                }
            }
        }

        unique_targets += targets.len();
    }

    (unique_targets, unique_paths)

}

#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT), 36);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve_part_2(INPUT), 81);
    }
    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
}
