use std::collections::{HashMap, VecDeque};

use aoc_lib::grid::{Grid, Point, Pos};
use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let (grid_str, steps_str) = input.split("\n\n").collect_tuple::<(_, _)>().unwrap();
    let robot = Robot::new(grid_str.parse::<Grid<char>>().unwrap());

    solve(robot, steps_str)
}

pub fn solve_part_2(input: &str) -> usize {
    let (grid_str, steps_str) = input.split("\n\n").collect_tuple::<(_, _)>().unwrap();
    let grid_str = grid_str
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| match c {
                    '.' => "..".chars(),
                    'O' => "[]".chars(),
                    '#' => "##".chars(),
                    '@' => "@.".chars(),
                    _ => panic!("Unexpected char {}", c),
                })
                .collect::<String>()
        })
        .join("\n");
    let robot = Robot::new(grid_str.parse::<Grid<char>>().unwrap());
    solve(robot, steps_str)
}

fn solve(mut robot: Robot, steps: &str) -> usize {
    println!("{:?}", robot.grid);

    for step in steps.trim().chars() {
        robot.step(step);
    }
    println!("{:?}", robot.grid);

    let mut result = 0;
    for b in robot
        .grid
        .iter()
        .filter(|p| p.value == '[' || p.value == 'O')
    {
        result += b.pos.row() * 100 + b.pos.col();
    }

    result
}

struct Robot {
    grid: Grid<char>,
    current_pos: Pos,
}

impl Robot {
    fn new(grid: Grid<char>) -> Self {
        let current_pos = grid
            .iter()
            .find(|p| p.value == '@')
            .unwrap_or_else(|| panic!("No robot found"))
            .pos;
        Self { grid, current_pos }
    }
    fn step(&mut self, step: char) {
        let mut iter = match step {
            '^' => self.grid.up(self.current_pos),
            '>' => self.grid.right(self.current_pos),
            'v' => self.grid.down(self.current_pos),
            '<' => self.grid.left(self.current_pos),
            _ => return,
        };
        iter.next();

        if let Some(next) = iter.next() {
            if next.value == '#' {
                return;
            }

            if next.value == '.' {
                self.grid.insert(next.pos, '@');
                self.grid.insert(self.current_pos, '.');
                self.current_pos = next.pos;
                return;
            }

            if next.value == 'O' {
                let first_box = next.pos;
                for p in iter {
                    match p.value {
                        '#' => break,
                        '.' => {
                            self.grid.insert(first_box, '@');
                            self.grid.insert(self.current_pos, '.');
                            self.grid.insert(p.pos, 'O');
                            self.current_pos = first_box;
                            return;
                        }
                        _ => {}
                    }
                }
            } else if next.value == ']' || next.value == '[' {
                if step == '<' || step == '>' {
                    let mut stack = VecDeque::new();
                    let first_box = next.pos;
                    stack.push_back(next);
                    for p in iter {
                        match p.value {
                            '#' => break,
                            '.' => {
                                let mut current = p.pos;
                                while let Some(prev) = stack.pop_back() {
                                    self.grid.insert(current, prev.value);
                                    current = prev.pos;
                                }
                                self.grid.insert(self.current_pos, '.');
                                self.grid.insert(first_box, '@');
                                self.current_pos = first_box;
                                return;
                            }
                            _ => stack.push_back(p),
                        }
                    }
                } else if step == '^' || step == 'v' {
                    let mut boxes = VecDeque::new();
                    let first_box = next.pos;
                    boxes.push_back(next);
                    let mut boxes_to_move = HashMap::<Pos, char>::new();

                    while let Some(b) = boxes.pop_front() {
                        //Check neighbour...
                        if b.value == ']' {
                            let pos = b.pos.left();
                            if !boxes_to_move.contains_key(&pos) {
                                boxes.push_back(Point {
                                    value: *self.grid.get(pos).expect("Must be"),
                                    pos,
                                })
                            }
                        } else if b.value == '[' {
                            let pos = b.pos.right();
                            if !boxes_to_move.contains_key(&pos) {
                                boxes.push_back(Point {
                                    value: *self.grid.get(pos).expect("Must be"),
                                    pos,
                                })
                            }
                        }
                        //Add up/down box
                        let up_down_pos = if step == '^' {
                            b.pos.up()
                        } else {
                            b.pos.down()
                        };
                        let up_down_val = *self.grid.get(up_down_pos).expect("Must be");
                        match up_down_val {
                            '.' => {}
                            '[' | ']' => boxes.push_back(Point {
                                value: up_down_val,
                                pos: up_down_pos,
                            }),
                            _ => return, //Hit a wall, give up
                        }
                        boxes_to_move.insert(b.pos, b.value);
                    }

                    if step == '^' {
                        self.move_all(
                            boxes_to_move.iter().sorted_by_key(|(pos, _)| pos.row()),
                            Pos::up,
                        );
                    } else {
                        self.move_all(
                            boxes_to_move.iter().sorted_by_key(|(pos, _)| pos.row()).rev(),
                            Pos::down,
                        );
                    }
                    self.grid.insert(self.current_pos, '.');
                    self.grid.insert(first_box, '@');
                    self.current_pos = first_box;
                }
            }
        }
    }

    fn move_all<'a, I, F>(&mut self, iter: I, move_fn: F)
    where
        I: Iterator<Item = (&'a Pos, &'a char)>,
        F: Fn(Pos) -> Pos,
    {
        for (&pos, &value) in iter {
            let new_pos = (move_fn)(pos);
            self.grid.insert(new_pos, value);
            self.grid.insert(pos, '.');
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT_1), 2028);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve_part_2(INPUT_LARGE), 9021);
    }

    const INPUT_1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    const INPUT_LARGE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
}
