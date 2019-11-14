use std::str::FromStr;
use std::collections::HashSet;
use regex::Regex;
use std::num::ParseIntError;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Attack {
    Cold,
    Fire,
    Radiation,
    Bludgeoning,
    Slashing
}

impl FromStr for Attack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
       match s {
           "cold" => Ok(Attack::Cold),
           "fire" => Ok(Attack::Fire),
           "radiation" => Ok(Attack::Radiation),
           "bludgeoning" => Ok(Attack::Bludgeoning),
           "slashing" => Ok(Attack::Slashing),
           _ => Err(format!("unknown attack kind {}", s)),
       }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Group {
    units: u32,
    unit_hit_pts: u32,
    immunity: HashSet<Attack>,
    weakness: HashSet<Attack>,
    attack_strength: u32,
    attack_type: Attack,
    initiative: u32
}

impl Group {
    fn effective_power(&self) -> u32 {
        self.units * self.attack_strength
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
enum ParseGroupError {
    ParseIntError(std::num::ParseIntError),
    Unknown(String)
}

impl From<ParseIntError> for ParseGroupError {
    fn from(f: ParseIntError) -> Self {
        ParseGroupError::ParseIntError(f)
    }
}

impl From<String> for ParseGroupError {
    fn from(f: String) -> Self {
        ParseGroupError::Unknown(f)
    }
}

impl FromStr for Group {
    type Err = ParseGroupError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 3134 units each with 1909 hit points (immune to slashing, cold; weak to radiation) with an attack that does 5 bludgeoning damage at initiative 16
        let re = Regex::new(r#"^(\d+) units each with (\d+) hit points (?:\((.*)\) )?with an attack that does (\d+) (.*) damage at initiative (\d+)$"#).unwrap();
        let caps = re.captures(s).expect(&format!("invalid line {}", s));

        let units = u32::from_str(&caps[1])?;
        let unit_hit_pts = u32::from_str(&caps[2])?;
        let (immunity, weakness) = match caps.get(3) {
            None => (HashSet::new(), HashSet::new()),
            Some(cap) => {
                let parts: Vec<&str> = cap.as_str().split(';').collect();
                let mut immunity = HashSet::new();
                let mut weakness = HashSet::new();
                for part in parts {
                    let caps = Regex::new(r#"^(.*) to (.*)$"#).unwrap().captures(part).unwrap();

                    let attacks = caps[2].split(",").map(|a| Attack::from_str(a.trim()).unwrap());

                    match caps[1].trim() {
                        "immune" => immunity.extend(attacks),
                        "weak" => weakness.extend(attacks),
                        ability => Err(ParseGroupError::Unknown(format!("invalid ability '{}'", ability).into()))?
                    };
                }

                (immunity, weakness)
            }
        };

        let attack_strength = u32::from_str(&caps[caps.len()-3])?;
        let attack_type = Attack::from_str(&caps[caps.len()-2])?;
        let initiative = u32::from_str(&caps[caps.len()-1])?;

        Ok(
            Group {
                units,
                unit_hit_pts,
                immunity,
                weakness,
                attack_strength,
                attack_type,
                initiative
            }
        )
    }
}

struct Army {
    groups: Vec<Group>
}

struct Battlefield {
    immune_system: Army,
    infection: Army
}

impl FromStr for Battlefield {
    type Err = ParseGroupError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let immune_system = Army {
            groups: Result::from(s.lines().skip(1).take_while(|line| !line.is_empty()).map(|gr| Group::from_str(gr)).collect())?
        };
        let infection = Army {
            groups: Result::from(s.lines().skip_while(|line| !line.is_empty()).skip(2).map(|gr| Group::from_str(gr)).collect())?
        };

        Ok(Battlefield { immune_system, infection })
    }
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new( Puzzle24 { battlefield: Battlefield::from_str(&input).unwrap() } )
}

struct Puzzle24 {
    battlefield: Battlefield
}

impl crate::Puzzle for Puzzle24 {
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

    const EXAMPLE: &str = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    #[test]
    fn test_parse() {
        let battlefield = Battlefield::from_str(EXAMPLE).unwrap();
        assert_eq!(2, battlefield.immune_system.groups.len());
        assert_eq!(2, battlefield.infection.groups.len());
    }

    #[test]
    fn test_parse_group() {
        let group = Group {
            units: 17,
            unit_hit_pts: 5390,
            immunity: HashSet::new(),
            weakness: vec![Attack::Radiation, Attack::Bludgeoning].iter().cloned().collect(),
            attack_strength: 4507,
            attack_type: Attack::Fire,
            initiative: 2
        };
        assert_eq!(Ok(group), Group::from_str("17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2"));

        let group = Group {
            units: 4485,
            unit_hit_pts: 2961,
            immunity: vec![Attack::Radiation].iter().cloned().collect(),
            weakness: vec![Attack::Fire, Attack::Cold].iter().cloned().collect(),
            attack_strength: 12,
            attack_type: Attack::Slashing,
            initiative: 4
        };
        assert_eq!(Ok(group), Group::from_str("4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"));

        let group = Group {
            units: 5463,
            unit_hit_pts: 1741,
            immunity: HashSet::new(),
            weakness: HashSet::new(),
            attack_strength: 2,
            attack_type: Attack::Cold,
            initiative: 2
        };
        assert_eq!(Ok(group), Group::from_str("5463 units each with 1741 hit points with an attack that does 2 cold damage at initiative 2"));
    }
}