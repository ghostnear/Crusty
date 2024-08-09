use std::io::Write;
use std::{borrow::BorrowMut, fs};

struct Instruction
{
    operation: fn(&mut State, isize),
    parameter: isize
}

impl std::fmt::Display for Instruction
{
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        if self.operation == State::op_add_to_dp
        {
            return write!(out, "ADD dp, {};", self.parameter);
        }
        if self.operation == State::op_add_to_data
        {
            return write!(out, "ADD [dp], {};", self.parameter);
        }
        if self.operation == State::op_jump
        {
            return write!(out, "JMP {};", self.parameter);
        }
        panic!("(BF): Unknown instruction in format.")
    }
}

struct State
{
    data: Vec<usize>,
    instructions: Vec<Instruction>,
    output: Vec<u8>,
    pc: usize,
    dp: usize
}

impl State
{
    fn op_add_to_dp(&mut self, parameter: isize)
    {
        self.dp = self.dp.wrapping_add_signed(parameter);
        if self.dp >= self.data.len()
        {
            self.data.resize((self.dp + 1) as usize, 0);
        }
        self.pc += 1;
    }

    fn op_add_to_data(&mut self, parameter: isize)
    {
        self.data[self.dp as usize] = self.data[self.dp as usize].wrapping_add_signed(parameter);
        self.pc += 1;
    }

    fn op_print(&mut self, _parameter: isize)
    {
        write!(self.output, "{}", (self.data[self.dp as usize] as u8) as char).ok();
        self.pc += 1;
    }

    fn op_jump(&mut self, parameter: isize)
    {
        if (parameter < 0 && self.data[self.dp as usize] == 0) || (parameter > 0 && self.data[self.dp as usize] != 0)
        {
            self.pc += 1;
            return;
        }

        self.pc = self.pc.wrapping_add_signed(parameter);
    }

    pub fn new() -> State
    {
        let mut result = State{
            data: Vec::new(),
            output: Vec::new(),
            instructions: Vec::new(),
            pc: 0,
            dp: 0,
        };
        result.data.resize(1, 0);
        return result;
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
                    bf_symbol!(State::op_add_to_dp, 1);
                }
                '<' => {
                    bf_symbol!(State::op_add_to_dp, -1);
                },
                '+' => {
                    bf_symbol!(State::op_add_to_data, 1);
                }
                '-' => {
                    bf_symbol!(State::op_add_to_data, -1);
                }
                '[' => {
                    // First parantheses pushed to the 'stack'.
                    bf_instruction!(State::op_jump, 0);
                }
                ']' => {
                    // Find the previous one and update jumps if found.
                    let length = self.instructions.len() as isize;
                    let mut position = self.instructions.len() - 1;
                    let mut stop = false;
                    while !stop
                    {
                        let value = self.instructions[position].borrow_mut();
                        if value.operation == State::op_jump && value.parameter == 0
                        {
                            // It's the last one, optimize it out.
                            if length - position as isize == 1 {
                                self.instructions.pop();
                                break;
                            }

                            value.parameter = length - position as isize;
                            self.instructions.push(Instruction{
                                operation: State::op_jump,
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
                    bf_instruction!(State::op_print, 0);
                }
                // Ignore everything else.
                _ => ()
            }
        }

        // Sanity checks.
        for instruction in self.instructions.iter()
        {
            if instruction.operation == State::op_jump && instruction.parameter == 0
            {
                panic!("(BF): Unmatched [ found in the program.")
            }
        }
    }
}

impl std::fmt::Display for State
{
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        writeln!(out, "Instructions:").ok();
        for instruction in self.instructions.iter().skip(self.pc as usize) {
            writeln!(out, "\t{}", instruction).ok();
        }
        writeln!(out, "PC: {}\nDP: {}", self.pc, self.dp).ok();
        for value in self.data.iter() {
            write!(out, "{} ", value).ok();
        }
        return Ok(());
    }
}

pub struct Emulator
{
    state: State
}

impl Emulator
{
    fn step(&mut self)
    {
        let state = &mut self.state;
        let instruction = &state.instructions[state.pc as usize];
        (instruction.operation)(state, instruction.parameter);
    }

    pub fn run(&mut self)
    {
        while self.running()
        {
            self.step();
        }
    }

    pub fn new() -> Emulator
    {
        let result = Emulator{
            state: State::new()
        };
        return result;
    }

    pub fn running(&self) -> bool
    {
        return self.state.pc as usize != self.state.instructions.len();
    }

    pub fn get_output(&self) -> String
    {
        return String::from_utf8(self.state.output.clone()).unwrap();
    }

    pub fn load_from_string(&mut self, program: &str) -> &Self
    {
        self.state.parse(program);
        return self;
    }

    pub fn load_from_file(&mut self, path: &str) -> &Self
    {
        let input = match fs::read_to_string(path) {
            Ok(data) => data,
            Err(error) => panic!("(BF): Could not read input from file at path '{path}'. ({error})")
        };
        return self.load_from_string(&input);
    }
}

impl std::fmt::Display for Emulator
{
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        writeln!(out, "{}", self.state)
    }
}