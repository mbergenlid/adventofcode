#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Vector(i32, i32, i32);

impl Vector {
    fn energy(&self) -> u32 {
        self.0.abs() as u32 + self.1.abs() as u32 + self.2.abs() as u32
    }
}

impl std::ops::Add for Vector {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Add for &Vector {
    type Output = Vector;
    fn add(self, other: Self) -> Self::Output {
        Vector(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += self.1;
        self.2 += other.2;
    }
}

//impl std::ops::AddAssign for &Vector {
//    fn add_assign(&mut self, other: Self) {
//        self.0 = self.0 + other.0;
//        self.1 += self.1;
//        self.2 += other.2;
//    }
//}

type Position = Vector;
type Velocity = Vector;

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Moon {
    position: Position,
    velocity: Velocity,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            position: Vector(x, y, z),
            velocity: Vector(0, 0, 0),
        }
    }
}

pub fn solve_part_1() {
    let mut moons = [
        Moon::new(14, 2, 8),
        Moon::new(7, 4, 10),
        Moon::new(1, 17, 16),
        Moon::new(-4, -1, 1),
    ];
    for _i in 0..1000 {
        step(&mut moons);
    }
    println!("Part 1: {}", total_energy(&moons));
}

pub fn solve_part_2() {
    let cycle = solve_for_part_2(
        Moon::new(14, 2, 8),
        Moon::new(7, 4, 10),
        Moon::new(1, 17, 16),
        Moon::new(-4, -1, 1),
    );
    println!("Part 2: {}", cycle);
}

fn solve_for_part_2(moon1: Moon, moon2: Moon, moon3: Moon, moon4: Moon) -> u64 {
    let mut moons = [moon1.clone(), moon2.clone(), moon3.clone(), moon4.clone()];
    step(&mut moons);
    let mut steps = 1 as u64;
    while moons[0].position.2 != moon1.position.2
        || moons[1].position.2 != moon2.position.2
        || moons[2].position.2 != moon3.position.2
        || moons[3].position.2 != moon4.position.2
        || moons[0].velocity.2 != 0
        || moons[1].velocity.2 != 0
        || moons[2].velocity.2 != 0
        || moons[3].velocity.2 != 0
    {
        step(&mut moons);
        steps += 1;
    }
    let z_cycle = steps;

    let mut moons = [moon1.clone(), moon2.clone(), moon3.clone(), moon4.clone()];
    step(&mut moons);
    let mut steps = 1 as u64;
    while moons[0].position.1 != moon1.position.1
        || moons[1].position.1 != moon2.position.1
        || moons[2].position.1 != moon3.position.1
        || moons[3].position.1 != moon4.position.1
        || moons[0].velocity.1 != 0
        || moons[1].velocity.1 != 0
        || moons[2].velocity.1 != 0
        || moons[3].velocity.1 != 0
    {
        step(&mut moons);
        steps += 1;
    }
    let y_cycle = steps;

    let mut moons = [moon1.clone(), moon2.clone(), moon3.clone(), moon4.clone()];
    step(&mut moons);
    let mut steps = 1 as u64;
    while moons[0].position.0 != moon1.position.0
        || moons[1].position.0 != moon2.position.0
        || moons[2].position.0 != moon3.position.0
        || moons[3].position.0 != moon4.position.0
        || moons[0].velocity.0 != 0
        || moons[1].velocity.0 != 0
        || moons[2].velocity.0 != 0
        || moons[3].velocity.0 != 0
    {
        step(&mut moons);
        steps += 1;
    }
    let x_cycle = steps;

    //solve x_cycle*a = y_cycle*b
    let mut x_y_cycle = 0;
    for a in 1..y_cycle {
        if a * x_cycle % y_cycle == 0 {
            x_y_cycle = x_cycle * a;
            break;
        }
    }

    //solve x_y_cycle*a = z_cycle*b
    let mut cycle = 0;
    for a in 1..z_cycle {
        if a * x_y_cycle % z_cycle == 0 {
            cycle = a * x_y_cycle;
        }
    }
    println!("{} {} {}", x_cycle, y_cycle, z_cycle);
    println!("{}", x_y_cycle);
    return cycle;
}

