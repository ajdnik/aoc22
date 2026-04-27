use anyhow::{anyhow, bail, Context, Result};
use aoc22::days;
use clap::Parser;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Day number (1-25)
    day: u8,
    /// Part number (1 or 2; day 25 only has part 1)
    part: u8,
    /// Path to input file
    path: String,
    /// Optional extra argument(s) (day15: row/max, day16: minutes, day17: rock count)
    extra: Vec<String>,
    /// Print only the answer (no [INFO] prefix)
    #[arg(long)]
    raw: bool,
    /// Print elapsed time after the answer
    #[arg(long)]
    time: bool,
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

type Handler = fn(&str, &[String]) -> Result<String>;

const HANDLERS: [Handler; 50] = [
    |i, _| days::day1::part1(i),
    |i, _| days::day1::part2(i),
    |i, _| days::day2::part1(i),
    |i, _| days::day2::part2(i),
    |i, _| days::day3::part1(i),
    |i, _| days::day3::part2(i),
    |i, _| days::day4::part1(i),
    |i, _| days::day4::part2(i),
    |i, _| days::day5::part1(i),
    |i, _| days::day5::part2(i),
    |i, _| days::day6::part1(i),
    |i, _| days::day6::part2(i),
    |i, _| days::day7::part1(i),
    |i, _| days::day7::part2(i),
    |i, _| days::day8::part1(i),
    |i, _| days::day8::part2(i),
    |i, _| days::day9::part1(i),
    |i, _| days::day9::part2(i),
    |i, _| days::day10::part1(i),
    |i, _| days::day10::part2(i),
    |i, _| days::day11::part1(i),
    |i, _| days::day11::part2(i),
    |i, _| days::day12::part1(i),
    |i, _| days::day12::part2(i),
    |i, _| days::day13::part1(i),
    |i, _| days::day13::part2(i),
    |i, _| days::day14::part1(i),
    |i, _| days::day14::part2(i),
    |i, e| days::day15::part1(i, parse_extra::<i32>(e, 2_000_000)?),
    |i, e| days::day15::part2(i, parse_extra::<i32>(e, 4_000_000)?),
    |i, e| days::day16::part1(i, parse_extra::<u32>(e, 30)?),
    |i, e| days::day16::part2(i, parse_extra::<u32>(e, 26)?),
    |i, e| days::day17::part1(i, parse_extra::<u64>(e, 2022)?),
    |i, e| days::day17::part2(i, parse_extra::<u64>(e, 1_000_000_000_000)?),
    |i, _| days::day18::part1(i),
    |i, _| days::day18::part2(i),
    |i, _| days::day19::part1(i),
    |i, _| days::day19::part2(i),
    |i, _| days::day20::part1(i),
    |i, _| days::day20::part2(i),
    |i, _| days::day21::part1(i),
    |i, _| days::day21::part2(i),
    |i, _| days::day22::part1(i),
    |i, _| days::day22::part2(i),
    |i, _| days::day23::part1(i),
    |i, _| days::day23::part2(i),
    |i, _| days::day24::part1(i),
    |i, _| days::day24::part2(i),
    |i, _| days::day25::part1(i),
    |_, _| bail!("day 25 has no part 2 — it unlocks once you've collected all 49 stars"),
];

fn dispatch(day: u8, part: u8, input: &str, extra: &[String]) -> Result<String> {
    if !(1..=25).contains(&day) || !(1..=2).contains(&part) {
        bail!("unknown day/part: {day} {part}");
    }
    let idx = (day as usize - 1) * 2 + (part as usize - 1);
    HANDLERS[idx](input, extra)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let input = std::fs::read_to_string(&cli.path)
        .with_context(|| format!("failed to read input file {:?}", cli.path))?;
    let started = Instant::now();
    let result = dispatch(cli.day, cli.part, &input, &cli.extra)?;
    let elapsed = started.elapsed();
    for line in result.lines() {
        if cli.raw {
            println!("{line}");
        } else {
            println!("[INFO] {line}");
        }
    }
    if cli.time {
        let prefix = if cli.raw { "" } else { "[INFO] " };
        println!("{prefix}elapsed: {:.3?}", elapsed);
    }
    Ok(())
}
