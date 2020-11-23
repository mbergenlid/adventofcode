use std::collections::HashSet;

pub fn solve_part_1() -> u32 {
    let map = Map::new(input().as_slice());
    map.solve_part_1()
}

pub fn solve_part_2() -> u32 {
    let map = Map::new(input().as_slice());
    map.solve_part_2()
}

type Location = String;

struct Map {
    edges: Vec<(Location, Location, u32)>,
}

impl Map {
    fn new(paths: &[(Location, Location, u32)]) -> Map {
        let mut edges = Vec::new();
        for (l1, l2, distance) in paths.iter() {
            edges.push((l2.clone(), l1.clone(), *distance));
            edges.push((l1.clone(), l2.clone(), *distance));
        }
        Map { edges }
    }

    fn solve_part_1(&self) -> u32 {
        let all_locations = self
            .edges
            .iter()
            .map(|(l1, _, _)| l1)
            .collect::<HashSet<_>>();

        return all_locations
            .iter()
            .map(|&l| {
                self.find_shortest_path(
                    l,
                    all_locations
                        .iter()
                        .filter(|&&loc| loc != l)
                        .map(|&l| l)
                        .collect(),
                    0,
                )
            })
            .min()
            .unwrap();
    }

    fn solve_part_2(&self) -> u32 {
        let all_locations = self
            .edges
            .iter()
            .map(|(l1, _, _)| l1)
            .collect::<HashSet<_>>();

        return all_locations
            .iter()
            .map(|&l| {
                self.find_longest_path(
                    l,
                    all_locations
                        .iter()
                        .filter(|&&loc| loc != l)
                        .map(|&l| l)
                        .collect(),
                    0,
                )
            })
            .max()
            .unwrap();
    }

    fn find_shortest_path(&self, from: &Location, to: Vec<&Location>, distance: u32) -> u32 {
        if to.is_empty() {
            return distance;
        }
        let mut shortest_distance = u32::MAX;
        for &next in to.iter() {
            let (_, _, dist) = self
                .edges
                .iter()
                .find(|(l1, l2, _)| l1 == from && l2 == next)
                .expect("Can't find edge");

            let distance_this_path = self.find_shortest_path(
                next,
                to.iter().filter(|&&l| l != next).map(|&l| l).collect(),
                distance + dist,
            );
            shortest_distance = std::cmp::min(shortest_distance, distance_this_path);
        }
        shortest_distance
    }

    fn find_longest_path(&self, from: &Location, to: Vec<&Location>, distance: u32) -> u32 {
        if to.is_empty() {
            return distance;
        }
        let mut longest_distance = u32::MIN;
        for &next in to.iter() {
            let (_, _, dist) = self
                .edges
                .iter()
                .find(|(l1, l2, _)| l1 == from && l2 == next)
                .expect("Can't find edge");

            let distance_this_path = self.find_longest_path(
                next,
                to.iter().filter(|&&l| l != next).map(|&l| l).collect(),
                distance + dist,
            );
            longest_distance = std::cmp::max(longest_distance, distance_this_path);
        }
        longest_distance
    }
}

#[cfg(test)]
mod test {
    use crate::prob9::Map;

    #[test]
    fn test_part_1() {
        let map = Map::new(&[
            ("London".to_string(), "Dublin".to_string(), 464),
            ("London".to_string(), "Belfast".to_string(), 518),
            ("Dublin".to_string(), "Belfast".to_string(), 141),
        ]);

        assert_eq!(
            map.find_shortest_path(
                &"London".to_string(),
                vec![&"Dublin".to_string(), &"Belfast".to_string()],
                0
            ),
            605
        );

        assert_eq!(map.solve_part_1(), 605);
    }
}

fn input() -> Vec<(Location, Location, u32)> {
    vec![
        ("Faerun".to_string(), "Norrath".to_string(), 129),
        ("Faerun".to_string(), "Tristram".to_string(), 58),
        ("Faerun".to_string(), "AlphaCentauri".to_string(), 13),
        ("Faerun".to_string(), "Arbre".to_string(), 24),
        ("Faerun".to_string(), "Snowdin".to_string(), 60),
        ("Faerun".to_string(), "Tambi".to_string(), 71),
        ("Faerun".to_string(), "Straylight".to_string(), 67),
        ("Norrath".to_string(), "Tristram".to_string(), 142),
        ("Norrath".to_string(), "AlphaCentauri".to_string(), 15),
        ("Norrath".to_string(), "Arbre".to_string(), 135),
        ("Norrath".to_string(), "Snowdin".to_string(), 75),
        ("Norrath".to_string(), "Tambi".to_string(), 82),
        ("Norrath".to_string(), "Straylight".to_string(), 54),
        ("Tristram".to_string(), "AlphaCentauri".to_string(), 118),
        ("Tristram".to_string(), "Arbre".to_string(), 122),
        ("Tristram".to_string(), "Snowdin".to_string(), 103),
        ("Tristram".to_string(), "Tambi".to_string(), 49),
        ("Tristram".to_string(), "Straylight".to_string(), 97),
        ("AlphaCentauri".to_string(), "Arbre".to_string(), 116),
        ("AlphaCentauri".to_string(), "Snowdin".to_string(), 12),
        ("AlphaCentauri".to_string(), "Tambi".to_string(), 18),
        ("AlphaCentauri".to_string(), "Straylight".to_string(), 91),
        ("Arbre".to_string(), "Snowdin".to_string(), 129),
        ("Arbre".to_string(), "Tambi".to_string(), 53),
        ("Arbre".to_string(), "Straylight".to_string(), 40),
        ("Snowdin".to_string(), "Tambi".to_string(), 15),
        ("Snowdin".to_string(), "Straylight".to_string(), 99),
        ("Tambi".to_string(), "Straylight".to_string(), 70),
    ]
}
