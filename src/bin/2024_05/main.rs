use aoc::{common, io};

// Only works when the input page_orderings have ALL NxN relationships / or all relationships
// that occur in the page_sequences. Otherwise all transitive relationships, such as A->C
// given A->B and B->C should be additionally added to the page_orderings to make this algorithm work.
fn solve<const PART: usize>(input: &str) -> usize {
    let mut page_orderings: Vec<(usize, usize)> = Vec::new();
    let mut page_sequences: Vec<Vec<usize>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.chars().any(|x| x == '|') {
            let page_nums: Vec<usize> = io::tokenize(line, "|")
                .into_iter()
                .map(io::parse_num)
                .collect();
            page_orderings.push((page_nums[0], page_nums[1]));
        } else {
            let seq: Vec<usize> = io::tokenize(line, ",")
                .into_iter()
                .map(io::parse_num)
                .collect();
            page_sequences.push(seq);
        }
    }

    let mut ans = 0;
    for seq in page_sequences {
        let l = seq.len();

        let mut ordered = true;
        for i in 1..l {
            if !page_orderings.contains(&(seq[i - 1], seq[i])) {
                ordered = false;
            }
        }

        if ordered {
            if PART == 1 {
                ans += seq[l / 2];
            }
        } else if PART == 2 {
            let mut new_seq = seq.clone();
            while !ordered {
                ordered = true;
                for i in 1..l {
                    if !page_orderings.contains(&(new_seq[i - 1], new_seq[i])) {
                        new_seq.swap(i - 1, i);
                        ordered = false;
                        break;
                    }
                }

                if ordered {
                    ans += new_seq[l / 2];
                }
            }
        }
    }

    ans
}

fn main() {
    let input = common::get_input();
    //println!("{input:?}");
    common::timed(&input, solve::<1>, true);
    common::timed(&input, solve::<2>, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_samples() {
        let sample_input =
            "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
        assert_eq!(solve::<1>(sample_input), 143);
        assert_eq!(solve::<2>(sample_input), 123);
    }
}
