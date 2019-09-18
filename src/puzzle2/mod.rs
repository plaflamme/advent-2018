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
        
        let mut common_chars: Option<Box<String>> = None;

        for a in input() {
            for b in input() {
                let char_dist = a.chars().zip(b.chars())
                    .fold(0, |diff, (left, right)| {
                        if left != right { diff + 1 }
                        else { diff }
                    });

                if char_dist == 1 {
                    let c = a.chars().zip(b.chars()).filter_map(|(l,r)| if l == r { Some(l) } else { None }).collect::<String>();
                    common_chars = Some(Box::new(c));
                    break;
                }
            }
            if common_chars.is_some() { break; }
        }

        println!("{}", common_chars.expect("couldn't find a box"));

        panic!(); // TODO: figure out how to return not-an-i32
    }
}