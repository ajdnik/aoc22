use std::collections::{VecDeque, HashMap};
use crate::utils::file;
use log::{debug, info};

fn neighbors(elevation: &Vec<Vec<u32>>, center: &file::Position<usize>) -> Vec<file::Position<usize>> {
    let mut neighbors = Vec::new();
    if center.y != 0 && elevation[center.y - 1][center.x] + 1 >= elevation[center.y][center.x] {
        neighbors.push(file::Position{x: center.x, y: center.y - 1});
    }
    if center.y != elevation.len() - 1 && elevation[center.y + 1][center.x] + 1 >= elevation[center.y][center.x] {
        neighbors.push(file::Position{x: center.x, y: center.y + 1});
    }
    if center.x != 0 && elevation[center.y][center.x - 1] + 1 >= elevation[center.y][center.x] {
        neighbors.push(file::Position{x: center.x - 1, y: center.y});
    }
    if center.x != elevation[center.y].len() - 1 && elevation[center.y][center.x + 1] + 1 >= elevation[center.y][center.x] {
        neighbors.push(file::Position{x: center.x + 1, y: center.y});
    }
    neighbors
}

fn path_length(elevation: &Vec<Vec<u32>>, start: &file::Position<usize>, end: &file::Position<usize>) -> Option<usize> {
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
    if let Some(result) = distances.get(end) {
        return Some(*result);
    }
    None
}

fn find_beginnings(elevation: &Vec<Vec<u32>>, beginning: u32) -> Vec<file::Position<usize>> {
    let mut y = 0;
    let mut x = 0;
    elevation.iter().fold(Vec::new(), |mut beginnings, row| {
        x = 0;
        for val in row.iter() {
            if *val == beginning {
                beginnings.push(file::Position::<usize>{x, y});
            }
            x += 1;
        }
        y += 1;
        beginnings
    })
}

pub fn task1(path: &str) {
    if let Ok(lines) = file::read_lines(path) {
        let (elevation, start, end) = file::to_elevation_map::<u32>(lines);
        debug!("Loaded a {} x {} map", elevation.len(), elevation[0].len());
        debug!("Starting point is ({}, {})", start.x, start.y);
        debug!("Ending point is ({}, {})", end.x, end.y);
        let path_length = path_length(&elevation, &end, &start);
        info!("Shortest path is {}", path_length.unwrap());
    }
}

pub fn task2(path: &str) {
    if let Ok(lines) = file::read_lines(path) {
        let (elevation, _, end) = file::to_elevation_map::<u32>(lines);
        debug!("Loaded a {} x {} map", elevation.len(), elevation[0].len());
        debug!("Ending point is ({}, {})", end.x, end.y);
        let beginnings = find_beginnings(&elevation, 97);
        debug!("Found {} starting points", beginnings.len());
        let mut min_length = usize::MAX;
        for beginning in beginnings.iter() {
            let path_length = path_length(&elevation, &end, beginning);
            if let Some(length) = path_length {
                if length < min_length {
                    min_length = length;
                }
            }
        }
        info!("Shortest path is {}", min_length);
    } 
}
