use std::collections::HashMap;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Chemical {
    unit: String,
    quantity: u32,
}

impl Chemical {
    fn new(quantity: u32, unit: &str) -> Chemical {
        Chemical {
            quantity: quantity,
            unit: unit.to_owned(),
        }
    }
}

#[derive(Debug)]
struct Reaction {
    result: Chemical,
    source: Vec<Chemical>,
}

pub fn solve_part_1() {
    let ore = solve_for_part_1(&parse(&input()));
    println!("Part 1: {}", ore);
}

pub fn solve_part_2() {
    let solution = solve_for_part_2(&parse(&input()));
    println!("Part 2: {}", solution);
}

fn solve_for_part_2(reactions: &[Reaction]) -> u64 {
    let mut total_ore: u64 = 1000000000000;
    let mut extras = HashMap::new();
    let mut total_fuel = 0;
    while total_ore > 0 {
        println!("{}", total_ore);
        let ore_for_one_fuel = find_least_amount_of_ore(reactions, &mut extras) as u64;

        if ore_for_one_fuel > total_ore {
            total_ore = 0;
        } else {
            total_fuel += 1;
            total_ore -= ore_for_one_fuel;
        }
    }
    total_fuel
}

fn find_least_amount_of_ore(
    reactions: &[Reaction],
    extra_chemicals: &mut HashMap<String, u32>,
) -> u32 {
    let fuel = reactions
        .iter()
        .filter(|r| r.result.unit == "FUEL")
        .next()
        .unwrap();
    let mut required_ore = 0;
    let mut required_stuff: Vec<Chemical> = Vec::from(fuel.source.as_slice());
    while !required_stuff.is_empty() {
        let stuff: Chemical = required_stuff.remove(0);
        if stuff.unit == "ORE" {
            required_ore += stuff.quantity;
        } else {
            let reaction = reactions
                .iter()
                .filter(|r| r.result.unit == stuff.unit)
                .next()
                .unwrap();
            let how_much_we_get = reaction.result.quantity;
            let extras = *extra_chemicals.get(&stuff.unit).unwrap_or(&0);
            let how_much_we_need = if extras > 0 {
                if extras > stuff.quantity {
                    extra_chemicals.insert(stuff.unit.clone(), extras - stuff.quantity);
                    0
                } else {
                    extra_chemicals.insert(stuff.unit.clone(), 0);
                    stuff.quantity - extras
                }
            } else {
                stuff.quantity
            };
            let how_many_we_need = how_much_we_need / how_much_we_get;
            if how_much_we_need > 0 {
                for c in reaction.source.iter() {
                    required_stuff.push(Chemical::new(
                        c.quantity * how_many_we_need
                            + if how_much_we_need % how_much_we_get == 0 {
                                0
                            } else {
                                c.quantity
                            },
                        c.unit.as_str(),
                    ));
                }
            }
            if how_many_we_need * how_much_we_get < how_much_we_need {
                extra_chemicals.insert(
                    reaction.result.unit.clone(),
                    (how_many_we_need + 1) * how_much_we_get - how_much_we_need,
                );
            }
            //println!(
            //    "{} {} -> {}/{} :: {} => {:?}",
            //    stuff.quantity,
            //    stuff.unit,
            //    how_much_we_need,
            //    how_many_we_need,
            //    extra_chemicals.get(&reaction.result.unit).unwrap_or(&0),
            //    required_stuff,
            //);
        }
    }
    required_ore
}

fn solve_for_part_1(reactions: &[Reaction]) -> u32 {
    find_least_amount_of_ore(reactions, &mut HashMap::new())
}

fn parse_chemical(s: &str) -> Chemical {
    let mut split = s.split(" ");
    Chemical::new(
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap(),
    )
}

fn parse_row(row: &str) -> Reaction {
    let mut source_and_result = row.split(" => ");
    Reaction {
        source: source_and_result
            .next()
            .unwrap()
            .split(", ")
            .map(|s| parse_chemical(s))
            .collect(),
        result: parse_chemical(source_and_result.next().unwrap()),
    }
}

fn parse(s: &str) -> Vec<Reaction> {
    s.split("\n").map(|r| parse_row(r)).collect()
}

