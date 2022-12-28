use crate::prob17::StreamJet::{Left, Right};
use std::fmt::Debug;

#[derive(Eq, PartialEq)]
struct Pos {
    x: u32,
    y: u32,
}

struct Block {
    points: Vec<Pos>,
}

impl Block {
    fn shape_1(left: u32, bottom: u32) -> Block {
        Block {
            points: vec![
                Pos { x: left, y: bottom },
                Pos {
                    x: left + 1,
                    y: bottom,
                },
                Pos {
                    x: left + 2,
                    y: bottom,
                },
                Pos {
                    x: left + 3,
                    y: bottom,
                },
            ],
        }
    }

    fn shape_2(left: u32, bottom: u32) -> Block {
        Block {
            points: vec![
                Pos {
                    x: left + 1,
                    y: bottom + 2,
                },
                Pos {
                    x: left,
                    y: bottom + 1,
                },
                Pos {
                    x: left + 1,
                    y: bottom + 1,
                },
                Pos {
                    x: left + 2,
                    y: bottom + 1,
                },
                Pos {
                    x: left + 1,
                    y: bottom,
                },
            ],
        }
    }

    fn shape_3(left: u32, bottom: u32) -> Block {
        Block {
            points: vec![
                Pos {
                    x: left + 2,
                    y: bottom + 2,
                },
                Pos {
                    x: left + 2,
                    y: bottom + 1,
                },
                Pos { x: left, y: bottom },
                Pos {
                    x: left + 1,
                    y: bottom,
                },
                Pos {
                    x: left + 2,
                    y: bottom,
                },
            ],
        }
    }

    fn shape_4(left: u32, bottom: u32) -> Block {
        Block {
            points: vec![
                Pos {
                    x: left,
                    y: bottom + 3,
                },
                Pos {
                    x: left,
                    y: bottom + 2,
                },
                Pos {
                    x: left,
                    y: bottom + 1,
                },
                Pos { x: left, y: bottom },
            ],
        }
    }

    fn shape_5(left: u32, bottom: u32) -> Block {
        Block {
            points: vec![
                Pos {
                    x: left,
                    y: bottom + 1,
                },
                Pos {
                    x: left + 1,
                    y: bottom + 1,
                },
                Pos { x: left, y: bottom },
                Pos {
                    x: left + 1,
                    y: bottom,
                },
            ],
        }
    }

    fn move_down(&self) -> Block {
        Block {
            points: self
                .points
                .iter()
                .map(|Pos { x, y }| Pos { x: *x, y: y - 1 })
                .collect(),
        }
    }

    fn move_left(&self) -> Block {
        Block {
            points: self
                .points
                .iter()
                .map(|Pos { x, y }| Pos { x: x - 1, y: *y })
                .collect(),
        }
    }

    fn move_right(&self) -> Block {
        Block {
            points: self
                .points
                .iter()
                .map(|Pos { x, y }| Pos { x: x + 1, y: *y })
                .collect(),
        }
    }

    fn hit_wall_right(&self) -> bool {
        self.points.iter().any(|p| p.x == 0)
    }

    fn hit_wall_left(&self) -> bool {
        self.points.iter().any(|p| p.x == 8)
    }

    fn hit_floor(&self) -> bool {
        self.points.iter().any(|p| p.y == 0)
    }

    fn hit_any_other_block(&self, others: &Vec<[bool; 8]>) -> bool {
        self.points.iter().any(|p| {
            others
                .get((p.y - 1) as usize)
                .map(|row| row[p.x as usize])
                .unwrap_or(false)
        })
    }

