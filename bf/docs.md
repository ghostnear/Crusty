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

| Device                | CPU                                                       | Mandelbrot    | Factoring         | Long Run  | Self Interpret    | Golden Ratio      | Hanoi     | 99 Bottles of Beer    | Simple Benchmark  |
| :-------------------- | :-------------------------------------------------------: | :-----------: | :---------------: | :-------: | :---------------: | :---------------: | :-------: | :-------------------: | :---------------: |
| Windows 11 Laptop     | 13th Gen Intel(R) Core(TM) i7-13850HX (20) @ max 5.3GHz   | 10.00s        | 2.73s             | 6.44s     | 32.54s            | 174ms             | 803ms     | 1ms                   | 8ms               |
| Windows 11 Laptop     | 12th Gen Intel(R) Core(TM) i7-12700H (20) @ max 4.7GHz    | 16.65s        | 4.69s             | 11.09s    | 55.07s            | 333ms             | 1.37s     | 2ms                   | 13ms              |
| Motorola Edge 30      | Qualcomm Snapdragon 778G (SM7325-AE) (8) @ max 2.5GHz     | -             | -                 | -         | -                 | -                 | -         | -                     | -                 |
| Samsung Galaxy A70q   | Qualcomm Snapdragon 675 (SM6150) (8) @ max 2GHz           | -             | -                 | -         | -                 | -                 | -         | -                     | -                 |

Benchmarks have been done in the same style as the ones done [here](https://esolangs.org/wiki/User:David.werecat/BFBench).