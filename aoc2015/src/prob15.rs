use std::cmp::max;

pub fn solve_part_1() -> u64 {
    //Sugar: capacity 3, durability 0, flavor 0, texture -3, calories 2
    // Sprinkles: capacity -3, durability 3, flavor 0, texture 0, calories 9
    // Candy: capacity -1, durability 0, flavor 4, texture 0, calories 1
    // Chocolate: capacity 0, durability 0, flavor -2, texture 2, calories 8
    let mut max_score = 0;
    for sugar in 0..101 {
        for sprinkles in 0..101 {
            for candy in 0..101 {
                let chocolate: i64 = 100 - (candy + sprinkles + sugar);
                let capacity = max(sugar*3 + sprinkles*-3 + candy*-1 + chocolate*0, 0);
                let durability = max(sugar*0 + sprinkles*3 + candy*0 + chocolate*0, 0);
                let flavor = max(sugar*0 + sprinkles*0 + candy*4 + chocolate*-2, 0);
                let texture = max(sugar*-3 + sprinkles*0 + candy*0 + chocolate*2, 0);

                let current_score = (capacity*durability*flavor*texture) as u64;
                if current_score > max_score {
                    max_score = current_score;
                }
            }
        }
    }
    max_score
}

pub fn solve_part_2() -> u64 {
    //Sugar: capacity 3, durability 0, flavor 0, texture -3, calories 2
    // Sprinkles: capacity -3, durability 3, flavor 0, texture 0, calories 9
    // Candy: capacity -1, durability 0, flavor 4, texture 0, calories 1
    // Chocolate: capacity 0, durability 0, flavor -2, texture 2, calories 8
    let mut max_score = 0;
    for sugar in 0..101 {
        for sprinkles in 0..101 {
            for candy in 0..101 {
                let chocolate: i64 = 100 - (candy + sprinkles + sugar);
                let calories = sugar*2 + sprinkles*9 + candy*1 + chocolate*8;
                if calories == 500 {
                    let capacity = max(sugar*3 + sprinkles*-3 + candy*-1 + chocolate*0, 0);
                    let durability = max(sugar*0 + sprinkles*3 + candy*0 + chocolate*0, 0);
                    let flavor = max(sugar*0 + sprinkles*0 + candy*4 + chocolate*-2, 0);
                    let texture = max(sugar*-3 + sprinkles*0 + candy*0 + chocolate*2, 0);

                    let current_score = (capacity*durability*flavor*texture) as u64;
                    if current_score > max_score {
                        max_score = current_score;
                    }
                }
            }
        }
    }
    max_score
}
