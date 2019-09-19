use std::collections::HashSet;

fn parse(input: String) -> Vec<i32> {
    input.lines()
        .filter(|s| !s.is_empty())
        .map(|s| {
            let mut st = String::from(s);
            let first = st.remove(0);
            let value = st.parse::<i32>().expect("not an int");
            match first {
                '-' => value * -1,
                '+' => value,
                c => panic!("unexpected char in input {}", c)
            }
        })
        .collect()
}

pub struct Puzzle1;

impl crate::Puzzle for Puzzle1 {

    fn part1(&self, input: String) -> String {
        parse(input).iter().sum::<i32>().to_string()
    }

    fn part2(&self, input: String) -> String {
        let mut seen_freqs = HashSet::new();
        seen_freqs.insert(0);

        parse(input)
            .iter()
            .cycle()
            .scan(0, |freq, inc| {
                *freq = *freq + inc; // NOTE: we're mutating the internal state here. The state of scan isn't an accumulator like fold.
                Some(*freq)
            })
            .find(|x| !seen_freqs.insert(*x))
            .expect("couldn't find a freq")
            .to_string()
    }
}