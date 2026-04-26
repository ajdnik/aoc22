use anyhow::{anyhow, Context, Result};
use aoc22::days;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Day number (1-21)
    day: u8,
    /// Part number (1 or 2)
    part: u8,
    /// Path to input file
    path: String,
    /// Optional extra argument(s) (day15: row/max, day16: minutes, day17: rock count)
    extra: Vec<String>,
}

fn parse_extra<T: std::str::FromStr>(extra: &[String], default: T) -> Result<T>
where
    T::Err: std::fmt::Display,
{
    match extra.first() {
        Some(s) => s
            .parse::<T>()
            .map_err(|e| anyhow!("invalid extra arg {s:?}: {e}")),
        None => Ok(default),
    }
}

fn dispatch(day: u8, part: u8, input: &str, extra: &[String]) -> Result<String> {
    match (day, part) {
        (1, 1) => days::day1::part1(input),
        (1, 2) => days::day1::part2(input),
        (2, 1) => days::day2::part1(input),
        (2, 2) => days::day2::part2(input),
        (3, 1) => days::day3::part1(input),
        (3, 2) => days::day3::part2(input),
        (4, 1) => days::day4::part1(input),
        (4, 2) => days::day4::part2(input),
        (5, 1) => days::day5::part1(input),
        (5, 2) => days::day5::part2(input),
        (6, 1) => days::day6::part1(input),
        (6, 2) => days::day6::part2(input),
        (7, 1) => days::day7::part1(input),
        (7, 2) => days::day7::part2(input),
        (8, 1) => days::day8::part1(input),
        (8, 2) => days::day8::part2(input),
        (9, 1) => days::day9::part1(input),
        (9, 2) => days::day9::part2(input),
        (10, 1) => days::day10::part1(input),
        (10, 2) => days::day10::part2(input),
        (11, 1) => days::day11::part1(input),
        (11, 2) => days::day11::part2(input),
        (12, 1) => days::day12::part1(input),
        (12, 2) => days::day12::part2(input),
        (13, 1) => days::day13::part1(input),
        (13, 2) => days::day13::part2(input),
        (14, 1) => days::day14::part1(input),
        (14, 2) => days::day14::part2(input),
        (15, 1) => days::day15::part1(input, parse_extra::<i32>(extra, 2_000_000)?),
        (15, 2) => days::day15::part2(input, parse_extra::<i32>(extra, 4_000_000)?),
        (16, 1) => days::day16::part1(input, parse_extra::<u32>(extra, 30)?),
        (16, 2) => days::day16::part2(input, parse_extra::<u32>(extra, 26)?),
        (17, 1) => days::day17::part1(input, parse_extra::<u64>(extra, 2022)?),
        (17, 2) => days::day17::part2(input, parse_extra::<u64>(extra, 1_000_000_000_000)?),
        (18, 1) => days::day18::part1(input),
        (18, 2) => days::day18::part2(input),
        (19, 1) => days::day19::part1(input),
        (19, 2) => days::day19::part2(input),
        (20, 1) => days::day20::part1(input),
        (20, 2) => days::day20::part2(input),
        (21, 1) => days::day21::part1(input),
        (21, 2) => days::day21::part2(input),
        _ => Err(anyhow!("unknown day/part: {day} {part}")),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let input = std::fs::read_to_string(&cli.path)
        .with_context(|| format!("failed to read input file {:?}", cli.path))?;
    let result = dispatch(cli.day, cli.part, &input, &cli.extra)?;
    for line in result.lines() {
        println!("[INFO] {line}");
    }
    Ok(())
}