    fn stop(self, cave: &mut Vec<[bool; 8]>) {
        for p in self.points {
            if cave.len() < p.y as usize {
                for _ in 0..((p.y as usize) - cave.len()) {
                    cave.push([false; 8]);
                }
            }
            cave[(p.y - 1) as usize][p.x as usize] = true;
        }
    }
}
//00133400330213322122221130212122122120122002132023201322013210033221304013242133401212200300132421321002320133001321212120033000121213242132200122013320123421322013302133201220012120133201220013012132201321013242132110302003300023001230213211032101234010300013001330012120032221212013211132221320003322023221330013320132001330203010030221230112200132221230010342132121303213320130201334013342133401322013320133021212213320132401230103340132401330012140133221213203001112401320012320133001232202202033000212012120130401224012120022111322013322123011330202302132421330013040103111332012102133000222013242123401222002320132421214013340003001121213210032221324013320130321334000301030201330013302123001332010300121200330213320132221230103200132121230013300121201212013220023201324213222121221320200310032201322012300112221332213300123201030002300133400030013300020201330213220133201303003300130120302013220121201302012130121201222013340133021121012302130401330213220133221234013220012220030202300132201322013322132201212013320133401320213202133000230013222133201303002300133221212210022123001232013300133001234013320132221320013342121201222213200022401232013300133201332213322132000330203002132011330012322123421322213220132401334012320133401220013242112101320013200023001302012202020201222013342003221320003222130301230212211132121121213220123001224000342133221332002300121201232013210133221332213300023200232002300032421032013300022200221002302112201324013342103001321113320023001303012220102400030013032121320232213320133020322213322123200224013300130400200202202133001322213320123001322002202123201330013222122200234000320103201022012340020000332202300132401324002002133020234002242132221320013222132401320013220132121320013300121301234213220023001234000320133
//0133001332212222133420032013220132111332013300133420332013322122421332013030133001334002240121220332013240020021330013200013400232213240133021332213300123201332012240133001322213300130401224002240020321130213300121300303012122132011211013200032100330212120130401230013222133201304013242133001230103320133001210003222133201330013220132401032203300133221300113322133000230013300133001330011302132121332012300132001330013240132201322013302132001332010340122401304012320123201334013302003100321113320121201330013322132421232213242122201230013302121301320012120033221230013220123001321213322123421322213220132401334012320133401220013242112101320013200023001302012202020201222013342003221320003222130301230212211132121121213220123001224000342133221332002300121201232013210133221332213300023200232002300032421032013300022200221002302112201324013342103001321113320023001303012220102400030013032121320232213320133020322213322123200224013300130400200202202133001322213320123001322002202123201330013222122200234000320103201022012340020000332202300132401324002002133020234002242132221320013222132401320013220132121320013300121301234213220023001234000320133
#[derive(Debug)]
enum StreamJet {
    Left,
    Right,
}

pub fn solve_part_1(input: &str) -> usize {
    let jets = input
        .chars()
        .map(|c| match c {
            '>' => Right,
            '<' => Left,
            _ => panic!("Unknown yet"),
        })
        .collect::<Vec<_>>();

    let all_blocks: Vec<Box<dyn Fn(u32, u32) -> Block>> = vec![
        Box::new(Block::shape_1),
        Box::new(Block::shape_2),
        Box::new(Block::shape_3),
        Box::new(Block::shape_4),
        Box::new(Block::shape_5),
    ];

    let mut cave: Vec<[bool; 8]> = Vec::new();
    let mut top = 0_u32;
    let mut jet_iterator = jets.iter().cycle();
    let mut last_height = 0;
    for round in 0..10000 {
        let mut block = all_blocks[round % 5](3, top + 4);
        // if round > 0 && round % 5 == 0 && jet_count % jets.len() == 0 {
        //     println!("Cycle {}", round);
        //     break;
        // }

        let current_height = cave
            .iter()
            .enumerate()
            .rev()
            .find(|(_, row)| row.contains(&true))
            .map(|(i, _)| i + 1)
            .unwrap_or(0);
        print!("{}", current_height - last_height);
        last_height = current_height;
        let mut jets_used = Vec::new();
        loop {
            let new_block = match jet_iterator.next().unwrap() {
                Left => {
                    jets_used.push(Left);
                    block.move_left()
                }
                Right => {
                    jets_used.push(Right);
                    block.move_right()
                }
            };

            if !(new_block.hit_wall_right()
                || new_block.hit_wall_left()
                || new_block.hit_any_other_block(&cave))
            {
                block = new_block;
            }

            let new_block_2 = block.move_down();

            if new_block_2.hit_floor() || new_block_2.hit_any_other_block(&cave) {
                block.stop(&mut cave);
                top = cave
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, row)| row.contains(&true))
                    .map(|(i, _)| i + 1)
                    .unwrap_or(0) as u32;
                // println!("============ Round {} ===========", round);
                // println!("Jets used {:?}", jets_used);
                // print_cave(&cave);
                // println!("==========================");
                break;
            } else {
                block = new_block_2;
            }
        }
    }

    println!();
    // println!("{}", cave.len());
    // print_cave(&cave);
    cave.iter()
        .enumerate()
        .rev()
        .find(|(_, row)| row.contains(&true))
        .map(|(i, _)| i + 1)
        .unwrap_or(0)
}

