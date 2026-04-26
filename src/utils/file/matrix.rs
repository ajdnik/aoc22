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
                        .with_context(|| format!("parsing digit {s:?}"))
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_2d_grid() {
        let lines = ["123", "456"].map(String::from);
        let m = to_matrix::<u8, _>(lines).unwrap();
        assert_eq!(m, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn errors_on_non_digit() {
        let lines = ["12x"].map(String::from);
        assert!(to_matrix::<u8, _>(lines).is_err());
    }
}
