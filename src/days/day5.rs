use crate::utils::file;
use anyhow::{bail, Context, Result};
use std::str::FromStr;

#[allow(clippy::type_complexity)]
fn parse_crates<N, I>(lines: I) -> Result<(Vec<String>, Vec<(N, N, N)>)>
where
    N: FromStr + Copy,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    let mut stacks: Vec<String> = Vec::new();
    let mut operations = Vec::new();
    for line in lines {
        if line.trim().starts_with('[') {
            let columns = line.split(' ').collect::<Vec<&str>>();
            let mut space_count = 0;
            let mut column_idx = 0;
            for column in columns {
                if column.starts_with('[') {
                    if stacks.len() <= column_idx {
                        for _ in stacks.len()..column_idx + 1 {
                            stacks.push(String::new());
                        }
                    }
                    stacks[column_idx].push(
                        column
                            .chars()
                            .nth(1)
                            .with_context(|| format!("malformed crate cell {:?}", column))?,
                    );
                    column_idx += 1;
                    space_count = 0;
                } else {
                    space_count += 1;
                    if space_count == 4 {
                        column_idx += 1;
                        space_count = 0;
                    }
                }
            }
        } else if line.starts_with("move") {
            let parts = line.split(' ').collect::<Vec<&str>>();
            if parts.len() < 6 {
                bail!("malformed move line {:?}", line);
            }
            let a: N = parts[1]
                .parse()
                .with_context(|| format!("parsing move count {:?}", parts[1]))?;
            let b: N = parts[3]
                .parse()
                .with_context(|| format!("parsing move from {:?}", parts[3]))?;
            let c: N = parts[5]
                .parse()
                .with_context(|| format!("parsing move to {:?}", parts[5]))?;
            operations.push((a, b, c));
        }
    }
    Ok((stacks, operations))
}

fn move_crates(
    mut stacks: Vec<String>,
    operations: Vec<(usize, usize, usize)>,
    all_at_once: bool,
) -> Vec<String> {
    for (size, from, to) in operations.iter() {
        let (first, rest) = stacks[from - 1].split_at(*size);
        let mut new_column: String = first.to_string();
        if !all_at_once {
            new_column = new_column.chars().rev().collect();
        }
        new_column.push_str(stacks[to - 1].as_str());
        stacks[from - 1] = rest.to_string();
        stacks[to - 1] = new_column;
    }
    stacks
}

fn get_top_crates(stacks: &[String]) -> String {
    stacks.iter().fold(String::new(), |mut result, stack| {
        result.push(stack.chars().next().unwrap());
        result
    })
}

pub fn part1(input: &str) -> Result<String> {
    let (stacks, operations) = parse_crates(file::lines_of(input))?;
    let reordered = move_crates(stacks, operations, false);
    let top = get_top_crates(&reordered);
    Ok(format!("After reordering the top crates are {}", top))
}

pub fn part2(input: &str) -> Result<String> {
    let (stacks, operations) = parse_crates(file::lines_of(input))?;
    let reordered = move_crates(stacks, operations, true);
    let top = get_top_crates(&reordered);
    Ok(format!("After reordering the top crates are {}", top))
}
