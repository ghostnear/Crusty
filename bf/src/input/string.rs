use std::collections::VecDeque;

use super::Input;

pub struct StringInput
{
    data: VecDeque<u8>
}

impl StringInput
{
    pub fn new(data: &str) -> Self
    {
        let mut result = Self {
            data: VecDeque::new()
        };
        for character in data.as_bytes()
        {
            result.data.push_back(character.clone());
        }
        return result;
    }
}

impl Input for StringInput
{
    fn read(&mut self) -> u8
    {
        if self.data.len() == 0
        {
            return 0x0A;    // '\n'.
        }

        return self.data.pop_front().unwrap();
    }
}