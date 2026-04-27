use crate::utils::file;
use anyhow::{bail, Context, Result};

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn to_movements<I>(lines: I) -> Result<Vec<Direction>>
where
    I: IntoIterator<Item = String>,
{
    let mut movements = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (direction, length) = line
            .split_once(' ')
            .with_context(|| format!("malformed movement line {line:?}"))?;
        let val: usize = length
            .parse()
            .with_context(|| format!("parsing movement length {length:?}"))?;
        let dir = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => bail!("unknown direction {direction:?}"),
        };
        for _ in 0..val {
            movements.push(dir);
        }
    }
    Ok(movements)
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
}

fn follow(head: Position, tail: Position) -> Position {
    let dx = head.x - tail.x;
    let dy = head.y - tail.y;
    if dx.abs() <= 1 && dy.abs() <= 1 {
        return tail;
    }
    Position {
        x: tail.x + dx.signum(),
        y: tail.y + dy.signum(),
    }
}

fn simulate(movements: &[Direction], rope_len: usize) -> usize {
    let mut rope = vec![Position { x: 0, y: 0 }; rope_len];
    let mut visited = vec![rope[rope_len - 1]];
    for movement in movements {
        match movement {
            Direction::Up => rope[0].y += 1,
            Direction::Down => rope[0].y -= 1,
            Direction::Left => rope[0].x -= 1,
            Direction::Right => rope[0].x += 1,
        }
        for i in 1..rope_len {
            rope[i] = follow(rope[i - 1], rope[i]);
        }
        visited.push(rope[rope_len - 1]);
    }
    visited.sort();
    visited.dedup();
    visited.len()
}

pub fn part1(input: &str) -> Result<String> {
    let movements = to_movements(file::lines_of(input))?;
    let count = simulate(&movements, 2);
    Ok(format!("The tail visited {count} places"))
}

pub fn part2(input: &str) -> Result<String> {
    let movements = to_movements(file::lines_of(input))?;
    let count = simulate(&movements, 10);
    Ok(format!("The tail visited {count} places"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dirs(input: &[&str]) -> Vec<Direction> {
        let lines = input.iter().map(|s| String::from(*s)).collect::<Vec<_>>();
        to_movements(lines).unwrap()
    }

    #[test]
    fn expands_count() {
        let m = dirs(&["R 3"]);
        assert_eq!(m.len(), 3);
        assert!(matches!(m[0], Direction::Right));
    }

    #[test]
    fn handles_all_dirs() {
        let m = dirs(&["U 1", "D 1", "L 1", "R 1"]);
        assert!(matches!(
            m.as_slice(),
            [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right
            ]
        ));
    }

    #[test]
    fn unknown_direction_errors() {
        let lines = ["X 1"].map(String::from);
        assert!(to_movements(lines).is_err());
    }
}
