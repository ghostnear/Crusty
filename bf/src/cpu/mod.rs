mod interpreter;
mod state;

pub trait CPU
{
    fn parse(&mut self, input: &str);

    fn is_running(&self) -> bool;
    fn step(&mut self);

    fn set_state(&mut self, state: State);
    fn get_state(&mut self) -> &mut State;
}

pub use state::State as State;
pub use interpreter::InterpreterCPU as InterpreterCPU;
