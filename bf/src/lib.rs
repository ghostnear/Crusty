pub mod cpu;
pub use cpu as CPU;
pub mod input;
pub use input as Input;

use std::fs;

pub struct Emulator
{
    state: cpu::State,
    cpu: cpu::InterpreterCPU
}

impl Emulator
{
    pub fn run(&mut self)
    {
        while self.is_running()
        {
            self.cpu.step(&mut self.state);
        }
    }

    pub fn new() -> Self
    {
        return Self{
            state: cpu::State::new(Box::new(input::StringInput::new(""))),
            cpu: cpu::InterpreterCPU::new()
        };
    }

    pub fn set_input(&mut self, new_input: Box<dyn input::Input + Send>) -> &Self
    {
        self.state.input = new_input;
        return self;
    }

    pub fn reset(&mut self)
    {
        self.state.reset();
    }

    pub fn is_running(&self) -> bool
    {
        return self.state.pc as usize != self.cpu.instructions.len();
    }

    pub fn get_output(&self) -> Vec<u8>
    {
        return self.state.output.clone();
    }

    pub fn get_output_as_string(&self) -> String
    {
        return String::from_utf8(self.state.output.clone()).expect("(BF): Output bytes are not valid UTF-8.");
    }

    pub fn load_from_string(&mut self, program: &str) -> &Self
    {
        self.cpu.parse(program);
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
        write!(out, "{}", self.cpu).ok();
        write!(out, "{}", self.state)
    }
}