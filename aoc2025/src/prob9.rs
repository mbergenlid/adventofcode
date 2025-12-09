pub fn solve_part_1(input: &str) -> usize {
    let tiles = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut largest_rect = 0;
    for (c1_x, c1_y) in &tiles {
        for (c2_x, c2_y) in &tiles {
            let area = ((c1_x - c2_x).abs() + 1) * ((c1_y - c2_y).abs() + 1);
            if area > largest_rect {
                largest_rect = area;
            }
        }
    }
    largest_rect as usize
}

pub fn solve_part_2(input: &str) -> usize {
    let tiles = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut rectangles = Vec::with_capacity(tiles.len() * tiles.len());
    for (index, (c1_x, c1_y)) in tiles.iter().enumerate() {
        for (c2_x, c2_y) in tiles.iter().skip(index + 1) {
            let area = ((c1_x - c2_x).abs() + 1) * ((c1_y - c2_y).abs() + 1);
            rectangles.push((area, Rectangle::new((*c1_x, *c1_y), (*c2_x, *c2_y))));
        }
    }
    rectangles.sort_by_key(|(area, _)| *area);

    let path = Path::new(tiles);
    for (area, rect) in rectangles.iter().rev() {
        if path.is_inside_path(rect) {
            return *area as usize;
        }
    }

    panic!("No solution found");
}

#[derive(Debug)]
struct VerticalLineSegment {
    x: i64,
    low_y: i64,
    high_y: i64,
}

struct HorizontalLineSegment {
    y: i64,
    low_x: i64,
    high_x: i64,
}

struct Path {
    vertical_edges: Vec<VerticalLineSegment>,
    horizontal_edges: Vec<HorizontalLineSegment>,
}

//
// #########
// #       #
// #  #### #
// #  #  # #
// ####  ###
//    +  +
//    ++++
impl Path {
    fn new(nodes: Vec<(i64, i64)>) -> Self {
        let mut verticals = Vec::new();
        let mut horizontals = Vec::new();
        for i in 0..nodes.len() {
            let start = nodes[i];
            let end = nodes[(i + 1) % nodes.len()];

            if start.0 == end.0 {
                //Vertical line
                let low_y = start.1.min(end.1);
                let high_y = start.1.max(end.1);
                verticals.push(VerticalLineSegment {
                    x: start.0,
                    low_y,
                    high_y,
                });
            }
            if start.1 == end.1 {
                //Vertical line
                let low_x = start.0.min(end.0);
                let high_x = start.0.max(end.0);
                horizontals.push(HorizontalLineSegment {
                    y: start.1,
                    low_x,
                    high_x,
                });
            }
        }
        Self {
            vertical_edges: verticals,
            horizontal_edges: horizontals,
        }
    }

    fn is_inside_path(&self, rect: &Rectangle) -> bool {
        // println!("Rect: {rect:?}");
        if !self.is_vertical_line_inside(&VerticalLineSegment {
            x: rect.upper_left.0,
            low_y: rect.upper_left.1,
            high_y: rect.lower_right.1,
        }) {
            //println!("left edge");
            return false;
        }
        if !self.is_vertical_line_inside(&VerticalLineSegment {
            x: rect.lower_right.0,
            low_y: rect.upper_left.1,
            high_y: rect.lower_right.1,
        }) {
            //println!("right edge");
            return false;
        }
        if !self.is_horizontal_line_inside(&HorizontalLineSegment {
            y: rect.upper_left.1,
            low_x: rect.upper_left.0,
            high_x: rect.lower_right.0,
        }) {
            //println!("upper edge");
            return false;
        }
        if !self.is_horizontal_line_inside(&HorizontalLineSegment {
            y: rect.lower_right.1,
            low_x: rect.upper_left.0,
            high_x: rect.lower_right.0,
        }) {
            //println!("lower edge");
            return false;
        }
        true
    }

    //
    // ################
    // #  ####        #
    // #  #  #   ######
    // ####  #####
    //    +  +
    //    ++++
    //
    //            #
    //            #
    //  ###########
    //  #
    //  #
    //  #
    //  #######
    //
    fn is_point_inside_path(&self, (x, y): &(i64, i64)) -> bool {
        for e in &self.horizontal_edges {
            if e.y == *y && e.low_x <= *x && e.high_x >= *x {
                return true;
            }
        }

        let point_in_distance = *y as f64 - 0.5;
        let mut count = 0;
        for e in &self.vertical_edges {
            if e.x <= *x
                && point_in_distance >= e.low_y as f64
                && point_in_distance <= e.high_y as f64
            {
                count += 1;
            }
        }

        count % 2 == 1
    }

    fn is_vertical_line_inside(&self, line: &VerticalLineSegment) -> bool {
        self.is_point_inside_path(&(line.x, line.low_y))
            && self.is_point_inside_path(&(line.x, line.high_y))
            && !self.does_vertical_line_cross_any(line)
    }

    fn does_vertical_line_cross_any(&self, line: &VerticalLineSegment) -> bool {
        for e in &self.horizontal_edges {
            if e.y > line.low_y && e.y < line.high_y && line.x > e.low_x && line.x < e.high_x {
                return true;
            }
        }
        false
    }

    fn is_horizontal_line_inside(&self, line: &HorizontalLineSegment) -> bool {
        self.is_point_inside_path(&(line.low_x, line.y))
            && self.is_point_inside_path(&(line.high_x, line.y))
            && !self.does_horizontal_line_cross_any(line)
    }

    fn does_horizontal_line_cross_any(&self, line: &HorizontalLineSegment) -> bool {
        for e in &self.vertical_edges {
            if e.x > line.low_x && e.x < line.high_x && line.y > e.low_y && line.y < e.high_y {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
struct Rectangle {
    upper_left: (i64, i64),
    lower_right: (i64, i64),
}

impl Rectangle {
    fn new(p1: (i64, i64), p2: (i64, i64)) -> Self {
        let min_x = p1.0.min(p2.0);
        let max_x = p1.0.max(p2.0);
        let min_y = p1.1.min(p2.1);
        let max_y = p1.1.max(p2.1);

        Self {
            upper_left: (min_x, min_y),
            lower_right: (max_x, max_y),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prob9::{solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), 50);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT), 24);
    }

    const INPUT: &'static str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
}
