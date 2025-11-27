use aoc::{common, range::Range};

#[derive(Clone)]
enum SpaceType {
    Free,
    Occupied(usize),
}

struct Space {
    t: SpaceType,
    r: Range<usize>,
}

impl Space {
    fn print(spaces: &[Space]) {
        let mut s = String::new();
        for sp in spaces {
            for _ in sp.r.a..sp.r.b {
                let t = match &sp.t {
                    SpaceType::Free => ".",
                    SpaceType::Occupied(id) => &id.to_string(),
                };
                s.push_str(t);
            }
        }
        println!("{s}");
    }
}

fn solve<const PART: i32>(input: &str) -> usize {
    let mut idx = 0;
    let mut spaces = Vec::new();

    for (i, c) in input.chars().enumerate() {
        let x = c.to_digit(10).unwrap() as usize;
        spaces.push(Space {
            t: if i % 2 == 0 {
                SpaceType::Occupied(i / 2)
            } else {
                SpaceType::Free
            },
            r: Range::new(idx, idx + x),
        });
        idx += x;
    }

    loop {
        let l = spaces.len();
        let mut found = false;
        for i in (0..l).rev() {
            if let SpaceType::Occupied(id) = spaces[i].t {
                for j in 0..i {
                    if let SpaceType::Free = spaces[j].t {
                        let file = spaces[i].r;
                        let free = spaces[j].r;
                        if PART == 1 {
                            let spr = file.spread().min(free.spread());
                            found = true;
                            spaces[i].r = Range::new(free.a, free.a + spr);
                            spaces[j].r = Range::new(free.a + spr, free.b);
                            spaces.push(Space {
                                t: SpaceType::Occupied(id),
                                r: Range::new(file.a, file.b - spr),
                            });
                            break;
                        } else if free.spread() >= file.spread() {
                            let spr = file.spread();
                            found = true;
                            spaces[i].r = Range::new(free.a, free.a + spr);
                            spaces[j].r = Range::new(free.a + spr, free.b);
                            break;
                        }
                    }
                }
                if found {
                    break;
                }
            }
        }

        if found {
            spaces.retain(|s| s.r.spread() > 0);
            spaces.sort_by(|s1, s2| s1.r.a.cmp(&s2.r.a));
        } else {
            break;
        }
    }

    Space::print(&spaces);

    let mut ans = 0;
    for s in spaces {
        if let SpaceType::Occupied(id) = s.t {
            for i in s.r.a..s.r.b {
                ans += id * i;
            }
        }
    }

    ans
}

fn main() {
    if let Some(input) = common::get_input() {
        common::timed(&input, solve::<1>, true);
        common::timed(&input, solve::<2>, false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input = "2333133121414131402";
        assert_eq!(solve::<1>(sample_input), 1928);
        assert_eq!(solve::<2>(sample_input), 2858);
    }
}