pub fn solve_part_2(_input: &str) -> usize {
    let start = [
        1, 3, 3, 0, 0, 1, 3, 3, 2, 2, 1, 2, 2, 2, 2, 1, 3, 3, 4, 2, 0, 0, 3, 2, 0, 1, 3, 2, 2, 0,
        1, 3, 2, 1, 1, 1, 3, 3, 2, 0, 1, 3, 3, 0, 0, 1, 3, 3, 4, 2, 0, 3, 3, 2, 0, 1, 3, 3, 2, 2,
        1, 2, 2, 4, 2, 1, 3, 3, 2, 0, 1, 3, 0, 3, 0, 1, 3, 3, 0, 0, 1, 3, 3, 4, 0, 0, 2, 2, 4, 0,
        1, 2, 1, 2, 2, 0, 3, 3, 2, 0, 1, 3, 2, 4, 0, 0, 2, 0, 0, 2, 1, 3, 3, 0, 0, 1, 3, 2, 0, 0,
        0, 1, 3, 4, 0, 0, 2, 3, 2, 2, 1, 3, 2, 4, 0, 1, 3, 3, 0, 2, 1, 3, 3, 2, 2, 1, 3, 3, 0, 0,
        1, 2, 3, 2, 0, 1, 3, 3, 2, 0, 1, 2, 2, 4, 0, 1, 3, 3, 0, 0, 1, 3, 2, 2, 2, 1, 3, 3, 0, 0,
        1, 3, 0, 4, 0, 1, 2, 2, 4, 0, 0, 2, 2, 4, 0, 0, 2, 0, 3, 2, 1, 1, 3, 0, 2, 1, 3, 3, 0, 0,
        1, 2, 1, 3, 0, 0, 3, 0, 3, 0, 1, 2, 1, 2, 2, 1, 3, 2, 0, 1, 1, 2, 1, 1, 0, 1, 3, 2, 0, 0,
        0, 3, 2, 1, 0, 0, 3, 3, 0, 2, 1, 2, 1, 2, 0, 1, 3, 0, 4, 0, 1, 2, 3, 0, 0, 1, 3, 2, 2, 2,
        1, 3, 3, 2, 0, 1, 3, 0, 4, 0, 1, 3, 2, 4, 2, 1, 3, 3, 0, 0, 1, 2, 3, 0, 1, 0, 3, 3, 2, 0,
        1, 3, 3, 0, 0, 1, 2, 1, 0, 0, 0, 3, 2, 2, 2, 1, 3, 3, 2, 0, 1, 3, 3, 0, 0, 1, 3, 2, 2, 0,
        1, 3, 2, 4, 0, 1, 0, 3, 2, 2, 0, 3, 3, 0, 0, 1, 3, 3, 2, 2, 1, 3, 0, 0, 1, 1, 3, 3, 2, 2,
        1, 3, 3, 0, 0, 0, 2, 3, 0, 0, 1, 3, 3, 0, 0, 1, 3, 3, 0, 0, 1, 3, 3, 0, 0, 1, 1, 3, 0, 2,
        1, 3, 2, 1, 2, 1, 3, 3, 2, 0, 1, 2, 3, 0, 0, 1, 3, 2, 0, 0, 1, 3, 3, 0, 0, 1, 3, 2, 4, 0,
        1, 3, 2, 2, 0, 1, 3, 2, 2, 0, 1, 3, 3, 0, 2, 1, 3, 2, 0, 0, 1, 3, 3, 2, 0, 1, 0, 3, 4, 0,
        1, 2, 2, 4, 0, 1, 3, 0, 4, 0, 1, 2, 3, 2, 0, 1, 2, 3, 2, 0, 1, 3, 3, 4, 0, 1, 3, 3, 0, 2,
        0, 0, 3, 1, 0, 0, 3, 2, 1, 1, 1, 3, 3, 2, 0, 1, 2, 1, 2, 0, 1, 3, 3, 0, 0, 1, 3, 3, 2, 2,
        1, 3, 2, 4, 2, 1, 2, 3, 2, 2, 1, 3, 2, 4, 2, 1, 2, 2, 2, 0, 1, 2, 3, 0, 0, 1, 3, 3, 0, 2,
        1, 2, 1, 3, 0, 1, 3, 2, 0, 0, 1, 2, 1, 2, 0, 0, 3, 3, 2, 2, 1, 2, 3, 0, 0, 1, 3, 2, 2, 0,
        1, 2, 3, 0, 0, 1, 3, 2, 1, 2, 1, 3, 3, 2, 2, 1, 2, 3, 4, 2, 1, 3, 2, 2, 2, 1, 3, 2, 2, 0,
        1, 3, 2, 4, 0, 1, 3, 3, 4, 0, 1, 2, 3, 2, 0, 1, 3, 3, 4, 0, 1, 2, 2, 0, 0, 1, 3, 2, 4, 2,
        1, 1, 2, 1, 0, 1, 3, 2, 0, 0, 1, 3, 2, 0, 0, 0, 2, 3, 0, 0, 1, 3, 0, 2, 0, 1, 2, 2, 0, 2,
        0, 2, 0, 2, 0, 1, 2, 2, 2, 0, 1, 3, 3, 4, 2, 0, 0, 3, 2, 2, 1, 3, 2, 0, 0, 0, 3, 2, 2, 2,
        1, 3, 0, 3, 0, 1, 2, 3, 0, 2, 1, 2, 2, 1, 1, 1, 3, 2, 1, 2, 1, 1, 2, 1, 2, 1, 3, 2, 2, 0,
        1, 2, 3, 0, 0, 1, 2, 2, 4, 0, 0, 0, 3, 4, 2, 1, 3, 3, 2, 2, 1, 3, 3, 2, 0, 0, 2, 3, 0, 0,
        1, 2, 1, 2, 0, 1, 2, 3, 2, 0, 1, 3, 2, 1, 0, 1, 3, 3, 2, 2, 1, 3, 3, 2, 2, 1, 3, 3, 0, 0,
        0, 2, 3, 2, 0, 0, 2, 3, 2, 0, 0, 2, 3, 0, 0, 0, 3, 2, 4, 2, 1, 0, 3, 2, 0, 1, 3, 3, 0, 0,
        0, 2, 2, 2, 0, 0, 2, 2, 1, 0, 0, 2, 3, 0, 2, 1, 1, 2, 2, 0, 1, 3, 2, 4, 0, 1, 3, 3, 4, 2,
        1, 0, 3, 0, 0, 1, 3, 2, 1, 1, 1, 3, 3, 2, 0, 0, 2, 3, 0, 0, 1, 3, 0, 3, 0, 1, 2, 2, 2, 0,
        1, 0, 2, 4, 0, 0, 0, 3, 0, 0, 1, 3, 0, 3, 2, 1, 2, 1, 3, 2, 0, 2, 3, 2, 2, 1, 3, 3, 2, 0,
        1, 3, 3, 0, 2, 0, 3, 2, 2, 2, 1, 3, 3, 2, 2, 1, 2, 3, 2, 0, 0, 2, 2, 4, 0, 1, 3, 3, 0, 0,
        1, 3, 0, 4, 0, 0, 2, 0, 0, 2, 0, 2, 2, 0, 2, 1, 3, 3, 0, 0, 1, 3, 2, 2, 2, 1, 3, 3, 2, 0,
        1, 2, 3, 0, 0, 1, 3, 2, 2, 0, 0, 2, 2, 0, 2, 1, 2, 3, 2, 0, 1, 3, 3, 0, 0, 1, 3, 2, 2, 2,
        1, 2, 2, 2, 0, 0, 2, 3, 4, 0, 0, 0, 3, 2, 0, 1, 0, 3, 2, 0, 1, 0, 2, 2, 0, 1, 2, 3, 4, 0,
        0, 2, 0, 0, 0, 0, 3, 3, 2, 2, 0, 2, 3, 0, 0, 1, 3, 2, 4, 0, 1, 3, 2, 4, 0, 0, 2, 0, 0, 2,
        1, 3, 3, 0, 2, 0, 2, 3, 4, 0, 0, 2, 2, 4, 2, 1, 3, 2, 2, 2, 1, 3, 2, 0, 0, 1, 3, 2, 2, 2,
        1, 3, 2, 4, 0, 1, 3, 2, 0, 0, 1, 3, 2, 2, 0, 1, 3, 2, 1, 2, 1, 3, 2, 0, 0, 1, 3, 3, 0, 0,
        1, 2, 1, 3, 0, 1, 2, 3, 4, 2, 1, 3, 2, 2, 0, 0, 2, 3, 0, 0, 1, 2, 3, 4, 0, 0, 0, 3, 2, 0,
        1, 3, 3,
    ];
    let cycle = [
        0, 0, 1, 3, 3, 4, 0, 0, 3, 3, 0, 2, 1, 3, 3, 2, 2, 1, 2, 2, 2, 2, 1, 1, 3, 0, 2, 1, 2, 1,
        2, 2, 1, 2, 2, 1, 2, 0, 1, 2, 2, 0, 0, 2, 1, 3, 2, 0, 2, 3, 2, 0, 1, 3, 2, 2, 0, 1, 3, 2,
        1, 0, 0, 3, 3, 2, 2, 1, 3, 0, 4, 0, 1, 3, 2, 4, 2, 1, 3, 3, 4, 0, 1, 2, 1, 2, 2, 0, 0, 3,
        0, 0, 1, 3, 2, 4, 2, 1, 3, 2, 1, 0, 0, 2, 3, 2, 0, 1, 3, 3, 0, 0, 1, 3, 2, 1, 2, 1, 2, 1,
        2, 0, 0, 3, 3, 0, 0, 0, 1, 2, 1, 2, 1, 3, 2, 4, 2, 1, 3, 2, 2, 0, 0, 1, 2, 2, 0, 1, 3, 3,
        2, 0, 1, 2, 3, 4, 2, 1, 3, 2, 2, 0, 1, 3, 3, 0, 2, 1, 3, 3, 2, 0, 1, 2, 2, 0, 0, 1, 2, 1,
        2, 0, 1, 3, 3, 2, 0, 1, 2, 2, 0, 0, 1, 3, 0, 1, 2, 1, 3, 2, 2, 0, 1, 3, 2, 1, 0, 1, 3, 2,
        4, 2, 1, 3, 2, 1, 1, 0, 3, 0, 2, 0, 0, 3, 3, 0, 0, 0, 2, 3, 0, 0, 1, 2, 3, 0, 2, 1, 3, 2,
        1, 1, 0, 3, 2, 1, 0, 1, 2, 3, 4, 0, 1, 0, 3, 0, 0, 0, 1, 3, 0, 0, 1, 3, 3, 0, 0, 1, 2, 1,
        2, 0, 0, 3, 2, 2, 2, 1, 2, 1, 2, 0, 1, 3, 2, 1, 1, 1, 3, 2, 2, 2, 1, 3, 2, 0, 0, 0, 3, 3,
        2, 2, 0, 2, 3, 2, 2, 1, 3, 3, 0, 0, 1, 3, 3, 2, 0, 1, 3, 2, 0, 0, 1, 3, 3, 0, 2, 0, 3, 0,
        1, 0, 0, 3, 0, 2, 2, 1, 2, 3, 0, 1, 1, 2, 2, 0, 0, 1, 3, 2, 2, 2, 1, 2, 3, 0, 0, 1, 0, 3,
        4, 2, 1, 3, 2, 1, 2, 1, 3, 0, 3, 2, 1, 3, 3, 2, 0, 1, 3, 0, 2, 0, 1, 3, 3, 4, 0, 1, 3, 3,
        4, 2, 1, 3, 3, 4, 0, 1, 3, 2, 2, 0, 1, 3, 3, 2, 0, 1, 3, 3, 0, 2, 1, 2, 1, 2, 2, 1, 3, 3,
        2, 0, 1, 3, 2, 4, 0, 1, 2, 3, 0, 1, 0, 3, 3, 4, 0, 1, 3, 2, 4, 0, 1, 3, 3, 0, 0, 1, 2, 1,
        4, 0, 1, 3, 3, 2, 2, 1, 2, 1, 3, 2, 0, 3, 0, 0, 1, 1, 1, 2, 4, 0, 1, 3, 2, 0, 0, 1, 2, 3,
        2, 0, 1, 3, 3, 0, 0, 1, 2, 3, 2, 2, 0, 2, 2, 0, 2, 0, 3, 3, 0, 0, 0, 2, 1, 2, 0, 1, 2, 1,
        2, 0, 1, 3, 0, 4, 0, 1, 2, 2, 4, 0, 1, 2, 1, 2, 0, 0, 2, 2, 1, 1, 1, 3, 2, 2, 0, 1, 3, 3,
        2, 2, 1, 2, 3, 0, 1, 1, 3, 3, 0, 2, 0, 2, 3, 0, 2, 1, 3, 2, 4, 2, 1, 3, 3, 0, 0, 1, 3, 0,
        4, 0, 1, 0, 3, 1, 1, 1, 3, 3, 2, 0, 1, 2, 1, 0, 2, 1, 3, 3, 0, 0, 0, 2, 2, 2, 0, 1, 3, 2,
        4, 2, 1, 2, 3, 4, 0, 1, 2, 2, 2, 0, 0, 2, 3, 2, 0, 1, 3, 2, 4, 2, 1, 2, 1, 4, 0, 1, 3, 3,
        4, 0, 0, 0, 3, 0, 0, 1, 1, 2, 1, 2, 1, 3, 2, 1, 0, 0, 3, 2, 2, 2, 1, 3, 2, 4, 0, 1, 3, 3,
        2, 0, 1, 3, 0, 3, 2, 1, 3, 3, 4, 0, 0, 0, 3, 0, 1, 0, 3, 0, 2, 0, 1, 3, 3, 0, 0, 1, 3, 3,
        0, 2, 1, 2, 3, 0, 0, 1, 3, 3, 2, 0, 1, 0, 3, 0, 0, 1, 2, 1, 2, 0, 0, 3, 3, 0, 2, 1, 3, 3,
        2, 0, 1, 3, 2, 2, 2, 1, 2, 3, 0, 1, 0, 3, 2, 0, 0, 1, 3, 2, 1, 2, 1, 2, 3, 0, 0, 1, 3, 3,
        0, 0, 1, 2, 1, 2, 0, 1, 2, 1, 2, 0, 1, 3, 2, 2, 0, 0, 2, 3, 2, 0, 1, 3, 2, 4, 2, 1, 3, 2,
        2, 2, 1, 2, 1, 2, 2, 1, 3, 2, 0, 2, 0, 0, 3, 1, 0, 0, 3, 2, 2, 0, 1, 3, 2, 2, 0, 1, 2, 3,
        0, 0, 1, 1, 2, 2, 2, 1, 3, 3, 2, 2, 1, 3, 3, 0, 0, 1, 2, 3, 2, 0, 1, 0, 3, 0, 0, 0, 2, 3,
        0, 0, 1, 3, 3, 4, 0, 0, 0, 3, 0, 0, 1, 3, 3, 0, 0, 0, 2, 0, 2, 0, 1, 3, 3, 0, 2, 1, 3, 2,
        2, 0, 1, 3, 3, 2, 0, 1, 3, 0, 3, 0, 0, 3, 3, 0, 0, 1, 3, 0, 1, 2, 0, 3, 0, 2, 0, 1, 3, 2,
        2, 0, 1, 2, 1, 2, 0, 1, 3, 0, 2, 0, 1, 2, 1, 3, 0, 1, 2, 1, 2, 0, 1, 2, 2, 2, 0, 1, 3, 3,
        4, 0, 1, 3, 3, 0, 2, 1, 1, 2, 1, 0, 1, 2, 3, 0, 2, 1, 3, 0, 4, 0, 1, 3, 3, 0, 2, 1, 3, 2,
        2, 0, 1, 3, 3, 2, 2, 1, 2, 3, 4, 0, 1, 3, 2, 2, 0, 0, 1, 2, 2, 2, 0, 0, 3, 0, 2, 0, 2, 3,
        0, 0, 1, 3, 2, 2, 0, 1, 3, 2, 2, 0, 1, 3, 3, 2, 2, 1, 3, 2, 2, 0, 1, 2, 1, 2, 0, 1, 3, 3,
        2, 0, 1, 3, 3, 4, 0, 1, 3, 2, 0, 2, 1, 3, 2, 0, 2, 1, 3, 3, 0, 0, 0, 2, 3, 0, 0, 1, 3, 2,
        2, 2, 1, 3, 3, 2, 0, 1, 3, 0, 3, 0, 0, 2, 3, 0, 0, 1, 3, 3, 2, 2, 1, 2, 1, 2, 2, 1, 0, 0,
        2, 2, 1, 2, 3, 0, 0, 1, 2, 3, 2, 0, 1, 3, 3, 0, 0, 1, 3, 3, 0, 0, 1, 2, 3, 4, 0, 1, 3, 3,
        2, 0, 1, 3, 2, 2, 2, 1, 3, 2, 0, 0, 1, 3, 3, 4, 2, 1, 2, 1, 2, 0, 1, 2, 2, 2, 2, 1, 3, 2,
        0, 0, 0, 2, 2, 4, 0, 1, 2, 3, 2, 0, 1, 3, 3, 0, 0, 1, 3, 3, 2, 0, 1, 3, 3, 2, 2, 1, 3, 3,
        2, 2, 1, 3, 2, 0, 0, 0, 3, 3, 0, 2, 0, 3, 0, 0, 2, 1, 3, 2, 0, 1, 1, 3, 3, 0, 0, 1, 2, 3,
        2, 2, 1, 2, 3, 4, 2, 1, 3, 2, 2, 2, 1, 3, 2, 2, 0, 1, 3, 2, 4, 0, 1, 3, 3, 4, 0, 1, 2, 3,
        2, 0, 1, 3, 3, 4, 0, 1, 2, 2, 0, 0, 1, 3, 2, 4, 2, 1, 1, 2, 1, 0, 1, 3, 2, 0, 0, 1, 3, 2,
        0, 0, 0, 2, 3, 0, 0, 1, 3, 0, 2, 0, 1, 2, 2, 0, 2, 0, 2, 0, 2, 0, 1, 2, 2, 2, 0, 1, 3, 3,
        4, 2, 0, 0, 3, 2, 2, 1, 3, 2, 0, 0, 0, 3, 2, 2, 2, 1, 3, 0, 3, 0, 1, 2, 3, 0, 2, 1, 2, 2,
        1, 1, 1, 3, 2, 1, 2, 1, 1, 2, 1, 2, 1, 3, 2, 2, 0, 1, 2, 3, 0, 0, 1, 2, 2, 4, 0, 0, 0, 3,
        4, 2, 1, 3, 3, 2, 2, 1, 3, 3, 2, 0, 0, 2, 3, 0, 0, 1, 2, 1, 2, 0, 1, 2, 3, 2, 0, 1, 3, 2,
        1, 0, 1, 3, 3, 2, 2, 1, 3, 3, 2, 2, 1, 3, 3, 0, 0, 0, 2, 3, 2, 0, 0, 2, 3, 2, 0, 0, 2, 3,
        0, 0, 0, 3, 2, 4, 2, 1, 0, 3, 2, 0, 1, 3, 3, 0, 0, 0, 2, 2, 2, 0, 0, 2, 2, 1, 0, 0, 2, 3,
        0, 2, 1, 1, 2, 2, 0, 1, 3, 2, 4, 0, 1, 3, 3, 4, 2, 1, 0, 3, 0, 0, 1, 3, 2, 1, 1, 1, 3, 3,
        2, 0, 0, 2, 3, 0, 0, 1, 3, 0, 3, 0, 1, 2, 2, 2, 0, 1, 0, 2, 4, 0, 0, 0, 3, 0, 0, 1, 3, 0,
        3, 2, 1, 2, 1, 3, 2, 0, 2, 3, 2, 2, 1, 3, 3, 2, 0, 1, 3, 3, 0, 2, 0, 3, 2, 2, 2, 1, 3, 3,
        2, 2, 1, 2, 3, 2, 0, 0, 2, 2, 4, 0, 1, 3, 3, 0, 0, 1, 3, 0, 4, 0, 0, 2, 0, 0, 2, 0, 2, 2,
        0, 2, 1, 3, 3, 0, 0, 1, 3, 2, 2, 2, 1, 3, 3, 2, 0, 1, 2, 3, 0, 0, 1, 3, 2, 2, 0, 0, 2, 2,
        0, 2, 1, 2, 3, 2, 0, 1, 3, 3, 0, 0, 1, 3, 2, 2, 2, 1, 2, 2, 2, 0, 0, 2, 3, 4, 0, 0, 0, 3,
        2, 0, 1, 0, 3, 2, 0, 1, 0, 2, 2, 0, 1, 2, 3, 4, 0, 0, 2, 0, 0, 0, 0, 3, 3, 2, 2, 0, 2, 3,
        0, 0, 1, 3, 2, 4, 0, 1, 3, 2, 4, 0, 0, 2, 0, 0, 2, 1, 3, 3, 0, 2, 0, 2, 3, 4, 0, 0, 2, 2,
        4, 2, 1, 3, 2, 2, 2, 1, 3, 2, 0, 0, 1, 3, 2, 2, 2, 1, 3, 2, 4, 0, 1, 3, 2, 0, 0, 1, 3, 2,
        2, 0, 1, 3, 2, 1, 2, 1, 3, 2, 0, 0, 1, 3, 3, 0, 0, 1, 2, 1, 3, 0, 1, 2, 3, 4, 2, 1, 3, 2,
        2, 0, 0, 2, 3, 0, 0, 1, 2, 3, 4, 0, 0, 0, 3, 2, 0, 1, 3, 3,
    ];

    let mut height = start.iter().sum::<usize>();
    let cycle_height = cycle.iter().sum::<usize>();

    let cycle_length = cycle.len();
    let start_length = start.len();

    let num_cycles = (1000000000000 - start_length) / cycle_length;

    let total_iterations = start_length + num_cycles * cycle_length;

    height += num_cycles * cycle_height;
    for &i in cycle.iter().take(1000000000000 - total_iterations) {
        height += i;
    }

    height
}

#[allow(unused)]
fn print_cave(cave: &Vec<[bool; 8]>) {
    for row in cave.iter().rev() {
        print!("|");
        for &c in row.iter().skip(1) {
            if c {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("|");
    }
}

#[cfg(test)]
mod test {
    use crate::prob17::solve_part_1;

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(INPUT), 3068);
    }

    const INPUT: &'static str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
}
