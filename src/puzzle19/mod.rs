use std::ops::{Index, IndexMut};
use std::str::FromStr;
use regex::Regex;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Hash, enum_utils::FromStr, Clone, Debug)]
enum OpCode {
    addr,
    addi,

    mulr,
    muli,

    banr,
    bani,

    borr,
    bori,

    setr,
    seti,

    gtir,
    gtri,
    gtrr,

    eqir,
    eqri,
    eqrr,
}

impl OpCode {
    fn run(&self, bench: &mut Bench, a: &u16, b: &u16, c: &u16) {
        use OpCode::*;
        match self {
            // addr (add register) stores into register C the result of adding register A and register B.
            addr => bench[c] = bench[a] + bench[b],
            // addi (add immediate) stores into register C the result of adding register A and value B.
            addi => bench[c] = bench[a] + *b,

            // mulr (multiply register) stores into register C the result of multiplying register A and register B.
            mulr => bench[c] = bench[a] * bench[b],
            // muli (multiply immediate) stores into register C the result of multiplying register A and value B.
            muli => bench[c] = bench[a] * *b,

            // banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
            banr => bench[c] = bench[a] & bench[b],
            // bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
            bani => bench[c] = bench[a] & *b,

            // borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
            borr => bench[c] = bench[a] | bench[b],
            // bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
            bori => bench[c] = bench[a] | *b,

            // setr (set register) copies the contents of register A into register C. (Input B is ignored.)
            setr => bench[c] = bench[a],
            // seti (set immediate) stores value A into register C. (Input B is ignored.)
            seti => bench[c] = *a,

            // gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
            gtir => bench[c] = if *a > bench[b] { 1 } else { 0 },
            // gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
            gtri => bench[c] = if bench[a] > *b { 1 } else { 0 },
            // gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
            gtrr => bench[c] = if bench[a] > bench[b] { 1 } else { 0 },

            // eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
            eqir => bench[c] = if *a == bench[b] { 1 } else { 0 },
            // eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
            eqri => bench[c] = if bench[a] == *b { 1 } else { 0 },
            // eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
            eqrr => bench[c] = if bench[a] == bench[b] { 1 } else { 0 },
        }
    }
}

#[derive(PartialEq, Eq, Default, Clone, Debug)]
struct Bench([u16; 6]);

impl Index<&u16> for Bench {
    type Output = u16;

    fn index(&self, index: &u16) -> &Self::Output {
        assert!(*index < 4, format!("Register should be [0,4[, but was {}", index));
        &self.0[*index as usize]
    }
}

impl IndexMut<&u16> for Bench {
    fn index_mut(&mut self, index: &u16) -> &mut Self::Output {
        assert!(*index < 4, format!("Register should be [0,4[, but was {}", index));
        &mut self.0[*index as usize]
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Instr {
    code: OpCode,
    a: u16,
    b: u16,
    c: u16
}

impl FromStr for Instr {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
        Ok(
            Instr {
                code: OpCode::from_str(parts[0]).unwrap(), // using ? requires converting the error, not sure what's the best approach
                a: u16::from_str(parts[1])?,
                b: u16::from_str(parts[2])?,
                c: u16::from_str(parts[3])?,
            }
        )
    }
}

struct Cpu {
    ip_register: u8,
    ip: u8,
    bench: Bench
}

impl Cpu {
    fn new(ip_register: u8) -> Self {
        Cpu { ip_register, ip: 0, bench: Bench::default() }
    }

    fn run(&mut self, program: &Vec<Instr>) {
        program
            .iter()
            .for_each(|i| {
                i.code.run(&mut self.bench, &i.a, &i.b, &i.c);
            });
    }
}

fn parse(input: &str) -> (Cpu, Vec<Instr>) {

    let ip_register = match input.lines().take(1).last() {
        None => panic!("empty input"),
        Some(line) => {
            let re = Regex::new(r"^#ip (\d)$").unwrap();
            let caps = re.captures(line).expect("invalid instr register");
            u8::from_str(&caps[1]).expect("invalid input")
        }
    };

    let program = input
        .lines()
        .skip(1)
        .map(|line| Instr::from_str(line).expect(&format!("invalid line {}", line)))
        .collect::<Vec<_>>();

    (Cpu::new(ip_register), program)
}

pub fn mk(input: String) -> Box<dyn crate::Puzzle> {
    let (cpu, program) = parse(&input);
    Box::new(Puzzle19 { cpu, program })
}

struct Puzzle19 {
    cpu: Cpu,
    program: Vec<Instr>
}

impl crate::Puzzle for Puzzle19 {
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

    const EXAMPLE: &str = r#"#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5"#;

    #[test]
    fn test_parse() {
        let (cpu, mut program) = parse(EXAMPLE);
        assert_eq!(0, cpu.ip_register);
        assert_eq!(7, program.len());
        assert_eq!(Some(Instr { code: OpCode::seti, a: 9, b: 0, c: 5 }), program.pop());
        assert_eq!(Some(Instr { code: OpCode::seti, a: 8, b: 0, c: 4 }), program.pop());
        assert_eq!(Some(Instr { code: OpCode::setr, a: 1, b: 0, c: 0 }), program.pop());
    }
}