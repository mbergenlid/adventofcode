pub fn solve_part_1(input: &str) -> u64 {
    let mut lines = input.lines();
    let target = lines.next().unwrap().parse::<u64>().unwrap();
    let result = lines
        .next()
        .unwrap()
        .split(",")
        .filter(|c| *c != "x")
        .map(|s| s.parse::<u64>().unwrap())
        .map(|n| (n, ((target / n) + 1) * n - target))
        .min_by(|(_, m1), (_, m2)| m1.cmp(m2))
        .unwrap();
    result.0 * result.1
}


//t   = 0 mod 17
//t   = -2 mod 13
pub fn solve_part_2(input: &str) -> i128 {
    let mut iter = input
        .lines()
        .nth(1)
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|(_, n)| *n != "x")
        .map(|(i, n)| (i as i128, n.parse::<i128>().unwrap()));

    let first = iter.next().unwrap();
    let (an, _) = iter.map(|(i, n)| (n-i, n)).fold(first, |(a1, n1), (a2, n2)| {
        let next_n = n1*n2;
        let (m1, m2) = bezouts_identity(n1, n2);
        let mut next = a1 * m2 * n2 + a2 * m1 * n1;
        while next < 0 {
            next += next_n;
        }
        while next >= next_n {
            let q = next/next_n;
            next -= q*next_n;
        }
        (next, next_n)
    });

    an
}

fn bezouts_identity(a: i128, b: i128) -> (i128, i128) {
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;
    let mut old_t = 0;
    let mut t = 1;

    while r != 0 {
        let quotient = old_r / r;
        let tmp = old_r;
        old_r = r;
        r = tmp - quotient * r;

        let tmp = old_s;
        old_s = s;
        s = tmp - quotient * s;

        let tmp = old_t;
        old_t = t;
        t = tmp - quotient * t;
    }
    return (old_s, old_t);
}

#[cfg(test)]
mod test {
    use crate::prob13::{bezouts_identity, solve_part_1, solve_part_2};

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2("xx\n17,x,13,19"), 3417);
        assert_eq!(solve_part_2("xx\n67,7,59,61"), 754018);
        assert_eq!(solve_part_2("xx\n67,x,7,59,61"), 779210);
        assert_eq!(solve_part_2("xx\n67,7,x,59,61"), 1261476);
        assert_eq!(solve_part_2("xx\n1789,37,47,1889"), 1202161486);

    }

    #[test]
    fn test_bezout() {
        assert_eq!(bezouts_identity(3, 4), (-1, 1));
        assert_eq!(bezouts_identity(5, 12), (5, -2));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            solve_part_1(
                "939
7,13,x,x,59,x,31,19"
            ),
            295
        );
    }
}
