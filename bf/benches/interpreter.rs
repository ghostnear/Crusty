/*
 *  The following benchmarks are being run:
 *  https://esolangs.org/wiki/User:David.werecat/BFBench
 *  The ROM files are stored in the `roms/benchmarks/bf` directory.
 */

use std::fs;

use bf::{Emulator, input};

macro_rules! profile {
    ($($token:tt)+) => {
        {
            let _instant = std::time::Instant::now();
            let _result = {
                $($token)+
            };
            
            (_instant.elapsed(), _result)
        }
    }
}

pub fn run_test(input: &str, rom: &str)
{
    let mut emulator = Emulator::new();
    if input != ""
    {
        let input = fs::read_to_string(input).unwrap();
        emulator.set_input(Box::new(input::StringInput::new(&input)));

    }
    emulator.load_from_file(rom);
    emulator.run();
}

pub fn bench_interpreter()
{
    println!("Benching BF in Interpreter Mode:");
    let tests = [
        ("Mandlebrot", "", "../roms/benchmarks/bf/mandelbrot.b",  "../roms/benchmarks/bf/mandelbrot.out"),
        ("Factoring", "../roms/benchmarks/bf/factor.in", "../roms/benchmarks/bf/factor.b",  ""),
        ("Long Run", "", "../roms/benchmarks/bf/long.b",  "../roms/benchmarks/bf/long.out"),
        ("Self Interpret", "../roms/benchmarks/bf/Bootstrap.in", "../roms/benchmarks/bf/Bootstrap.b", ""),
        ("Golden Ratio", "", "../roms/benchmarks/bf/golden.b",  ""),
        ("Hanoi", "", "../roms/benchmarks/bf/hanoi.b",  "../roms/benchmarks/bf/hanoi.out"),
        ("99 Bottles of Beer", "", "../roms/benchmarks/bf/beer.b",  "../roms/benchmarks/bf/beer.out"),
        ("Simple Benchmark", "", "../roms/benchmarks/bf/bench.b",  ""),
    ];
    for test in tests
    {
        let (time, _) = profile!(run_test(test.1, test.2));
        if time.as_secs_f64() < 1.0
        {
            println!("\t{}: {}ms", test.0, time.as_millis())
        }
        else
        {
            println!("\t{}: {}s", test.0, time.as_secs_f64())
        }
    }
}