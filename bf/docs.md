# BrainFuck - Crusty module.

This folder contains the submodule for running code written in the esoteric programming language BrainFuck.

## The how.

### Execution modes:

At the current moment there are several ways to run this:
- in a partially optimizing interpreter mode with optimizations such as:
    - [x] Reducing multiple sequences of instructions into one. (i.e >>> gets translated to op_add_dp(3))
    - [x] Reducing conflicting sequences of instructions. (i.e >>><< gets translated to op_add_dp(1))
    - [ ] Replacing simple common known patterns such as [-] or [+] with equivalent sequences.
    - [ ] Replacing more common patterns such as cell copying with equivalent simpler sequences.

More info on optimization levels [here](https://code.google.com/archive/p/esotope-bfc/wikis/Comparison.wiki).

## The how fast.

### Benchmarks:

#### In Interpreter Mode:

| Device                | CPU                                                   | Mandelbrot    | Factoring         | Long Run  | Self Interpret    | Golden Ratio      | Hanoi     | 99 Bottles of Beer    | Simple Benchmark  |
| :-------------------- | :---------------------------------------------------- | :-----------: | :---------------: | :-------: | :---------------: | :---------------: | :-------: | :-------------------: | :---------------: |
| Windows 11 Laptop     | 12th Gen Intel(R) Core(TM) i7-12700H (20) @ 2.688GHz  | 9.93s         | More than 50s.    | 15.68s    | 36.39s            | 0.18s             | 13.77s    | More than 5s.         | More than 5s.     |
| Motorola Edge 30      | Qualcomm SM7325-AE (8) @ 1.804GHz                     | 14.27s        | -                 | 25.09s    | 43.41s            | 0.28s             | 19.68s    | -                     | -                 |
| Samsung Galaxy A70q   | Qualcomm SM6150 (8) @ 1.708GHz                        | 25.30s        | -                 | 42.72s    | 80.22s            | 0.44s             | 37.08s    | -                     | -                 |

Median times have been used for tests that are run multiple times because they are fast.

Benchmarks have been done in the same style as the ones done [here](https://esolangs.org/wiki/User:David.werecat/BFBench).