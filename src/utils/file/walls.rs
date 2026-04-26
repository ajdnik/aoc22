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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_polyline() {
        let lines = ["1,2 -> 3,4 -> 5,6"].map(String::from);
        let w = to_walls::<u32, _>(lines).unwrap();
        assert_eq!(w.len(), 1);
        assert_eq!(w[0].len(), 3);
        assert_eq!(w[0][0], Position { x: 1, y: 2 });
    }

    #[test]
    fn malformed_point_errors() {
        let lines = ["1,2 -> 3"].map(String::from);
        assert!(to_walls::<u32, _>(lines).is_err());
    }
}
