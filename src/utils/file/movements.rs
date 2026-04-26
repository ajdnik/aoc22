use anyhow::{bail, Context, Result};

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn to_movements<I>(lines: I) -> Result<Vec<Direction>>
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
            .with_context(|| format!("malformed movement line {:?}", line))?;
        let val: usize = length
            .parse()
            .with_context(|| format!("parsing movement length {:?}", length))?;
        let dir = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => bail!("unknown direction {:?}", direction),
        };
        for _ in 0..val {
            movements.push(dir);
        }
    }
    Ok(movements)
}
