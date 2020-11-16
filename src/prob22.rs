use num_integer::Integer;

pub fn solve_part_1() {
    let vec = SpaceDeck::new(10007)
        .deal_with_increment(65)
        .deal_into_new_stack()
        .deal_with_increment(25)
        .cut(-6735)
        .deal_with_increment(3)
        .cut(8032)
        .deal_with_increment(71)
        .cut(-4990)
        .deal_with_increment(66)
        .deal_into_new_stack()
        .cut(-8040)
        .deal_into_new_stack()
        .deal_with_increment(18)
        .cut(-8746)
        .deal_with_increment(42)
        .deal_into_new_stack()
        .deal_with_increment(17)
        .cut(-8840)
        .deal_with_increment(55)
        .cut(-4613)
        .deal_with_increment(10)
        .cut(-5301)
        .deal_into_new_stack()
        .deal_with_increment(21)
        .cut(-5653)
        .deal_with_increment(2)
        .cut(5364)
        .deal_with_increment(72)
        .cut(-3468)
        .deal_into_new_stack()
        .cut(-9661)
        .deal_with_increment(63)
        .cut(6794)
        .deal_with_increment(43)
        .cut(2935)
        .deal_with_increment(66)
        .cut(-1700)
        .deal_with_increment(6)
        .cut(5642)
        .deal_with_increment(64)
        .deal_into_new_stack()
        .cut(-5699)
        .deal_with_increment(43)
        .cut(-9366)
        .deal_with_increment(42)
        .deal_into_new_stack()
        .cut(2364)
        .deal_with_increment(13)
        .cut(8080)
        .deal_with_increment(2)
        .deal_into_new_stack()
        .cut(-9602)
        .deal_with_increment(51)
        .cut(3214)
        .deal_into_new_stack()
        .cut(-2995)
        .deal_with_increment(57)
        .cut(-8169)
        .deal_into_new_stack()
        .cut(362)
        .deal_with_increment(41)
        .cut(-4547)
        .deal_with_increment(56)
        .cut(-1815)
        .deal_into_new_stack()
        .cut(1554)
        .deal_with_increment(71)
        .cut(2884)
        .deal_with_increment(44)
        .cut(-2423)
        .deal_with_increment(4)
        .deal_into_new_stack()
        .deal_with_increment(20)
        .cut(-2242)
        .deal_with_increment(48)
        .cut(-716)
        .deal_with_increment(29)
        .cut(-6751)
        .deal_with_increment(45)
        .cut(-9511)
        .deal_with_increment(75)
        .cut(-440)
        .deal_with_increment(35)
        .cut(6861)
        .deal_with_increment(52)
        .cut(-4702)
        .deal_into_new_stack()
        .deal_with_increment(28)
        .cut(305)
        .deal_with_increment(16)
        .cut(7094)
        .deal_into_new_stack()
        .cut(5175)
        .deal_with_increment(30)
        .deal_into_new_stack()
        .deal_with_increment(61)
        .cut(1831)
        .deal_into_new_stack()
        .deal_with_increment(25)
        .cut(4043)
        .0;

    let (position, _) = vec
        .iter()
        .enumerate()
        .find(|(_, &card)| card == 2019)
        .unwrap();
    println!("Part 1: {}", position);
}

