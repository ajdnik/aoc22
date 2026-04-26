use anyhow::{bail, Context, Result};
use std::{ops::Range, str::FromStr};

pub fn to_range_tuple<N, I>(lines: I) -> Result<Vec<(Range<N>, Range<N>)>>
where
    N: FromStr + Copy,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let ranges: Vec<&str> = line.split(',').collect();
            if ranges.len() != 2 {
                bail!("expected 2 ranges in line {:?}", line);
            }
            let first: Vec<&str> = ranges[0].split('-').collect();
            let second: Vec<&str> = ranges[1].split('-').collect();
            if first.len() != 2 || second.len() != 2 {
                bail!("malformed range in line {:?}", line);
            }
            let a: N = first[0]
                .parse()
                .with_context(|| format!("parsing {:?}", first[0]))?;
            let b: N = first[1]
                .parse()
                .with_context(|| format!("parsing {:?}", first[1]))?;
            let c: N = second[0]
                .parse()
                .with_context(|| format!("parsing {:?}", second[0]))?;
            let d: N = second[1]
                .parse()
                .with_context(|| format!("parsing {:?}", second[1]))?;
            Ok((a..b, c..d))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_pair_of_ranges() {
        let lines = ["2-4,6-8"].map(String::from);
        let r = to_range_tuple::<i32, _>(lines).unwrap();
        assert_eq!(r, vec![(2..4, 6..8)]);
    }

    #[test]
    fn skips_blank_lines() {
        let lines = ["", "1-2,3-4"].map(String::from);
        assert_eq!(to_range_tuple::<i32, _>(lines).unwrap().len(), 1);
    }

    #[test]
    fn errors_on_malformed() {
        let lines = ["2-4"].map(String::from);
        assert!(to_range_tuple::<i32, _>(lines).is_err());
    }
}
