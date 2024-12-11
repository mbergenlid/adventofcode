use std::{fmt::Display, str::FromStr};

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let mut disk = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Not a digit"))
        .collect_vec();

    let mut checksum = 0;
    let mut block_index = 0;
    let mut copy_index = if disk.len() % 2 == 0 {
        disk.len() - 2
    } else {
        disk.len() - 1
    };
    for index in 0..disk.len() {
        if copy_index < index {
            break;
        }
        let f = disk[index];
        if index % 2 == 0 {
            //Real file
            let file_index = index / 2;
            for x in block_index..block_index + f {
                //println!("{} * {}", x, file_index);
                checksum += (x as usize) * file_index;
            }

            block_index += f;
        } else {
            while disk[index] > 0 {
                let file_to_copy = disk[copy_index];
                let blocks_copied = file_to_copy.min(disk[index]);
                for x in block_index..block_index + blocks_copied {
                    //println!("{} * {}", x, copy_index/2);
                    checksum += (x as usize) * copy_index / 2;
                }
                disk[copy_index] -= blocks_copied;
                disk[index] -= blocks_copied;
                if disk[copy_index] == 0 {
                    copy_index -= 2;
                }
                block_index += blocks_copied;
            }
        }

        //println!("{:?}", disk);
    }
    checksum
}

pub fn solve_part_2(input: &str) -> usize {
    let mut blocks = input.parse::<Blocks>().expect("Invalid input");
    blocks.defragment();

    println!("{}", blocks);

    blocks.checksum()
}
//0099811188827773336446555566
//0123456789111111111122222222
//          012345678901234567
//

#[derive(Debug)]
struct Blocks(Vec<Block>);

impl Display for Blocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for b in &self.0 {
            if let Some(file) = b.file {
                for _ in 0..b.length {
                    write!(f, "{}", file)?;
                }
            } else {
                for _ in 0..b.length {
                    write!(f, ".")?;
                }
            }
        }
        Ok(())
    }
}

impl FromStr for Blocks {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Blocks(
            s.trim()
                .chars()
                .enumerate()
                .map(|(index, f)| Block {
                    file: if index % 2 == 0 {
                        Some(index / 2)
                    } else {
                        None
                    },
                    length: f.to_digit(10).expect("Invalid digit"),
                })
                .collect_vec(),
        ))
    }
}

impl Blocks {
    fn defragment(&mut self) {
        for b in (0..self.0.len()).rev() {
            let current_block = &self.0[b];
            let block_length = current_block.length;
            if let Some(file_index) = current_block.file {
                //try to move it
                if let Some((index, target)) = self
                    .0
                    .iter_mut()
                    .enumerate()
                    .take(b)
                    .find(|(_, b)| b.file.is_none() && b.length >= block_length)
                {
                    if target.length == block_length {
                        target.file.replace(file_index);
                        self.0.get_mut(b).expect("Must be there").file.take();
                    } else {
                        target.length -= block_length;
                        self.0.get_mut(b).expect("Must be there").file.take();
                        self.0.insert(
                            index,
                            Block {
                                file: Some(file_index),
                                length: block_length,
                            },
                        );
                    }
                }
            }
        }
    }

    fn checksum(&self) -> usize {
        let mut block_index = 0;
        let mut checksum = 0;
        for b in self.0.iter() {
            if let Some(file) = b.file {
                for b_i in block_index..block_index + b.length {
                    checksum += file * b_i as usize;
                }
            }
            block_index += b.length;
        }
        checksum
    }
}

#[derive(Debug)]
struct Block {
    file: Option<usize>,
    length: u32,
}

#[cfg(test)]
mod test {

    //#[test]
    //fn pack_1() {
    //    let mut disk = vec![2,0,2];
    //    super::pack(&mut disk);
    //    assert_eq!(disk, vec![2,2]);
    //}

    #[test]
    fn part_1() {
        // assert_eq!(super::solve_part_1("202"), 0+2*1+3*1); //0011
        // assert_eq!(super::solve_part_1("212"), 0+2*1+3*1); //00.11 -> 0011
        // assert_eq!(super::solve_part_1("12345"), 0+1*2+2*2+3*1+4*1+5*1+6*2+7*2+8*2);
        //022111222
        //012345678
        assert_eq!(super::solve_part_1(INPUT), 1928);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve_part_2(INPUT), 2858);
    }

    const INPUT: &str = "2333133121414131402";
}
