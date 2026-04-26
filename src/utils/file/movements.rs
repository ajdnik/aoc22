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
