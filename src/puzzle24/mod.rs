use std::str::FromStr;
use std::collections::{HashSet, BinaryHeap};
use regex::Regex;
use std::num::ParseIntError;
use std::cmp::{Reverse, Ordering};
use std::fmt::{Display, Formatter, Error};
use itertools::Itertools;

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

    // By default, an attacking group would deal damage equal to its effective power to the defending group.
    //   However, if the defending group is immune to the attacking group's attack type, the defending group instead takes no damage;
    //   if the defending group is weak to the attacking group's attack type, the defending group instead takes double damage.
    fn damage_dealt(&self, other: &Group) -> u32 {
        let base_damage = self.effective_power();
        if other.immunity.contains(&self.attack_type) {
            0
        } else if other.weakness.contains(&self.attack_type) {
            base_damage * 2
        } else {
            base_damage
        }
    }

    fn take_damage(&mut self, damage: u32) -> u32 {
        let deaths = u32::min(self.units, damage / self.unit_hit_pts);
        self.units -= deaths;
        deaths
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

#[derive(PartialEq, Eq, Clone, Debug)]
struct AttackTarget {
    side: Side,
    attacking_group: usize,
    attacking_initiative: u32,
    selection: Option<TargetSelection>
}

impl PartialOrd for AttackTarget {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.attacking_initiative.cmp(&other.attacking_initiative))
    }
}

impl Ord for AttackTarget {
    fn cmp(&self, other: &Self) -> Ordering {
        self.attacking_initiative.cmp(&other.attacking_initiative)
    }
}

impl Display for AttackTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self.selection {
            None =>
                write!(f, "{:?} group {} selects no target", self.side, self.attacking_group),
            Some(selection) =>
                write!(f, "{:?} group {} would deal defending group {} {} damage", self.side, self.attacking_group, selection.defending_group, selection.damage)
        }

    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct TargetSelection {
    defending_group: usize,
    damage: u32
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Side {
    ImmuneSystem,
    Infection
}

#[derive(Clone, Debug)]
struct Army {
    side: Side,
    groups: Vec<Group>
}

impl Army {
    fn target_selection(&self, enemies: &Army) -> Vec<AttackTarget> {
        let mut our_groups = self.groups.iter().cloned().enumerate().collect::<Vec<_>>();
        our_groups.sort_by_key(|(_, group)| {
            // In decreasing order of effective power, groups choose their targets; in a tie, the group with the higher initiative chooses first.
            Reverse((group.effective_power(), group.initiative))
        });

        // Defending groups can only be chosen as a target by one attacking group
        let mut chosen = HashSet::new();

        our_groups
            .iter()
            .map(|(group_idx, group)| {
                let targets = enemies.groups.iter()
                    .enumerate()
                    // The attacking group chooses to target the group in the enemy army to which it would deal the most damage
                    //   If an attacking group is considering two defending groups to which it would deal equal damage, it chooses to target the defending group with the largest effective power;
                    //   if there is still a tie, it chooses the defending group with the highest initiative.
                    .map(|(idx, enemy)| (group.damage_dealt(&enemy), enemy.effective_power(), enemy.initiative, idx))
                    .filter(|(damage, _, _, idx)| *damage > 0 && !chosen.contains(idx))
                    .collect::<BinaryHeap<_>>();

                match targets.peek() {
                    None => AttackTarget { side: self.side, attacking_group: *group_idx, attacking_initiative: group.initiative, selection: None },
                    Some((damage, _, _, target)) => {
                        chosen.insert(*target);
                        AttackTarget { side: self.side, attacking_group: *group_idx, attacking_initiative: group.initiative, selection: Some(TargetSelection{ defending_group: *target, damage: *damage }) }
                    }
                }
            })
            .collect::<Vec<_>>()
    }

    fn boost(&self, by: u32) -> Army {
        Army {
            side: self.side,
            groups: self.groups.iter().map(|g| {
                let mut boosted = g.clone();
                boosted.attack_strength += by;
                boosted
            }).collect()
        }
    }

    fn total_units(&self) -> u32 {
        self.groups.iter().map(|g|g.units).sum()
    }
}

#[derive(Debug)]
struct AttackOutcome {
    attack_side: Side,
    attacking_group: usize,
    defending_group: usize,
    damage_dealt: u32,
    unit_loss: u32
}

impl Display for AttackOutcome {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:?} group {} attacks defending group {}, dealing {} damage killing {} units", self.attack_side, self.attacking_group, self.defending_group, self.damage_dealt, self.unit_loss)
    }
}

