
pub fn solve_part_1(input: &str) -> String {
    let mut cups: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let highest_cup_value = *cups.iter().max().unwrap();

    let mut current_cup_index = 0;
    for _ in 0..100 {
        let current_cup = cups[current_cup_index];
        // println!("{:?}", cups);
        // println!("Current {:?}", current_cup);
        // println!("Pick up {}, {}, {}", cups[(current_cup_index + 1) % cups.len()], cups[(current_cup_index + 2) % cups.len()], cups[(current_cup_index + 3) % cups.len()]);
        let cup1 = cups[(current_cup_index + 1) % cups.len()];
        let cup2 = cups[(current_cup_index + 2) % cups.len()];
        let cup3 = cups[(current_cup_index + 3) % cups.len()];

        cups.remove(cups.iter().position(|&d| d == cup1).unwrap());
        cups.remove(cups.iter().position(|&d| d == cup2).unwrap());
        cups.remove(cups.iter().position(|&d| d == cup3).unwrap());

        let mut destination_cup = if current_cup == 1 {
            highest_cup_value
        } else {
            current_cup - 1
        };
        while cup1 == destination_cup || cup2 == destination_cup || cup3 == destination_cup {
            if destination_cup == 1 {
                destination_cup = highest_cup_value;
            } else {
                destination_cup -= 1;
            }
        }

        // println!("Destination {}", destination_cup);
        let destination_cup_index = cups
            .iter()
            .enumerate()
            .find(|(_, &c)| c == destination_cup)
            .expect(&format!(
                "Couldn't find destination cup {}",
                destination_cup
            ))
            .0;

        cups.insert(destination_cup_index + 1, cup1);
        cups.insert(destination_cup_index + 2, cup2);
        cups.insert(destination_cup_index + 3, cup3);

        current_cup_index = (cups
            .iter()
            .enumerate()
            .find(|(_, &c)| c == current_cup)
            .unwrap()
            .0
            + 1)
            % cups.len();
        // current_cup_index = (current_cup_index + 1) % cups.len();
    }

    // println!("Final {:?}", cups);
    let mut res = String::new();
    for d in cups
        .into_iter()
        .cycle()
        .skip_while(|&d| d != 1)
        .skip(1)
        .take_while(|&d| d != 1)
    {
        res += d.to_string().as_str();
    }
    res
}

pub fn solve_part_2(input: &str) -> usize {

    let digits: Vec<_> = input.trim().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
    let mut cups = vec![0; 1_000_001];

    for (prev, next) in digits.iter().zip(digits.iter().skip(1)) {
        cups[*prev] = *next;
    }
    cups[*digits.last().unwrap()] = digits.len()+1;
    for x in (digits.len()+1)..1_000_000 {
        cups[x] = x+1;
    }
    let highest_cup_value: usize = 1_000_000;
    cups[highest_cup_value] = digits[0];

    let mut current = digits[0];
    for _ in 0..10_000_000 {
        let cup1 = cups[current];
        let cup2 = cups[cup1];
        let cup3 = cups[cup2];
        cups[current] = cups[cup3];

        //find destination
        let mut destination_cup = if current == 1 {
            highest_cup_value
        } else {
            if current == 0 {
                println!("oops");
            }
            current - 1
        };
        while cup1 == destination_cup || cup2 == destination_cup || cup3 == destination_cup {
            if destination_cup == 1 {
                destination_cup = highest_cup_value;
            } else {
                destination_cup -= 1;
            }
        }
        let after_destination = cups[destination_cup];
        cups[destination_cup] = cup1;
        cups[cup3] = after_destination;
        current = cups[current];
    }

    cups[1] * cups[cups[1]]
}

#[cfg(test)]
mod test {
    use crate::prob23::{solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1("389125467"), "67384529")
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2("389125467"), 149245887792);
    }
}
