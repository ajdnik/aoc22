use crate::utils::file;
use anyhow::{Context, Result};
use std::cmp::Ordering;

fn divider(n: u32) -> file::Signal<u32> {
    file::Signal::List(vec![file::Signal::List(vec![file::Signal::Number(n)])])
}

pub fn part1(input: &str) -> Result<String> {
    let signals = file::to_signals::<u32, _>(file::lines_of(input))?;
    let sum: usize = signals
        .chunks(2)
        .enumerate()
        .filter_map(|(idx, pair)| match pair {
            [left, right] if left.cmp(right) != Ordering::Greater => Some(idx + 1),
            _ => None,
        })
        .sum();
    Ok(format!("Sum of indices of ordered signals is {}", sum))
}

pub fn part2(input: &str) -> Result<String> {
    let mut signals = file::to_signals::<u32, _>(file::lines_of(input))?;
    let div2 = divider(2);
    let div6 = divider(6);
    signals.push(div2.clone());
    signals.push(div6.clone());
    signals.sort();
    let pos2 = signals
        .iter()
        .position(|s| s == &div2)
        .context("divider [[2]] not found")?;
    let pos6 = signals
        .iter()
        .position(|s| s == &div6)
        .context("divider [[6]] not found")?;
    Ok(format!("Decode key is {}", (pos2 + 1) * (pos6 + 1)))
}
