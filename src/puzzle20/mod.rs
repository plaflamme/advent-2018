fn parse(input: &str) -> u32 {
    unimplemented!()
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new(Puzzle20 { regex: input })
}

struct Puzzle20 {
    regex: String
}

impl crate::Puzzle for Puzzle20 {
    fn part1(&self) -> String {
        unimplemented!()
    }

    fn part2(&self) -> String {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const example: &str = "^ENWWW(NEEE|SSE(EE|N))$";
    const result: &str = r#"#########
#.|.|.|.#
#-#######
#.|.|.|.#
#-#####-#
#.#.#X|.#
#-#-#####
#.|.|.|.#
#########"#;

    #[test]
    fn test_examples() {
        let input = "^WNE$";
        assert_eq!(3, parse(input));
        let input = "^ENWWW(NEEE|SSE(EE|N))$";
        assert_eq!(10, parse(input));
        let input = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        assert_eq!(18, parse(input));
        let input = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        assert_eq!(23, parse(input));
        let input = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        assert_eq!(31, parse(input));
    }
}