use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let mut dots: Vec<_> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .collect();

    for line in input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .take(1)
    {
        if line.starts_with("fold along y") {
            let fold_y: u32 = line.split_once("=").unwrap().1.parse().unwrap();
            for (_, y) in dots.iter_mut() {
                if *y > fold_y {
                    *y = fold_y - (*y - fold_y);
                }
            }
        } else if line.starts_with("fold along x") {
            let fold_x: u32 = line.split_once("=").unwrap().1.parse().unwrap();
            for (x, _) in dots.iter_mut() {
                if *x > fold_x {
                    *x = fold_x - (*x - fold_x);
                }
            }
        }
    }
    dots.iter().unique().count()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut dots: Vec<_> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.split_once(",").unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .collect();

    let (top, mut bottom, left, mut right) = if let (MinMax(&top, &bottom), MinMax(&left, &right)) = (
        dots.iter().map(|(_, y)| y).minmax(),
        dots.iter().map(|(x, _)| x).minmax(),
    ) {
        (top, bottom, left, right)
    } else {
        panic!()
    };
    for line in input.lines().skip_while(|line| !line.is_empty()).skip(1) {
        if line.starts_with("fold along y") {
            let fold_y: u32 = line.split_once("=").unwrap().1.parse().unwrap();
            for (_, y) in dots.iter_mut() {
                bottom = fold_y;
                if *y > fold_y {
                    *y = fold_y - (*y - fold_y);
                }
            }
        } else if line.starts_with("fold along x") {
            let fold_x: u32 = line.split_once("=").unwrap().1.parse().unwrap();
            for (x, _) in dots.iter_mut() {
                right = fold_x;
                if *x > fold_x {
                    *x = fold_x - (*x - fold_x);
                }
            }
        }
    }
    use itertools::MinMaxResult::MinMax;
    let dots = dots.iter().unique().collect::<Vec<_>>();
    for y in top..bottom {
        for x in left..right {
            if dots.contains(&&(x, y)) {
                print!(".");
            } else {
                print!("#");
            }
            print!(" ");
        }
        println!();
    }
    0
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 17);
    }

    const TESTCASE: &'static str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
}
