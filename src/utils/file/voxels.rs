use anyhow::{bail, Context, Result};
use std::str::FromStr;

pub fn to_voxels<N, I>(lines: I) -> Result<Vec<(N, N, N)>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() != 3 {
                bail!("expected 3 comma-separated values, got {line:?}");
            }
            let x: N = parts[0]
                .parse()
                .with_context(|| format!("parsing x in {line:?}"))?;
            let y: N = parts[1]
                .parse()
                .with_context(|| format!("parsing y in {line:?}"))?;
            let z: N = parts[2]
                .parse()
                .with_context(|| format!("parsing z in {line:?}"))?;
            Ok((x, y, z))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_three_ints() {
        let lines = ["1,2,3", "4,5,6"].map(String::from);
        let v = to_voxels::<i32, _>(lines).unwrap();
        assert_eq!(v, vec![(1, 2, 3), (4, 5, 6)]);
    }

    #[test]
    fn skips_empty_lines() {
        let lines = ["1,2,3", "", "4,5,6"].map(String::from);
        let v = to_voxels::<i32, _>(lines).unwrap();
        assert_eq!(v, vec![(1, 2, 3), (4, 5, 6)]);
    }

    #[test]
    fn errors_on_wrong_arity() {
        let lines = ["1,2".to_string()];
        assert!(to_voxels::<i32, _>(lines).is_err());
    }

    #[test]
    fn errors_on_unparsable() {
        let lines = ["1,abc,3".to_string()];
        assert!(to_voxels::<i32, _>(lines).is_err());
    }
}
