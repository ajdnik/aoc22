use crate::utils::file;
use anyhow::{Context, Result};

fn group_sums(input: &str) -> Result<Vec<i32>> {
    let groups = file::to_number_groups::<i32, _>(file::lines_of(input))?;
    Ok(groups.iter().map(|g| g.iter().sum()).collect())
}

pub fn part1(input: &str) -> Result<String> {
    let sums = group_sums(input)?;
    let max = sums.iter().max().context("no calorie counts found")?;
    Ok(format!("The maximum calorie count is {}", max))
}

pub fn part2(input: &str) -> Result<String> {
    let mut sums = group_sums(input)?;
    sums.sort_by(|a, b| b.cmp(a));
    if sums.len() < 3 {
        anyhow::bail!("expected at least 3 calorie groups, got {}", sums.len());
    }
    let top3 = sums[0] + sums[1] + sums[2];
    Ok(format!("The top 3 calorie counts sum up to {}", top3))
}
