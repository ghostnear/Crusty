/*
 *  The following benchmarks are being run:
 *  https://esolangs.org/wiki/User:David.werecat/BFBench
 *  The ROM files are stored in the `roms/benchmarks/bf` directory.
 */
use std::hint::black_box;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(
    max_time = 200,
    sample_size = 1,
    sample_count = 1
)]
fn bench_mandlebrot() {
    let mut emulator = bf::Emulator::new();
    emulator.load_from_file(black_box("../roms/benchmarks/bf/mandelbrot.b"));
    emulator.run()
}

#[divan::bench(
    max_time = 50,
    sample_size = 1,
    sample_count = 1
)]
fn bench_long_run() {
    let mut emulator = bf::Emulator::new();
    emulator.load_from_file(black_box("../roms/benchmarks/bf/long.b"));
    emulator.run()
}

#[divan::bench(
    max_time = 80,
    sample_size = 1,
    sample_count = 1
)]
fn bench_hanoi() {
    let mut emulator = bf::Emulator::new();
    emulator.load_from_file(black_box("../roms/benchmarks/bf/hanoi.b"));
    emulator.run()
}

/*
 *  TODO: this test doesn't even get close to 5s.
#[divan::bench(
    max_time = 5,
    sample_size = 1,
    sample_count = 1
)]
fn bench_beer() {
    let mut emulator = bf::Emulator::new();
    emulator.load_from_file(black_box("../roms/benchmarks/bf/beer.b"));
    emulator.run()
}
*/

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
