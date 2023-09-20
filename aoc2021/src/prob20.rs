use std::{
    convert::{TryFrom, TryInto},
    fmt::Debug,
    str::FromStr,
    vec,
};

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    enhance_image(input, 2)
}

pub fn solve_part_2(input: &str) -> usize {
    enhance_image(input, 50)
}

fn enhance_image(input: &str, times: usize) -> usize {
    let (enhancement, image) = input.split("\n\n").collect_tuple().unwrap();

    let enhancement = enhancement.parse::<EnhancementAlgorithm>().unwrap();
    let mut image = image.parse::<Image>().unwrap();

    for _ in 0..times {
        image = image.enhance(&enhancement);
    }
    image
        .data
        .iter()
        .map(|row| row.iter().filter(|&c| *c == Pixel::Light).count())
        .sum()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Pixel {
    Light,
    Dark,
}

impl TryFrom<char> for Pixel {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Pixel::Light),
            '.' => Ok(Pixel::Dark),
            p => Err(format!("Invalid pixel {}", p)),
        }
    }
}

struct EnhancementAlgorithm(Vec<Pixel>);

impl FromStr for EnhancementAlgorithm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EnhancementAlgorithm(
            s.chars().map(|c| c.try_into().unwrap()).collect(),
        ))
    }
}

struct Image {
    data: Vec<Vec<Pixel>>,
    top_left: (isize, isize),
    surrounding: Pixel,
}

impl FromStr for Image {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for pixel in line.chars() {
                row.push(pixel.try_into()?);
            }
            data.push(row);
        }
        Ok(Image {
            data,
            surrounding: Pixel::Dark,
            top_left: (10, 10),
        })
    }
}

impl Image {
    fn enhance(self, enhancement: &EnhancementAlgorithm) -> Self {
        let surrounding = match self.surrounding {
            Pixel::Dark => enhancement.0[0],
            Pixel::Light => *enhancement.0.last().unwrap(),
        };

        let new_data: Vec<Vec<Pixel>> =
            vec![vec![surrounding; self.data[0].len() + 2]; self.data.len() + 2];

        let mut new_image = Image {
            data: new_data,
            surrounding,
            top_left: (self.top_left.0 - 1, self.top_left.1 - 1),
        };

        let (start_row, start_col) = new_image.top_left;
        for abs_row in start_row..(start_row + (new_image.data.len() as isize)) {
            for abs_col in start_col..(start_col + (new_image.data[0].len() as isize)) {
                let mut enhance_index = 0;
                for s_row in (abs_row - 1)..=(abs_row + 1) {
                    for s_col in (abs_col - 1)..=(abs_col + 1) {
                        let bit = match self.pixel(s_row, s_col) {
                            Pixel::Dark => 0,
                            Pixel::Light => 1,
                        };
                        enhance_index = (enhance_index << 1) | bit;
                    }
                }
                *new_image.pixel_mut(abs_row, abs_col) = enhancement.0[enhance_index];
            }
        }

        new_image
    }

    #[inline]
    fn pixel(&self, row: isize, col: isize) -> Pixel {
        if row < self.top_left.0 || ((row - self.top_left.0) as usize) >= self.data.len() {
            return self.surrounding;
        }
        if col < self.top_left.1 || ((col - self.top_left.1) as usize) >= self.data[0].len() {
            return self.surrounding;
        }
        self.data[(row - self.top_left.0) as usize][(col - self.top_left.1) as usize]
    }

    #[inline]
    fn pixel_mut(&mut self, row: isize, col: isize) -> &mut Pixel {
        if row < self.top_left.0 || ((row - self.top_left.0) as usize) >= self.data.len() {
            panic!("Index out of bounds: ({}, {})", row, col);
        }
        if col < self.top_left.1 || ((col - self.top_left.1) as usize) >= self.data[0].len() {
            panic!("Index out of bounds: ({}, {})", row, col);
        }
        &mut self.data[(row - self.top_left.0) as usize][(col - self.top_left.1) as usize]
    }

    //fn to_relative(&self, row: isize, col: is)
}

impl Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for pixel in row.iter() {
                match pixel {
                    Pixel::Light => write!(f, "#")?,
                    Pixel::Dark => write!(f, ".")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 35);
    }

    #[test]
    fn test_2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 3351);
    }

    const TESTCASE: &'static str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";
}
