use anyhow::{Context, Result};
use std::str::FromStr;

pub fn lines_of(input: &str) -> impl Iterator<Item = String> + '_ {
    input.lines().map(String::from)
}

/// Splits input on blank lines, parsing each non-blank line as `N`.
pub fn to_number_groups<N, I>(lines: I) -> Result<Vec<Vec<N>>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    let mut groups: Vec<Vec<N>> = vec![Vec::new()];
    for line in lines {
        if line.is_empty() {
            groups.push(Vec::new());
            continue;
        }
        let val: N = line
            .parse()
            .with_context(|| format!("parsing number {:?}", line))?;
        groups.last_mut().unwrap().push(val);
    }
    Ok(groups)
}

pub fn to_groups<I>(lines: I, size: usize) -> Vec<Vec<String>>
where
    I: IntoIterator<Item = String>,
{
    let mut groups: Vec<Vec<String>> = vec![Vec::new()];
    for line in lines {
        if groups.last().map(|g| g.len()).unwrap_or(0) >= size {
            groups.push(Vec::new());
        }
        groups.last_mut().unwrap().push(line);
    }
    groups
}
