use std::iter::Cloned;
use std::mem::swap;
use std::slice::Iter;
use std::collections::HashMap;

pub fn solve_part_1() -> i32 {
    let mut permutations = Permutations::new(vec!["Bob", "Carol", "David", "Eric", "Frank", "George", "Mallory"]);
    let config = vec![
        (("Alice".to_string(), "Bob".to_string()), 54),
        (("Alice".to_string(), "Carol".to_string()), -81),
        (("Alice".to_string(), "David".to_string()), -42),
        (("Alice".to_string(), "Eric".to_string()), 89),
        (("Alice".to_string(), "Frank".to_string()), -89),
        (("Alice".to_string(), "George".to_string()), 97),
        (("Alice".to_string(), "Mallory".to_string()), -94),
        (("Bob".to_string(), "Alice".to_string()), 3),
        (("Bob".to_string(), "Carol".to_string()), -70),
        (("Bob".to_string(), "David".to_string()), -31),
        (("Bob".to_string(), "Eric".to_string()), 72),
        (("Bob".to_string(), "Frank".to_string()), -25),
        (("Bob".to_string(), "George".to_string()), -95),
        (("Bob".to_string(), "Mallory".to_string()), 11),
        (("Carol".to_string(), "Alice".to_string()), -83),
        (("Carol".to_string(), "Bob".to_string()), 8),
        (("Carol".to_string(), "David".to_string()), 35),
        (("Carol".to_string(), "Eric".to_string()), 10),
        (("Carol".to_string(), "Frank".to_string()), 61),
        (("Carol".to_string(), "George".to_string()), 10),
        (("Carol".to_string(), "Mallory".to_string()), 29),
        (("David".to_string(), "Alice".to_string()), 67),
        (("David".to_string(), "Bob".to_string()), 25),
        (("David".to_string(), "Carol".to_string()), 48),
        (("David".to_string(), "Eric".to_string()), -65),
        (("David".to_string(), "Frank".to_string()), 8),
        (("David".to_string(), "George".to_string()), 84),
        (("David".to_string(), "Mallory".to_string()), 9),
        (("Eric".to_string(), "Alice".to_string()), -51),
        (("Eric".to_string(), "Bob".to_string()), -39),
        (("Eric".to_string(), "Carol".to_string()), 84),
        (("Eric".to_string(), "David".to_string()), -98),
        (("Eric".to_string(), "Frank".to_string()), -20),
        (("Eric".to_string(), "George".to_string()), -6),
        (("Eric".to_string(), "Mallory".to_string()), 60),
        (("Frank".to_string(), "Alice".to_string()), 51),
        (("Frank".to_string(), "Bob".to_string()), 79),
        (("Frank".to_string(), "Carol".to_string()), 88),
        (("Frank".to_string(), "David".to_string()), 33),
        (("Frank".to_string(), "Eric".to_string()), 43),
        (("Frank".to_string(), "George".to_string()), 77),
        (("Frank".to_string(), "Mallory".to_string()), -3),
        (("George".to_string(), "Alice".to_string()), -14),
        (("George".to_string(), "Bob".to_string()), -12),
        (("George".to_string(), "Carol".to_string()), -52),
        (("George".to_string(), "David".to_string()), 14),
        (("George".to_string(), "Eric".to_string()), -62),
        (("George".to_string(), "Frank".to_string()), -18),
        (("George".to_string(), "Mallory".to_string()), -17),
        (("Mallory".to_string(), "Alice".to_string()), -36),
        (("Mallory".to_string(), "Bob".to_string()), 76),
        (("Mallory".to_string(), "Carol".to_string()), -34),
        (("Mallory".to_string(), "David".to_string()), 37),
        (("Mallory".to_string(), "Eric".to_string()), 40),
        (("Mallory".to_string(), "Frank".to_string()), 18),
        (("Mallory".to_string(), "George".to_string()), 7),
    ];
    let guest_list = GuestList::new(config);
    (0..7*6*5*4*3*2).map(|_| guest_list.happiness(permutations.next())).max().unwrap()
}

