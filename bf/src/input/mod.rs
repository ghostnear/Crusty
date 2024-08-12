mod string;
mod stdin;

pub trait Input
{
    fn read(&mut self) -> u8;
}

pub use string::StringInput as StringInput;
pub use stdin::StdinInput as StdinInput;
