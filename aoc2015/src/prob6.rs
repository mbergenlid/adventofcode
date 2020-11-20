
pub fn solve_part_1() {
    let mut grid = Grid::new();

    for cmd in input() {
        match cmd {
            Command::TurnOff(rect) => {
                grid.apply(&rect, |_| false);
            }
            Command::TurnOn(rect) => {
                grid.apply(&rect, |_| true);
            }
            Command::Toggle(rect) => {
                grid.apply(&rect, |b| !b);
            }
        }
    }

    println!("Part 1: {}", grid.count_lights_on());
}

pub fn solve_part_2() {
    let mut grid: Grid<u32> = Grid::new();

    for cmd in input() {
        match cmd {
            Command::TurnOff(rect) => {
                grid.apply(&rect, |v| if v == 0 { 0 } else { v - 1 });
            }
            Command::TurnOn(rect) => {
                grid.apply(&rect, |v| v + 1);
            }
            Command::Toggle(rect) => {
                grid.apply(&rect, |v| v + 2);
            }
        }
    }

    println!("Part 2: {}", grid.iter().sum::<u32>());
}

struct Grid<T>([[T; 1000]; 1000]);

impl <T: Default + Copy> Grid<T> {
    fn new() -> Grid<T> {
        Grid([[T::default(); 1000]; 1000])
    }

    fn apply<F>(&mut self, rect: &Rect, func: F) where F: Fn(T) -> T + Sized {
        for y in ((rect.0).1)..((rect.1).1+1) {
            for x in ((rect.0).0)..((rect.1).0+1) {
                self.0[y][x] = func(self.0[y][x]);
            }
        }
    }

    fn iter(&self) -> impl Iterator<Item=&T> {
        self.0.iter().flat_map(|r| r.iter())
    }

}

impl Grid<bool> {
    fn count_lights_on(&self) -> usize {
        self.0.iter().flat_map(|r| r.iter()).filter(|&&b| b).count()
    }
}

struct Rect((usize, usize), (usize, usize));


enum Command {
    TurnOff(Rect),
    TurnOn(Rect),
    Toggle(Rect),
}