fn step(moons: &mut [Moon]) {
    let mut gravity = vec![Vector(0, 0, 0); moons.len()];
    for (i1, m1) in moons.iter().enumerate() {
        for (i2, m2) in moons.iter().enumerate().skip(i1 + 1) {
            if m1.position.0 < m2.position.0 {
                gravity.get_mut(i1).unwrap().0 += 1;
                gravity.get_mut(i2).unwrap().0 -= 1;
            } else if m1.position.0 > m2.position.0 {
                gravity.get_mut(i1).unwrap().0 -= 1;
                gravity.get_mut(i2).unwrap().0 += 1;
            }

            if m1.position.1 < m2.position.1 {
                gravity.get_mut(i1).unwrap().1 += 1;
                gravity.get_mut(i2).unwrap().1 -= 1;
            } else if m1.position.1 > m2.position.1 {
                gravity.get_mut(i1).unwrap().1 -= 1;
                gravity.get_mut(i2).unwrap().1 += 1;
            }

            if m1.position.2 < m2.position.2 {
                gravity.get_mut(i1).unwrap().2 += 1;
                gravity.get_mut(i2).unwrap().2 -= 1;
            } else if m1.position.2 > m2.position.2 {
                gravity.get_mut(i1).unwrap().2 -= 1;
                gravity.get_mut(i2).unwrap().2 += 1;
            }
        }
    }

    //println!("{:?}", gravity);
    for (i, g) in gravity.into_iter().enumerate() {
        let mut moon = moons.get_mut(i).unwrap();
        let velocity = &moon.velocity + &g;
        let position = &moon.position + &velocity;
        moon.velocity = velocity;
        moon.position = position;
    }
}

fn total_energy(moons: &[Moon]) -> u32 {
    moons
        .iter()
        .map(|m| m.position.energy() * m.velocity.energy())
        .sum()
}

#[cfg(test)]
mod test {
    use super::{Moon, Vector};
    #[test]
    fn test_step() {
        let mut moons = [
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];
        super::step(&mut moons);
        assert_eq!(
            moons,
            [
                Moon {
                    position: Vector(2, -1, 1),
                    velocity: Vector(3, -1, -1)
                },
                Moon {
                    position: Vector(3, -7, -4),
                    velocity: Vector(1, 3, 3)
                },
                Moon {
                    position: Vector(1, -7, 5),
                    velocity: Vector(-3, 1, -3)
                },
                Moon {
                    position: Vector(2, 2, 0),
                    velocity: Vector(-1, -3, 1)
                },
            ]
        );
        super::step(&mut moons);
        assert_eq!(
            moons,
            [
                Moon {
                    position: Vector(5, -3, -1),
                    velocity: Vector(3, -2, -2)
                },
                Moon {
                    position: Vector(1, -2, 2),
                    velocity: Vector(-2, 5, 6)
                },
                Moon {
                    position: Vector(1, -4, -1),
                    velocity: Vector(0, 3, -6)
                },
                Moon {
                    position: Vector(1, -4, 2),
                    velocity: Vector(-1, -6, 2)
                },
            ]
        );
    }

    #[test]
    fn test_energy() {
        let mut moons = [
            Moon::new(-1, 0, 2),
            Moon::new(2, -10, -7),
            Moon::new(4, -8, 8),
            Moon::new(3, 5, -1),
        ];
        for _i in 0..10 {
            super::step(&mut moons);
            println!("{:?}", moons);
        }
        assert_eq!(super::total_energy(&moons), 179,);
    }

    #[test]
    fn test_energy_2() {
        let mut moons = [
            Moon::new(-8, -10, 0),
            Moon::new(5, 5, 10),
            Moon::new(2, -7, 3),
            Moon::new(9, -8, -3),
        ];
        for _i in 0..100 {
            super::step(&mut moons);
        }
        assert_eq!(super::total_energy(&moons), 1940);
    }

    #[test]
    fn test_a_lot_2() {
        assert_eq!(
            super::solve_for_part_2(
                Moon::new(-8, -10, 0),
                Moon::new(5, 5, 10),
                Moon::new(2, -7, 3),
                Moon::new(9, -8, -3)
            ),
            4686774924
        );
    }

}
