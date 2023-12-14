# Advent of Code

This repo contains my solutions to [Advent of Code](https://adventofcode.com/) problems in [Rust](https://www.rust-lang.org/). It also includes a helper library that I've developed over time, consisting of data structures and functions that have proven useful in solving past problems.

## How to run

Currently the file structure assumes that all input files are contained in the respective binary folder for each day. Once that is setup, we need to run the command

```
cargo run --release --bin <binary_name>
```

For example for binary `2022_01` we have an `input.txt` for the actual input and a `sample_input.txt` for a sample input. Then we run 

```
cargo run --release --bin 2022_01
```

which gives the output for both parts

```
Sample Input
Part 1 Answer : 24000
Part 2 Answer : 41000
Input
Part 1 Answer : 69626
Part 2 Answer : 206780
```