#[derive(Debug)]
struct FightOutcome {
    battlefield: Battlefield,
    target_selections: Vec<AttackTarget>,
    attack_outcomes: Vec<AttackOutcome>
}

#[derive(Clone, Debug)]
struct Battlefield {
    immune_system: Army,
    infection: Army
}

impl Battlefield {

    fn fight(&self) -> FightOutcome {
        let immune_selection = self.immune_system.target_selection(&self.infection);
        let infection_selection = self.infection.target_selection(&self.immune_system);

        let mut attack_order = immune_selection.iter().chain(&infection_selection).collect::<BinaryHeap<_>>();

        let mut immune_system_groups = self.immune_system.groups.clone();
        let mut infection_groups = self.infection.groups.clone();
        let mut attack_outcomes = Vec::new();

        while let Some(attack) = attack_order.pop() {
            let attacking_group = match attack.side {
                Side::ImmuneSystem => immune_system_groups.get(attack.attacking_group).unwrap().clone(),
                Side::Infection => infection_groups.get(attack.attacking_group).unwrap().clone()
            };

            if attacking_group.units == 0 { continue }

            if let Some(selection) = &attack.selection {
                let defending_group = match attack.side {
                    Side::ImmuneSystem => infection_groups.get_mut(selection.defending_group).unwrap(),
                    Side::Infection => immune_system_groups.get_mut(selection.defending_group).unwrap()
                };

                // recompute damage since the attacking group size has potentially changed
                let damage_dealt = attacking_group.damage_dealt(defending_group);
                let unit_loss = defending_group.take_damage(damage_dealt);
                attack_outcomes.push(AttackOutcome { attack_side: attack.side, attacking_group: attack.attacking_group, defending_group: selection.defending_group, damage_dealt, unit_loss } )
            }
        }

        immune_system_groups.retain(|g| g.units > 0);
        infection_groups.retain(|g| g.units > 0);

        FightOutcome {
            battlefield: Battlefield {
                immune_system: Army { side: Side::ImmuneSystem, groups: immune_system_groups },
                infection: Army { side: Side::Infection, groups: infection_groups },
            },
            target_selections: immune_selection.iter().chain(infection_selection.iter()).cloned().collect(),
            attack_outcomes
        }
    }

    fn boost(&self, by: u32) -> Battlefield {
        Battlefield {
            immune_system: self.immune_system.boost(by),
            infection: self.infection.clone(),
        }
    }

    fn total_units(&self) -> u32 {
        self.immune_system.total_units() + self.infection.total_units()
    }
}

impl FromStr for Battlefield {
    type Err = ParseGroupError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let immune_system = Army {
            side: Side::ImmuneSystem,
            groups: Result::from(s.lines().skip(1).take_while(|line| !line.is_empty()).map(|gr| Group::from_str(gr)).collect())?
        };
        let infection = Army {
            side: Side::Infection,
            groups: Result::from(s.lines().skip_while(|line| !line.is_empty()).skip(2).map(|gr| Group::from_str(gr)).collect())?
        };

        Ok(Battlefield { immune_system, infection })
    }
}

