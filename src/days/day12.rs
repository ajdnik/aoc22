use crate::utils::file;
use anyhow::{Context, Result};
use std::collections::{HashMap, VecDeque};

fn neighbors(elevation: &[Vec<u32>], center: &file::Position<usize>) -> Vec<file::Position<usize>> {
    let mut neighbors = Vec::new();
    if center.y != 0 && elevation[center.y - 1][center.x] + 1 >= elevation[center.y][center.x] {
        neighbors.push(file::Position {
            x: center.x,
            y: center.y - 1,
        });
    }
    if center.y != elevation.len() - 1
        && elevation[center.y + 1][center.x] + 1 >= elevation[center.y][center.x]
    {
        neighbors.push(file::Position {
            x: center.x,
            y: center.y + 1,
        });
    }
    if center.x != 0 && elevation[center.y][center.x - 1] + 1 >= elevation[center.y][center.x] {
        neighbors.push(file::Position {
            x: center.x - 1,
            y: center.y,
        });
    }
    if center.x != elevation[center.y].len() - 1
        && elevation[center.y][center.x + 1] + 1 >= elevation[center.y][center.x]
    {
        neighbors.push(file::Position {
            x: center.x + 1,
            y: center.y,
        });
    }
    neighbors
}

fn path_length(
    elevation: &[Vec<u32>],
    start: &file::Position<usize>,
    end: &file::Position<usize>,
) -> Option<usize> {
    let mut must_visit = VecDeque::new();
    must_visit.push_back(*start);
    let mut distances: HashMap<file::Position<usize>, usize> = HashMap::new();
    distances.insert(*start, 0);
    while let Some(current_position) = must_visit.pop_front() {
        let neighbor_distance = distances.get(&current_position).unwrap_or(&usize::MAX) + 1;
        let neighbors = neighbors(elevation, &current_position);
        for neighbor in neighbors.iter() {
            if let Some(existing_distance) = distances.get(neighbor) {
                if neighbor_distance >= *existing_distance {
                    continue;
                }
            }
            distances.insert(*neighbor, neighbor_distance);
            if *neighbor != *end {
                must_visit.push_back(*neighbor);
            }
        }
    }
    distances.get(end).copied()
}

fn find_beginnings(elevation: &[Vec<u32>], beginning: u32) -> Vec<file::Position<usize>> {
    let mut beginnings = Vec::new();
    for (y, row) in elevation.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val == beginning {
                beginnings.push(file::Position::<usize> { x, y });
            }
        }
    }
    beginnings
}

pub fn part1(input: &str) -> Result<String> {
    let (elevation, start, end) = file::to_elevation_map::<u32, _>(file::lines_of(input));
    let length = path_length(&elevation, &end, &start).context("no path from start to end")?;
    Ok(format!("Shortest path is {}", length))
}

pub fn part2(input: &str) -> Result<String> {
    let (elevation, _, end) = file::to_elevation_map::<u32, _>(file::lines_of(input));
    let beginnings = find_beginnings(&elevation, 97);
    let mut min_length = usize::MAX;
    for beginning in beginnings.iter() {
        if let Some(length) = path_length(&elevation, &end, beginning) {
            if length < min_length {
                min_length = length;
            }
        }
    }
    Ok(format!("Shortest path is {}", min_length))
}
