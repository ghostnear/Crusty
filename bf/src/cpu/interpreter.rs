use crate::cpu::State;

use std::io::Write;
use std::borrow::BorrowMut;

pub struct Instruction
{
    pub operation: fn(&mut State, isize),
    pub parameter: isize
}

impl Instruction
{
    fn op_add_to_dp(state: &mut State, parameter: isize)
    {
        state.dp = state.dp.wrapping_add_signed(parameter);
        if state.dp >= state.data.len()
        {
            state.data.resize((state.dp + 1) as usize, 0);
        }
        state.pc += 1;
    }

    fn op_add_to_data(state: &mut State, parameter: isize)
    {
        state.data[state.dp as usize] = state.data[state.dp as usize].wrapping_add_signed(parameter);
        state.pc += 1;
    }

    fn op_print(state: &mut State, _parameter: isize)
    {
        write!(state.output, "{}", (state.data[state.dp as usize] as u8) as char).ok();
        state.pc += 1;
    }

    fn op_read(state: &mut State, _parameter: isize)
    {
        state.data[state.dp as usize] = state.input.read() as usize;
        state.pc += 1;
    }

    fn op_jump(state: &mut State, parameter: isize)
    {
        if (parameter < 0 && state.data[state.dp as usize] == 0) || (parameter > 0 && state.data[state.dp as usize] != 0)
        {
            state.pc += 1;
            return;
        }

        state.pc = state.pc.wrapping_add_signed(parameter);
    }
}

impl std::fmt::Display for Instruction
{
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        if self.operation == Instruction::op_add_to_dp
        {
            return write!(out, "ADD dp, {};", self.parameter);
        }
        if self.operation == Instruction::op_add_to_data
        {
            return write!(out, "ADD [dp], {};", self.parameter);
        }
        if self.operation == Instruction::op_jump
        {
            if self.parameter > 0
            {
                return write!(out, "JMPZ {};", self.parameter);
            }
            return write!(out, "JMPNZ {};", self.parameter);
        }
        if self.operation == Instruction::op_print
        {
            return write!(out, "PRINT;");
        }
        if self.operation == Instruction::op_read
        {
            return write!(out, "READ;");
        }
        panic!("(BF): Unknown instruction in format.")
    }
}

pub struct InterpreterCPU
{
    pub instructions: Vec<Instruction>
}

impl InterpreterCPU
{
    pub fn new() -> Self
    {
        return Self{
            instructions: Vec::new()
        };
    }

    pub fn step(&mut self, state: &mut State)
    {
        let instruction = &self.instructions[state.pc as usize];
        (instruction.operation)(state, instruction.parameter);
    }

    pub fn parse(&mut self, input: &str)
    {
        macro_rules! bf_instruction {
            ($function: expr, $value: expr) => {
                self.instructions.push(Instruction{
                    operation: $function,
                    parameter: $value
                })
            };
        }

        macro_rules! bf_symbol {
            ($function: expr, $change: expr) => {
                let last_instruction = self.instructions.last_mut();
                // If instruction should be added to the list.
                if last_instruction.is_none() || last_instruction.unwrap().operation != $function
                {
                    bf_instruction!($function, $change);
                }
                else
                {
                    // Update last instruction
                    let last_instruction = self.instructions.last_mut().unwrap();
                    last_instruction.parameter += $change;
                    if last_instruction.parameter == 0
                    {
                        self.instructions.pop();
                    }
                }
            };
        }

        for character in input.chars()
        {
            match character
            {
                '>' => {
                    bf_symbol!(Instruction::op_add_to_dp, 1);
                }
                '<' => {
                    bf_symbol!(Instruction::op_add_to_dp, -1);
                },
                '+' => {
                    bf_symbol!(Instruction::op_add_to_data, 1);
                }
                '-' => {
                    bf_symbol!(Instruction::op_add_to_data, -1);
                }
                '[' => {
                    // First parantheses pushed to the 'stack'.
                    bf_instruction!(Instruction::op_jump, 0);
                }
                ']' => {
                    // Find the previous one and update jumps if found.
                    let length = self.instructions.len() as isize;
                    let mut position = self.instructions.len() - 1;
                    let mut stop = false;
                    while !stop
                    {
                        let value = self.instructions[position].borrow_mut();
                        if value.operation == Instruction::op_jump && value.parameter == 0
                        {
                            // It's the last one, optimize it out.
                            if length - position as isize == 1 {
                                self.instructions.pop();
                                break;
                            }

                            value.parameter = length - position as isize;
                            self.instructions.push(Instruction{
                                operation: Instruction::op_jump,
                                parameter: position as isize - length
                            });
                            break;
                        }
                        (position, stop) = position.overflowing_sub(1);
                    }
                    if stop == true
                    {
                        panic!("(BF): Unmatched ] found in the program.");
                    }
                }
                '.' => {
                    bf_instruction!(Instruction::op_print, 0);
                }
                ',' => {
                    bf_instruction!(Instruction::op_read, 0);
                }
                // Ignore everything else.
                _ => ()
            }
        }

        // Sanity checks.
        for instruction in &self.instructions
        {
            if instruction.operation == Instruction::op_jump && instruction.parameter == 0
            {
                panic!("(BF): Unmatched [ found in the program.")
            }
        }
    }
}

impl std::fmt::Display for InterpreterCPU
{
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        writeln!(out, "Mode: Interpreter").ok();
        writeln!(out, "Instruction count: {}", self.instructions.len()).ok();
        writeln!(out, "Instructions:").ok();
        for instruction in &self.instructions
        {
            writeln!(out, "\t{}", instruction).ok();
        }
        return Ok(());
    }
}