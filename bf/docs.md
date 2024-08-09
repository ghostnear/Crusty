# BF - Crusty module.

This folder contains the submodule for the esoteric programming language BrainFuck.

### Execution modes:

At the current moment ther are several ways to run this:
- in a partially optimizing interpreter mode with optimizations such as:
    - [x] Reducing multiple sequences of instructions into one. (i.e >>> gets translated to op_add_dp(3))
    - [x] Reducing conflicting sequences of instructions. (i.e >>><< gets translated to op_add_dp(1))
    - [ ] Replacing simple common known patterns such as [-] or [+] with equivalent sequences.
    - [ ] Replacing more common patterns such as cell copying with equivalent simpler sequences.

More info on optimization levels [here](https://code.google.com/archive/p/esotope-bfc/wikis/Comparison.wiki).

### Benchmarks:

As of the latest version, these results were achieved:
- On a laptop:
    - Name:
        - 
    - Components:
        - CPU: 12th Gen Intel i7-12700H (20) @ 2.688GHz
    - Tests:
        - bench_hanoi
            - 13.2s
        - bench_long_run
            - 16.58s
        - bench_mandlebrot
            - 11.07s

- On a phone:
    - Name:
        - Samsung Galaxy A70q
    - Components:
        - CPU: Qualcomm SM6150 (8) @ 1.708GHz
    - Tests:
        - bench_hanoi
            - 30.4s
        - bench_long_run
            - 39.54s
        - bench_mandlebrot
            - 25.91s