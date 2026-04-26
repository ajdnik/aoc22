use crate::utils::{file, vec};
use anyhow::{Context, Result};

pub fn part1(input: &str) -> Result<String> {
    let numbers: Vec<Option<i32>> = file::lines_to_numbers(file::lines_of(input));
    let counts = vec::sum_continuous_numbers(numbers);
    let max = counts.iter().max().context("no calorie counts found")?;
    Ok(format!("The maximum calorie count is {}", max))
}

pub fn part2(input: &str) -> Result<String> {
    let numbers: Vec<Option<i32>> = file::lines_to_numbers(file::lines_of(input));
    let mut counts = vec::sum_continuous_numbers(numbers);
    counts.sort_by(|a, b| b.cmp(a));
    let top3 = counts[0] + counts[1] + counts[2];
    Ok(format!("The top 3 calorie counts sum up to {}", top3))
}
