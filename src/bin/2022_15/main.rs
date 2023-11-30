const INPUT: [(&str, &str, i32); 1] = [
    ("Sample Input", include_str!("sample_input.txt"), 10), //("Input", include_str!("input.txt"), 2000000),
];

#[derive(Debug)]
struct RangeUnion {
    ranges: Vec<(i32, i32)>,
}

impl RangeUnion {
    fn new() -> Self {
        RangeUnion { ranges: Vec::new() }
    }

    fn add_range(&mut self, range: (i32, i32)) {
        let contains_l = self
            .ranges
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, r)| r.0 >= range.0 && r.1 <= range.0)
            .collect::<Vec<_>>();
        let contains_r = self
            .ranges
            .clone()
            .into_iter()
            .enumerate()
            .filter(|(_, r)| r.0 >= range.1 && r.1 <= range.1)
            .collect::<Vec<_>>();

        if contains_l.is_empty() && contains_r.is_empty() {
            self.ranges.push(range);
        } else if !contains_l.is_empty() && contains_r.is_empty() {
            self.ranges[contains_l[0].0].1 = range.1;
        } else if contains_l.is_empty() && !contains_r.is_empty() {
            self.ranges[contains_r[0].0].0 = range.0;
        } else if contains_l[0].0 != contains_r[0].0 {
            self.ranges[contains_l[0].0].1 = self.ranges[contains_r[0].0].1;
            self.ranges.remove(contains_r[0].0);
        }
    }
}

fn part1(input_lines: &str, y: i32) {
    let mut coords: Vec<(i32, i32, i32, i32)> = Vec::new();
    for line in input_lines.split('\n') {
        let tokens = line
            .split(' ')
            .filter(|&s| s.contains('='))
            .collect::<Vec<_>>();
        //println!("{:?}", tokens);
        coords.push((
            tokens[0][2..tokens[0].len() - 1].parse::<i32>().unwrap(),
            tokens[1][2..tokens[1].len() - 1].parse::<i32>().unwrap(),
            tokens[2][2..tokens[2].len() - 1].parse::<i32>().unwrap(),
            tokens[3][2..tokens[3].len()].parse::<i32>().unwrap(),
        ));
    }

    let _known_beacon_positions = coords
        .iter()
        .filter_map(|coord| if y == coord.3 { Some(coord.2) } else { None })
        .collect::<Vec<_>>();
    //println!("{:?}", known_beacon_positions);

    //let mut no_beacon_pos = Vec::new();

    let mut range_union = RangeUnion::new();

    for coord in &coords {
        let d = coord.0.abs_diff(coord.2) + coord.1.abs_diff(coord.3);
        let yd = y.abs_diff(coord.1);

        if yd > d {
            continue;
        }

        let xd = d as i32 - yd as i32;
        //println!("X Range for Sensor at ({},{}) is {}", coord.0, coord.1, xd);

        range_union.add_range((coord.0 - xd, coord.0 + xd));

        // for x in coord.0 - xd..=coord.0 + xd {
        //     if !no_beacon_pos.contains(&x) && !known_beacon_positions.contains(&x){
        //         no_beacon_pos.push(x);
        //     }
        // }
    }

    println!("{:?}", range_union);
    //println!("At y={}, a beacon cannot be present at {:?} locations", y, no_beacon_pos.len());

    //    println!("Part 1 Answer : {best}");
}

fn main() {
    for input in INPUT {
        println!("{}", input.0);
        part1(input.1, input.2);
        //part2(input.1);
    }
}
