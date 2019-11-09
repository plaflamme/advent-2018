use std::str::FromStr;
use regex::Regex;
use std::collections::HashSet;

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
    fn run(&self, bench: &mut [usize;6], a: usize, b: usize, c: usize) {
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

#[derive(PartialEq, Eq, Clone, Debug)]
struct Instr {
    code: OpCode,
    a: usize,
    b: usize,
    c: usize
}

impl FromStr for Instr {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
        Ok(
            Instr {
                code: OpCode::from_str(parts[0]).unwrap(), // using ? requires converting the error, not sure what's the best approach
                a: usize::from_str(parts[1])?,
                b: usize::from_str(parts[2])?,
                c: usize::from_str(parts[3])?,
            }
        )
    }
}

#[derive(Clone)]
struct Cpu {
    ip_register: usize,
    ip: usize,
    bench: [usize; 6]
}

impl Cpu {
    fn new(ip_register: usize) -> Self {
        Cpu { ip_register, ip: 0, bench: [0;6] }
    }

    fn step(&mut self, i: &Instr) {
        // before the instruction, set the instr pointer register to the value of the instr pointer.
        self.bench[self.ip_register] = self.ip;

//        print!("ip={} {:?} {:?}", self.ip, self.bench, i);
        i.code.run(&mut self.bench, i.a, i.b, i.c);
        // after the instruction, set the instr pointer to the value of the instr register and increment by one
        self.ip = self.bench[self.ip_register] + 1;
//        println!(" {:?}", self.bench);
    }
}

enum State {
    Breakpoint,
    Halt
}

struct Debugger {
    cpu: Cpu,
    program: Vec<Instr>,
    breakpoint: usize
}

impl Debugger {
    fn run(&mut self) -> State {
        loop {
            match self.program.get(self.cpu.ip) {
                None => break  State::Halt,
                Some(instr) => {
                    self.cpu.step(instr);
                    if self.cpu.ip == self.breakpoint {
                       break State::Breakpoint;
                    }
                }
            };
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
    Box::new(Puzzle21 { cpu, program })
}

struct Puzzle21 {
    cpu: Cpu,
    program: Vec<Instr>
}
/*
#ip 5

L0:
  ip=0 R3 = 123
  ip=1 R3 = R3 & 456
  ip=2 if R3 == 72 {
         GOTO L1 // ip=5
       } else {
         R5=0 // GOTO ip=1
       }

ip=0  seti 123 0 3
ip=1  bani 3 456 3
ip=2  eqri 3 72 3
ip=3  addr 3 5 5
ip=4  seti 0 0 5

L1:
  ip=5  R3 = 0
  ip=6  R2 = R3 | 65536
  ip=7  R3 = 14070682
  ip=8  R1 = R2 & 255
  ip=9  R3 = R3 + R1
  ip=10 R3 = R3 & 16777215 // R3 & FFFFFF
  ip=11 R3 = R3 * 65899
  ip=12 R3 = R3 & 16777215 // R3 & FFFFFF

  ip=13 if(256 > R2) {
  ip=14   GOTO L2 // ip=16
        } else {
  ip=15   GOTO L3 // R5 += 1 ip=17
        }

ip=5  seti 0 0 3
ip=6  bori 3 65536 2
ip=7  seti 14070682 0 3
ip=8  bani 2 255 1
ip=9  addr 3 1 3
ip=10 bani 3 16777215 3
ip=11 muli 3 65899 3
ip=12 bani 3 16777215 3
ip=13 gtir 256 2 1
ip=14 addr 1 5 5
ip=15 addi 5 1 5

L2:
  ip=16 GOTO L4 // R5 = 27

ip=16 seti 27 8 5

L3:
  ip=17 R1 = 0
  ip=18 R4 = R1 + 1
  ip=19 R4 = R4 * 256

  ip=20 if R4 > R2 {
  ip=21   GOTO L6 // ip=23
        } else {
          GOTO L7 // ip=22
        }

ip=17 seti 0 3 1
ip=18 addi 1 1 4
ip=19 muli 4 256 4
ip=20 gtrr 4 2 4
ip=21 addr 4 5 5
ip=22 addi 5 1 5

L6:
  ip=23 GOTO L8

ip=23 seti 25 8 5

L7:
  ip=24 R1 = R1 + 1
  ip=25 R5 = 17 // GOTO ip=18
ip=24 addi 1 1 1
ip=25 seti 17 9 5

L8:
    ip=26 R2=R1
    ip=27 R5 = 7 // GOTO ip=8

ip=26 setr 1 4 2
ip=27 seti 7 5 5

L4:
  ip=28 if R3 == 0 {
          HALT // R5 will become 31
        } else {
          R5 = 5 // GOTO ip=6
        }

ip=28 eqrr 3 0 1
ip=29 addr 1 5 5
ip=30 seti 5 4 5


===
R0..R5=0

R3=123
do {
    R3 = R3 & 456
} while R3 != 72

R3=0
do {
  R2 = R3 | 65536

  L1:
  R1 = R2 & 255
  R3 = R3 + R1
  R3 = R3 & 16777215 // R3 & FFFFFF
  R3 = R3 * 65899
  R3 = R3 & 16777215 // R3 & FFFFFF

  if R2 <= 256 {
    for(R1 = 0; R4 <= R2; R1++) {
      R4 = R1 + 1;
      R4 *= 256;
    }
    R2 = R1;
    GOTO L1;
  }
} while R3 != R0

*/
impl crate::Puzzle for Puzzle21 {
    fn part1(&self) -> String {
        // ip=28 is when we compare against register 0 with R3, so we simply need to stop at that point and check what the contents of R3 is
        let mut debug = Debugger { cpu: self.cpu.clone(), program: self.program.clone(), breakpoint: 28 };
        debug.run();
        debug.cpu.bench[3].to_string()
    }

    fn part2(&self) -> String {
        // the most instructions is right before R3 loops around to some value we've seen before.
        let mut debug = Debugger { cpu: self.cpu.clone(), program: self.program.clone(), breakpoint: 28 };
        let mut seen = HashSet::new();
        let mut prev = 0 as usize;
        let found = loop {
            debug.run();
            if !seen.insert(debug.cpu.bench[3]) {
                break prev;
            }
            prev = debug.cpu.bench[3];
        };
        found.to_string()
    }
}