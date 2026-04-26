use crate::utils::file;
use anyhow::Result;
use log::{debug, info};

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

fn simulate(movements: &[file::Direction], rope_len: usize) -> usize {
    let mut rope = vec![Position { x: 0, y: 0 }; rope_len];
    let mut visited = vec![rope[rope_len - 1]];
    for movement in movements {
        match movement {
            file::Direction::Up => rope[0].y += 1,
            file::Direction::Down => rope[0].y -= 1,
            file::Direction::Left => rope[0].x -= 1,
            file::Direction::Right => rope[0].x += 1,
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

pub fn task1(path: &str) -> Result<()> {
    let lines = file::read_lines(path)?;
    let movements = file::to_movements(lines);
    debug!("Found {} rope movements", movements.len());
    let count = simulate(&movements, 2);
    info!("The tail visited {} places", count);
    Ok(())
}

pub fn task2(path: &str) -> Result<()> {
    let lines = file::read_lines(path)?;
    let movements = file::to_movements(lines);
    debug!("Found {} rope movements", movements.len());
    let count = simulate(&movements, 10);
    info!("The tail visited {} places", count);
    Ok(())
}
