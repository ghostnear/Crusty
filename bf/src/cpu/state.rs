use crate::input;

pub struct State
{
    pub data: Vec<u8>,
    pub output: Vec<u8>,
    pub pc: usize,
    pub dp: usize,
    pub input: Box<dyn input::Input>
}

impl State
{
    pub fn new(new_input: Box<dyn input::Input>) -> Self
    {
        let mut result = Self{
            data: Vec::new(),
            output: Vec::new(),
            input: new_input,
            pc: 0,
            dp: 0
        };
        result.data.resize(64, 0);
        return result;
    }

    pub fn reset(&mut self)
    {
        self.data = Vec::new();
        self.data.resize(1, 0);
        self.output = Vec::new();
        // NOTE: do not reset input here as it is given from outside.
        self.pc = 0;
        self.dp = 0;
    }
}