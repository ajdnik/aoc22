use crate::utils::{bfs, file, file::Position};
use anyhow::{Context, Result};

fn to_elevation_map<N, I>(input: I) -> (Vec<Vec<N>>, Position<usize>, Position<usize>)
where
    N: From<u8>,
    I: IntoIterator<Item = String>,
{
    let mut elevation: Vec<Vec<N>> = Vec::new();
    let mut start = Position { x: 0, y: 0 };
    let mut end = Position { x: 0, y: 0 };
    for (y, line) in input.into_iter().enumerate() {
        let mut row: Vec<N> = Vec::new();
        for (x, chr) in line.chars().enumerate() {
            let mut ascii = chr as u8;
            if chr == 'S' {
                start = Position { x, y };
                ascii = b'a';
            } else if chr == 'E' {
                end = Position { x, y };
                ascii = b'z';
            }
            row.push(ascii.into());
        }
        elevation.push(row);
    }
    (elevation, start, end)
}

fn neighbors(elevation: &[Vec<u32>], center: Position<usize>) -> Vec<Position<usize>> {
    let mut out = Vec::with_capacity(4);
    let here = elevation[center.y][center.x];
    let mut try_push = |x: usize, y: usize| {
        if elevation[y][x] + 1 >= here {
            out.push(Position { x, y });
        }
    };
    if center.y != 0 {
        try_push(center.x, center.y - 1);
    }
    if center.y != elevation.len() - 1 {
        try_push(center.x, center.y + 1);
    }
    if center.x != 0 {
        try_push(center.x - 1, center.y);
    }
    if center.x != elevation[center.y].len() - 1 {
        try_push(center.x + 1, center.y);
    }
    out
}

fn path_length(
    elevation: &[Vec<u32>],
    start: Position<usize>,
    end: Position<usize>,
) -> Option<usize> {
    bfs::shortest(start, |&p| neighbors(elevation, p), |&p| p == end)
}

fn find_beginnings(elevation: &[Vec<u32>], beginning: u32) -> Vec<Position<usize>> {
    let mut beginnings = Vec::new();
    for (y, row) in elevation.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val == beginning {
                beginnings.push(Position::<usize> { x, y });
            }
        }
    }
    beginnings
}

pub fn part1(input: &str) -> Result<String> {
    let (elevation, start, end) = to_elevation_map::<u32, _>(file::lines_of(input));
    let length = path_length(&elevation, end, start).context("no path from start to end")?;
    Ok(format!("Shortest path is {length}"))
}

pub fn part2(input: &str) -> Result<String> {
    let (elevation, _, end) = to_elevation_map::<u32, _>(file::lines_of(input));
    let beginnings = find_beginnings(&elevation, 97);
    let mut min_length = usize::MAX;
    for beginning in &beginnings {
        if let Some(length) = path_length(&elevation, end, *beginning) {
            if length < min_length {
                min_length = length;
            }
        }
    }
    Ok(format!("Shortest path is {min_length}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_start_end_to_a_z() {
        let lines = ["SbE"].map(String::from);
        let (m, s, e) = to_elevation_map::<u8, _>(lines);
        assert_eq!(s, Position { x: 0, y: 0 });
        assert_eq!(e, Position { x: 2, y: 0 });
        assert_eq!(m, vec![vec![b'a', b'b', b'z']]);
    }
}