pub fn solve_part_2() {
    let mut deck = SpaceDeck2::new(119315717514047, 2020);
    deck.cut(4043);
    deck.deal_with_increment(25);
    deck.deal_into_new_stack();
    deck.cut(1831);
    deck.deal_with_increment(61);
    deck.deal_into_new_stack();
    deck.deal_with_increment(30);
    deck.cut(5175);
    deck.deal_into_new_stack();
    deck.cut(7094);
    deck.deal_with_increment(16);
    deck.cut(305);
    deck.deal_with_increment(28);
    deck.deal_into_new_stack();
    deck.cut(-4702);
    deck.deal_with_increment(52);
    deck.cut(6861);
    deck.deal_with_increment(35);
    deck.cut(-440);
    deck.deal_with_increment(75);
    deck.cut(-9511);
    deck.deal_with_increment(45);
    deck.cut(-6751);
    deck.deal_with_increment(29);
    deck.cut(-716);
    deck.deal_with_increment(48);
    deck.cut(-2242);
    deck.deal_with_increment(20);
    deck.deal_into_new_stack();
    deck.deal_with_increment(4);
    deck.cut(-2423);
    deck.deal_with_increment(44);
    deck.cut(2884);
    deck.deal_with_increment(71);
    deck.cut(1554);
    deck.deal_into_new_stack();
    deck.cut(-1815);
    deck.deal_with_increment(56);
    deck.cut(-4547);
    deck.deal_with_increment(41);
    deck.cut(362);
    deck.deal_into_new_stack();
    deck.cut(-8169);
    deck.deal_with_increment(57);
    deck.cut(-2995);
    deck.deal_into_new_stack();
    deck.cut(3214);
    deck.deal_with_increment(51);
    deck.cut(-9602);
    deck.deal_into_new_stack();
    deck.deal_with_increment(2);
    deck.cut(8080);
    deck.deal_with_increment(13);
    deck.cut(2364);
    deck.deal_into_new_stack();
    deck.deal_with_increment(42);
    deck.cut(-9366);
    deck.deal_with_increment(43);
    deck.cut(-5699);
    deck.deal_into_new_stack();
    deck.deal_with_increment(64);
    deck.cut(5642);
    deck.deal_with_increment(6);
    deck.cut(-1700);
    deck.deal_with_increment(66);
    deck.cut(2935);
    deck.deal_with_increment(43);
    deck.cut(6794);
    deck.deal_with_increment(63);
    deck.cut(-9661);
    deck.deal_into_new_stack();
    deck.cut(-3468);
    deck.deal_with_increment(72);
    deck.cut(5364);
    deck.deal_with_increment(2);
    deck.cut(-5653);
    deck.deal_with_increment(21);
    deck.deal_into_new_stack();
    deck.cut(-5301);
    deck.deal_with_increment(10);
    deck.cut(-4613);
    deck.deal_with_increment(55);
    deck.cut(-8840);
    deck.deal_with_increment(17);
    deck.deal_into_new_stack();
    deck.deal_with_increment(42);
    deck.cut(-8746);
    deck.deal_with_increment(18);
    deck.deal_into_new_stack();
    deck.cut(-8040);
    deck.deal_into_new_stack();
    deck.deal_with_increment(66);
    deck.cut(-4990);
    deck.deal_with_increment(71);
    deck.cut(8032);
    deck.deal_with_increment(3);
    deck.cut(-6735);
    deck.deal_with_increment(25);
    deck.deal_into_new_stack();
    deck.deal_with_increment(65);

    println!("Part 2: {}", deck.iterations(101741582076661));
}

struct SpaceDeck(Vec<u32>);

impl SpaceDeck {
    fn new(size: u32) -> SpaceDeck {
        SpaceDeck((0..size).collect())
    }

    fn deal_into_new_stack(self) -> SpaceDeck {
        let mut new_deck = Vec::with_capacity(self.0.len());
        for &c in self.0.iter().rev() {
            new_deck.push(c);
        }
        SpaceDeck(new_deck)
    }

    fn cut(self, n: i32) -> SpaceDeck {
        let skip = if n < 0 {
            self.0.len() - n.abs() as usize
        } else {
            n as usize
        };
        SpaceDeck(
            self.0
                .iter()
                .cycle()
                .skip(skip as usize)
                .take(self.0.len())
                .cloned()
                .collect(),
        )
    }

    fn deal_with_increment(self, n: usize) -> SpaceDeck {
        let mut new_deck = vec![0; self.0.len()];
        for (i, &c) in self.0.iter().enumerate() {
            new_deck[i * n % self.0.len()] = c;
        }
        SpaceDeck(new_deck)
    }
}

struct SpaceDeck2 {
    size: usize,
    value: i64,
    a: i64,
    b: i64,
}

impl SpaceDeck2 {
    fn new(size: usize, position: usize) -> SpaceDeck2 {
        SpaceDeck2 {
            size,
            value: position as i64,
            a: 0,
            b: 1,
        }
    }

