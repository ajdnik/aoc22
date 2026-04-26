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
                .with_context(|| format!("malformed sensor line {:?}", line))?;
            let sensor_loc = left
                .strip_prefix("Sensor at ")
                .with_context(|| format!("missing 'Sensor at' prefix {:?}", line))?;
            let beacon_loc = right
                .strip_prefix(" closest beacon is at ")
                .with_context(|| format!("missing 'closest beacon is at' {:?}", line))?;
            let (sx, sy) = sensor_loc
                .split_once(", ")
                .with_context(|| format!("malformed sensor coords {:?}", line))?;
            let (bx, by) = beacon_loc
                .split_once(", ")
                .with_context(|| format!("malformed beacon coords {:?}", line))?;
            let parse_coord = |s: &str, axis: &str| -> Result<N> {
                s.strip_prefix(axis)
                    .with_context(|| format!("missing '{}' prefix in {:?}", axis, s))?
                    .parse()
                    .with_context(|| format!("parsing coord {:?}", s))
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
