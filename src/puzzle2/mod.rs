use std::collections::HashMap;

pub struct Puzzle2;

fn input() -> Vec<String> {

    let content = std::fs::read_to_string("src/puzzle2/input.txt").expect("cannot read puzzle input.");
    content.lines().map(|x| x.to_owned()).collect::<Vec<_>>()
}

#[derive(Debug)]
struct Checksum {
    twos: i32,
    threes: i32
}

fn compute_checksum(s: &str) -> Checksum {
    let char_freqs = s.chars().fold(HashMap::new(), |mut freqs, c| {
        match freqs.get(&c) {
            Some(freq) => {
                let nfreq = *freq + 1;
                freqs.insert(c, nfreq);
            }
            None => { freqs.insert(c, 1); }
        };
        freqs
    });

    char_freqs.iter().fold(Checksum { twos: 0, threes: 0 }, |mut check, (_key, value)| {
        if *value == 2 { check.twos = 1; }
        if *value == 3 { check.threes = 1; }
        check
    })
}
impl crate::Puzzle for Puzzle2 {
    
    fn part1(&self) -> i32 {
        let checksum: Checksum = input()
            .iter()
            .map(|word| compute_checksum(word))
            .fold(Checksum { twos: 0, threes: 0 }, |acc, c| {
                Checksum {
                    twos: acc.twos + c.twos,
                    threes: acc.threes + c.threes
                }
            });

        checksum.twos * checksum.threes
    }
    
    fn part2(&self) -> i32 {
        panic!();
    }
}