use super::position::Position;
use anyhow::{bail, Context, Result};
use std::str::FromStr;

pub fn to_walls<N, I>(lines: I) -> Result<Vec<Vec<Position<N>>>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let dim: Vec<&str> = point.split(',').collect();
                    if dim.len() != 2 {
                        bail!("malformed point {:?}", point);
                    }
                    let x: N = dim[0]
                        .parse()
                        .with_context(|| format!("parsing x {:?}", dim[0]))?;
                    let y: N = dim[1]
                        .parse()
                        .with_context(|| format!("parsing y {:?}", dim[1]))?;
                    Ok(Position { x, y })
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect()
}
