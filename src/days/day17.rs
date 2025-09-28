use regex::Regex;

use crate::{Error, SolveError, SolveResult};

#[derive(Default)]
struct Computer {
    registers: [i64; 3],
    program: Vec<u8>,
    instruction_pointer: usize,
    output: Vec<u8>,
}

impl Computer {
    fn run(&mut self) {
        while let Some([opcode, operand]) = self
            .program
            .get(self.instruction_pointer..=self.instruction_pointer + 1)
        {
            if self.execute(Instruction::from(*opcode, *operand)) {
                self.instruction_pointer += 2;
            }
        }
    }

    fn execute(&mut self, instr: Instruction) -> bool {
        match instr {
            Instruction::Adv(operand) => {
                self.registers[Register::A as usize] /= 2_i64.pow(operand.value(self) as u32)
            }
            Instruction::Bxl(operand) => {
                self.registers[Register::B as usize] ^= operand as i64;
            }
            Instruction::Bst(operand) => {
                self.registers[Register::B as usize] = operand.value(self) % 8
            }
            Instruction::Jnz(operand) => {
                if self.registers[Register::A as usize] != 0 {
                    self.instruction_pointer = operand as usize;
                    return false;
                }
            }
            Instruction::Bxc => {
                self.registers[Register::B as usize] ^= self.registers[Register::C as usize]
            }
            Instruction::Out(operand) => self.output.push((operand.value(self) % 8) as u8),
            Instruction::Bdv(operand) => {
                self.registers[Register::B as usize] =
                    self.registers[Register::A as usize] / 2_i64.pow(operand.value(self) as u32)
            }
            Instruction::Cdv(operand) => {
                self.registers[Register::C as usize] =
                    self.registers[Register::A as usize] / 2_i64.pow(operand.value(self) as u32)
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
    fn value(&self, computer: &Computer) -> i64;
}

type LiteralOperand = u8;
impl Operand for LiteralOperand {
    fn value(&self, _: &Computer) -> i64 {
        *self as i64
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
    fn value(&self, computer: &Computer) -> i64 {
        match self {
            ComboOperand::Literal(v) => *v as i64,
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
    let mut computer = parse(input)?;
    computer.run();
    Ok((part1(&computer), part2()))
}

fn parse(input: &str) -> Result<Computer, SolveError> {
    let mut computer = Computer::default();
    let blank = Regex::new(r"\r?\n\r?\n")?;
    if let [registers, program] = blank.splitn(input, 2).collect::<Vec<_>>()[..] {
        let re = Regex::new(r"Register\s*(\w):\s*(\d+)")?;
        for register in re.captures_iter(registers) {
            let (_, [r, v]) = register.extract();
            match r {
                "A" => computer.registers[Register::A as usize] = v.parse()?,
                "B" => computer.registers[Register::B as usize] = v.parse()?,
                "C" => computer.registers[Register::C as usize] = v.parse()?,
                _ => {}
            };
        }
        let re = Regex::new(r"Program:\s*([,\d]+)")?;
        let program = re
            .captures(program)
            .ok_or(Error::InputError("Couldn't parse program"))?;
        let (_, [program]) = program.extract();
        computer.program = program
            .split(",")
            .map(str::parse)
            .collect::<Result<_, _>>()?;
        Ok(computer)
    } else {
        Err(Error::InputError("Couldn't find blank line between registers and program").into())
    }
}

fn part1(computer: &Computer) -> String {
    computer
        .output
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(",")
        .to_string()
}

fn part2() -> String {
    String::new()
}
