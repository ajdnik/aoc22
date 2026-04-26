use crate::utils::{file, vec};
use anyhow::{Context, Result};
use log::{debug, info};

pub fn task1(path: &str) -> Result<()> {
    let calories = file::read_lines(path)?;
    let numbers: Vec<Option<i32>> = file::lines_to_numbers(calories);
    debug!("Found {} calorie counts in the list", numbers.len());
    let counts = vec::sum_continuous_numbers(numbers);
    debug!("The calorie counts belong to {} elves", counts.len());
    let max = counts.iter().max().context("no calorie counts found")?;
    info!("The maximum calorie count is {}", max);
    Ok(())
}

pub fn task2(path: &str) -> Result<()> {
    let calories = file::read_lines(path)?;
    let numbers: Vec<Option<i32>> = file::lines_to_numbers(calories);
    debug!("Found {} calorie counts in the list", numbers.len());
    let mut counts = vec::sum_continuous_numbers(numbers);
    debug!("The calorie counts belong to {} elves", counts.len());

    counts.sort_by(|a, b| b.cmp(a));
    let top3 = counts[0] + counts[1] + counts[2];
    info!("The top 3 calorie counts sum up to {}", top3);
    Ok(())
}
