use crate::utils::file;
use anyhow::Result;
use num::{abs, Signed};
use std::{
    cmp::{max, min},
    ops::Sub,
};

fn distance<N>(a: &file::Position<N>, b: &file::Position<N>) -> N
where
    N: Sub<Output = N> + Copy,
    <N as Sub>::Output: Signed,
{
    abs(a.x - b.x) + abs(a.y - b.y)
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
    data: &[(file::Position<i32>, file::Position<i32>)],
    y: i32,
    max_coord: Option<i32>,
) -> Vec<(i32, i32)> {
    let mut short_circuit = false;
    data.iter().fold(Vec::new(), |ranges, sensor_data| {
        if short_circuit {
            return ranges;
        }
        let (sensor, beacon) = sensor_data;
        let y_dist = abs(sensor.y - y);
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

fn find_frequency(
    data: &[(file::Position<i32>, file::Position<i32>)],
    max_coord: i32,
) -> Option<u64> {
    for y in 0..max_coord + 1 {
        let mut ranges = get_searched_ranges_for_y(data, y, Some(max_coord));
        if ranges.len() == 2 {
            ranges.sort_by_key(|a| a.0);
            return Some((ranges[0].1 as u64 + 1) * 4000000 + y as u64);
        }
    }
    None
}

pub fn part1(input: &str, row: i32) -> Result<String> {
    let sensor_data = file::to_sensor_data::<i32, _>(file::lines_of(input))?;
    let ranges = get_searched_ranges_for_y(&sensor_data, row, None);
    let searched: i32 = ranges
        .iter()
        .fold(0, |sum, range| sum + (range.1 - range.0));
    Ok(format!(
        "Row {} has {} positions that cannot contain a beacon",
        row, searched
    ))
}

pub fn part2(input: &str, max: i32) -> Result<String> {
    let sensor_data = file::to_sensor_data::<i32, _>(file::lines_of(input))?;
    let freq = find_frequency(&sensor_data, max);
    Ok(match freq {
        Some(val) => format!("Tuning frequency of the missing beacon is {}", val),
        None => String::from("Couldn't find the missing becaon"),
    })
}
