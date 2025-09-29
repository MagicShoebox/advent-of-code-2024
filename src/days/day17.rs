use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use regex::Regex;

use crate::{Error, SolveError, SolveResult};

type Program = Vec<Instruction>;

#[derive(Default)]
struct State {
    registers: [u64; 3],
    instruction_pointer: usize,
    output: Vec<u8>,
}

impl State {
    fn run(&mut self, program: &Program, reg_a: u64) {
        self.registers[Register::A as usize] = reg_a;
        self.registers[Register::B as usize] = 0;
        self.registers[Register::C as usize] = 0;
        self.instruction_pointer = 0;
        self.output.clear();
        while let Some(instruction) = program.get(self.instruction_pointer) {
            if self.execute(instruction) {
                self.instruction_pointer += 1;
            }
        }
    }

    fn execute(&mut self, instr: &Instruction) -> bool {
        match instr {
            Instruction::Adv(operand) => {
                self.registers[Register::A as usize] /= 2_u64.pow(operand.value(self) as u32)
            }
            Instruction::Bxl(operand) => {
                self.registers[Register::B as usize] ^= *operand as u64;
            }
            Instruction::Bst(operand) => {
                self.registers[Register::B as usize] = operand.value(self) % 8
            }
            Instruction::Jnz(operand) => {
                if operand % 2 != 0 {
                    panic!("Jump operand must be multiple of 2")
                }
                if self.registers[Register::A as usize] != 0 {
                    self.instruction_pointer = *operand as usize / 2;
                    return false;
                }
            }
            Instruction::Bxc => {
                self.registers[Register::B as usize] ^= self.registers[Register::C as usize]
            }
            Instruction::Out(operand) => self.output.push((operand.value(self) % 8) as u8),
            Instruction::Bdv(operand) => {
                self.registers[Register::B as usize] =
                    self.registers[Register::A as usize] / 2_u64.pow(operand.value(self) as u32)
            }
            Instruction::Cdv(operand) => {
                self.registers[Register::C as usize] =
                    self.registers[Register::A as usize] / 2_u64.pow(operand.value(self) as u32)
            }
        }
        true
    }
}

#[repr(usize)]
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Register {
    A,
    B,
    C,
}

trait Operand {
    fn value(&self, computer: &State) -> u64;
}

type LiteralOperand = u8;
impl Operand for LiteralOperand {
    fn value(&self, _: &State) -> u64 {
        *self as u64
    }
}

enum ComboOperand {
    Literal(u8),
    Register(Register),
}

impl ComboOperand {
    fn from(operand: u8) -> Self {
        match operand {
            0..=3 => ComboOperand::Literal(operand),
            4 => ComboOperand::Register(Register::A),
            5 => ComboOperand::Register(Register::B),
            6 => ComboOperand::Register(Register::C),
            _ => panic!("Invalid operand"),
        }
    }
}
impl Operand for ComboOperand {
    fn value(&self, computer: &State) -> u64 {
        match self {
            ComboOperand::Literal(v) => *v as u64,
            ComboOperand::Register(r) => computer.registers[*r as usize],
        }
    }
}

enum Instruction {
    Adv(ComboOperand),
    Bxl(LiteralOperand),
    Bst(ComboOperand),
    Jnz(LiteralOperand),
    Bxc,
    Out(ComboOperand),
    Bdv(ComboOperand),
    Cdv(ComboOperand),
}

impl Instruction {
    fn from(opcode: u8, operand: u8) -> Self {
        match opcode {
            0 => Instruction::Adv(ComboOperand::from(operand)),
            1 => Instruction::Bxl(operand),
            2 => Instruction::Bst(ComboOperand::from(operand)),
            3 => Instruction::Jnz(operand),
            4 => Instruction::Bxc,
            5 => Instruction::Out(ComboOperand::from(operand)),
            6 => Instruction::Bdv(ComboOperand::from(operand)),
            7 => Instruction::Cdv(ComboOperand::from(operand)),
            _ => panic!("Invalid opcode"),
        }
    }
}

pub fn solve(input: &str) -> SolveResult {
    let (raw_program, reg_a) = parse(input)?;
    let program = raw_program
        .chunks_exact(2)
        .map(|c| Instruction::from(c[0], c[1]))
        .collect();

    Ok((part1(&program, reg_a), part2(&program, &raw_program)))
}

fn parse(input: &str) -> Result<(Vec<u8>, u64), SolveError> {
    let blank = Regex::new(r"\r?\n\r?\n")?;
    if let [registers, program] = blank.splitn(input, 2).collect::<Vec<_>>()[..] {
        let re = Regex::new(r"Register\s*A:\s*(\d+)")?;
        let reg_a = re
            .captures(registers)
            .ok_or(Error::InputError("Couldn't find register A"))?;
        let (_, [reg_a]) = reg_a.extract();
        let reg_a = reg_a.parse()?;

        let re = Regex::new(r"Program:\s*([,\d]+)")?;
        let raw_program = re
            .captures(program)
            .ok_or(Error::InputError("Couldn't parse program"))?;
        let (_, [raw_program]) = raw_program.extract();
        let raw_program = raw_program
            .split(",")
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok((raw_program, reg_a))
    } else {
        Err(Error::InputError("Couldn't find blank line between registers and program").into())
    }
}

fn part1(program: &Program, reg_a: u64) -> String {
    let mut state = State::default();
    state.run(program, reg_a);
    state
        .output
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(",")
        .to_string()
}

fn part2(program: &Program, target: &Vec<u8>) -> String {
    let mut checked: HashSet<u64> = HashSet::new();
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse((usize::MAX, 0)));
    while let Some(Reverse((s, reg_a))) = priority_queue.pop() {
        if s == 0 {
            return reg_a.to_string();
        }
        let neighbors = (0..=63)
            .map(|i| reg_a ^ (1 << i))
            .filter(|x| !checked.contains(x))
            .collect::<Vec<_>>();
        checked.extend(neighbors.iter().copied());
        priority_queue.extend(
            neighbors
                .into_iter()
                .map(|x| Reverse((score(program, target, x), x))),
        );
    }
    "Not found!".to_string()
}

fn score(program: &Program, target: &Vec<u8>, reg_a: u64) -> usize {
    let mut state = State::default();
    state.run(program, reg_a);
    10 * target.len().abs_diff(state.output.len())
        + target
            .iter()
            .zip(state.output)
            .map(|(x, y)| x.abs_diff(y) as usize)
            .sum::<usize>()
}
