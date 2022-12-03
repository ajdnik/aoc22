# aoc22
Advent of Code 2022 🎄

This repository contains [Advent of Code](https://adventofcode.com) task solutions for year 2022. All of the tasks are exposed via a singular command line tool.

### Running

In order to run the solution you need to have [Rust](https://www.rust-lang.org) installed. Then you will need to compile the code and run it as shown:

```
$ cargo build
   Compiling autocfg v1.1.0
   
   ...

   Finished dev [unoptimized + debuginfo] target(s) in 7.73s

$ ./target/debug/aoc22 --help
Advent Of Code 2022 CLI

Usage: aoc22 [OPTIONS] <COMMAND>

Commands:
  day1
  day2
  day3
  help  Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose...  Print verbose output
  -h, --help        Print help information
  -V, --version     Print version information

$ ./target/debug/aoc22 day1 task1 ./input/day01/test.txt
[INFO] The maximum calorie count is 24000
```
