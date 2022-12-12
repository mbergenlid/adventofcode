use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

struct Grid {
    data: Vec<u8>,
    width: usize,
    start_pos: usize,
    end_pos: usize,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::new();
        let width = s.lines().next().unwrap().len();
        let mut start_pos = 0;
        let mut end_pos = 0;
        for (i, c) in s.chars().filter(|&c| c != '\n').enumerate() {
            match c {
                'S' => {
                    data.push(0);
                    start_pos = i;
                }
                'E' => {
                    data.push(('z' as u8) - ('a' as u8));
                    end_pos = i;
                }
                _ => data.push((c as u8) - ('a' as u8)),
            }
        }
        Ok(Grid {
            data,
            width,
            start_pos,
            end_pos,
        })
    }
}

impl Grid {
    fn find_shortest_path_to_goal(&self) -> usize {
        let end_pos = (self.end_pos % self.width, self.end_pos / self.width);
        self.find_shortest_path(
            (self.start_pos % self.width, self.start_pos / self.width),
            |pos, _| pos == end_pos,
            |neighbour_height, current_height| neighbour_height <= current_height + 1,
        )
    }

    fn find_shortest_path_to_start(&self) -> usize {
        self.find_shortest_path(
            (self.end_pos % self.width, self.end_pos / self.width),
            |_, height| height == 0,
            |neighbour_height, current_height| neighbour_height >= current_height - 1,
        )
    }

    fn find_shortest_path<E, V: Fn(u8, u8) -> bool>(
        &self,
        from: (usize, usize),
        end_func: E,
        valid_move: V,
    ) -> usize
    where
        E: Fn((usize, usize), u8) -> bool,
        // V: Fn(u8, u8) -> bool,
    {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((from, 0));

        while let Some((next, steps)) = queue.pop_front() {
            // visited.insert(next);

            let current_height = *self.get(next.0, next.1).unwrap();

            //find the neighbours ..
            for neighbour in
                [(0, 1_isize), (1, 0), (0, -1), (-1, 0)]
                    .iter()
                    .flat_map(|&(dx, dy)| {
                        next.0
                            .checked_add_signed(dx)
                            .and_then(|x| next.1.checked_add_signed(dy).map(|y| (x, y)))
                    })
            {
                if visited.contains(&neighbour) {
                    continue;
                }
                if let Some(&neighbour_height) = self.get(neighbour.0, neighbour.1) {
                    if valid_move(neighbour_height, current_height) {
                        if end_func(neighbour, neighbour_height) {
                            return steps + 1;
                        }

                        visited.insert(neighbour);
                        queue.push_back((neighbour, steps + 1))
                    }
                }
            }
        }

        panic!("Couldn't find the path")
    }

    fn get(&self, x: usize, y: usize) -> Option<&u8> {
        self.data.get(y * self.width + x)
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap();

    grid.find_shortest_path_to_goal()
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap();

    grid.find_shortest_path_to_start()
}

#[cfg(test)]
mod test {
    use crate::prob12::{solve_part_1, solve_part_2};

    #[test]
    fn part_1() {
        assert_eq!(solve_part_1(INPUT), 31);
    }

    #[test]
    fn part_2() {
        assert_eq!(solve_part_2(INPUT), 29);
    }

    const INPUT: &'static str = r"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
}
