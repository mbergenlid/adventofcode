use std::str::FromStr;

pub fn solve_part_1(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        let game = line.parse::<Game>().unwrap();

        if game
            .drawn_cubes
            .iter()
            .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
        {
            result += game.id;
        }
    }
    result
}

pub fn solve_part_2(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        let game = line.parse::<Game>().unwrap();

        let mut min_red = 0;
        let mut min_blue = 0;
        let mut min_green = 0;

        for cube_set in game.drawn_cubes {
            min_red = min_red.max(cube_set.red);
            min_blue = min_blue.max(cube_set.blue);
            min_green = min_green.max(cube_set.green);
        }

        result += min_red * min_blue * min_green;
    }
    result
}

#[derive(PartialEq, Eq, Debug)]
struct Game {
    id: usize,
    drawn_cubes: Vec<CubeSet>,
}

#[derive(PartialEq, Eq, Debug)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon = s.find(":").ok_or("No colon found".to_string())?;
        let id = s["Game ".len()..colon]
            .parse::<usize>()
            .map_err(|e| format!("{:?}", e))?;

        let mut drawn_cubes = Vec::new();
        for cube_set in s[(colon + 1)..].split(";") {
            drawn_cubes.push(cube_set.parse::<CubeSet>()?);
        }
        Ok(Game { id, drawn_cubes })
    }
}

impl FromStr for CubeSet {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for x in s.split(",") {
            if x.ends_with(" red") {
                red = x[1..(x.len() - " red".len())]
                    .parse()
                    .map_err(|e| format!("{:?}: '{}'", e, &x[1..(x.len() - " red".len())]))?
            } else if x.ends_with(" blue") {
                blue = x[1..(x.len() - " blue".len())]
                    .parse()
                    .map_err(|e| format!("{:?}: '{}'", e, &x[1..(x.len() - " blue".len())]))?
            } else if x.ends_with(" green") {
                green = x[1..(x.len() - " green".len())]
                    .parse()
                    .map_err(|e| format!("{:?}: '{}'", e, &x[1..(x.len() - " green".len())]))?
            }
        }
        Ok(CubeSet { red, green, blue })
    }
}

#[cfg(test)]
mod test {
    use crate::prob2::{CubeSet, Game};

    #[test]
    fn parse() {
        assert_eq!(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
                .parse::<Game>()
                .unwrap(),
            Game {
                id: 1,
                drawn_cubes: vec![
                    CubeSet {
                        red: 4,
                        blue: 3,
                        green: 0
                    },
                    CubeSet {
                        red: 1,
                        blue: 6,
                        green: 2,
                    },
                    CubeSet {
                        red: 0,
                        green: 2,
                        blue: 0
                    },
                ]
            }
        )
    }

    #[test]
    fn part1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 8);
    }

    #[test]
    fn part2() {
        assert_eq!(super::solve_part_2(TEST_INPUT), 2286);
    }

    const TEST_INPUT: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
}
