
#[derive(Deserialize, Recap)]
#[recap(regex = r"(?P<start_1>\d+)-(?P<end_1>\d+),(?P<start_2>\d+)-(?P<end_2>\d+)")]
struct Segments {
    start_1: u32,
    end_1: u32,

    start_2: u32,
    end_2: u32,
}

impl Segments {
    fn one_fully_contains_the_other(&self) -> bool {
        (self.start_1..=self.end_1).all(|n| (self.start_2..=self.end_2).contains(&n))
            || (self.start_2..=self.end_2).all(|n| (self.start_1..=self.end_1).contains(&n))
    }

    fn segments_overlap(&self) -> bool {
        (self.start_1..=self.end_1).any(|n| (self.start_2..=self.end_2).contains(&n))
    }
}

pub fn solve_part_1(input: &str) -> usize {
    input.lines()
        .map(|line| line.parse::<Segments>().unwrap())
        .filter(|line| line.one_fully_contains_the_other())
        .count()
}

pub fn solve_part_2(input: &str) -> usize {
    input.lines()
        .map(|line| line.parse::<Segments>().unwrap())
        .filter(|line| line.segments_overlap())
        .count()
}
