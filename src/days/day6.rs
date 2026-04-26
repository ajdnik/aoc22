use crate::utils::vec;
use anyhow::Result;
use std::fmt::Write;

fn run(input: &str, size: usize) -> String {
    let mut out = String::new();
    for (idx, buffer) in input.lines().enumerate() {
        let start = vec::find_first_distinct_substring(buffer.to_string(), size);
        writeln!(
            out,
            "The first distinct {} character substring for buffer at position {} starts at {}",
            size, idx, start
        )
        .unwrap();
    }
    out.trim_end().to_string()
}

pub fn part1(input: &str) -> Result<String> {
    Ok(run(input, 4))
}

pub fn part2(input: &str) -> Result<String> {
    Ok(run(input, 14))
}
