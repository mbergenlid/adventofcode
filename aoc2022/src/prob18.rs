use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub fn solve_part_1(input: &str) -> usize {
    let cubes = input
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            (
                split.next().unwrap().parse::<i64>().unwrap(),
                split.next().unwrap().parse::<i64>().unwrap(),
                split.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut total_sides = 0;
    for (index, cube) in cubes.iter().enumerate() {
        total_sides += 6;
        total_sides -= 2 * cubes
            .iter()
            .take(index)
            .filter(|c1| share_one_side(cube, c1))
            .count();
    }
    total_sides
}

fn share_one_side(c1: &(i64, i64, i64), c2: &(i64, i64, i64)) -> bool {
    let x = (c1.0 - c2.0).abs();
    let y = (c1.1 - c2.1).abs();
    let z = (c1.2 - c2.2).abs();

    x + y + z == 1
}

pub fn solve_part_2(input: &str) -> usize {
    let cubes = input
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            (
                split.next().unwrap().parse::<i64>().unwrap(),
                split.next().unwrap().parse::<i64>().unwrap(),
                split.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let x_minmax = cubes
        .iter()
        .map(|(x, _, _)| x)
        .minmax()
        .into_option()
        .unwrap();
    let y_minmax = cubes
        .iter()
        .map(|(_, y, _)| y)
        .minmax()
        .into_option()
        .unwrap();
    let z_minmax = cubes
        .iter()
        .map(|(_, _, z)| z)
        .minmax()
        .into_option()
        .unwrap();

    let x_bounds = (x_minmax.0 - 1, x_minmax.1 + 1);
    let y_bounds = (y_minmax.0 - 1, y_minmax.1 + 1);
    let z_bounds = (z_minmax.0 - 1, z_minmax.1 + 1);

    let mut queue = VecDeque::new();
    queue.push_back((x_bounds.0, y_bounds.0, z_bounds.0));
    let mut visited = HashSet::new();
    visited.insert((x_bounds.0, y_bounds.0, z_bounds.0));

    let mut sides_visible = 0;
    while let Some((x, y, z)) = queue.pop_front() {
        for n @ (n_x, n_y, n_z) in [
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ] {
            if n_x < x_bounds.0
                || n_x > x_bounds.1
                || n_y < y_bounds.0
                || n_y > y_bounds.1
                || n_z < z_bounds.0
                || n_z > z_bounds.1
            {
                continue;
            }
            if cubes.contains(&n) {
                sides_visible += 1;
            } else {
                if !visited.contains(&n) {
                    visited.insert(n);

                    queue.push_back(n);
                }
            }
        }
    }
    sides_visible
}

#[cfg(test)]
mod test {
    use crate::prob18::{solve_part_1, solve_part_2};

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(INPUT), 64);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_part_2(INPUT), 58);
    }

    const INPUT: &'static str = r"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
}
