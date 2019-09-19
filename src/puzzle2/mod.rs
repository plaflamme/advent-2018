use std::collections::HashMap;

pub struct Puzzle2;

fn parse(input: String) -> Vec<String> {
    input.lines().map(|x| x.to_owned()).collect::<Vec<_>>()
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

    fn part1(&self, input: String) -> String {
        let checksum: Checksum = parse(input)
            .iter()
            .map(|word| compute_checksum(word))
            .fold(Checksum { twos: 0, threes: 0 }, |acc, c| {
                Checksum {
                    twos: acc.twos + c.twos,
                    threes: acc.threes + c.threes
                }
            });

        (checksum.twos * checksum.threes).to_string()
    }
    
    fn part2(&self, input: String) -> String {

        let words = parse(input);

        let found = words.iter()
            .flat_map(|a| words.iter().map(move |b| (a,b)))
            .find_map(|(a,b)| {
                let common = a.chars().zip(b.chars())
                    .filter_map(|(l,r)| if l == r { Some(l) } else { None } )
                    .collect::<String>();

                if common.len() != a.len() -1 { None }
                else { Some(common) }
            });

        found.expect("couldn't find a box")
    }
}