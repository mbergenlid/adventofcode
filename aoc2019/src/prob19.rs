use crate::intcode::IntCode;

pub fn solve_part_1() {
    let mut int_code = input();
    let mut points_affected = 0;
    for y in 700..900 {
        print!("{}: ", y);
        for x in 900..1100 {
            let result = int_code.run([x, y].iter())[0];
            points_affected += result;
            print!("{}", if result == 1 { '#' } else { '.' });
        }
        println!();
    }
    println!("{}", points_affected);
}

pub fn solve_part_2() {
    let mut code = input();
    let mut x = 4;
    let mut y = 3;
    loop {

        let result = code.run([x,y].iter())[0];
        if result == 1 {
            //Look downward
            while code.run([x,y+1].iter())[0] == 1 {
                y += 1;
            }
        } else {
            //Look upwards
            y -= 1;
            while code.run([x,y].iter())[0] == 0 {
                y -= 1;
            }
        }
        //We found the edge
        if y >= 100 {
            let top_left = code.run([x, y-99].iter())[0];
            if top_left == 1 {
                let top_right = code.run([x+99,y-99].iter())[0];
                if top_right == 1 {
                    //Look up to see if it still fits
                    while code.run([x+99, y-100].iter())[0] == 1 {
                        y -= 1;
                    }
                    println!("Found square at {},{} = {}", x, y-99, x*10000+(y-99));
                    break;
                }
            }
        }

        if x % 100 == 0 {
            println!("x,y = {},{}", x, y);
        }
        x += 1;
        y += 1;

    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_part_1() {
        super::solve_part_1();
    }

    #[test]
    fn test_part_2() {
        super::solve_part_2();
    }
}

fn input() -> IntCode {
    IntCode::new(vec![
        109, 424, 203, 1, 21102, 1, 11, 0, 1105, 1, 282, 21102, 18, 1, 0, 1106, 0, 259, 2102, 1, 1,
        221, 203, 1, 21102, 31, 1, 0, 1105, 1, 282, 21101, 38, 0, 0, 1106, 0, 259, 21002, 23, 1, 2,
        22101, 0, 1, 3, 21102, 1, 1, 1, 21101, 57, 0, 0, 1105, 1, 303, 1202, 1, 1, 222, 20102, 1,
        221, 3, 20102, 1, 221, 2, 21102, 1, 259, 1, 21102, 80, 1, 0, 1105, 1, 225, 21102, 72, 1, 2,
        21101, 91, 0, 0, 1105, 1, 303, 1201, 1, 0, 223, 20102, 1, 222, 4, 21101, 0, 259, 3, 21102,
        1, 225, 2, 21102, 1, 225, 1, 21102, 1, 118, 0, 1105, 1, 225, 20102, 1, 222, 3, 21101, 104,
        0, 2, 21101, 0, 133, 0, 1105, 1, 303, 21202, 1, -1, 1, 22001, 223, 1, 1, 21102, 148, 1, 0,
        1106, 0, 259, 1201, 1, 0, 223, 20101, 0, 221, 4, 20102, 1, 222, 3, 21101, 0, 18, 2, 1001,
        132, -2, 224, 1002, 224, 2, 224, 1001, 224, 3, 224, 1002, 132, -1, 132, 1, 224, 132, 224,
        21001, 224, 1, 1, 21101, 195, 0, 0, 106, 0, 109, 20207, 1, 223, 2, 20101, 0, 23, 1, 21102,
        1, -1, 3, 21102, 214, 1, 0, 1106, 0, 303, 22101, 1, 1, 1, 204, 1, 99, 0, 0, 0, 0, 109, 5,
        2102, 1, -4, 249, 22102, 1, -3, 1, 22102, 1, -2, 2, 22102, 1, -1, 3, 21101, 250, 0, 0,
        1105, 1, 225, 22102, 1, 1, -4, 109, -5, 2106, 0, 0, 109, 3, 22107, 0, -2, -1, 21202, -1, 2,
        -1, 21201, -1, -1, -1, 22202, -1, -2, -2, 109, -3, 2105, 1, 0, 109, 3, 21207, -2, 0, -1,
        1206, -1, 294, 104, 0, 99, 21202, -2, 1, -2, 109, -3, 2105, 1, 0, 109, 5, 22207, -3, -4,
        -1, 1206, -1, 346, 22201, -4, -3, -4, 21202, -3, -1, -1, 22201, -4, -1, 2, 21202, 2, -1,
        -1, 22201, -4, -1, 1, 21202, -2, 1, 3, 21101, 0, 343, 0, 1105, 1, 303, 1105, 1, 415, 22207,
        -2, -3, -1, 1206, -1, 387, 22201, -3, -2, -3, 21202, -2, -1, -1, 22201, -3, -1, 3, 21202,
        3, -1, -1, 22201, -3, -1, 2, 21201, -4, 0, 1, 21102, 384, 1, 0, 1106, 0, 303, 1105, 1, 415,
        21202, -4, -1, -4, 22201, -4, -3, -4, 22202, -3, -2, -2, 22202, -2, -4, -4, 22202, -3, -2,
        -3, 21202, -4, -1, -2, 22201, -3, -2, 1, 21202, 1, 1, -4, 109, -5, 2105, 1, 0,
    ])
}
