pub mod cpu;
pub mod input;

use std::fs;

pub struct Emulator
{
    cpu: Box<dyn cpu::CPU>
}

impl Emulator
{
    pub fn run(&mut self)
    {
        while self.cpu.is_running()
        {
            self.cpu.step();
        }
    }

    pub fn new() -> Self
    {
        return Self{
            cpu: Box::new(cpu::InterpreterCPU::new())
        };
    }

    pub fn set_input(&mut self, new_input: Box<dyn input::Input>) -> &Self
    {
        self.cpu.get_state().input = new_input;
        return self;
    }

    pub fn reset(&mut self)
    {
        self.cpu.as_mut().get_state().reset();
    }

    pub fn get_output(&mut self) -> Vec<u8>
    {
        return self.cpu.get_state().output.clone();
    }

    pub fn get_output_as_string(&mut self) -> String
    {
        return String::from_utf8(self.cpu.get_state().output.clone()).expect("(BF): Output bytes are not valid UTF-8.");
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