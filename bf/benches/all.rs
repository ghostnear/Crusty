/*
 *  The following benchmarks are being run:
 *  https://esolangs.org/wiki/User:David.werecat/BFBench
 *  The ROM files are stored in the `roms/benchmarks/bf` directory.
 */
use std::sync::{Arc, Mutex};        // divan runs on a different thread so we have to protect the emulator with mutexes.
use std::{fs, hint::black_box};

use bf::{Emulator, Input};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

macro_rules! bench_simple {
    ($emulator: expr, $bencher: expr, $path: expr, $reset: expr) => {
        $emulator.lock().unwrap().load_from_file(black_box($path));
        
        $bencher
            .bench(|| {
                $emulator.lock().unwrap().run();
                if $reset {
                    $emulator.lock().unwrap().reset();
                }
            });
    };
}

macro_rules! bench_with_output {
    ($emulator: expr, $bencher: expr, $path: expr, $output_path: expr, $reset: expr) => {
        bench_simple!($emulator, $bencher, $path, $reset);
        let output = fs::read($output_path).expect("(Bench): Test output not found.");
        // NOTE: use this to overwrite if specific bytes are not identical, this happens mainly due to platform differences.
        // fs::write($output_path, $emulator.lock().unwrap().get_output()).ok();
        assert!($emulator.lock().unwrap().get_output() == output, "(Bench): Invalid output for this test.");
    };
}

#[divan::bench(
    max_time = 200,
    sample_size = 1,
    sample_count = 1
)]
fn bench_mandlebrot(bencher: divan::Bencher) {
    let emulator = Arc::new(Mutex::new(Emulator::new()));
    bench_with_output!(emulator, bencher, "../roms/benchmarks/bf/mandelbrot.b", "../roms/benchmarks/bf/mandelbrot.out", false);
}

/*
 *  TODO: this test doesn't even get close to 50s.
#[divan::bench(
    max_time = 50,
    sample_size = 1,
    sample_count = 1
)]
fn bench_factoring(bencher: divan::Bencher) {
    let mut emulator = Emulator::new();
    let input = fs::read_to_string("../roms/benchmarks/bf/factor.in").unwrap();
    emulator.set_input(Box::new(Input::StringInput::new(&input)));
    let emulator_safe = Arc::new(Mutex::new(emulator));
    bench_simple!(emulator_safe, bencher, "../roms/benchmarks/bf/factor.b");
}
*/

#[divan::bench(
    max_time = 50,
    sample_size = 1,
    sample_count = 1
)]
fn bench_long_run(bencher: divan::Bencher) {
    let emulator = Arc::new(Mutex::new(Emulator::new()));
    bench_with_output!(emulator, bencher, "../roms/benchmarks/bf/long.b", "../roms/benchmarks/bf/long.out", false);
}

#[divan::bench(
    max_time = 50,
    sample_size = 1,
    sample_count = 1
)]
fn bench_self_interpret(bencher: divan::Bencher) {
    let mut emulator = Emulator::new();
    let input = fs::read_to_string("../roms/benchmarks/bf/Bootstrap.in").unwrap();
    emulator.set_input(Box::new(Input::StringInput::new(&input)));
    let emulator_safe = Arc::new(Mutex::new(emulator));
    bench_simple!(emulator_safe, bencher, "../roms/benchmarks/bf/Bootstrap.b", false);
}

#[divan::bench(
    max_time = 5,
    sample_size = 5,
    sample_count = 5
)]
fn bench_golden(bencher: divan::Bencher) {
    let emulator = Arc::new(Mutex::new(bf::Emulator::new()));
    bench_simple!(emulator, bencher, "../roms/benchmarks/bf/golden.b", true);
}

#[divan::bench(
    max_time = 80,
    sample_size = 1,
    sample_count = 1
)]
fn bench_hanoi(bencher: divan::Bencher) {
    let emulator = Arc::new(Mutex::new(bf::Emulator::new()));
    bench_with_output!(emulator, bencher, "../roms/benchmarks/bf/hanoi.b", "../roms/benchmarks/bf/hanoi.out", false);
}

/*
 *  TODO: this test doesn't even get close to 5s.
#[divan::bench(
    max_time = 5,
    sample_size = 1,
    sample_count = 1
)]
fn bench_beer(bencher: divan::Bencher) {
    let emulator = Arc::new(Mutex::new(bf::Emulator::new()));
    bench_simple!(emulator, bencher, "../roms/benchmarks/bf/beer.b");
}
*/

/*
 *  TODO: this test doesn't even get close to 5s.
#[divan::bench(
    max_time = 5,
    sample_size = 1,
    sample_count = 1
)]
fn bench_simple_benchmark() {
    let mut emulator = bf::Emulator::new();
    emulator.load_from_file(black_box("../roms/benchmarks/bf/bench.b"));
    emulator.run()
}
*/