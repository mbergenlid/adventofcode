
pub fn solve_part_1() {
    let secret_key = "ckczppom";
    for i in 0.. {
        let digest = md5::compute(format!("{}{}", secret_key, i));
        if format!("{:x}", digest).starts_with("00000") {
            println!("Part 1: {}", i);
            break;
        }
    }
}

pub fn solve_part_2() {
    let secret_key = "ckczppom";
    for i in 0.. {
        let digest = md5::compute(format!("{}{}", secret_key, i));
        if format!("{:x}", digest).starts_with("000000") {
            println!("Part 1: {}", i);
            break;
        }
    }
}
