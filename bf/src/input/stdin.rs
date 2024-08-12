use std::{collections::VecDeque, io::{self, BufRead}};

use super::Input;

pub struct StdinInput
{
    data: VecDeque<u8>
}

impl StdinInput
{
    pub fn new() -> Self
    {
        return Self {
            data: VecDeque::new()
        }
    }
}

impl Input for StdinInput
{
    fn read(&mut self) -> u8
    {
        if self.data.len() == 0
        {
            let mut input = String::new();
            io::stdin().lock().read_line(&mut input).ok();
            for character in input.chars() {
                self.data.push_back(character as u8);
            }
        }

        return self.data.pop_front().unwrap();
    }
}