use crate::cpu::State;
use crate::input;

use super::CPU;

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
            let mut length = state.data.len();
            while state.dp >= length
            {
                length *= 2;
            }
            state.data.resize(length, 0);
        }
        state.pc += 1;
    }

    fn op_clear_cell(state: &mut State, _parameter: isize)
    {
        state.data[state.dp as usize] = 0;
        state.pc += 1;
    }

    fn op_add_to_data(state: &mut State, parameter: isize)
    {
        state.data[state.dp as usize] = state.data[state.dp as usize].wrapping_add_signed(parameter as i8);
        state.pc += 1;
    }

    fn op_print(state: &mut State, _parameter: isize)
    {
        write!(state.output, "{}", (state.data[state.dp as usize] as u8) as char).ok();
        state.pc += 1;
    }

    fn op_read(state: &mut State, _parameter: isize)
    {
        state.data[state.dp as usize] = state.input.read() as u8;
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

impl PartialEq for Instruction
{
    fn eq(&self, other: &Instruction) -> bool
    {
        return self.operation == other.operation && self.parameter == other.parameter;
    }
}

pub struct InterpreterCPU
{
    pub instructions: Vec<Instruction>,
    state: State
}

impl InterpreterCPU
{
    pub fn new() -> Self
    {
        return Self{
            state: State::new(
                Box::new(input::StringInput::new(""))
            ),
            instructions: Vec::new()
        };
    }
}

impl CPU for InterpreterCPU
{
    fn get_state(&mut self) -> &mut State {
        return &mut self.state
    }

    fn set_state(&mut self, state: State) {
        self.state = state;
    }

    fn step(&mut self)
    {
        let instruction = &self.instructions[self.state.pc as usize];
        (instruction.operation)(&mut self.state, instruction.parameter);
    }

    fn is_running(&self) -> bool
    {
        return self.state.pc as usize != self.instructions.len()
    }

    fn parse(&mut self, input: &str)
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
                            
                            // Optimization: [-] is basically data[dp] = 0;
                            if self.instructions.ends_with(&[
                                Instruction{operation: Instruction::op_jump, parameter: 2},
                                Instruction{operation: Instruction::op_add_to_data, parameter: -1},
                                Instruction{operation: Instruction::op_jump, parameter: -2},
                            ]) || self.instructions.ends_with(&[
                                Instruction{operation: Instruction::op_jump, parameter: 2},
                                Instruction{operation: Instruction::op_add_to_data, parameter: 1},
                                Instruction{operation: Instruction::op_jump, parameter: -2},
                            ])
                            {
                                for _ in 0..3
                                {
                                    self.instructions.pop();
                                }
                                self.instructions.push(Instruction{operation: Instruction::op_clear_cell, parameter: 0});
                            }

                            // Optimization: [</>x] is basically 
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