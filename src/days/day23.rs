use anyhow::Result;
use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);

const DIR_CHECKS: [[(i32, i32); 3]; 4] = [
    [(-1, -1), (-1, 0), (-1, 1)],
    [(1, -1), (1, 0), (1, 1)],
    [(-1, -1), (0, -1), (1, -1)],
    [(-1, 1), (0, 1), (1, 1)],
];

const DIR_MOVES: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

const NEIGHBORS_8: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn parse(input: &str) -> HashSet<Pos> {
    let mut elves = HashSet::new();
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                elves.insert((r as i32, c as i32));
            }
        }
    }
    elves
}

fn round(elves: &HashSet<Pos>, dir_offset: usize) -> (HashSet<Pos>, bool) {
    let mut proposals: HashMap<Pos, Pos> = HashMap::new();
    let mut counts: HashMap<Pos, u32> = HashMap::new();
    for &(r, c) in elves {
        let alone = !NEIGHBORS_8
            .iter()
            .any(|(dr, dc)| elves.contains(&(r + dr, c + dc)));
        if alone {
            continue;
        }
        for i in 0..4 {
            let dir = (dir_offset + i) % 4;
            let clear = DIR_CHECKS[dir]
                .iter()
                .all(|(dr, dc)| !elves.contains(&(r + dr, c + dc)));
            if clear {
                let (mr, mc) = DIR_MOVES[dir];
                let target = (r + mr, c + mc);
                proposals.insert((r, c), target);
                *counts.entry(target).or_insert(0) += 1;
                break;
            }
        }
    }
    let mut new_elves = HashSet::with_capacity(elves.len());
    let mut moved = false;
    for &p in elves {
        if let Some(&target) = proposals.get(&p) {
            if counts[&target] == 1 {
                new_elves.insert(target);
                moved = true;
                continue;
            }
        }
        new_elves.insert(p);
    }
    (new_elves, moved)
}

fn bounding_empty(elves: &HashSet<Pos>) -> u32 {
    let (min_r, max_r, min_c, max_c) = elves.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |(mn_r, mx_r, mn_c, mx_c), &(r, c)| (mn_r.min(r), mx_r.max(r), mn_c.min(c), mx_c.max(c)),
    );
    let w = (max_c - min_c + 1) as u32;
    let h = (max_r - min_r + 1) as u32;
    w * h - elves.len() as u32
}

pub fn part1(input: &str) -> Result<String> {
    let mut elves = parse(input);
    for i in 0..10 {
        let (next, _) = round(&elves, i);
        elves = next;
    }
    let empty = bounding_empty(&elves);
    Ok(format!("Empty tiles in bounding rectangle: {empty}"))
}

pub fn part2(input: &str) -> Result<String> {
    let mut elves = parse(input);
    let mut round_num = 0u32;
    loop {
        let (next, moved) = round(&elves, round_num as usize);
        round_num += 1;
        elves = next;
        if !moved {
            break;
        }
    }
    Ok(format!("First round with no movement: {round_num}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "....#..\n..###.#\n#...#.#\n.#...##\n#.###..\n##.#.##\n.#..#..\n";

    #[test]
    fn part1_sample() {
        assert!(part1(SAMPLE).unwrap().contains("110"));
    }

    #[test]
    fn part2_sample() {
        assert!(part2(SAMPLE).unwrap().contains("20"));
    }

    #[test]
    fn parses_grid() {
        let e = parse("#.#\n.#.\n");
        assert_eq!(e.len(), 3);
        assert!(e.contains(&(0, 0)));
        assert!(e.contains(&(0, 2)));
        assert!(e.contains(&(1, 1)));
    }
}
