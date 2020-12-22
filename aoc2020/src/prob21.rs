use std::collections::{HashMap, HashSet};

pub fn solve_part_1(input: &str) -> u32 {
    let all_lists: Vec<_> = input
        .lines()
        .map(|line| line.parse::<IngredientList>().unwrap())
        .collect();
    let allergens = extract_allergens(&all_lists);
    let mut occurences = 0;
    for list in all_lists {
        for ingredient in list.ingredients.split(" ").filter(|i| !i.is_empty()) {
            if !allergens.values().any(|set| set.contains(ingredient)) {
                occurences += 1;
            }
        }
    }

    occurences
}

fn extract_allergens(all_lists: &Vec<IngredientList>) -> HashMap<String, HashSet<String>> {
    let mut allergens: HashMap<String, HashSet<String>> = HashMap::new();
    for list in all_lists.iter() {
        let ingredients = list
            .ingredients
            .split(" ")
            .filter(|i| !i.is_empty())
            .collect::<HashSet<_>>();
        for allergen in list.allergens.split(",").map(|a| a.trim()) {
            if let Some(prev_ingredients) = allergens.get_mut(allergen) {
                prev_ingredients.retain(|i| ingredients.contains(i.as_str()));
            } else {
                allergens.insert(
                    allergen.to_string(),
                    ingredients.iter().map(|i| i.to_string()).collect(),
                );
            }
        }
    }
    allergens
}

pub fn solve_part_2(input: &str) -> String {
    let all_lists: Vec<_> = input
        .lines()
        .map(|line| line.parse::<IngredientList>().unwrap())
        .collect();
    let mut allergens = extract_allergens(&all_lists);

    let mut assignments: HashMap<String, String> = HashMap::new();
    while let Some((allergen, set)) = allergens.iter().filter(|(_, set)| set.len() == 1).next() {
        let ingredient_to_remove = set.iter().next().unwrap().to_string();
        let allergen = allergen.to_string();
        for (_, ingredients) in allergens.iter_mut() {
            ingredients.remove(&ingredient_to_remove);
        }
        assignments.insert(allergen.to_string(), ingredient_to_remove);
    }

    let mut keys = assignments.keys().map(|k| k.to_string()).collect::<Vec<_>>();
    keys.sort();

    let s = keys.iter()
        .map(|k| assignments.get(k).unwrap())
        .fold(String::new(), |acc, i| acc + i + ",")
        .to_string();

    s[0..s.len()-1].to_string()
}

#[derive(Deserialize, Recap)]
#[recap(regex = r"(?P<ingredients>[^\(]+) \(contains (?P<allergens>[^\)]+)\)")]
struct IngredientList {
    ingredients: String,
    allergens: String,
}

#[cfg(test)]
mod test {
    use crate::prob21::{solve_part_1, solve_part_2};

    const TESTCASE_1: &str = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(TESTCASE_1), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TESTCASE_1), "mxmxvkd,sqjhc,fvjkl".to_string());
    }
}
