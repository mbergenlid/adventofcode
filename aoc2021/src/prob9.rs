use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

pub fn solve_part_1(input: &str) -> usize {
    let height_map = parse(input);
    find_all_low_points(&height_map).iter().map(|(row, col)| (height_map[*row][*col] + 1) as usize).sum()
}

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect()
}

fn find_all_low_points(map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (row_index, levels) in map.iter().enumerate() {
        for (col_index, height) in levels.iter().enumerate() {
            if neighbours(map, &(row_index, col_index))
                .iter()
                .all(|(row, col)| map[*row][*col] > *height)
            {
                result.push((row_index, col_index));
            }
        }
    }
    result
}

pub fn solve_part_2(input: &str) -> usize {
    let map = parse(input);
    let low_points = find_all_low_points(&map);

    low_points
        .iter()
        .map(|point| basin_size(&map, point))
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn basin_size(map: &Vec<Vec<u8>>, point: &(usize, usize)) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(*point);

    let mut size = 0;
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        size += 1;

        let point_value = map[current.0][current.1];

        for (n_row, n_col) in neighbours(map, &current) {
            let neighbour_value = map[n_row][n_col];
            if !visited.contains(&(n_row, n_col))
                && neighbour_value > point_value
                && neighbour_value < 9
            {
                queue.push_back((n_row, n_col));
                visited.insert((n_row, n_col));
            }
        }

        visited.insert(current);
    }
    size
}

fn neighbours(map: &Vec<Vec<u8>>, point: &(usize, usize)) -> Vec<(usize, usize)> {
    let &(row_index, col_index) = point;
    let mut result = Vec::new();
    if col_index > 0 {
        result.push((row_index, col_index - 1));
    }
    if col_index < map[0].len() - 1 {
        result.push((row_index, col_index + 1));
    }
    if row_index > 0 {
        result.push((row_index - 1, col_index));
    }
    if row_index < map.len() - 1 {
        result.push((row_index + 1, col_index));
    }
    result
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {
        let res = super::solve_part_1(
            "2199943210
3987894921
9856789892
8767896789
9899965678",
        );

        assert_eq!(res, 15);
    }

    #[test]
    fn test_2() {
        let res = super::solve_part_2(
            "2199943210
3987894921
9856789892
8767896789
9899965678",
        );

        assert_eq!(res, 1134);
    }
}