pub fn solve_part_2() -> i32 {
    let mut permutations = Permutations::new(vec!["Me", "Bob", "Carol", "David", "Eric", "Frank", "George", "Mallory"]);
    let config = vec![
        (("Alice".to_string(), "Bob".to_string()), 54),
        (("Alice".to_string(), "Carol".to_string()), -81),
        (("Alice".to_string(), "David".to_string()), -42),
        (("Alice".to_string(), "Eric".to_string()), 89),
        (("Alice".to_string(), "Frank".to_string()), -89),
        (("Alice".to_string(), "George".to_string()), 97),
        (("Alice".to_string(), "Mallory".to_string()), -94),
        (("Bob".to_string(), "Alice".to_string()), 3),
        (("Bob".to_string(), "Carol".to_string()), -70),
        (("Bob".to_string(), "David".to_string()), -31),
        (("Bob".to_string(), "Eric".to_string()), 72),
        (("Bob".to_string(), "Frank".to_string()), -25),
        (("Bob".to_string(), "George".to_string()), -95),
        (("Bob".to_string(), "Mallory".to_string()), 11),
        (("Carol".to_string(), "Alice".to_string()), -83),
        (("Carol".to_string(), "Bob".to_string()), 8),
        (("Carol".to_string(), "David".to_string()), 35),
        (("Carol".to_string(), "Eric".to_string()), 10),
        (("Carol".to_string(), "Frank".to_string()), 61),
        (("Carol".to_string(), "George".to_string()), 10),
        (("Carol".to_string(), "Mallory".to_string()), 29),
        (("David".to_string(), "Alice".to_string()), 67),
        (("David".to_string(), "Bob".to_string()), 25),
        (("David".to_string(), "Carol".to_string()), 48),
        (("David".to_string(), "Eric".to_string()), -65),
        (("David".to_string(), "Frank".to_string()), 8),
        (("David".to_string(), "George".to_string()), 84),
        (("David".to_string(), "Mallory".to_string()), 9),
        (("Eric".to_string(), "Alice".to_string()), -51),
        (("Eric".to_string(), "Bob".to_string()), -39),
        (("Eric".to_string(), "Carol".to_string()), 84),
        (("Eric".to_string(), "David".to_string()), -98),
        (("Eric".to_string(), "Frank".to_string()), -20),
        (("Eric".to_string(), "George".to_string()), -6),
        (("Eric".to_string(), "Mallory".to_string()), 60),
        (("Frank".to_string(), "Alice".to_string()), 51),
        (("Frank".to_string(), "Bob".to_string()), 79),
        (("Frank".to_string(), "Carol".to_string()), 88),
        (("Frank".to_string(), "David".to_string()), 33),
        (("Frank".to_string(), "Eric".to_string()), 43),
        (("Frank".to_string(), "George".to_string()), 77),
        (("Frank".to_string(), "Mallory".to_string()), -3),
        (("George".to_string(), "Alice".to_string()), -14),
        (("George".to_string(), "Bob".to_string()), -12),
        (("George".to_string(), "Carol".to_string()), -52),
        (("George".to_string(), "David".to_string()), 14),
        (("George".to_string(), "Eric".to_string()), -62),
        (("George".to_string(), "Frank".to_string()), -18),
        (("George".to_string(), "Mallory".to_string()), -17),
        (("Mallory".to_string(), "Alice".to_string()), -36),
        (("Mallory".to_string(), "Bob".to_string()), 76),
        (("Mallory".to_string(), "Carol".to_string()), -34),
        (("Mallory".to_string(), "David".to_string()), 37),
        (("Mallory".to_string(), "Eric".to_string()), 40),
        (("Mallory".to_string(), "Frank".to_string()), 18),
        (("Mallory".to_string(), "George".to_string()), 7),
    ];
    let guest_list = GuestList::new(config);
    (0..8*7*6*5*4*3*2).map(|_| guest_list.happiness(permutations.next())).max().unwrap()
}

