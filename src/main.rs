use std::io;
use std::env::args;

fn main()
{
    let mut emulator = bf::Emulator::new();

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
    
    print!("{}", emulator.get_output());
}
