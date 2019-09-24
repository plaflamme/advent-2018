use std::str::FromStr;
use regex::Regex;
use crate::puzzle4::What::{FallAsleep, WakeUp, ShiftStart};
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Eq, PartialEq, PartialOrd, Debug)]
struct Ts {
    day: String,
    hour: u8,
    minute: u8
}

impl FromStr for Ts {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^\[(\d{4}-\d{2}-\d{2}) (\d{2}):(\d{2})\]$").unwrap();
        let caps = re.captures(s).expect("invalid date input");
        let day = caps[1].to_string();
        let hour = u8::from_str(&caps[2])?;
        let minute = u8::from_str(&caps[3])?;
        Ok(Ts { day, hour, minute })
    }
}

#[derive(PartialEq, Debug)]
enum What {
    ShiftStart(u32),
    FallAsleep,
    WakeUp
}

impl FromStr for What {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^Guard #(\d+) begins shift$").unwrap();
        match s {
            "falls asleep" => Ok(FallAsleep),
            "wakes up" => Ok(WakeUp),
            _ => {
                let caps = re.captures(s).expect("invalid event");
                let id = u32::from_str(&caps[1])?;
                Ok(ShiftStart(id))
            }
        }
    }
}

#[derive(PartialEq, Debug)]
struct Event {
    ts: Ts,
    event: What
}

impl FromStr for Event {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(\[.*\]) (.*)$").unwrap();
        let caps = re.captures(s).expect("invalid line");
        let ts = Ts::from_str(&caps[1])?;
        let event = What::from_str(&caps[2])?;

        Ok(Event { ts, event })
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Shift {
    day: String,
    sleeping: Vec<u16>
}

#[derive(Clone, PartialEq, Debug)]
struct Guard {
    id: u32,
    shifts: Vec<Shift>
}

impl Guard {
    fn alertness(&self) -> Vec<u16> {
        let mut a: [u16; 60] = [0; 60];
        for s in self.shifts.iter().map(|x| &x.sleeping) {
            for i in 0..60 {
                let value = s.get(i).expect("invalid shift");
                a[i] += *value;
            }
        }

        a.to_vec()
    }

    fn summary(&self) -> Summary {
        let a = self.alertness();

        // total sleep time for all shifts.
        let total = a.iter().map(|x| *x as u32).sum();

        // captures the minute the guard was alseep the most
        let mut worse = WorseMinute { minute: 0, sleeping: 0 };
        for i in 0 .. 60 {
            let sleep = *a.get(i).expect("invalid schedule");
            if sleep > worse.sleeping {
                worse.minute = i.try_into().unwrap();
                worse.sleeping = sleep;
            }
        }

        Summary { id: self.id, total_sleep: total, worse_minute: worse }
    }
}

#[derive(Debug)]
struct Summary {
    id: u32,
    total_sleep: u32,
    worse_minute: WorseMinute
}
#[derive(Debug)]
struct WorseMinute {
    minute: u16,
    sleeping: u16
}

fn parse(input: String) -> Vec<Event> {
    let mut events = input.lines()
        .map(|x| Event::from_str(x).unwrap_or_else(|_| panic!("invalid line {}", x)))
        .collect::<Vec<_>>();
    events.sort_by(|a,b| a.ts.partial_cmp(&b.ts).unwrap());
    events
}

fn to_shifts(events: &Vec<Event>) -> Vec<Guard> {
    let mut guard_shifts: HashMap<u32, Vec<Shift>> = HashMap::new();

    match events.first() {
        Some(Event { ts, event: What::ShiftStart(id)}) => {
            let mut current_guard = id;
            let mut shift = [0; 60];
            let mut shift_day = ts.day.clone();
            for event in events.iter().skip(1) {
                match event {
                    Event { ts, event: What::FallAsleep } => {
                        for i in ts.minute .. 60 {
                            shift[i as usize] = 1;
                        }
                    },
                    Event { ts, event: What::WakeUp } => {
                        for i in ts.minute .. 60 {
                            shift[i as usize] = 0;
                        }
                    },
                    Event { ts, event: What::ShiftStart(id) } => {
                        let current_shift = Shift { day: shift_day, sleeping: shift.to_vec() };
                        guard_shifts.entry(*current_guard).or_insert(Vec::new()).push(current_shift);
                        current_guard = id;
                        shift = [0; 60];
                        shift_day = ts.day.clone();
                    }
                }
            }
            let current_shift = Shift { day: shift_day, sleeping: shift.to_vec() };
            guard_shifts.entry(*current_guard).or_insert(Vec::new()).push(current_shift);
        },
        Some(event) => panic!("inalid first event {:?}", event),
        None => unimplemented!() // TODO
    }

    guard_shifts.iter()
        .map(|(g,s)| Guard { id: *g, shifts: s.clone()} )
        .collect::<Vec<_>>()
}

pub struct Puzzle4;

impl crate::Puzzle for Puzzle4 {