    fn deal_into_new_stack(&mut self) {
        self.a = (self.size as i64 - 1 - self.a).mod_floor(&(self.size as i64));
        self.b = (self.b * -1).mod_floor(&(self.size as i64));
    }

    fn cut(&mut self, n: i64) {
        self.a = (self.a + n).mod_floor(&(self.size as i64));
    }

    fn deal_with_increment(&mut self, n: i64) {
        let inverse = extended_gcd(n, self.size as i64);
        self.a = (self.a as i128 * inverse as i128).mod_floor(&(self.size as i128)) as i64;
        self.b = (self.b as i128 * inverse as i128).mod_floor(&(self.size as i128)) as i64;
    }

    fn iterations(&self, n: usize) -> i128 {
        //a * (1 - b^n)/(1 - b) + p*b^n
        //a*inv - a*inv*b^n + p*b^n
        let inverse = extended_gcd((1 - self.b).mod_floor(&(self.size as i64)), self.size as i64) as i128;
        let term1 = (self.a as i128 * inverse).mod_floor(&(self.size as i128));

        let big_exponent = modular_pow(self.b as i128, n as i128, self.size as i128);
        let term2 = ((self.a as i128 * inverse).mod_floor(&(self.size as i128)) * big_exponent).mod_floor(&(self.size as i128));

        let term3 = (self.value as i128 * big_exponent).mod_floor(&(self.size as i128));

        let sum = (term1 - term2 + term3).mod_floor(&(self.size as i128));
        sum
    }
}

fn extended_gcd(a: i64, b: i64) -> i64 {
    let mut old_r = a;
    let mut r = b;
    let mut old_s = 1;
    let mut s = 0;

    while r != 0 {
        let quotient = old_r / r;
        let tmp = old_r;
        old_r = r;
        r = tmp - quotient * r;

        let tmp = old_s;
        old_s = s;
        s = tmp - quotient * s;
    }
    return old_s;
}

fn modular_pow(mut base: i128, mut exponent: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    base = base.mod_floor(&modulus);
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base).mod_floor(&(modulus));
        }
        exponent = exponent >> 1;
        base = (base * base).mod_floor(&(modulus));
    }
    return result;
}

#[cfg(test)]
mod test {
    use crate::prob22::{extended_gcd, modular_pow, SpaceDeck, SpaceDeck2};

    #[test]
    fn test_extended_gcd() {
        assert_eq!(extended_gcd(7, 10), 3);
    }

    #[test]
    fn test_power_modulo() {
        assert_eq!(modular_pow(2, 5, 10), 2);
        assert_eq!(
            modular_pow(49620702840396, 101741582076661, 119315717514047),
            89238618579438
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(
            SpaceDeck::new(10)
                .deal_with_increment(7)
                .deal_into_new_stack()
                .deal_into_new_stack()
                .0,
            vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            SpaceDeck::new(10)
                .cut(6)
                .deal_with_increment(7)
                .deal_into_new_stack()
                .0,
            vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            SpaceDeck::new(10)
                .deal_with_increment(7)
                .deal_with_increment(9)
                .cut(-2)
                .0,
            vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]
        )
    }

