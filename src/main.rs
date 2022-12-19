use clap::{Args, Parser, Subcommand};
use log::LevelFilter;
use fern::{InitError, Dispatch};
mod days;
mod utils;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Print verbose output
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[command(subcommand)]
    day: Days,
}

#[derive(Subcommand)]
enum Days {
    #[command(subcommand)]
    Day1(TwoTasks),
    #[command(subcommand)]
    Day2(TwoTasks),
    #[command(subcommand)]
    Day3(TwoTasks),
    #[command(subcommand)]
    Day4(TwoTasks),
    #[command(subcommand)]
    Day5(TwoTasks),
    #[command(subcommand)]
    Day6(TwoTasks),
    #[command(subcommand)]
    Day7(TwoTasks),
    #[command(subcommand)]
    Day8(TwoTasks),
    #[command(subcommand)]
    Day9(TwoTasks),
    #[command(subcommand)]
    Day10(TwoTasks),
    #[command(subcommand)]
    Day11(TwoTasks),
    #[command(subcommand)]
    Day12(TwoTasks),
    #[command(subcommand)]
    Day13(TwoTasks),
    #[command(subcommand)]
    Day14(TwoTasks),
    #[command(subcommand)]
    Day15(Day15Tasks),
    #[command(subcommand)]
    Day16(Day16Tasks),
}

#[derive(Subcommand)]
enum TwoTasks {
    Task1(TaskWithPath),
    Task2(TaskWithPath),
}

#[derive(Subcommand)]
enum Day15Tasks {
    Task1(TaskWithPathAndRow),
    Task2(TaskWithPathAndMax),
}

#[derive(Subcommand)]
enum Day16Tasks {
    Task1(TaskWithPathAndMinutes30),
    Task2(TaskWithPathAndMinutes26),
}

#[derive(Args)]
struct TaskWithPath {
    path: String,
}

#[derive(Args)]
struct TaskWithPathAndRow {
    path: String,
    #[arg(default_value_t = 2000000)]
    row: i32,
}

#[derive(Args)]
struct TaskWithPathAndMax {
    path: String,
    #[arg(default_value_t = 4000000)]
    max: i32,
}

#[derive(Args)]
struct TaskWithPathAndMinutes30 {
    path: String,
    #[arg(default_value_t = 30)]
    minutes: u32,
}

#[derive(Args)]
struct TaskWithPathAndMinutes26 {
    path: String,
    #[arg(default_value_t = 26)]
    minutes: u32,
}

fn setup_logger(level: LevelFilter) -> Result<(), InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let mut level = LevelFilter::Info;
    if cli.verbose > 0 {
        level = LevelFilter::Debug;
    }
    if let Err(err) = setup_logger(level) {
        panic!("Problem setting up logger: {}", err)
    }

    match &cli.day {
        Days::Day1(task) => 
            match task {
                TwoTasks::Task1(args) => days::day1::task1(&args.path),
                TwoTasks::Task2(args) => days::day1::task2(&args.path),
            },
        Days::Day2(task) =>
            match task {
                TwoTasks::Task1(args) => days::day2::task1(&args.path),
                TwoTasks::Task2(args) => days::day2::task2(&args.path),
            },
        Days::Day3(task) =>
            match task {
                TwoTasks::Task1(args) => days::day3::task1(&args.path),
                TwoTasks::Task2(args) => days::day3::task2(&args.path),
            },
        Days::Day4(task) =>
            match task {
                TwoTasks::Task1(args) => days::day4::task1(&args.path),
                TwoTasks::Task2(args) => days::day4::task2(&args.path),
            },
        Days::Day5(task) =>
            match task {
                TwoTasks::Task1(args) => days::day5::task1(&args.path),
                TwoTasks::Task2(args) => days::day5::task2(&args.path),
            },
        Days::Day6(task) =>
            match task {
                TwoTasks::Task1(args) => days::day6::task1(&args.path),
                TwoTasks::Task2(args) => days::day6::task2(&args.path),
            },
        Days::Day7(task) =>
            match task {
                TwoTasks::Task1(args) => days::day7::task1(&args.path),
                TwoTasks::Task2(args) => days::day7::task2(&args.path),
            },
        Days::Day8(task) =>
            match task {
                TwoTasks::Task1(args) => days::day8::task1(&args.path),
                TwoTasks::Task2(args) => days::day8::task2(&args.path),
            },
        Days::Day9(task) =>
            match task {
                TwoTasks::Task1(args) => days::day9::task1(&args.path),
                TwoTasks::Task2(args) => days::day9::task2(&args.path),
            },
        Days::Day10(task) =>
            match task {
                TwoTasks::Task1(args) => days::day10::task1(&args.path),
                TwoTasks::Task2(args) => days::day10::task2(&args.path),
            },
        Days::Day11(task) =>
            match task {
                TwoTasks::Task1(args) => days::day11::task1(&args.path),
                TwoTasks::Task2(args) => days::day11::task2(&args.path),
            },
        Days::Day12(task) =>
            match task {
                TwoTasks::Task1(args) => days::day12::task1(&args.path),
                TwoTasks::Task2(args) => days::day12::task2(&args.path),
            },
        Days::Day13(task) =>
            match task {
                TwoTasks::Task1(args) => days::day13::task1(&args.path),
                TwoTasks::Task2(args) => days::day13::task2(&args.path),
            },
        Days::Day14(task) =>
            match task {
                TwoTasks::Task1(args) => days::day14::task1(&args.path),
                TwoTasks::Task2(args) => days::day14::task2(&args.path),
            },
        Days::Day15(task) =>
            match task {
                Day15Tasks::Task1(args) => days::day15::task1(&args.path, args.row),
                Day15Tasks::Task2(args) => days::day15::task2(&args.path, args.max),
            },
        Days::Day16(task) =>
            match task {
                Day16Tasks::Task1(args) => days::day16::task1(&args.path, args.minutes),
                Day16Tasks::Task2(args) => days::day16::task2(&args.path, args.minutes),
            },
    }
}
