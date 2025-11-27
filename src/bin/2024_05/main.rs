use aoc::{common, io};

fn ordered(sequence: &[usize], page_orderings: &[(usize, usize)]) -> bool {
    sequence
        .windows(2)
        .all(|x| page_orderings.contains(&(x[0], x[1])))
}

fn first_unordered_index(sequence: &[usize], page_orderings: &[(usize, usize)]) -> Option<usize> {
    let unordered_indices: Vec<usize> = sequence
        .windows(2)
        .enumerate()
        .filter(|(_, x)| !page_orderings.contains(&(x[0], x[1])))
        .map(|(i, _)| i)
        .collect();

    unordered_indices.first().copied()
}

// Only works when the input page_orderings have ALL NxN relationships / or all relationships
// that occur in the page_sequences. Otherwise all transitive relationships, such as A->C
// given A->B and B->C should be additionally added to the page_orderings to make this algorithm work.
fn solve<const PART: usize>(input: &str) -> usize {
    let batches = io::line_batches(input);
    let page_orderings: Vec<(usize, usize)> = batches[0].iter().fold(vec![], |mut acc, x| {
        if let Some((page1, page2)) = x.split_once('|') {
            acc.push((io::parse_num(page1), io::parse_num(page2)));
        }
        acc
    });
    let page_sequences: Vec<Vec<usize>> = batches[1].iter().fold(vec![], |mut acc, x| {
        acc.push(io::tokenize_nums(x, ","));
        acc
    });

    page_sequences
        .iter()
        .map(|seq| {
            if ordered(seq, &page_orderings) {
                if PART == 1 {
                    seq[seq.len() / 2]
                } else {
                    0
                }
            } else if PART == 2 {
                let mut new_seq = seq.clone();
                while let Some(unordered_index) = first_unordered_index(&new_seq, &page_orderings) {
                    new_seq.swap(unordered_index, unordered_index + 1);
                }
                new_seq[seq.len() / 2]
            } else {
                0
            }
        })
        .sum()
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
        let sample_input =
            "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
        assert_eq!(solve::<1>(sample_input), 143);
        assert_eq!(solve::<2>(sample_input), 123);
    }
}