    #[test]
    fn part_2_example_1() {
        let mut deck = SpaceDeck::new(10);

        for _ in 0..10 {
            deck = deck.cut(6).deal_with_increment(7).deal_into_new_stack();
            println!("{:?}", deck.0);
        }

        //10 = 4          => 1/2.5
        //100 = 20        => 1/5
        //1000 = 100
        //10000 = 500
        //100000 = 2500
        println!("========================");
        let mut deck = SpaceDeck::new(1021);
        deck = deck.deal_with_increment(7).deal_with_increment(9).cut(-2);
        let first_round = deck.0.clone();
        let mut iterations = 0;
        loop {
            deck = deck.deal_with_increment(7).deal_with_increment(9).cut(-2);
            iterations += 1;
            if deck.0 == first_round {
                break;
            }
        }
        println!("Iterations: {}", iterations);

        // let round2 = round1.cut(6).deal_with_increment(7).deal_into_new_stack();
        // println!("{:?}")
        // let result =
        //     .0;
        //
        // println!("{:?}", result);
        // let initial_position = 4;
        // let mut position = initial_position as u32;
        // position = 10 - 1 - position;
        // println!("{}", position);
        //
        // position = (position * 3) % 10;
        // println!("{}", position);
        //
        // position = (position + 6) % 10;
        // println!("{}", position);
        //
        // position = (10 - 1 - position) % 10;
        //
        // position = (position * 3) %10;
        //
        // position = (position + 6) % 10;
        //
        // println!("{}", position);
        // assert_eq!(position, result[initial_position]);

        //p = 10 - 1 - p mod 10
        //p = p * 3 mod 10
        //p = p + 6 mod 10

        //p1 = ((10 - 1 - p0)*3) + 6 mod 10
        //p1 = (9 - p0)*3 + 6 = 3 - 3p0
        //p2 = 3 - 3*(3 - 3p0) = 3 - (9 - 9p0) = 3 - 9 + 9p0 = 4 + 9p0
        //p2 = (3 - ((10 - 1 - p0)*3))*3 + 6 mod 10
        let mut sum: u64 = 0;
        for i in 0..(101741582076661 as u64) {
            sum += 1;
            if i % 100000 == 0 {
                println!("Iteration: {}, Partial Sum: {}", i, sum);
            }
        }
        println!("{}", sum);
    }

    #[test]
    fn solve_part_2() {
        //p1 = ((10 - 1 - p0)*3) + 6 mod 10
        //p1 = (9 - p0)*3 + 6 = 3 - 3p0
        //p2 = 3 - 3*(3 - 3p0) = 3 - (9 - 9p0) = 3 - 9 + 9p0 = 4 + 9p0

        let deck = SpaceDeck::new(10)
            .cut(6)
            .deal_with_increment(7)
            .deal_into_new_stack()
            .cut(6)
            .deal_with_increment(7)
            .deal_into_new_stack();
        assert_eq!(deck.0[3], (4 + 9 * 3) % 10);

        //p3 = 3 - 3*(4 + 9p0) = 3 - (2 + 7p0) = 1 - 7p0
        assert_eq!(
            deck.cut(6).deal_with_increment(7).deal_into_new_stack().0[3],
            ((1 as i32 - 7 * 3) % 10) as u32
        );

        //p4 = 3 - 3*(1 - 7p0) = 3 - (3 - p0) = 0 + p0
    }

    #[test]
    fn test_space_deck_2() {
        let mut deck = SpaceDeck2::new(10, 3);
        deck.deal_into_new_stack();
        deck.deal_with_increment(7);
        deck.cut(6);
        //
        assert_eq!(
            deck.iterations(3),
            SpaceDeck::new(10)
                .cut(6)
                .deal_with_increment(7)
                .deal_into_new_stack()
                .cut(6)
                .deal_with_increment(7)
                .deal_into_new_stack()
                .cut(6)
                .deal_with_increment(7)
                .deal_into_new_stack()
                .0[3] as i128
        );
        // assert_eq!(
        //     SpaceDeck2::new(10, 3)
        //         .cut(-1)
        //         .deal_with_increment(3)
        //         .deal_with_increment(9)
        //         .cut(3)
        //         .deal_with_increment(7)
        //         .cut(-4)
        //         .cut(8)
        //         .deal_with_increment(7)
        //         .cut(-2)
        //         .deal_into_new_stack()
        //         .value,
        //     SpaceDeck::new(10)
        //         .deal_into_new_stack()
        //         .cut(-2)
        //         .deal_with_increment(7)
        //         .cut(8)
        //         .cut(-4)
        //         .deal_with_increment(7)
        //         .cut(3)
        //         .deal_with_increment(9)
        //         .deal_with_increment(3)
        //         .cut(-1)
        //         .0[3] as i64
        // );
    }

