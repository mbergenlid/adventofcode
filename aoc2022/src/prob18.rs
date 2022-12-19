
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
        total_sides -= 2*cubes.iter().take(index).filter(|c1| share_one_side(cube, c1)).count();
    }
    total_sides
}

fn share_one_side(c1: &(i64, i64, i64), c2: &(i64, i64, i64)) -> bool {
    let x = (c1.0 - c2.0).abs();
    let y = (c1.1 - c2.1).abs();
    let z = (c1.2 - c2.2).abs();

    x + y + z == 1
}

pub fn solve_part_2(_input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::prob18::solve_part_1;

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(INPUT), 64);
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
