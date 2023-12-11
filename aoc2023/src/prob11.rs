use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> usize {
    solve(input, 1)
}

pub fn solve_part_2(input: &str) -> usize {
    solve(input, 1000000 - 1)
}

fn solve(input: &str, increase: usize) -> usize {
    let galaxies = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(col, _)| (row, col))
        })
        .collect::<HashSet<_>>();

    let original_width = input.lines().next().unwrap().len();
    let original_height = input.lines().count();

    let rows_with_galaxies = galaxies
        .iter()
        .copied()
        .map(|(row, _)| row)
        .collect::<HashSet<_>>();
    let cols_with_galaxies = galaxies
        .iter()
        .copied()
        .map(|(_, col)| col)
        .collect::<HashSet<_>>();

    let rows_without_galaxies = (0..original_height)
        .filter(|row| !rows_with_galaxies.contains(&row))
        .collect::<HashSet<_>>();
    let cols_without_galaxies = (0..original_width)
        .filter(|col| !cols_with_galaxies.contains(&col))
        .collect::<HashSet<_>>();

    let galaxies = galaxies
        .into_iter()
        .map(|(row, col)| {
            let prev_rows = rows_without_galaxies.iter().filter(|&r| *r < row).count();
            let prev_cols = cols_without_galaxies.iter().filter(|&c| *c < col).count();

            (row + prev_rows * increase, col + prev_cols * increase)
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (i, (r1, c1)) in galaxies.iter().enumerate().take(galaxies.len() - 1) {
        for (r2, c2) in galaxies.iter().skip(i + 1) {
            let distance = (*r2 as i64 - *r1 as i64).abs() + (*c2 as i64 - *c1 as i64).abs();
            sum += distance as usize;
        }
    }
    sum
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 374);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve(TEST_INPUT, 100 - 1), 8410);
    }

    const TEST_INPUT: &'static str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
}
