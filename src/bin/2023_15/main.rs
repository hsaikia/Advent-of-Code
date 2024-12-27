use aoc::{common, io};

fn hash(s: &str) -> u32 {
    s.chars()
        .fold(0, |acc, c| ((acc + u32::from(c as u8)) * 17) % 256)
}

fn part1(input: &str) -> u32 {
    input.split(',').map(hash).sum::<u32>()
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_lossless)]
fn part2(input: &str) -> u32 {
    const VAL: Vec<(&str, u32)> = Vec::new();
    let mut box_map: [Vec<(&str, u32)>; 256] = [VAL; 256];

    let seqs = io::tokenize(input, ",");
    for seq in &seqs {
        if seq.find('=').is_some() {
            if let Some((id, val)) = seq.split_once('=') {
                let val: u32 = io::parse_num(val);
                let box_idx = hash(id) as usize;
                let mut found = false;
                for (id1, val1) in &mut box_map[box_idx] {
                    if id == *id1 {
                        *val1 = val;
                        found = true;
                        break;
                    }
                }
                if !found {
                    box_map[box_idx].push((id, val));
                }
            }
        } else if let Some((id, _)) = seq.split_once('-') {
            let box_idx = hash(id) as usize;
            box_map[box_idx].retain(|(id1, _)| *id1 != id);
        }
    }

    (0..256)
        .map(|idx| {
            box_map[idx]
                .iter()
                .enumerate()
                .map(|(slot, (_, val))| (idx as u32 + 1) * (slot as u32 + 1) * val)
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn main() {
    let input = common::get_input();
    common::timed(&input, part1, true);
    common::timed(&input, part2, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample() {
        let example_id = "HASH";
        assert_eq!(hash(example_id), 52);
        assert_eq!(hash("rn"), 0);
        assert_eq!(hash("qp"), 1);
        assert_eq!(hash("cm"), 0);

        let sample_input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(sample_input), 1320);
        assert_eq!(part2(sample_input), 145);
    }
}
