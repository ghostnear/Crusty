use std::io;
use std::env::args;

use bf::{Emulator, input};

fn main()
{
    let mut emulator = Box::new(Emulator::new());
    emulator.set_input(Box::new(input::StdinInput::new()));

    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        // Read ROM from input.
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok();
        emulator.load_from_string(&input);
    }
    else
    {
        // Read ROM from first argument.
        emulator.load_from_file(&args[1]);
    }

    emulator.run();

    print!("{}", emulator.get_output_as_string());
}