struct GuestList {
    first_guest: String,
    config: HashMap<String, HashMap<String, i32>>,
}

impl GuestList {
    fn new(conf: Vec<((String, String), i32)>) -> GuestList {
        let first_guest = conf[0].0.0.clone();
        let mut config = HashMap::new();
        for ((g1, g2), happiness) in conf.into_iter() {
            if !config.contains_key(&g1) {
                config.insert(g1.clone(), HashMap::new());
            }
            config.get_mut(&g1).unwrap().insert(g2, happiness);
        }
        GuestList {
            first_guest,
            config
        }
    }

    fn happiness<'b, I: Iterator<Item=&'b String>>(&self, permutation: I) -> i32 {
        let mut previous_guest = &self.first_guest;
        let mut total = 0;
        let mut last_guest = &self.first_guest;
        for g in permutation {
            total += self.config.get(previous_guest).map(|m| m.get(g).unwrap_or(&0)).unwrap_or(&0);
            total += self.config.get(g).map(|m| m.get(previous_guest).unwrap_or(&0)).unwrap_or(&0);
            previous_guest = g;
            last_guest = g;
        }
        total += self.config.get(&self.first_guest).map(|m| m.get(last_guest).unwrap_or(&0)).unwrap_or(&0);
        total += self.config.get(last_guest).map(|m| m.get(&self.first_guest).unwrap_or(&0)).unwrap_or(&0);
        total
    }
}


struct Permutations {
    vec: Vec<String>,
    current_permutation: Vec<usize>,
    swap_indexes: Vec<usize>,
    current_swap: usize,
    first: bool,
}

impl Permutations {
    fn new<T: Into<String>>(vec: Vec<T>) -> Permutations {
        let length = vec.len();
        let mut swap_indexes = vec![0; length];
        Permutations {
            vec: vec.into_iter().map(|x| x.into()).collect(),
            current_permutation: (0..length).collect(),
            swap_indexes,
            current_swap: length - 1,
            first: true,
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        let temp = self.current_permutation[a];
        self.current_permutation[a] =
            self.current_permutation[b];
        self.current_permutation[b] = temp;
    }

    fn next(&mut self) -> Permutation<'_, Cloned<Iter<usize>>> {
        if self.first {
            self.first = false;
            self.current_swap = self.swap_indexes.len() - 2;
        } else {
            if self.current_swap == self.swap_indexes.len()-1 {
                self.back_track_current_swap();
                self.swap(self.current_swap+1, self.current_permutation.len()-1);
            }

            self.swap_indexes[self.current_swap] += 1;
            self.swap(self.current_swap, self.current_swap + self.swap_indexes[self.current_swap]);
            self.current_swap = self.swap_indexes.len() - 1;

        }

        Permutation {
            source: self.vec.as_slice(),
            index: self.current_permutation.iter().cloned(),
        }
    }

    fn back_track_current_swap(&mut self) {
        let mut current_swap = self.current_swap - 1;
        while current_swap + self.swap_indexes[current_swap] == self.current_permutation.len() - 1{
            self.swap_indexes[current_swap] = 0;
            current_swap -= 1;
        }
        self.current_swap = current_swap;
    }
}

struct Permutation<'a, I: Iterator<Item = usize>> {
    source: &'a [String],
    index: I,
}

impl<'a, I: Iterator<Item = usize>> Iterator for Permutation<'a, I> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index.next() {
            Some(i) => Some(self.source.get(i).expect("Eeeeh")),
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prob13::{Permutation, Permutations, GuestList};
    use std::collections::HashMap;

