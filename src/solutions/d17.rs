use std::io::{BufRead, Read};
use itertools::Itertools;
use regex_macro::regex;

use crate::{misc::option::OptionExt, output, Input, Output};


#[derive(Debug, Clone, Copy)]
enum Opcode {
    ADV, BDV, CDV,
    BXL, BXC,
    BST, OUT,
    JNZ
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
    fn to_number(&self, register_file: &[i32]) -> i32 {
        match self {
            Operand::Register(register) => register_file[*register as usize],
            Operand::Immediate(value) => *value as i32,
            Operand::Reserved => -1
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct Instruction {
    pub op: Opcode,
    pub operand: Operand
}


pub fn solve(input: Input) -> Output {
    let mut input = input.lines();
    let register_regex = regex!(r"Register (\w+): (-?\d+)");
    let mut register_file = [0i32; 3];
    for line in input.by_ref() {
        let line = line?;
        if line.len() == 0 {
            break;
        }
        let matches = register_regex
            .captures(line.as_str())
            .unwrap_or_err()?;

        register_file[(matches[1].as_bytes()[0] as u8 - b'A') as usize] = matches[2].parse()?;
    }
    dbg!(&register_file);

    let program_regex = regex!(r"Program: (\d[,\d]+)");
    let mut program = Vec::new();
    for line in input {
        let line = line?;
        let matches = &program_regex
            .captures(line.as_str())
            .unwrap_or_err()?;

        for n in matches[1].split(',') {
            program.push(n.parse::<u8>()?);
        }

        // for mut chunk in &matches[1].split(",").chunks(2) {
        //     let op = chunk.next().unwrap_or_err()?.parse::<u8>()?;
        //     let operand = chunk.next().unwrap_or_err()?.parse::<u8>()?;

        //     program.push(Instruction {
        //         op: match op {
        //             0 => Opcode::ADV,
        //             1 => Opcode::BXL,
        //             2 => Opcode::BST,
        //             3 => Opcode::JNZ,
        //             4 => Opcode::BXC,
        //             5 => Opcode::OUT,
        //             6 => Opcode::BDV,
        //             7 => Opcode::CDV,
        //             _ => unreachable!()
        //         },
        //         operand: match operand {
        //             0..=3 => Operand::Immediate(operand),
        //             4 => Operand::Register(Register::A),
        //             5 => Operand::Register(Register::B),
        //             6 => Operand::Register(Register::C),
        //             7 => Operand::Reserved,
        //             _ => unreachable!()
        //         }
        //     });
        // }
    }
    dbg!(&program);

    let mut output = Vec::new();
    let mut pc = 0;
    while pc < program.len() - 1 {
        // let instruction = &program[pc];
        let op = program[pc];
        let operand = program[pc + 1];
        let instruction = Instruction {
            op: match op {
                0 => Opcode::ADV,
                1 => Opcode::BXL,
                2 => Opcode::BST,
                3 => Opcode::JNZ,
                4 => Opcode::BXC,
                5 => Opcode::OUT,
                6 => Opcode::BDV,
                7 => Opcode::CDV,
                _ => unreachable!()
            },
            operand: match operand {
                0..=3 => Operand::Immediate(operand),
                4 => Operand::Register(Register::A),
                5 => Operand::Register(Register::B),
                6 => Operand::Register(Register::C),
                7 => Operand::Reserved,
                _ => unreachable!()
            }
        };

        println!("{pc}: {:?} {:?}", instruction.op, instruction.operand);
        match instruction.op {
            Opcode::ADV | Opcode::BDV | Opcode::CDV => {
                let numerator = register_file[Register::A as usize];
                let combo = instruction.operand.to_number(&register_file);
                let denominator = 2i32.pow(combo as u32);
                register_file[match instruction.op {
                    Opcode::ADV => Register::A,
                    Opcode::BDV => Register::B,
                    Opcode::CDV => Register::C,
                    _ => unreachable!()
                } as usize] = numerator / denominator;
            },
            Opcode::BXL | Opcode::BXC => {
                let input = register_file[Register::B as usize];
                let combo = match instruction.op {
                    Opcode::BXL => operand as i32,
                    Opcode::BXC => register_file[Register::C as usize],
                    _ => unreachable!()
                };
                register_file[Register::B as usize] = input ^ combo;
            },
            Opcode::BST | Opcode::OUT => {
                let combo = instruction.operand.to_number(&register_file);
                let result = combo % 8;
                match instruction.op {
                    Opcode::BST => register_file[Register::B as usize] = result,
                    Opcode::OUT => {
                        let result = result.to_string();
                        output.push(result);
                    },
                    _ => unreachable!()
                }
            },
            Opcode::JNZ => {
                if register_file[Register::A as usize] != 0 {
                    pc = operand as usize;
                    println!("jumped: {:?} to {}", &register_file, pc);
                    continue;
                }
            }
        }
        println!("{:?}", &register_file);
        pc += 2;
    }
    println!("{:?}", output);
    dbg!(output.join(","));

    output!(output.join("").parse::<i64>()?)
}
