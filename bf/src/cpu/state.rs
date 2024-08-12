use crate::Input;

pub struct State
{
    pub data: Vec<usize>,
    pub output: Vec<u8>,
    pub pc: usize,
    pub dp: usize,
    pub input: Box<dyn Input::Input + Send>
}

impl State
{
    pub fn new(new_input: Box<dyn Input::Input + Send>) -> Self
    {
        let mut result = Self{
            data: Vec::new(),
            output: Vec::new(),
            input: new_input,
            pc: 0,
            dp: 0
        };
        result.data.resize(1, 0);
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

impl std::fmt::Display for State
{
    fn fmt(&self, out: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(out, "State:\n\tPC: {}\n\tDP: {}\n\tMemory size: {}\n\tMemory: ", self.pc, self.dp, self.data.len()).ok();
        for value in self.data.iter() {
            write!(out, "{} ", value).ok();
        }
        return Ok(());
    }
}