    #[test]
    fn test() {
        //[1,2,3] -> [1,3,2] -> [2,3,1] -> [2,1,3] -> [3,1,2] -> [3,2,1]
        // (2-3) (1-3) (2-3) (1-3) (2-3)
        //[1,2,3] -> [2,1,3] -> [3,1,2] -> [1,3,2] -> [2,3,1] -> [3,2,1]

        //[1,2,3,4] ->
        //[2,1,3,4] ->
        //[3,1,2,4] ->
        //[4,1,2,3]
        let mut seating_arrangements = Permutations::new(vec!["1", "2", "3"]);

        assert_eq!(
            seating_arrangements.next().collect::<Vec<_>>(),
            vec!["1", "2", "3"]
        );
        assert_eq!(
            seating_arrangements.next().collect::<Vec<_>>(),
            vec!["1", "3", "2"]
        );
        //[1,2,3]

        assert_eq!(
            seating_arrangements.next().collect::<Vec<_>>(),
            vec!["2", "1", "3"]
        );
        assert_eq!(
            seating_arrangements.next().collect::<Vec<_>>(),
            vec!["2", "3", "1"]
        );
        //[2,1,3]

        assert_eq!(
            seating_arrangements.next().collect::<Vec<_>>(),
            vec!["3", "1", "2"]
        );
        assert_eq!(
            seating_arrangements.next().collect::<Vec<_>>(),
            vec!["3", "2", "1"]
        );
        //[1,2,3]
    }

    #[test]
    fn test_with_4_elements() {
        let mut seating_arrangements = Permutations::new(vec!["0", "1", "2", "3"]);

        for _ in 0..5 {
            seating_arrangements.next();
        }
        assert_eq!(seating_arrangements.next().collect::<Vec<_>>(), vec!["0", "3", "2", "1"]);

        assert_eq!(seating_arrangements.next().collect::<Vec<_>>(), vec!["1", "0", "2", "3"]);
        assert_eq!(seating_arrangements.next().collect::<Vec<_>>(), vec!["1", "0", "3", "2"]);

        assert_eq!(seating_arrangements.next().collect::<Vec<_>>(), vec!["1", "2", "0", "3"]);
        assert_eq!(seating_arrangements.next().collect::<Vec<_>>(), vec!["1", "2", "3", "0"]);

        assert_eq!(seating_arrangements.next().collect::<Vec<_>>(), vec!["1", "3", "0", "2"]);
        assert_eq!(seating_arrangements.next().collect::<Vec<_>>(), vec!["1", "3", "2", "0"]);

        assert_eq!(seating_arrangements.next().collect::<Vec<_>>(), vec!["2", "0", "1", "3"]);
    }

    #[test]
    fn cycles_with_4_elements() {
        let mut seating_arrangements = Permutations::new(vec!["0", "1", "2", "3"]);
        for _ in 0..24 {
            seating_arrangements.next();
        }
    }

    #[test]
    fn solve_part_1() {
        let all_guests = vec!["Alice".to_string(), "Bob".to_string(), "Carol".to_string(), "David".to_string()];
        let mut permutations = Permutations::new(vec!["Bob", "Carol", "David"]);
        let config = vec![
            (("Alice".to_string(), "Bob".to_string()), 54),
            (("Alice".to_string(), "Carol".to_string()), -79),
            (("Alice".to_string(), "David".to_string()), -2),
            (("Bob".to_string(), "Alice".to_string()), 83),
            (("Bob".to_string(), "Carol".to_string()), -7),
            (("Bob".to_string(), "David".to_string()), -63),
            (("Carol".to_string(), "Alice".to_string()), -62),
            (("Carol".to_string(), "Bob".to_string()), 60),
            (("Carol".to_string(), "David".to_string()), 55),
            (("David".to_string(), "Alice".to_string()), 46),
            (("David".to_string(), "Bob".to_string()), -7),
            (("David".to_string(), "Carol".to_string()), 41),
        ];
        let guest_list = GuestList::new(config);
        assert_eq!((0..6).map(|_| guest_list.happiness(permutations.next())).max().unwrap(), 330);
        // assert_eq!(guest_list.happiness(["Bob".to_string(), "Carol".to_string(), "David".to_string()].iter()), 330);
    }
}
