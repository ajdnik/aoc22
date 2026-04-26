use anyhow::{Context, Result};
use std::str::FromStr;

pub fn to_matrix<N, I>(lines: I) -> Result<Vec<Vec<N>>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.chars()
                .map(|chr| {
                    let s = chr.to_string();
                    s.parse::<N>()
                        .with_context(|| format!("parsing digit {:?}", s))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect()
}
