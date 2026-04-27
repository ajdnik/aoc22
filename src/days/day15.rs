use crate::utils::file::{self, Position};
use anyhow::{Context, Result};
use std::cmp::{max, min};
use std::str::FromStr;

fn to_sensor_data<N, I>(lines: I) -> Result<Vec<(Position<N>, Position<N>)>>
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

fn distance(a: &Position<i32>, b: &Position<i32>) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn is_range_overlap<N>(a: &(N, N), b: &(N, N)) -> bool
where
    N: PartialOrd,
{
    let (begin_a, end_a) = a;
    let (begin_b, end_b) = b;
    if begin_a <= begin_b && end_a >= begin_b {
        return true;
    }
    if begin_b <= begin_a && end_b >= begin_a {
        return true;
    }
    false
}

fn stitch_ranges<N>(ranges: &[(N, N)], add: &(N, N)) -> Vec<(N, N)>
where
    N: Ord + Copy,
{
    let mut new_range = (add.0, add.1);
    let mut new = ranges.iter().fold(Vec::new(), |mut new_ranges, range| {
        if !is_range_overlap(range, &new_range) {
            new_ranges.push(*range);
        } else {
            new_range.0 = min(range.0, new_range.0);
            new_range.1 = max(range.1, new_range.1);
        }
        new_ranges
    });
    new.push(new_range);
    new
}

fn get_searched_ranges_for_y(
    data: &[(Position<i32>, Position<i32>)],
    y: i32,
    max_coord: Option<i32>,
) -> Vec<(i32, i32)> {
    let mut short_circuit = false;
    data.iter().fold(Vec::new(), |ranges, sensor_data| {
        if short_circuit {
            return ranges;
        }
        let (sensor, beacon) = sensor_data;
        let y_dist = (sensor.y - y).abs();
        let max_dist = distance(sensor, beacon) - y_dist;
        if max_dist < 0 {
            return ranges;
        }
        let mut begin = sensor.x - max_dist;
        let mut end = sensor.x + max_dist;
        if let Some(val) = max_coord {
            begin = max(0, begin);
            end = min(val, end);
            if end <= begin {
                return ranges;
            }
        }
        let new_ranges = stitch_ranges(&ranges, &(begin, end));
        if let Some(val) = max_coord {
            if new_ranges.len() == 1 && new_ranges[0].0 == 0 && new_ranges[0].1 == val {
                short_circuit = true;
            }
        }
        new_ranges
    })
}

fn find_frequency(data: &[(Position<i32>, Position<i32>)], max_coord: i32) -> Option<u64> {
    for y in 0..max_coord + 1 {
        let mut ranges = get_searched_ranges_for_y(data, y, Some(max_coord));
        if ranges.len() == 2 {
            ranges.sort_by_key(|a| a.0);
            return Some((ranges[0].1 as u64 + 1) * 4_000_000 + y as u64);
        }
    }
    None
}

pub fn part1(input: &str, row: i32) -> Result<String> {
    let sensor_data = to_sensor_data::<i32, _>(file::lines_of(input))?;
    let ranges = get_searched_ranges_for_y(&sensor_data, row, None);
    let searched: i32 = ranges
        .iter()
        .fold(0, |sum, range| sum + (range.1 - range.0));
    Ok(format!(
        "Row {row} has {searched} positions that cannot contain a beacon"
    ))
}

pub fn part2(input: &str, max: i32) -> Result<String> {
    let sensor_data = to_sensor_data::<i32, _>(file::lines_of(input))?;
    let freq = find_frequency(&sensor_data, max);
    Ok(match freq {
        Some(val) => format!("Tuning frequency of the missing beacon is {val}"),
        None => String::from("Couldn't find the missing becaon"),
    })
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
