
pub fn solve_part_1() -> u32 {
    let reindeers = input();
    reindeers
        .iter()
        .map(|r| r.distance_after(2503))
        .max()
        .expect("No reindeers?")
}

pub fn solve_part_2() -> u32 {
    let reindeers = input();
    compete_part_2(reindeers, 2503)
}

fn compete_part_2(reindeers: Vec<Reindeer>, seconds: u32) -> u32 {
    let mut scores = vec![0; reindeers.len()];
    for second in 1..(seconds + 1) {
        let distances: Vec<_> = reindeers.iter().map(|r| r.distance_after(second)).collect();
        let max_distance = distances.iter().max().unwrap();

        for (i, _) in distances
            .iter()
            .enumerate()
            .filter(|(_, d)| *d == max_distance)
        {
            scores[i] = scores[i] + 1;
        }
    }
    assert!(scores.iter().sum::<u32>() >= seconds);
    scores.into_iter().max().unwrap()
}

struct Reindeer {
    speed: u32,
    duration: u32,
    resting: u32,
}

impl Reindeer {
    fn distance_after(&self, seconds: u32) -> u32 {
        let periods = seconds / (self.duration + self.resting);
        let rest = seconds % (self.duration + self.resting);
        periods * (self.speed * self.duration) + self.speed * std::cmp::min(self.duration, rest)
    }
}

#[cfg(test)]
mod test {
    use crate::prob14::{compete_part_2, solve_part_2, Reindeer};

    #[test]
    fn test_for_part_1() {
        let comet = Reindeer {
            speed: 14,
            duration: 10,
            resting: 127,
        };
        assert_eq!(comet.distance_after(1000), 1120);
        let dancer = Reindeer {
            speed: 16,
            duration: 11,
            resting: 162,
        };
        assert_eq!(dancer.distance_after(1000), 1056);

        assert_eq!(comet.distance_after(10), 14 * 10);
        assert_eq!(comet.distance_after(6), 14 * 6);
        assert_eq!(comet.distance_after(138), 14 * 10 + 14);
    }

    #[test]
    fn test_compete() {
        assert_eq!(
            compete_part_2(
                vec![
                    Reindeer {
                        speed: 14,
                        duration: 10,
                        resting: 127,
                    },
                    Reindeer {
                        speed: 16,
                        duration: 11,
                        resting: 162
                    }
                ],
                1000
            ),
            689
        );
    }

    #[test]
    fn test_part_2() {
        println!("{}", solve_part_2());
    }
}

fn input() -> Vec<Reindeer> {
    vec![
        Reindeer {
            speed: 19,
            duration: 7,
            resting: 124,
        },
        Reindeer {
            speed: 3,
            duration: 15,
            resting: 28,
        },
        Reindeer {
            speed: 19,
            duration: 9,
            resting: 164,
        },
        Reindeer {
            speed: 19,
            duration: 9,
            resting: 158,
        },
        Reindeer {
            speed: 13,
            duration: 7,
            resting: 82,
        },
        Reindeer {
            speed: 25,
            duration: 6,
            resting: 145,
        },
        Reindeer {
            speed: 14,
            duration: 3,
            resting: 38,
        },
        Reindeer {
            speed: 3,
            duration: 16,
            resting: 37,
        },
        Reindeer {
            speed: 25,
            duration: 6,
            resting: 143,
        },
    ]
}
