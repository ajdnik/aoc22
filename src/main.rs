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
}

#[derive(Subcommand)]
enum TwoTasks {
    Task1(TaskWithPath),
    Task2(TaskWithPath),
}

#[derive(Args)]
struct TaskWithPath {
    path: String,
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
    }
}
