use super::position::Position;
use anyhow::{Context, Result};
use std::str::FromStr;

pub fn to_sensor_data<N, I>(lines: I) -> Result<Vec<(Position<N>, Position<N>)>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (left, right) = line
                .split_once(':')
                .with_context(|| format!("malformed sensor line {line:?}"))?;
            let sensor_loc = left
                .strip_prefix("Sensor at ")
                .with_context(|| format!("missing 'Sensor at' prefix {line:?}"))?;
            let beacon_loc = right
                .strip_prefix(" closest beacon is at ")
                .with_context(|| format!("missing 'closest beacon is at' {line:?}"))?;
            let (sx, sy) = sensor_loc
                .split_once(", ")
                .with_context(|| format!("malformed sensor coords {line:?}"))?;
            let (bx, by) = beacon_loc
                .split_once(", ")
                .with_context(|| format!("malformed beacon coords {line:?}"))?;
            let parse_coord = |s: &str, axis: &str| -> Result<N> {
                s.strip_prefix(axis)
                    .with_context(|| format!("missing '{axis}' prefix in {s:?}"))?
                    .parse()
                    .with_context(|| format!("parsing coord {s:?}"))
            };
            let sensor_x = parse_coord(sx, "x=")?;
            let sensor_y = parse_coord(sy, "y=")?;
            let beacon_x = parse_coord(bx, "x=")?;
            let beacon_y = parse_coord(by, "y=")?;
            Ok((
                Position {
                    x: sensor_x,
                    y: sensor_y,
                },
                Position {
                    x: beacon_x,
                    y: beacon_y,
                },
            ))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_sensor_beacon_pair() {
        let lines = ["Sensor at x=2, y=18: closest beacon is at x=-2, y=15"].map(String::from);
        let d = to_sensor_data::<i32, _>(lines).unwrap();
        assert_eq!(d.len(), 1);
        assert_eq!(d[0].0, Position { x: 2, y: 18 });
        assert_eq!(d[0].1, Position { x: -2, y: 15 });
    }

    #[test]
    fn missing_prefix_errors() {
        let lines = ["something at x=1, y=1: closest beacon is at x=2, y=2"].map(String::from);
        assert!(to_sensor_data::<i32, _>(lines).is_err());
    }
}
