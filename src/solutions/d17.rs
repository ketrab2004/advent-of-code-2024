use std::{collections::VecDeque, io::BufRead};
use color_eyre::eyre::Result;
use itertools::Itertools;
use regex_macro::regex;
use crate::{misc::option::OptionExt, output, Input, Output};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Opcode {
    // adv, bdv, cdv
    DivisionA, DivisionB, DivisionC,
    // bxl, bxc
    XorLiteralB, XorCB,
    // bst, out
    ModuloB, ModuloOut,
    // jnz
    JumpANotZero
}
impl Opcode {
    fn from_number(op: u8) -> Self {
        match op {
            0 => Opcode::DivisionA,
            1 => Opcode::XorLiteralB,
            2 => Opcode::ModuloB,
            3 => Opcode::JumpANotZero,
            4 => Opcode::XorCB,
            5 => Opcode::ModuloOut,
            6 => Opcode::DivisionB,
            7 => Opcode::DivisionC,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Register {
    A, B, C
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Register(Register),
    Immediate(u8),
    Reserved
}
impl Operand {
    fn from_number(operand: u8) -> Self {
        match operand {
            0..=3 => Operand::Immediate(operand),
            4 => Operand::Register(Register::A),
            5 => Operand::Register(Register::B),
            6 => Operand::Register(Register::C),
            7 => Operand::Reserved,
            _ => unreachable!()
        }
    }
    fn as_number(&self, register_file: &[i64]) -> i64 {
        match self {
            Operand::Register(register) => register_file[*register as usize],
            Operand::Immediate(value) => *value as i64,
            Operand::Reserved => -1
        }
    }
    fn as_literal(&self) -> i64 {
        match self {
            Operand::Register(register) => *register as i64 + 4,
            Operand::Immediate(value) => *value as i64,
            Operand::Reserved => 7
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct Instruction {
    pub op: Opcode,
    pub operand: Operand
}


fn run(register_file: &mut [i64], program: &[Instruction], output: &mut Vec<u8>, expected: Option<&[u8]>) -> Result<bool> {
    let mut pc = 0;
    while pc < program.len() {
        let instruction = &program[pc];

        match instruction.op {
            Opcode::DivisionA | Opcode::DivisionB | Opcode::DivisionC => {
                let numerator = register_file[Register::A as usize];
                let combo = instruction.operand.as_number(register_file);
                let denominator = 2i64.pow(combo as u32);
                register_file[match instruction.op {
                    Opcode::DivisionA => Register::A,
                    Opcode::DivisionB => Register::B,
                    Opcode::DivisionC => Register::C,
                    _ => unreachable!()
                } as usize] = numerator / denominator;
            },
            Opcode::XorLiteralB | Opcode::XorCB => {
                let input = register_file[Register::B as usize];
                let combo = match instruction.op {
                    Opcode::XorLiteralB => instruction.operand.as_literal(),
                    Opcode::XorCB => register_file[Register::C as usize],
                    _ => unreachable!()
                };
                register_file[Register::B as usize] = input ^ combo;
            },
            Opcode::ModuloB | Opcode::ModuloOut => {
                let combo = instruction.operand.as_number(register_file);
                let result = combo % 8;

                let output_len_before = output.len();
                match instruction.op {
                    Opcode::ModuloB => register_file[Register::B as usize] = result,
                    Opcode::ModuloOut => {
                        let result = result.to_string();
                        for c in result.chars() {
                            output.push(c.to_digit(10).unwrap_or_err()? as u8);
                        }
                    },
                    _ => unreachable!()
                }

                if let Some(expected) = expected {
                    if instruction.op == Opcode::ModuloOut && output[output_len_before..] != expected[output_len_before..output.len()] {
                        return Ok(false);
                    }
                }
            },
            Opcode::JumpANotZero => {
                if register_file[Register::A as usize] != 0 {
                    pc = (instruction.operand.as_literal() / 2) as usize;
                    continue;
                }
            }
        }
        pc += 1;
    }

    Ok(true)
}

pub fn solve(input: Input) -> Output {
    let mut input = input.lines();
    let register_regex = regex!(r"Register (\w+): (-?\d+)");
    let mut register_file = [0i64; 3];
    for line in input.by_ref() {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let matches = register_regex
            .captures(line.as_str())
            .unwrap_or_err()?;

        register_file[(matches[1].as_bytes()[0] - b'A') as usize] = matches[2].parse()?;
    }

    let program_regex = regex!(r"Program: (\d[,\d]+)");
    let mut program = Vec::new();
    let mut raw_program = Vec::new();
    for line in input {
        let line = line?;
        let matches = &program_regex
            .captures(line.as_str())
            .unwrap_or_err()?;

        for mut chunk in &matches[1].split(",").chunks(2) {
            let op = chunk.next().unwrap_or_err()?.parse::<u8>()?;
            let operand = chunk.next().unwrap_or_err()?.parse::<u8>()?;

            raw_program.push(op);
            raw_program.push(operand);

            program.push(Instruction {
                op: Opcode::from_number(op),
                operand: Operand::from_number(operand)
            });
        }
    }

    let original_registers = register_file;
    let mut output = Vec::new();
    run(&mut register_file, program.as_slice(), &mut output,  None)?;
    let output_text = output.iter().map(|d| d.to_string()).join(",");


    // 2,4     BST A   B = A % 8 (A & 0b111)
    // 1,1     BXL 1   B = B ^ 1
    // 7,5     CDV B   C = C / 2**A (C >> A)
    // 0,3     ADV 3   A = A / 2**A (A >> A)
    // 4,7     BXC     B = B ^ C
    // 1,6     BXL 6   B = B ^ 6(0b110)
    // 5,5     OUT B   B % 8 (B & 0b111)
    // 3,0     JNZ 0   if A != 0: goto 0

    // B = (A & 0b111) ^ 0b0001
    // C = C >> A (C is always 0?)
    // A = A >> A
    // B = B ^ C ^ 0b110
    // print B & 0b111
    // repeat until a = 0

    let mut correct_a = -1;
    let mut queue = VecDeque::new();
    queue.push_back((raw_program.len() - 1, 0));

    'outer: while let Some((i, a)) = queue.pop_front() {
        let a = a << 3;
        let last_expected_outputs = &raw_program[i..];

        for d in 0..=7 {
            let mut registers = original_registers;
            let a = a + d;
            registers[Register::A as usize] = a;

            output.clear();
            let finished = run(&mut registers, program.as_slice(), &mut output, Some(last_expected_outputs))?;

            if finished {
                if i == 0 {
                    correct_a = a;
                    break 'outer;
                }

                queue.push_back((i - 1, a));
            }
        }
    }

    output!(output_text, correct_a)
}
