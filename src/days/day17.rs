use anyhow::{bail, Result};
use rustc_hash::FxHashMap;

const WIDTH: i64 = 7;
const SKYLINE_DEPTH: usize = 32;

type Rock = &'static [(i64, i64)];

// Rock shapes (bottom-left origin). Cells listed as (x, y) offsets.
const ROCKS: [Rock; 5] = [
    // ####  (horizontal bar)
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    // .#.   (plus)
    // ###
    // .#.
    &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
    // ..#   (J / reverse-L, base on bottom)
    // ..#
    // ###
    &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    // #     (vertical bar)
    // #
    // #
    // #
    &[(0, 0), (0, 1), (0, 2), (0, 3)],
    // ##    (square)
    // ##
    &[(0, 0), (1, 0), (0, 1), (1, 1)],
];

fn collides(grid: &[u8], cells: Rock, cx: i64, cy: i64) -> bool {
    cells.iter().any(|&(dx, dy)| {
        let x = cx + dx;
        let y = cy + dy;
        if !(0..WIDTH).contains(&x) || y < 0 {
            return true;
        }
        let yu = y as usize;
        yu < grid.len() && grid[yu] & (1 << x) != 0
    })
}

fn place(grid: &mut Vec<u8>, cells: Rock, cx: i64, cy: i64) {
    for &(dx, dy) in cells {
        let x = (cx + dx) as usize;
        let y = (cy + dy) as usize;
        if y >= grid.len() {
            grid.resize(y + 1, 0);
        }
        grid[y] |= 1 << x;
    }
}

fn simulate(jets: &[u8], count: u64) -> u64 {
    let mut grid: Vec<u8> = Vec::new();
    let mut jet_idx = 0usize;
    let mut seen: FxHashMap<(usize, usize, [u8; SKYLINE_DEPTH]), (u64, u64)> =
        FxHashMap::default();
    let mut bonus = 0u64;
    let mut i = 0u64;
    while i < count {
        let r = (i % 5) as usize;
        let cells = ROCKS[r];
        let mut cx: i64 = 2;
        let mut cy: i64 = grid.len() as i64 + 3;
        loop {
            let dir = jets[jet_idx % jets.len()];
            jet_idx += 1;
            let nx = cx + if dir == b'<' { -1 } else { 1 };
            if !collides(&grid, cells, nx, cy) {
                cx = nx;
            }
            if collides(&grid, cells, cx, cy - 1) {
                place(&mut grid, cells, cx, cy);
                break;
            }
            cy -= 1;
        }
        i += 1;
        if bonus == 0 && grid.len() >= SKYLINE_DEPTH {
            let mut top = [0u8; SKYLINE_DEPTH];
            let n = grid.len();
            for (k, slot) in top.iter_mut().enumerate() {
                *slot = grid[n - 1 - k];
            }
            let key = ((i % 5) as usize, jet_idx % jets.len(), top);
            if let Some(&(prev_i, prev_h)) = seen.get(&key) {
                let cycle_len = i - prev_i;
                let cycle_h = grid.len() as u64 - prev_h;
                let skips = (count - i) / cycle_len;
                bonus = skips * cycle_h;
                i += skips * cycle_len;
            } else {
                seen.insert(key, (i, grid.len() as u64));
            }
        }
    }
    grid.len() as u64 + bonus
}

fn parse_jets(input: &str) -> Result<&[u8]> {
    let bytes = input.trim().as_bytes();
    if bytes.is_empty() {
        bail!("empty jet pattern");
    }
    if let Some(&c) = bytes.iter().find(|&&c| c != b'<' && c != b'>') {
        bail!("invalid jet character {:?}", c as char);
    }
    Ok(bytes)
}

pub fn part1(input: &str, count: u64) -> Result<String> {
    let jets = parse_jets(input)?;
    let height = simulate(jets, count);
    Ok(format!("Tower height after {count} rocks is {height}"))
}

pub fn part2(input: &str, count: u64) -> Result<String> {
    let jets = parse_jets(input)?;
    let height = simulate(jets, count);
    Ok(format!("Tower height after {count} rocks is {height}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_sample_2022() {
        assert!(part1(SAMPLE, 2022).unwrap().contains("3068"));
    }

    #[test]
    fn part2_sample_trillion() {
        assert!(part2(SAMPLE, 1_000_000_000_000)
            .unwrap()
            .contains("1514285714288"));
    }

    #[test]
    fn empty_input_errors() {
        assert!(part1("", 10).is_err());
    }

    #[test]
    fn bad_char_errors() {
        assert!(part1(">>x<<", 10).is_err());
    }
}
