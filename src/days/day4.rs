use crate::utils::file;
use anyhow::Result;

pub fn part1(input: &str) -> Result<String> {
    let pairs = file::to_range_tuple::<i32, _>(file::lines_of(input))?;
    let mut count = 0;
    for (first, second) in pairs.iter() {
        if (first.start <= second.start && first.end >= second.end)
            || (second.start <= first.start && second.end >= first.end)
        {
            count += 1;
        }
    }
    Ok(format!("Found {} matching pairs", count))
}

pub fn part2(input: &str) -> Result<String> {
    let pairs = file::to_range_tuple::<i32, _>(file::lines_of(input))?;
    let mut count = 0;
    for (first, second) in pairs.iter() {
        if first.start <= second.end && second.start <= first.end {
            count += 1;
        }
    }
    Ok(format!("Found {} overlapping pairs", count))
}