fn input() -> String {
    "1 HVXJL, 1 JHGQ => 2 ZQFQ\n6 GRQTX => 6 VZWRS\n128 ORE => 2 GRQTX\n1 MJPSW => 4 MGZBH\n3 HLQX => 8 KSMW\n4 QLNS => 9 LFRW\n10 HBCN => 3 CZWP\n1 CQRJP => 9 MJPSW\n1 SLXC => 6 SDTGP\n1 MTGVK => 4 NZWLQ\n4 PMJX => 3 CVKM\n2 LDKGL, 2 SFKF => 5 XZDV\n1 QLNS, 1 VZWRS => 5 RSBT\n1 NRQS, 22 LQFDM => 4 PMJX\n17 XZDV, 8 GSRKQ => 3 ZGDC\n11 BPJLM, 18 ZGDC, 1 JHGQ => 5 BXNJX\n2 GRQTX, 1 CQRJP => 7 NRQS\n1 LJTL => 7 DBHXK\n15 HPBQ, 5 PSPCF, 1 JHGQ, 25 ZMXWG, 1 JTZS, 1 SDTGP, 3 NLBM => 6 MQVLS\n9 KSMW => 2 GXTBV\n3 HVXJL => 5 JHGQ\n1 ZWXT, 13 MJPSW, 10 HVXJL => 5 LDKGL\n1 GRQTX => 2 LQFDM\n190 ORE => 5 FQPNW\n1 GTQB => 9 HVHN\n1 TNLN, 9 HVHN, 1 WLGT, 4 NZMZ, 2 QTPC, 1 LPTF => 7 WFCH\n3 PMJX => 5 SFKF\n1 ZGDC => 9 HTVR\n193 ORE => 1 CQRJP\n1 BPJLM, 1 HPBQ, 3 HVHN => 6 NLBM\n2 SFKF => 1 GSRKQ\n1 ZGDC => 8 GTQB\n1 LSPMR, 53 LDKGL, 24 WFCH, 32 GDLH, 2 HLQX, 14 NLBM, 18 BDZK, 7 MDSRW, 9 MQVLS => 1 FUEL\n12 SFKF => 7 NZMZ\n13 PVJM => 3 XBTH\n7 GSRKQ, 7 LPTF, 1 HLQX, 1 FJHK, 1 DHVM, 3 SFKF, 15 NLBM, 2 SDTGP => 3 LSPMR\n4 LFRW, 28 MJPSW => 4 GDLH\n6 VZWRS, 8 MJPSW => 8 HVXJL\n13 LFRW => 4 ZWQW\n1 LQFDM, 7 NZWLQ, 2 HVXJL => 4 HLQX\n2 KSMW, 1 WDGN, 4 ZQFQ => 1 ZMXWG\n3 MGZBH => 2 LPTF\n1 LFRW, 1 CVKM, 3 LDKGL => 4 LJTL\n3 LJTL, 20 CZWP, 1 HPBQ => 9 WLGT\n3 FQPNW => 8 MTGVK\n1 MTDWJ, 1 CVKM => 9 WDGN\n5 ZWQW => 3 MTDWJ\n2 CVKM => 8 QTPC\n2 PVJM, 9 ZWQW, 1 MTDWJ => 4 HBCN\n5 RSBT, 2 WDGN, 6 GSRKQ => 1 BPJLM\n34 JHGQ, 6 ZGDC => 8 DHVM\n3 QTPC, 1 RSBT, 1 GXTBV => 9 JTZS\n1 BXNJX, 2 JTZS => 5 SLXC\n23 LPTF, 2 NZMZ => 4 TNLN\n24 HTVR, 5 DBHXK => 2 FJHK\n5 LPTF, 5 QTPC => 4 PSPCF\n17 MTGVK, 27 LQFDM => 4 QLNS\n1 CVKM, 5 HTVR => 8 HPBQ\n6 ZQFQ, 28 XBTH => 7 MDSRW\n13 WDGN => 5 BDZK\n1 MJPSW, 2 VZWRS => 4 ZWXT\n1 MGZBH, 1 GRQTX => 8 PVJM".to_owned()
}

#[cfg(test)]
mod test {
    use super::{Chemical, Reaction};

    #[test]
    fn single_reaction() {
        assert_eq!(
            super::solve_for_part_1(&vec!(
                Reaction {
                    source: vec!(Chemical::new(10, "ORE")),
                    result: Chemical::new(2, "A")
                },
                Reaction {
                    source: vec!(Chemical::new(3, "A")),
                    result: Chemical::new(1, "FUEL")
                }
            )),
            20
        );
    }

    #[test]
    fn test() {
        println!("{:?}", super::parse(&"10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL".to_owned()));
        assert_eq!(
            super::solve_for_part_1(
                &super::parse(&"10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL".to_owned())
            ),
            31
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(
            super::solve_for_part_1(&vec!(
                Reaction {
                    source: vec!(Chemical::new(9, "ORE")),
                    result: Chemical::new(2, "A")
                },
                Reaction {
                    source: vec!(Chemical::new(8, "ORE")),
                    result: Chemical::new(3, "B")
                },
                Reaction {
                    source: vec!(Chemical::new(7, "ORE")),
                    result: Chemical::new(5, "C")
                },
                Reaction {
                    source: vec!(Chemical::new(3, "A"), Chemical::new(4, "B")),
                    result: Chemical::new(1, "AB")
                },
                Reaction {
                    source: vec!(Chemical::new(5, "B"), Chemical::new(7, "C")),
                    result: Chemical::new(1, "BC")
                },
                Reaction {
                    source: vec!(Chemical::new(4, "C"), Chemical::new(1, "A")),
                    result: Chemical::new(1, "CA")
                },
                Reaction {
                    source: vec!(
                        Chemical::new(2, "AB"),
                        Chemical::new(3, "BC"),
                        Chemical::new(4, "CA")
                    ),
                    result: Chemical::new(1, "FUEL")
                },
            )),
            165
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            super::solve_for_part_1(&super::parse(
&"157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT".to_owned())),
            13312
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            super::solve_for_part_2(&super::parse(
&"157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT".to_owned())),
            82892753
        );
    }
}