fn resolve_battle(start: Battlefield) -> Option<Battlefield> { // None when it's a tie
    let mut battlefield = start;
    loop {
        println!("ImmuneSystem has {} groups", battlefield.immune_system.groups.len());
        println!("  {}", battlefield.immune_system.groups.iter().map(|g| format!("{:?}", g)).join(","));
        println!("Infection has {} groups", battlefield.infection.groups.len());
        println!("  {}", battlefield.infection.groups.iter().map(|g| format!("{:?}", g)).join(","));
        if battlefield.immune_system.groups.is_empty() || battlefield.infection.groups.is_empty() {
            break
        }
        let outcome = battlefield.fight();
        outcome.target_selections.iter().for_each(|outcome| println!("{}", outcome));
        println!("");
        outcome.attack_outcomes.iter().for_each(|outcome| println!("{}", outcome));
        println!("");

        // stalemate detection for part 2.
        if battlefield.total_units() == outcome.battlefield.total_units() {
            return None // using return sucks
        }

        battlefield = outcome.battlefield;
    }

    Some(battlefield)
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    Box::new( Puzzle24 { battlefield: Battlefield::from_str(&input).unwrap() } )
}

struct Puzzle24 {
    battlefield: Battlefield
}

impl crate::Puzzle for Puzzle24 {
    fn part1(&self) -> String {
        let resolved = resolve_battle(self.battlefield.clone()).expect("unexpected stalemate in part1");
        let winning = if resolved.immune_system.groups.len() > 0 {
            resolved.immune_system
        } else {
            resolved.infection
        };

        winning.groups.iter().map(|g| g.units).sum::<u32>().to_string()
    }

    fn part2(&self) -> String {
        let mut losing_max_heap = BinaryHeap::new();
        let mut winning_min_heap = BinaryHeap::new();
        let result = loop {
            let boost = match (losing_max_heap.peek(), winning_min_heap.peek()) {
                (Some(lose), Some(Reverse(win))) if *win == lose + 1 => break win,
                (Some(lose), Some(Reverse(win))) => lose + (win - lose) / 2,
                (Some(lose), None) => lose * 2,
                (None, Some(Reverse(win))) => win / 2,
                (None, None) => 1
            };

            println!("Boost: {}", boost);
            match resolve_battle(self.battlefield.boost(boost)) {
                None => losing_max_heap.push(boost),
                Some(resolved) => {
                    if resolved.immune_system.groups.len() > 0 {
                        winning_min_heap.push(Reverse(boost));
                    } else {
                        losing_max_heap.push(boost);
                    }
                }
            }
        };
        result.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Puzzle;

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

    #[test]
    fn test_target_selection() {
        let battlefield = Battlefield::from_str(EXAMPLE).unwrap();

        let immune_selection = [
            AttackTarget { side: Side::ImmuneSystem, attacking_group: 0, attacking_initiative: 2, selection: Some(TargetSelection { defending_group: 1, damage: 153238 }) },
            AttackTarget { side: Side::ImmuneSystem, attacking_group: 1, attacking_initiative: 3, selection: Some(TargetSelection { defending_group: 0, damage: 24725 }) }
        ];
        let infection_selection = [
            AttackTarget { side: Side::Infection, attacking_group: 0, attacking_initiative: 1, selection: Some(TargetSelection { defending_group: 0, damage: 185832 }) },
            AttackTarget { side: Side::Infection, attacking_group: 1, attacking_initiative: 4, selection: Some(TargetSelection { defending_group: 1, damage: 107640 }) }
        ];
        assert_eq!(immune_selection.to_vec(), battlefield.immune_system.target_selection(&battlefield.infection));
        assert_eq!(infection_selection.to_vec(), battlefield.infection.target_selection(&battlefield.immune_system));
    }

    #[test]
    fn test_fight() {
        let resolved = resolve_battle(Battlefield::from_str(EXAMPLE).unwrap()).unwrap();

        assert_eq!(0, resolved.immune_system.groups.len());
        assert_eq!(2, resolved.infection.groups.len());
        assert_eq!(782, resolved.infection.groups[0].units);
        assert_eq!(4434, resolved.infection.groups[1].units);
    }

    #[test]
    fn test_part2() {
        let pzl = Puzzle24 { battlefield: Battlefield::from_str(EXAMPLE).unwrap() };
        assert_eq!("1570", pzl.part2());
    }
}