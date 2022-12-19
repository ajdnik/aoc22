use std::{cmp::{max, min}, ops::Sub};
use crate::utils::file;
use log::{debug, info};
use num::{abs, Signed};

fn distance<N>(a: &file::Position<N>, b: &file::Position<N>) -> N
where N: Sub<Output=N> + Copy, <N as Sub>::Output: Signed {
    abs(a.x - b.x) + abs(a.y - b.y)
}

fn is_range_overlap<N>(a: &(N, N), b: &(N, N)) -> bool
where N: PartialOrd {
    let (begin_a, end_a) = a;
    let (begin_b, end_b) = b;
    if begin_a <= begin_b && end_a >= begin_b {
        return true;
    }
    if begin_b <= begin_a && end_b >= begin_a {
        return true;
    }
    return false;
}

fn stitch_ranges<N>(ranges: &Vec<(N, N)>, add: &(N, N)) -> Vec<(N, N)>
where N: Ord + Copy {
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

fn get_searched_ranges_for_y(data: &Vec<(file::Position<i32>, file::Position<i32>)>, y: i32, max_coord: Option<i32>) -> Vec<(i32, i32)> {
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

fn find_frequency(data: &Vec<(file::Position<i32>, file::Position<i32>)>, max_coord: i32) -> Option<u64> {
    for y in 0..max_coord+1 {
        let mut ranges = get_searched_ranges_for_y(data, y, Some(max_coord));
        if ranges.len() == 2 {
            ranges.sort_by(|a, b| a.0.cmp(&b.0));
            return Some((ranges[0].1 as u64 + 1) * 4000000 + y as u64);
        }
    }
    return None;
}

pub fn task1(path: &str, row: i32) {
    if let Ok(lines) = file::read_lines(path) {
        let sensor_data = file::to_sensor_data::<i32>(lines);
        debug!("Found {} sensors", sensor_data.len());
        let ranges = get_searched_ranges_for_y(&sensor_data, row, None);
        debug!("Row {} has been searched in the following ranges: {:?}", row, ranges);
        let searched_positions = ranges.iter().fold(0, |sum, range| {
            sum + (range.1 - range.0)
        });
        info!("Row {} has {} positions that cannot contain a beacon", row, searched_positions);
    }
}

pub fn task2(path: &str, max: i32) {
    if let Ok(lines) = file::read_lines(path) {
        let sensor_data = file::to_sensor_data::<i32>(lines);
        debug!("Found {} sensors", sensor_data.len());
        debug!("Searching using x=0..{} and y=0..{} space", max, max);
        let freq = find_frequency(&sensor_data, max);
        if let Some(val) = freq {
            info!("Tuning frequency of the missing beacon is {}", val);
        } else {
            info!("Couldn't find the missing becaon");
        }
    }
}