fn input() -> Vec<Command> {
    vec![
        Command::TurnOff(Rect((660, 55), (986, 197))),
        Command::TurnOff(Rect((341, 304), (638, 850))),
        Command::TurnOff(Rect((199, 133), (461, 193))),
        Command::Toggle(Rect((322, 558), (977, 958))),
        Command::Toggle(Rect((537, 781), (687, 941))),
        Command::TurnOn(Rect((226, 196), (599, 390))),
        Command::TurnOn(Rect((240, 129), (703, 297))),
        Command::TurnOn(Rect((317, 329), (451, 798))),
        Command::TurnOn(Rect((957, 736), (977, 890))),
        Command::TurnOn(Rect((263, 530), (559, 664))),
        Command::TurnOn(Rect((158, 270), (243, 802))),
        Command::Toggle(Rect((223, 39), (454, 511))),
        Command::Toggle(Rect((544, 218), (979, 872))),
        Command::TurnOn(Rect((313, 306), (363, 621))),
        Command::Toggle(Rect((173, 401), (496, 407))),
        Command::Toggle(Rect((333, 60), (748, 159))),
        Command::TurnOff(Rect((87, 577), (484, 608))),
        Command::TurnOn(Rect((809, 648), (826, 999))),
        Command::Toggle(Rect((352, 432), (628, 550))),
        Command::TurnOff(Rect((197, 408), (579, 569))),
        Command::TurnOff(Rect((1, 629), (802, 633))),
        Command::TurnOff(Rect((61, 44), (567, 111))),
        Command::Toggle(Rect((880, 25), (903, 973))),
        Command::TurnOn(Rect((347, 123), (864, 746))),
        Command::Toggle(Rect((728, 877), (996, 975))),
        Command::TurnOn(Rect((121, 895), (349, 906))),
        Command::TurnOn(Rect((888, 547), (931, 628))),
        Command::Toggle(Rect((398, 782), (834, 882))),
        Command::TurnOn(Rect((966, 850), (989, 953))),
        Command::TurnOff(Rect((891, 543), (914, 991))),
        Command::Toggle(Rect((908, 77), (916, 117))),
        Command::TurnOn(Rect((576, 900), (943, 934))),
        Command::TurnOff(Rect((580, 170), (963, 206))),
        Command::TurnOn(Rect((184, 638), (192, 944))),
        Command::Toggle(Rect((940, 147), (978, 730))),
        Command::TurnOff(Rect((854, 56), (965, 591))),
        Command::Toggle(Rect((717, 172), (947, 995))),
        Command::Toggle(Rect((426, 987), (705, 998))),
        Command::TurnOn(Rect((987, 157), (992, 278))),
        Command::Toggle(Rect((995, 774), (997, 784))),
        Command::TurnOff(Rect((796, 96), (845, 182))),
        Command::TurnOff(Rect((451, 87), (711, 655))),
        Command::TurnOff(Rect((380, 93), (968, 676))),
        Command::TurnOn(Rect((263, 468), (343, 534))),
        Command::TurnOn(Rect((917, 936), (928, 959))),
        Command::Toggle(Rect((478, 7), (573, 148))),
        Command::TurnOff(Rect((428, 339), (603, 624))),
        Command::TurnOff(Rect((400, 880), (914, 953))),
        Command::Toggle(Rect((679, 428), (752, 779))),
        Command::TurnOff(Rect((697, 981), (709, 986))),
        Command::Toggle(Rect((482, 566), (505, 725))),
        Command::TurnOff(Rect((956, 368), (993, 516))),
        Command::Toggle(Rect((735, 823), (783, 883))),
        Command::TurnOff(Rect((48, 487), (892, 496))),
        Command::TurnOff(Rect((116, 680), (564, 819))),
        Command::TurnOn(Rect((633, 865), (729, 930))),
        Command::TurnOff(Rect((314, 618), (571, 922))),
        Command::Toggle(Rect((138, 166), (936, 266))),
        Command::TurnOn(Rect((444, 732), (664, 960))),
        Command::TurnOff(Rect((109, 337), (972, 497))),
        Command::TurnOff(Rect((51, 432), (77, 996))),
        Command::TurnOff(Rect((259, 297), (366, 744))),
        Command::Toggle(Rect((801, 130), (917, 544))),
        Command::Toggle(Rect((767, 982), (847, 996))),
        Command::TurnOn(Rect((216, 507), (863, 885))),
        Command::TurnOff(Rect((61, 441), (465, 731))),
        Command::TurnOn(Rect((849, 970), (944, 987))),
        Command::Toggle(Rect((845, 76), (852, 951))),
        Command::Toggle(Rect((732, 615), (851, 936))),
        Command::Toggle(Rect((251, 128), (454, 778))),
        Command::TurnOn(Rect((324, 429), (352, 539))),
        Command::Toggle(Rect((52, 450), (932, 863))),
        Command::TurnOff(Rect((449, 379), (789, 490))),
        Command::TurnOn(Rect((317, 319), (936, 449))),
        Command::Toggle(Rect((887, 670), (957, 838))),
        Command::Toggle(Rect((671, 613), (856, 664))),
        Command::TurnOff(Rect((186, 648), (985, 991))),
        Command::TurnOff(Rect((471, 689), (731, 717))),
        Command::Toggle(Rect((91, 331), (750, 758))),
        Command::Toggle(Rect((201, 73), (956, 524))),
        Command::Toggle(Rect((82, 614), (520, 686))),
        Command::Toggle(Rect((84, 287), (467, 734))),
        Command::TurnOff(Rect((132, 367), (208, 838))),
        Command::Toggle(Rect((558, 684), (663, 920))),
        Command::TurnOn(Rect((237, 952), (265, 997))),
        Command::TurnOn(Rect((694, 713), (714, 754))),
        Command::TurnOn(Rect((632, 523), (862, 827))),
        Command::TurnOn(Rect((918, 780), (948, 916))),
        Command::TurnOn(Rect((349, 586), (663, 976))),
        Command::Toggle(Rect((231, 29), (257, 589))),
        Command::Toggle(Rect((886, 428), (902, 993))),
        Command::TurnOn(Rect((106, 353), (236, 374))),
        Command::TurnOn(Rect((734, 577), (759, 684))),
        Command::TurnOff(Rect((347, 843), (696, 912))),
        Command::TurnOn(Rect((286, 699), (964, 883))),
        Command::TurnOn(Rect((605, 875), (960, 987))),
        Command::TurnOff(Rect((328, 286), (869, 461))),
        Command::TurnOff(Rect((472, 569), (980, 848))),
        Command::Toggle(Rect((673, 573), (702, 884))),
        Command::TurnOff(Rect((398, 284), (738, 332))),
        Command::TurnOn(Rect((158, 50), (284, 411))),
        Command::TurnOff(Rect((390, 284), (585, 663))),
        Command::TurnOn(Rect((156, 579), (646, 581))),
        Command::TurnOn(Rect((875, 493), (989, 980))),
        Command::Toggle(Rect((486, 391), (924, 539))),
        Command::TurnOn(Rect((236, 722), (272, 964))),
        Command::Toggle(Rect((228, 282), (470, 581))),
        Command::Toggle(Rect((584, 389), (750, 761))),
        Command::TurnOff(Rect((899, 516), (900, 925))),
        Command::TurnOn(Rect((105, 229), (822, 846))),
        Command::TurnOff(Rect((253, 77), (371, 877))),
        Command::TurnOn(Rect((826, 987), (906, 992))),
        Command::TurnOff(Rect((13, 152), (615, 931))),
        Command::TurnOn(Rect((835, 320), (942, 399))),
        Command::TurnOn(Rect((463, 504), (536, 720))),
        Command::Toggle(Rect((746, 942), (786, 998))),
        Command::TurnOff(Rect((867, 333), (965, 403))),
        Command::TurnOn(Rect((591, 477), (743, 692))),
        Command::TurnOff(Rect((403, 437), (508, 908))),
        Command::TurnOn(Rect((26, 723), (368, 814))),
        Command::TurnOn(Rect((409, 485), (799, 809))),
        Command::TurnOn(Rect((115, 630), (704, 705))),
        Command::TurnOff(Rect((228, 183), (317, 220))),
        Command::Toggle(Rect((300, 649), (382, 842))),
        Command::TurnOff(Rect((495, 365), (745, 562))),
        Command::TurnOn(Rect((698, 346), (744, 873))),
        Command::TurnOn(Rect((822, 932), (951, 934))),
        Command::Toggle(Rect((805, 30), (925, 421))),
        Command::Toggle(Rect((441, 152), (653, 274))),
        Command::Toggle(Rect((160, 81), (257, 587))),
        Command::TurnOff(Rect((350, 781), (532, 917))),
        Command::Toggle(Rect((40, 583), (348, 636))),
        Command::TurnOn(Rect((280, 306), (483, 395))),
        Command::Toggle(Rect((392, 936), (880, 955))),
        Command::Toggle(Rect((496, 591), (851, 934))),
        Command::TurnOff(Rect((780, 887), (946, 994))),
        Command::TurnOff(Rect((205, 735), (281, 863))),
        Command::Toggle(Rect((100, 876), (937, 915))),
        Command::TurnOn(Rect((392, 393), (702, 878))),
        Command::TurnOn(Rect((956, 374), (976, 636))),
        Command::Toggle(Rect((478, 262), (894, 775))),
        Command::TurnOff(Rect((279, 65), (451, 677))),
        Command::TurnOn(Rect((397, 541), (809, 847))),
        Command::TurnOn(Rect((444, 291), (451, 586))),
        Command::Toggle(Rect((721, 408), (861, 598))),
        Command::TurnOn(Rect((275, 365), (609, 382))),
        Command::TurnOn(Rect((736, 24), (839, 72))),
        Command::TurnOff(Rect((86, 492), (582, 712))),
        Command::TurnOn(Rect((676, 676), (709, 703))),
        Command::TurnOff(Rect((105, 710), (374, 817))),
        Command::Toggle(Rect((328, 748), (845, 757))),
        Command::Toggle(Rect((335, 79), (394, 326))),
        Command::Toggle(Rect((193, 157), (633, 885))),
        Command::TurnOn(Rect((227, 48), (769, 743))),
        Command::Toggle(Rect((148, 333), (614, 568))),
        Command::Toggle(Rect((22, 30), (436, 263))),
        Command::Toggle(Rect((547, 447), (688, 969))),
        Command::Toggle(Rect((576, 621), (987, 740))),
        Command::TurnOn(Rect((711, 334), (799, 515))),
        Command::TurnOn(Rect((541, 448), (654, 951))),
        Command::Toggle(Rect((792, 199), (798, 990))),
        Command::TurnOn(Rect((89, 956), (609, 960))),
        Command::Toggle(Rect((724, 433), (929, 630))),
        Command::Toggle(Rect((144, 895), (201, 916))),
        Command::Toggle(Rect((226, 730), (632, 871))),
        Command::TurnOff(Rect((760, 819), (828, 974))),
        Command::Toggle(Rect((887, 180), (940, 310))),
        Command::Toggle(Rect((222, 327), (805, 590))),
        Command::TurnOff(Rect((630, 824), (885, 963))),
        Command::TurnOn(Rect((940, 740), (954, 946))),
        Command::TurnOn(Rect((193, 373), (779, 515))),
        Command::Toggle(Rect((304, 955), (469, 975))),
        Command::TurnOff(Rect((405, 480), (546, 960))),
        Command::TurnOn(Rect((662, 123), (690, 669))),
        Command::TurnOff(Rect((615, 238), (750, 714))),
        Command::TurnOn(Rect((423, 220), (930, 353))),
        Command::TurnOn(Rect((329, 769), (358, 970))),
        Command::Toggle(Rect((590, 151), (704, 722))),
        Command::TurnOff(Rect((884, 539), (894, 671))),
        Command::Toggle(Rect((449, 241), (984, 549))),
        Command::Toggle(Rect((449, 260), (496, 464))),
        Command::TurnOff(Rect((306, 448), (602, 924))),
        Command::TurnOn(Rect((286, 805), (555, 901))),
        Command::Toggle(Rect((722, 177), (922, 298))),
        Command::Toggle(Rect((491, 554), (723, 753))),
        Command::TurnOn(Rect((80, 849), (174, 996))),
        Command::TurnOff(Rect((296, 561), (530, 856))),
        Command::Toggle(Rect((653, 10), (972, 284))),
        Command::Toggle(Rect((529, 236), (672, 614))),
        Command::Toggle(Rect((791, 598), (989, 695))),
        Command::TurnOn(Rect((19, 45), (575, 757))),
        Command::Toggle(Rect((111, 55), (880, 871))),
        Command::TurnOff(Rect((197, 897), (943, 982))),
        Command::TurnOn(Rect((912, 336), (977, 605))),
        Command::Toggle(Rect((101, 221), (537, 450))),
        Command::TurnOn(Rect((101, 104), (969, 447))),
        Command::Toggle(Rect((71, 527), (587, 717))),
        Command::Toggle(Rect((336, 445), (593, 889))),
        Command::Toggle(Rect((214, 179), (575, 699))),
        Command::TurnOn(Rect((86, 313), (96, 674))),
        Command::Toggle(Rect((566, 427), (906, 888))),
        Command::TurnOff(Rect((641, 597), (850, 845))),
        Command::TurnOn(Rect((606, 524), (883, 704))),
        Command::TurnOn(Rect((835, 775), (867, 887))),
        Command::Toggle(Rect((547, 301), (897, 515))),
        Command::Toggle(Rect((289, 930), (413, 979))),
        Command::TurnOn(Rect((361, 122), (457, 226))),
        Command::TurnOn(Rect((162, 187), (374, 746))),
        Command::TurnOn(Rect((348, 461), (454, 675))),
        Command::TurnOff(Rect((966, 532), (985, 537))),
        Command::TurnOn(Rect((172, 354), (630, 606))),
        Command::TurnOff(Rect((501, 880), (680, 993))),
        Command::TurnOff(Rect((8, 70), (566, 592))),
        Command::Toggle(Rect((433, 73), (690, 651))),
        Command::Toggle(Rect((840, 798), (902, 971))),
        Command::Toggle(Rect((822, 204), (893, 760))),
        Command::TurnOff(Rect((453, 496), (649, 795))),
        Command::TurnOff(Rect((969, 549), (990, 942))),
        Command::TurnOff(Rect((789, 28), (930, 267))),
        Command::Toggle(Rect((880, 98), (932, 434))),
        Command::Toggle(Rect((568, 674), (669, 753))),
        Command::TurnOn(Rect((686, 228), (903, 271))),
        Command::TurnOn(Rect((263, 995), (478, 999))),
        Command::Toggle(Rect((534, 675), (687, 955))),
        Command::TurnOff(Rect((342, 434), (592, 986))),
        Command::Toggle(Rect((404, 768), (677, 867))),
        Command::Toggle(Rect((126, 723), (978, 987))),
        Command::Toggle(Rect((749, 675), (978, 959))),
        Command::TurnOff(Rect((445, 330), (446, 885))),
        Command::TurnOff(Rect((463, 205), (924, 815))),
        Command::TurnOff(Rect((417, 430), (915, 472))),
        Command::TurnOn(Rect((544, 990), (912, 999))),
        Command::TurnOff(Rect((201, 255), (834, 789))),
        Command::TurnOff(Rect((261, 142), (537, 862))),
        Command::TurnOff(Rect((562, 934), (832, 984))),
        Command::TurnOff(Rect((459, 978), (691, 980))),
        Command::TurnOff(Rect((73, 911), (971, 972))),
        Command::TurnOn(Rect((560, 448), (723, 810))),
        Command::TurnOn(Rect((204, 630), (217, 854))),
        Command::TurnOff(Rect((91, 259), (611, 607))),
        Command::TurnOn(Rect((877, 32), (978, 815))),
        Command::TurnOff(Rect((950, 438), (974, 746))),
        Command::Toggle(Rect((426, 30), (609, 917))),
        Command::Toggle(Rect((696, 37), (859, 201))),
        Command::Toggle(Rect((242, 417), (682, 572))),
        Command::TurnOff(Rect((388, 401), (979, 528))),
        Command::TurnOff(Rect((79, 345), (848, 685))),
        Command::TurnOff(Rect((98, 91), (800, 434))),
        Command::Toggle(Rect((650, 700), (972, 843))),
        Command::TurnOff(Rect((530, 450), (538, 926))),
        Command::TurnOn(Rect((428, 559), (962, 909))),
        Command::TurnOn(Rect((78, 138), (92, 940))),
        Command::Toggle(Rect((194, 117), (867, 157))),
        Command::Toggle(Rect((785, 355), (860, 617))),
        Command::TurnOff(Rect((379, 441), (935, 708))),
        Command::TurnOff(Rect((605, 133), (644, 911))),
        Command::Toggle(Rect((10, 963), (484, 975))),
        Command::TurnOff(Rect((359, 988), (525, 991))),
        Command::TurnOff(Rect((509, 138), (787, 411))),
        Command::Toggle(Rect((556, 467), (562, 773))),
        Command::TurnOn(Rect((119, 486), (246, 900))),
        Command::TurnOn(Rect((445, 561), (794, 673))),
        Command::TurnOff(Rect((598, 681), (978, 921))),
        Command::TurnOff(Rect((974, 230), (995, 641))),
        Command::TurnOff(Rect((760, 75), (800, 275))),
        Command::Toggle(Rect((441, 215), (528, 680))),
        Command::TurnOff(Rect((701, 636), (928, 877))),
        Command::TurnOn(Rect((165, 753), (202, 780))),
        Command::Toggle(Rect((501, 412), (998, 516))),
        Command::Toggle(Rect((161, 105), (657, 395))),
        Command::TurnOn(Rect((113, 340), (472, 972))),
        Command::Toggle(Rect((384, 994), (663, 999))),
        Command::TurnOn(Rect((969, 994), (983, 997))),
        Command::TurnOn(Rect((519, 600), (750, 615))),
        Command::TurnOff(Rect((363, 899), (948, 935))),
        Command::TurnOn(Rect((271, 845), (454, 882))),
        Command::TurnOff(Rect((376, 528), (779, 640))),
        Command::Toggle(Rect((767, 98), (854, 853))),
        Command::Toggle(Rect((107, 322), (378, 688))),
        Command::TurnOff(Rect((235, 899), (818, 932))),
        Command::TurnOn(Rect((445, 611), (532, 705))),
        Command::Toggle(Rect((629, 387), (814, 577))),
        Command::Toggle(Rect((112, 414), (387, 421))),
        Command::Toggle(Rect((319, 184), (382, 203))),
        Command::TurnOn(Rect((627, 796), (973, 940))),
        Command::Toggle(Rect((602, 45), (763, 151))),
        Command::TurnOff(Rect((441, 375), (974, 545))),
        Command::Toggle(Rect((871, 952), (989, 998))),
        Command::TurnOn(Rect((717, 272), (850, 817))),
        Command::Toggle(Rect((475, 711), (921, 882))),
        Command::Toggle(Rect((66, 191), (757, 481))),
        Command::TurnOff(Rect((50, 197), (733, 656))),
        Command::Toggle(Rect((83, 575), (915, 728))),
        Command::TurnOn(Rect((777, 812), (837, 912))),
        Command::TurnOn(Rect((20, 984), (571, 994))),
        Command::TurnOff(Rect((446, 432), (458, 648))),
        Command::TurnOn(Rect((715, 871), (722, 890))),
        Command::Toggle(Rect((424, 675), (740, 862))),
        Command::Toggle(Rect((580, 592), (671, 900))),
        Command::Toggle(Rect((296, 687), (906, 775))),
    ]
}