    use crate::num_integer::Integer;
    #[test]
    fn this_is_it() {
        //p1 = a + bp
        //p2 = a + b*(a + bp) = a + (ab + bbp) = a + ab + bbp
        //p3 = a + b*(a + ab + bbp) = a + ab + abb + bbbp
        //pn = a + ab + abb + ... + ab^(n-1) + pb^n = a*(1 - b^n)/(1 - b) + pb^n

        //pn = a + ab + abb + ... + ab^n + b^(n+1)p
        let mut deck = SpaceDeck2::new(119315717514047, 2020);
        deck.cut(4043);
        deck.deal_with_increment(25);
        deck.deal_into_new_stack();
        deck.cut(1831);
        deck.deal_with_increment(61);
        deck.deal_into_new_stack();
        deck.deal_with_increment(30);
        deck.cut(5175);
        deck.deal_into_new_stack();
        deck.cut(7094);
        deck.deal_with_increment(16);
        deck.cut(305);
        deck.deal_with_increment(28);
        deck.deal_into_new_stack();
        deck.cut(-4702);
        deck.deal_with_increment(52);
        deck.cut(6861);
        deck.deal_with_increment(35);
        deck.cut(-440);
        deck.deal_with_increment(75);
        deck.cut(-9511);
        deck.deal_with_increment(45);
        deck.cut(-6751);
        deck.deal_with_increment(29);
        deck.cut(-716);
        deck.deal_with_increment(48);
        deck.cut(-2242);
        deck.deal_with_increment(20);
        deck.deal_into_new_stack();
        deck.deal_with_increment(4);
        deck.cut(-2423);
        deck.deal_with_increment(44);
        deck.cut(2884);
        deck.deal_with_increment(71);
        deck.cut(1554);
        deck.deal_into_new_stack();
        deck.cut(-1815);
        deck.deal_with_increment(56);
        deck.cut(-4547);
        deck.deal_with_increment(41);
        deck.cut(362);
        deck.deal_into_new_stack();
        deck.cut(-8169);
        deck.deal_with_increment(57);
        deck.cut(-2995);
        deck.deal_into_new_stack();
        deck.cut(3214);
        deck.deal_with_increment(51);
        deck.cut(-9602);
        deck.deal_into_new_stack();
        deck.deal_with_increment(2);
        deck.cut(8080);
        deck.deal_with_increment(13);
        deck.cut(2364);
        deck.deal_into_new_stack();
        deck.deal_with_increment(42);
        deck.cut(-9366);
        deck.deal_with_increment(43);
        deck.cut(-5699);
        deck.deal_into_new_stack();
        deck.deal_with_increment(64);
        deck.cut(5642);
        deck.deal_with_increment(6);
        deck.cut(-1700);
        deck.deal_with_increment(66);
        deck.cut(2935);
        deck.deal_with_increment(43);
        deck.cut(6794);
        deck.deal_with_increment(63);
        deck.cut(-9661);
        deck.deal_into_new_stack();
        deck.cut(-3468);
        deck.deal_with_increment(72);
        deck.cut(5364);
        deck.deal_with_increment(2);
        deck.cut(-5653);
        deck.deal_with_increment(21);
        deck.deal_into_new_stack();
        deck.cut(-5301);
        deck.deal_with_increment(10);
        deck.cut(-4613);
        deck.deal_with_increment(55);
        deck.cut(-8840);
        deck.deal_with_increment(17);
        deck.deal_into_new_stack();
        deck.deal_with_increment(42);
        deck.cut(-8746);
        deck.deal_with_increment(18);
        deck.deal_into_new_stack();
        deck.cut(-8040);
        deck.deal_into_new_stack();
        deck.deal_with_increment(66);
        deck.cut(-4990);
        deck.deal_with_increment(71);
        deck.cut(8032);
        deck.deal_with_increment(3);
        deck.cut(-6735);
        deck.deal_with_increment(25);
        deck.deal_into_new_stack();
        deck.deal_with_increment(65);

    }

    #[test]
    fn test_period() {
        let b = 49620702840396_i128;
        let size = 119315717514047_i128;

        let mut product = (b * b).mod_floor(&(size));
        let mut iterations = 1;
        while product != b {
            product = (product * b).mod_floor(&(size));
            iterations += 1;
            if iterations % 1000 == 0 {
                println!("{}", iterations);
            }
        }

        println!("{}", iterations);
    }

    // #[test]
    // fn test_cut_n_cards() {
    //     let deck = SpaceDeck::new(10);
    //
    //     assert_eq!(deck.cut(3).take(10).collect::<Vec<_>>(), vec![3,4,5,6,7,8,9,0,1,2])
    // }
}
