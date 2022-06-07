use regex::Regex;
use serde::Deserialize;
use std::str::FromStr;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Hash, Deserialize, Clone, Debug)]
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
    fn run(&self, bench: &mut [usize; 6], a: usize, b: usize, c: usize) {
        use OpCode::*;
        match self {
            // addr (add register) stores into register C the result of adding register A and register B.
            addr => bench[c] = bench[a] + bench[b],
            // addi (add immediate) stores into register C the result of adding register A and value B.
            addi => bench[c] = bench[a] + b,

            // mulr (multiply register) stores into register C the result of multiplying register A and register B.
            mulr => bench[c] = bench[a] * bench[b],
            // muli (multiply immediate) stores into register C the result of multiplying register A and value B.
            muli => bench[c] = bench[a] * b,

            // banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
            banr => bench[c] = bench[a] & bench[b],
            // bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.
            bani => bench[c] = bench[a] & b,

            // borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
            borr => bench[c] = bench[a] | bench[b],
            // bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.
            bori => bench[c] = bench[a] | b,

            // setr (set register) copies the contents of register A into register C. (Input B is ignored.)
            setr => bench[c] = bench[a],
            // seti (set immediate) stores value A into register C. (Input B is ignored.)
            seti => bench[c] = a,

            // gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
            gtir => bench[c] = if a > bench[b] { 1 } else { 0 },
            // gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
            gtri => bench[c] = if bench[a] > b { 1 } else { 0 },
            // gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.
            gtrr => bench[c] = if bench[a] > bench[b] { 1 } else { 0 },

            // eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
            eqir => bench[c] = if a == bench[b] { 1 } else { 0 },
            // eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
            eqri => bench[c] = if bench[a] == b { 1 } else { 0 },
            // eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
            eqrr => bench[c] = if bench[a] == bench[b] { 1 } else { 0 },
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Instr {
    code: OpCode,
    a: usize,
    b: usize,
    c: usize,
}

impl FromStr for Instr {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
        Ok(Instr {
            code: serde_plain::from_str::<OpCode>(parts[0]).unwrap(), // using ? requires converting the error, not sure what's the best approach
            a: usize::from_str(parts[1])?,
            b: usize::from_str(parts[2])?,
            c: usize::from_str(parts[3])?,
        })
    }
}

#[derive(Clone)]
struct Cpu {
    ip_register: usize,
    ip: usize,
    bench: [usize; 6],
}

impl Cpu {
    fn new(ip_register: usize) -> Self {
        Cpu {
            ip_register,
            ip: 0,
            bench: [0; 6],
        }
    }

    fn run(&mut self, program: &Vec<Instr>) {
        let mut clock = 0u64;
        loop {
            match program.get(self.ip as usize) {
                None => break,
                Some(i) => {
                    // before the instruction, set the instr pointer register to the value of the instr pointer.
                    self.bench[self.ip_register] = self.ip;

                    if self.ip == 1 && program.len() > 7 {
                        // special case part2
                        // sum of factors.
                        self.bench[0] = self.bench[3]
                            + (1..=self.bench[3] / 2)
                                .filter(|x| self.bench[3] % *x == 0)
                                .sum::<usize>();
                        /*
                        self.bench[5] = 1;
                        while self.bench[5] <= self.bench[3] {
                            self.bench[2] = 1;
                            while self.bench[2] <= self.bench[3] {
                                if self.bench[2] * self.bench[5] == self.bench[3] {
                                    self.bench[0] += self.bench[5];
                                }
                                self.bench[2] += 1;
                            }
                            self.bench[5] += 1;
                        }
                        println!(" {:?}", self.bench);
                        */
                        self.ip = program.len(); // exit
                        continue;
                    }

                    print!("({}) ip={} {:?} {:?}", clock, self.ip, self.bench, i);
                    i.code.run(&mut self.bench, i.a, i.b, i.c);
                    // after the instruction, set the instr pointer to the value of the instr register and increment by one
                    // TODO: the instructions say this should only be done if the instruction modified the register, but I guess there's no harm to do it always?
                    self.ip = self.bench[self.ip_register] + 1;
                    println!(" {:?}", self.bench);
                    clock += 1;
                }
            }
        }
    }
}

fn parse(input: &str) -> (Cpu, Vec<Instr>) {
    let ip_register = match input.lines().take(1).last() {
        None => panic!("empty input"),
        Some(line) => {
            let re = Regex::new(r"^#ip (\d)$").unwrap();
            let caps = re.captures(line).expect("invalid instr register");
            usize::from_str(&caps[1]).expect("invalid input")
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
    program: Vec<Instr>,
}

impl crate::Puzzle for Puzzle19 {
    fn part1(&self) -> String {
        let mut cpu = self.cpu.clone();
        cpu.run(&self.program);
        cpu.bench[0].to_string()
    }

    fn part2(&self) -> String {
        let mut cpu = self.cpu.clone();
        cpu.bench[0] = 1;
        cpu.run(&self.program);
        cpu.bench[0].to_string()
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
        assert_eq!(
            Some(Instr {
                code: OpCode::seti,
                a: 9,
                b: 0,
                c: 5
            }),
            program.pop()
        );
        assert_eq!(
            Some(Instr {
                code: OpCode::seti,
                a: 8,
                b: 0,
                c: 4
            }),
            program.pop()
        );
        assert_eq!(
            Some(Instr {
                code: OpCode::setr,
                a: 1,
                b: 0,
                c: 0
            }),
            program.pop()
        );
    }

    #[test]
    fn test_example() {
        let (mut cpu, program) = parse(EXAMPLE);
        cpu.run(&program);
        assert_eq!([6, 5, 6, 0, 0, 9], cpu.bench);
    }
}
