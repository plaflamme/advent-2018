use bit_set::BitSet;

fn collapse(input: &String) -> BitSet {
    let chars = input.chars().collect::<Vec<_>>();
    let mut collapsed = BitSet::with_capacity(input.len());
    let mut left: usize = 0;
    let mut right: usize = 1;

    while right < input.len() {
        let left_c = chars.get(left).unwrap();
        let right_c = chars.get(right).unwrap();

        if left_c != right_c && left_c.to_ascii_lowercase() == right_c.to_ascii_lowercase() {
            collapsed.insert(left);
            collapsed.insert(right);

            // backtrack until we find a non-collapsed char
            while left > 0 && collapsed.contains(left) {
                left = left - 1;
            }

            // if we backtracked to the start, we must instead move forward
            if left == 0 && collapsed.contains(left) {
                left = right + 1;
                right = left + 1;
            } else {
                right = right + 1;
            }
        } else {
            left = left + 1;
            while left < input.len() && collapsed.contains(left) {
                left = left + 1;
            }
            right = left + 1;
        }
    }

    collapsed
}

pub struct Puzzle5;

impl crate::Puzzle for Puzzle5 {
    fn part1(&self, input: String) -> String {
        let trimmed = input.trim().to_string();
        let bits = collapse(&trimmed);
        let remains = trimmed.len() - bits.len();
        remains.to_string()
    }

    fn part2(&self, input: String) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::{Range, RangeInclusive};

    fn assert_collapsed(input: String, collapsed: Vec<RangeInclusive<usize>>) -> () {
        let mut bits = BitSet::with_capacity(input.len());
        for i in collapsed {
            for b in i {
                bits.insert(b);
            }
        }

        assert_eq!(collapse(&input), bits);
    }

    #[test]
    fn test_collapse() {
        assert_collapsed("aa".to_string(), vec![]);
        assert_collapsed("aabb".to_string(), vec![]);
        assert_collapsed("aA".to_string(), vec![0..=1]);
        assert_collapsed("aAa".to_string(), vec![0..=1]);
        assert_collapsed("aAbb".to_string(), vec![0..=1]);
        assert_collapsed("bbaA".to_string(), vec![2..=3]);
        assert_collapsed("aAbB".to_string(), vec![0..=3]);
        assert_collapsed("aAbbcC".to_string(), vec![0..=1,4..=5]);
        assert_collapsed("baAb".to_string(), vec![1..=2]);
        assert_collapsed("abBA".to_string(), vec![0..=3]);
        assert_collapsed("cabBAC".to_string(), vec![0..=5]);
        assert_collapsed("AcabBACA".to_string(), vec![1..=6]);
        assert_collapsed("AcabBACAbB".to_string(), vec![1..=6, 8..=9]);
        assert_collapsed("dabAcCaCBAcCcaDA".to_string(), vec![4..=5, 3..=6, 10..=11]);
        assert_collapsed("czYyZQMzZmSs".to_string(), vec![2..=3, 1..=4, 6..=11]);
    }
}