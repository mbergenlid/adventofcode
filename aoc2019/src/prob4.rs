pub fn solve_for_part_1() {
    let mut code = vec![2, 7, 3, 0, 2, 5];
    let mut count = 0;
    for _i in 273025..767253 {
        if is_valid(&code) {
            count += 1;
        }
        next(&mut code);
    }
    println!("{:?}", &code);
    println!("Part 1: {}", count);
}
pub fn solve_for_part_2() {
    let mut code = vec![2, 7, 3, 0, 2, 5];
    let mut count = 0;
    for _i in 273025..767253 {
        if is_valid_part_2(&code) {
            println!("{:?}", &code);
            count += 1;
        }
        next(&mut code);
    }
    println!("Part 1: {}", count);
}
fn next(code: &mut Vec<u8>) {
    for idx in 0..6 {
        let i = 5 - idx;
        if code[i] == 9 {
            code.as_mut_slice()[i] = 0;
        } else {
            code.as_mut_slice()[i] += 1;
            return;
        }
    }
}

fn is_valid(code: &Vec<u8>) -> bool {
    let mut previous = code[0];
    let mut double = false;
    for &c in code.iter().skip(1) {
        if c < previous {
            return false;
        }
        if c == previous {
            double = true;
        }
        previous = c;
    }
    double
}

fn is_valid_part_2(code: &Vec<u8>) -> bool {
    let mut prev1 = code[0];
    let mut double = false;
    let mut current_group_size = 1;
    for &c in code.iter().skip(1) {
        if c < prev1 {
            return false;
        }
        if c == prev1 {
            current_group_size += 1;
        } else {
            if current_group_size == 2 {
                double = true;
            }
            current_group_size = 1;
        }
        prev1 = c;
    }
    current_group_size == 2 || double
}

#[cfg(test)]
mod test {

    #[test]
    fn test_input() {
        assert!(super::is_valid(&vec!(1, 1, 1, 1, 1, 1)));
        assert_eq!(super::is_valid(&vec!(2, 2, 3, 4, 5, 0)), false);
        assert_eq!(super::is_valid(&vec!(1, 2, 3, 7, 8, 9)), false);
    }

    #[test]
    fn test_next() {
        let mut code = vec![2, 7, 3, 0, 2, 5];
        super::next(&mut code);
        assert_eq!(code, vec![2, 7, 3, 0, 2, 6]);
    }

    #[test]
    fn test_next_1() {
        let mut code = vec![2, 9, 9, 9, 9, 9];
        super::next(&mut code);
        assert_eq!(code, vec![3, 0, 0, 0, 0, 0]);
    }
}
