use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    input.split(",").map(|s| hash(s)).sum()
}

fn hash(s: &str) -> usize {
    let mut result: u8 = 0;
    for c in s.chars() {
        result = result.wrapping_add(c as u8);
        result = result.wrapping_mul(17);
    }

    return result as usize;
}

pub fn solve_part_2(input: &str) -> usize {
    let mut map = MyHashMap::default();
    for command in input.split(",") {
        if let Some(pos) = command.find("=") {
            let label = &command[0..pos];
            map.put(label, command[pos + 1..].parse::<usize>().unwrap());
        } else if let Some(pos) = command.find("-") {
            let label = &command[0..pos];
            map.remove(label);
        }
    }
    map.focusing_power()
}

const fn empty_vec() -> Vec<(String, usize)> {
    Vec::new()
}
struct MyHashMap {
    data: [Vec<(String, usize)>; 256],
}

impl Default for MyHashMap {
    fn default() -> Self {
        const VAL: Vec<(String, usize)> = empty_vec();
        Self { data: [VAL; 256] }
    }
}

impl MyHashMap {
    fn put(&mut self, key: &str, value: usize) {
        let h = hash(key);
        let bucket = &mut self.data[h];

        if let Some(current) = bucket.iter_mut().find(|(k, _)| k == key) {
            current.1 = value;
        } else {
            bucket.push((key.to_string(), value));
        }
    }

    fn remove(&mut self, key: &str) {
        let h = hash(key);
        let bucket = &mut self.data[h];

        if let Some((pos, _)) = bucket.iter().find_position(|(k, _)| k == key) {
            bucket.remove(pos);
        }
    }

    fn focusing_power(&self) -> usize {
        let mut result = 0;
        for (index, b) in self.data.iter().enumerate() {
            for (lens_index, lens) in b.into_iter().enumerate() {
                result += (index + 1) * (lens_index + 1) * lens.1;
            }
        }

        result
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 1320);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT), 145);
    }

    const TEST_INPUT: &'static str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
}
