use std::collections::{HashMap, VecDeque};

use aoc::{common, io, range::Range};

fn show(brs: &Vec<[Range<usize>; 3]>) {
    for br in brs {
        for r in br {
            println!("{:?}", r);
        }
        println!()
    }
}

// Returns true if br2 is DIRECTLY OR INDIRECTLY on top of br1
// 2nd comp shows if it is connected directly otherwise not
fn on_top_of(br1: &[Range<usize>; 3], br2: &[Range<usize>; 3]) -> (bool, bool) {
    let i2d = br1[0].intersect(&br2[0]).is_some() && br1[1].intersect(&br2[1]).is_some();
    let z_touching = br1[2].b == br2[2].a;
    (i2d, i2d && z_touching)
}

fn settle(brs: &mut Vec<[Range<usize>; 3]>) {
    // Sort according to Z
    brs.sort_by(|br1, br2| br1[2].a.cmp(&br2[2].a));
    let mut found = false;
    for j in 1..brs.len() {
        let mut highest_z = 0;
        for i in (0..j).rev() {
            let (j_top_of_i, directly) = on_top_of(&brs[i], &brs[j]);
            if j_top_of_i {
                highest_z = highest_z.max(brs[i][2].b);
                //println!("{} is on top of {}. Highest Z updated to {}", j, i, highest_z);

                if directly {
                    // cannot fall, break
                    assert!(brs[j][2].a == brs[i][2].b);
                    break;
                }
            }
        }

        if brs[j][2].a > highest_z {
            // Falls
            let h = brs[j][2].spread();
            brs[j][2].a = highest_z;
            brs[j][2].b = highest_z + h;
            found = true;
            break;
        }
    }

    if found {
        settle(brs);
    }
}

fn solve(input: &str) -> (usize, usize) {
    let mut brs: Vec<[Range<usize>; 3]> = Vec::new();

    for line in input.lines() {
        let (a, b) = line.split_once('~').unwrap();
        let xyz1 = io::tokenize(a, ",")
            .iter()
            .map(|s| io::parse_num::<usize>(s).unwrap())
            .collect::<Vec<_>>();
        let xyz2 = io::tokenize(b, ",")
            .iter()
            .map(|s| io::parse_num::<usize>(s).unwrap())
            .collect::<Vec<_>>();

        assert!(xyz1.len() == 3);
        assert!(xyz2.len() == 3);

        let mut r: [Range<usize>; 3] = [Range::new(0, 0); 3];
        for i in 0..3 {
            let (mi, mx) = (xyz1[i].min(xyz2[i]), xyz1[i].max(xyz2[i]));
            r[i].a = mi;
            r[i].b = mx + 1;
        }

        brs.push(r);
    }

    settle(&mut brs);
    //show(&brs);

    let n = brs.len();

    // Now settle in order, checking for intersections with all previous

    let mut top_of: HashMap<usize, Vec<(usize, bool)>> = HashMap::new();
    let mut num_d_supports: Vec<usize> = brs.iter().map(|_| 0).collect::<Vec<_>>();

    for i in 0..n {
        for j in i + 1..n {
            let (j_top_of_i, directly) = on_top_of(&brs[i], &brs[j]);
            if j_top_of_i {
                top_of
                    .entry(i)
                    .and_modify(|v| v.push((j, directly)))
                    .or_insert(vec![(j, directly)]);
                if directly {
                    num_d_supports[j] += 1;
                }
            }
        }
    }

    //println!("{:?}", top_of);
    //println!("{:?}", num_d_supports);

    let mut ans1 = 0;
    let mut bad_bricks: Vec<usize> = Vec::new();
    for i in 0..n {
        if let Some(supports) = top_of.get(&i) {
            let mut can = true;
            for (b, d) in supports {
                if *d && num_d_supports[*b] == 1 {
                    can = false;
                    break;
                }
            }
            if can {
                //println!("{}", i);
                ans1 += 1
            } else {
                bad_bricks.push(i);
            }
        } else {
            //println!("{}", i);
            ans1 += 1;
        }
    }

    //println!("{:?}", bad_bricks);

    let mut ans2 = 0;
    for bad_idx in bad_bricks {
        let mut falls = brs.iter().map(|_| false).collect::<Vec<_>>();
        let mut num_d_supports_tmp = num_d_supports.clone();
        let mut q: VecDeque<usize> = VecDeque::new();

        if let Some(supports) = top_of.get(&bad_idx) {
            for (b, d) in supports {
                if *d && num_d_supports_tmp[*b] == 1 {
                    num_d_supports_tmp[*b] -= 1;
                    q.push_back(*b);
                    //println!("Adding {} to fall queue. Supports Arr {:?}", *b, num_d_supports_tmp);
                }
            }
        }

        while !q.is_empty() {
            let pf = q.pop_front().unwrap();
            falls[pf] = true;

            if let Some(supports) = top_of.get(&pf) {
                for (b, d) in supports {
                    if *d {
                        if num_d_supports_tmp[*b] == 1 {
                            q.push_back(*b);
                            //println!("Adding {} to fall queue. Supports Arr {:?}", *b, num_d_supports_tmp);
                        }
                        num_d_supports_tmp[*b] -= 1;
                    }
                }
            }
        }
        let num_falls = falls.iter().filter(|&f| *f).count();
        // println!(
        //     "Disintegrating brick {} causes {} other bricks to fall.",
        //     bad_idx, num_falls
        // );
        ans2 += num_falls;
    }

    //let mut ans = can_be_disintegrated.iter().filter(|&x| *x).count();
    (ans1, ans2)
}

fn main() {
    let input = common::get_input();
    let (a1, a2) = solve(&input);
    println!("P1 {}", a1);
    println!("P2 {}", a2);
}