    fn part1(&self, input: String) -> String {
        let events = parse(input);
        let guard_shifts = to_shifts(&events);

        let worse = guard_shifts.iter()
            .map(|x| x.summary())
            .max_by(|a,b| a.total_sleep.cmp(&b.total_sleep)).expect("no shifts");

        println!("{:?}", worse);
        (worse.id * worse.worse_minute.minute as u32).to_string()
    }

    fn part2(&self, input: String) -> String {
        let events = parse(input);
        let guard_shifts = to_shifts(&events);

        let worse = guard_shifts.iter()
            .map(|x| x.summary())
            .max_by(|a,b| a.worse_minute.sleeping.cmp(&b.worse_minute.sleeping)).expect("no shifts");

        println!("{:?}", worse);
        (worse.id * worse.worse_minute.minute as u32).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Range;

    #[test]
    fn test_ts_from_str() {
        assert_eq!(Ts::from_str("[1518-04-22 00:56]"), Ok(Ts { day: "1518-04-22".to_string(), hour: 0, minute: 56} ));
    }

    #[test]
    fn test_what_from_str() {
        assert_eq!(What::from_str("falls asleep"), Ok(FallAsleep));
        assert_eq!(What::from_str("wakes up"), Ok(WakeUp));
        assert_eq!(What::from_str("Guard #2851 begins shift"), Ok(ShiftStart(2851)));
        assert_eq!(What::from_str("Guard #3491 begins shift"), Ok(ShiftStart(3491)));
    }

    #[test]
    fn test_event_from_str() {
        fn ts() -> Ts { Ts { day: "1518-04-22".to_string(), hour: 0, minute: 56 } }
        assert_eq!(Event::from_str("[1518-04-22 00:56] falls asleep"), Ok(Event { ts: ts(), event: FallAsleep }));
        assert_eq!(Event::from_str("[1518-04-22 00:56] wakes up"), Ok(Event { ts: ts(), event: WakeUp }));
        assert_eq!(Event::from_str("[1518-04-22 00:56] Guard #3491 begins shift"), Ok(Event { ts: ts(), event: ShiftStart(3491) }));
    }

    #[test]
    fn test_to_shifts() {
        let events = vec![
            Event { ts: Ts { day: "1518-02-14".to_string(), hour: 23, minute: 52 }, event: ShiftStart(2939) },
            Event { ts: Ts { day: "1518-02-15".to_string(), hour: 0, minute: 0 }, event: FallAsleep },
            Event { ts: Ts { day: "1518-02-15".to_string(), hour: 0, minute: 41 }, event: WakeUp },

            Event { ts: Ts { day: "1518-02-15".to_string(), hour: 23, minute: 57 }, event: ShiftStart(131) },
            Event { ts: Ts { day: "1518-02-16".to_string(), hour: 0, minute: 7 }, event: FallAsleep },
            Event { ts: Ts { day: "1518-02-16".to_string(), hour: 0, minute: 44 }, event: WakeUp },

            Event { ts: Ts { day: "1518-02-17".to_string(), hour: 0, minute: 0 }, event: ShiftStart(2399) },
            Event { ts: Ts { day: "1518-02-17".to_string(), hour: 0, minute: 13 }, event: FallAsleep },
            Event { ts: Ts { day: "1518-02-17".to_string(), hour: 0, minute: 36 }, event: WakeUp },

            Event { ts: Ts { day: "1518-02-17".to_string(), hour: 23, minute: 59 }, event: ShiftStart(3373) },
            Event { ts: Ts { day: "1518-02-18".to_string(), hour: 0, minute: 6 }, event: FallAsleep },
            Event { ts: Ts { day: "1518-02-18".to_string(), hour: 0, minute: 19 }, event: WakeUp },
            Event { ts: Ts { day: "1518-02-18".to_string(), hour: 0, minute: 46 }, event: FallAsleep },
            Event { ts: Ts { day: "1518-02-18".to_string(), hour: 0, minute: 51 }, event: WakeUp },
            Event { ts: Ts { day: "1518-02-18".to_string(), hour: 0, minute: 56 }, event: FallAsleep },
            Event { ts: Ts { day: "1518-02-18".to_string(), hour: 0, minute: 58 }, event: WakeUp }
        ];

        let mut guards = to_shifts(&events);
        guards.sort_by(|a,b| a.id.cmp(&b.id));
        assert_eq!(guards.len(), 4 as usize);

        fn assert_guard(guards: &Vec<Guard>, id: u32, date: String, asleep: Vec<Range<usize>>) -> () {
            let mut sleeping = [0; 60];
            for a in asleep {
                for i in a {
                    sleeping[i] = 1;
                }
            }
            let guard = guards.iter().find(|g| g.id == id);
            assert_eq!(guard, Some(&Guard { id, shifts: vec![Shift { day: date, sleeping: sleeping.to_vec()} ]}));
        }

        assert_guard(&guards, 2939, "1518-02-14".to_string(), vec![0..41]);
        assert_guard(&guards, 2399, "1518-02-17".to_string(), vec![13..36]);
        assert_guard(&guards, 131, "1518-02-15".to_string(), vec![7..44]);
        assert_guard(&guards, 3373, "1518-02-17".to_string(), vec![6..19, 46..51, 56..58]);
    }
}