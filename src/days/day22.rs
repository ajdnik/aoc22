use anyhow::{bail, Context, Result};
use std::collections::{HashMap, HashSet, VecDeque};

type V3 = [i32; 3];

fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn sub(a: V3, b: V3) -> V3 {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
}
fn neg(a: V3) -> V3 {
    [-a[0], -a[1], -a[2]]
}
fn scale(a: V3, k: i32) -> V3 {
    [a[0] * k, a[1] * k, a[2] * k]
}
fn dot(a: V3, b: V3) -> i32 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

#[derive(Clone, Debug)]
struct Face {
    br: usize,
    bc: usize,
    r: V3,
    d: V3,
    n: V3,
    o: V3,
}

#[derive(Clone, Debug)]
enum Step {
    Forward(u32),
    Left,
    Right,
}

fn parse(input: &str) -> Result<(Vec<Vec<char>>, Vec<Step>)> {
    let split = input
        .find("\n\n")
        .context("expected blank line between map and path")?;
    let (map_part, rest) = input.split_at(split);
    let path_part = rest.trim_start_matches('\n').trim_end();
    let grid: Vec<Vec<char>> = map_part.lines().map(|l| l.chars().collect()).collect();
    let path = parse_path(path_part)?;
    Ok((grid, path))
}

fn parse_path(s: &str) -> Result<Vec<Step>> {
    let mut steps = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            let mut n = 0u32;
            while let Some(&c) = chars.peek() {
                if let Some(d) = c.to_digit(10) {
                    n = n * 10 + d;
                    chars.next();
                } else {
                    break;
                }
            }
            steps.push(Step::Forward(n));
        } else if c == 'L' {
            chars.next();
            steps.push(Step::Left);
        } else if c == 'R' {
            chars.next();
            steps.push(Step::Right);
        } else {
            bail!("invalid path character {c:?}");
        }
    }
    Ok(steps)
}

fn cell(grid: &[Vec<char>], r: usize, c: usize) -> char {
    grid.get(r)
        .and_then(|row| row.get(c))
        .copied()
        .unwrap_or(' ')
}

fn face_size(grid: &[Vec<char>]) -> usize {
    let total: usize = grid
        .iter()
        .map(|r| r.iter().filter(|&&c| c == '.' || c == '#').count())
        .sum();
    let n = total / 6;
    let mut s = 1usize;
    while s * s < n {
        s += 1;
    }
    s
}

fn find_blocks(grid: &[Vec<char>], s: usize) -> Vec<(usize, usize)> {
    let max_br = grid.len().div_ceil(s);
    let max_bc = grid.iter().map(|r| r.len()).max().unwrap_or(0).div_ceil(s);
    let mut blocks = Vec::new();
    for br in 0..max_br {
        for bc in 0..max_bc {
            let ch = cell(grid, br * s, bc * s);
            if ch == '.' || ch == '#' {
                blocks.push((br, bc));
            }
        }
    }
    blocks
}

fn fold(p: &Face, dir: usize, s: i32) -> (V3, V3, V3, V3) {
    match dir {
        0 => (neg(p.n), p.d, p.r, add(p.o, scale(p.r, s))),
        1 => (p.r, neg(p.n), p.d, add(p.o, scale(p.d, s))),
        2 => {
            let r_c = p.n;
            (r_c, p.d, neg(p.r), sub(p.o, scale(r_c, s)))
        }
        3 => {
            let d_c = p.n;
            (p.r, d_c, neg(p.d), sub(p.o, scale(d_c, s)))
        }
        _ => unreachable!(),
    }
}

fn build_faces(blocks: &[(usize, usize)], s: usize) -> Vec<Face> {
    let block_set: HashSet<(usize, usize)> = blocks.iter().copied().collect();
    let mut by_block: HashMap<(usize, usize), Face> = HashMap::new();
    let start = blocks[0];
    by_block.insert(
        start,
        Face {
            br: start.0,
            bc: start.1,
            r: [1, 0, 0],
            d: [0, 1, 0],
            n: [0, 0, -1],
            o: [0, 0, 0],
        },
    );
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back(start);
    let neighbors = [(0i32, 1i32, 0usize), (1, 0, 1), (0, -1, 2), (-1, 0, 3)];
    while let Some(b) = q.pop_front() {
        let f = by_block.get(&b).unwrap().clone();
        for &(dr, dc, dir) in &neighbors {
            let nbr = b.0 as i32 + dr;
            let nbc = b.1 as i32 + dc;
            if nbr < 0 || nbc < 0 {
                continue;
            }
            let nb = (nbr as usize, nbc as usize);
            if !block_set.contains(&nb) || by_block.contains_key(&nb) {
                continue;
            }
            let (r_c, d_c, n_c, o_c) = fold(&f, dir, s as i32);
            by_block.insert(
                nb,
                Face {
                    br: nb.0,
                    bc: nb.1,
                    r: r_c,
                    d: d_c,
                    n: n_c,
                    o: o_c,
                },
            );
            q.push_back(nb);
        }
    }
    by_block.into_values().collect()
}

fn step_2d(grid: &[Vec<char>], r: i32, c: i32, facing: usize) -> (i32, i32) {
    let (dr, dc) = match facing {
        0 => (0, 1),
        1 => (1, 0),
        2 => (0, -1),
        3 => (-1, 0),
        _ => unreachable!(),
    };
    let rows = grid.len() as i32;
    let mut nr = r + dr;
    let mut nc = c + dc;
    loop {
        if nr < 0 {
            nr = rows - 1;
        } else if nr >= rows {
            nr = 0;
        }
        if dc != 0 {
            let row_len = grid[nr as usize].len() as i32;
            if nc < 0 {
                nc = row_len - 1;
            } else if nc >= row_len {
                nc = 0;
            }
        }
        let ch = cell(grid, nr as usize, nc as usize);
        if ch == '.' || ch == '#' {
            return (nr, nc);
        }
        nr += dr;
        nc += dc;
    }
}

fn walk_2d(grid: &[Vec<char>], path: &[Step]) -> (i32, i32, usize) {
    let r0 = 0i32;
    let c0 = grid[0].iter().position(|&c| c == '.').unwrap() as i32;
    let mut r = r0;
    let mut c = c0;
    let mut facing = 0usize;
    for step in path {
        match step {
            Step::Forward(n) => {
                for _ in 0..*n {
                    let (nr, nc) = step_2d(grid, r, c, facing);
                    if grid[nr as usize][nc as usize] == '#' {
                        break;
                    }
                    r = nr;
                    c = nc;
                }
            }
            Step::Left => facing = (facing + 3) % 4,
            Step::Right => facing = (facing + 1) % 4,
        }
    }
    (r, c, facing)
}

#[allow(clippy::too_many_arguments)]
fn step_cube(
    faces: &[Face],
    s: usize,
    face_idx: usize,
    r: usize,
    c: usize,
    facing: usize,
) -> (usize, usize, usize, usize) {
    let face = &faces[face_idx];
    let (dr, dc) = match facing {
        0 => (0i32, 1i32),
        1 => (1, 0),
        2 => (0, -1),
        3 => (-1, 0),
        _ => unreachable!(),
    };
    let nr = r as i32 + dr;
    let nc = c as i32 + dc;
    if nr >= 0 && nr < s as i32 && nc >= 0 && nc < s as i32 {
        return (face_idx, nr as usize, nc as usize, facing);
    }
    let walk_dir = match facing {
        0 => face.r,
        1 => face.d,
        2 => neg(face.r),
        3 => neg(face.d),
        _ => unreachable!(),
    };
    let f_prime_idx = faces
        .iter()
        .position(|f| f.n == walk_dir)
        .expect("missing adjacent face");
    let f_prime = &faces[f_prime_idx];
    let new_walk_dir = neg(face.n);

    let p_f_double = add(
        scale(face.o, 2),
        add(
            scale(face.d, 2 * r as i32 + 1),
            scale(face.r, 2 * c as i32 + 1),
        ),
    );
    let p_fprime_double = add(p_f_double, add(walk_dir, new_walk_dir));
    let diff = sub(p_fprime_double, scale(f_prime.o, 2));
    let two_c_plus_1 = dot(diff, f_prime.r);
    let two_r_plus_1 = dot(diff, f_prime.d);
    let new_c = ((two_c_plus_1 - 1) / 2) as usize;
    let new_r = ((two_r_plus_1 - 1) / 2) as usize;

    let new_facing = if new_walk_dir == f_prime.r {
        0
    } else if new_walk_dir == f_prime.d {
        1
    } else if new_walk_dir == neg(f_prime.r) {
        2
    } else if new_walk_dir == neg(f_prime.d) {
        3
    } else {
        panic!("direction mismatch");
    };
    (f_prime_idx, new_r, new_c, new_facing)
}

fn walk_cube(grid: &[Vec<char>], faces: &[Face], s: usize, path: &[Step]) -> (usize, usize, usize) {
    let c0 = grid[0].iter().position(|&c| c == '.').unwrap();
    let bc0 = c0 / s;
    let mut idx = faces
        .iter()
        .position(|f| f.br == 0 && f.bc == bc0)
        .expect("starting face missing");
    let mut r = 0usize;
    let mut c = c0 - bc0 * s;
    let mut facing = 0usize;
    for step in path {
        match step {
            Step::Forward(n) => {
                for _ in 0..*n {
                    let (nidx, nr, nc, nf) = step_cube(faces, s, idx, r, c, facing);
                    let abs_r = faces[nidx].br * s + nr;
                    let abs_c = faces[nidx].bc * s + nc;
                    if grid[abs_r][abs_c] == '#' {
                        break;
                    }
                    idx = nidx;
                    r = nr;
                    c = nc;
                    facing = nf;
                }
            }
            Step::Left => facing = (facing + 3) % 4,
            Step::Right => facing = (facing + 1) % 4,
        }
    }
    let abs_r = faces[idx].br * s + r;
    let abs_c = faces[idx].bc * s + c;
    (abs_r, abs_c, facing)
}

pub fn part1(input: &str) -> Result<String> {
    let (grid, path) = parse(input)?;
    let (r, c, facing) = walk_2d(&grid, &path);
    let password = 1000 * (r + 1) + 4 * (c + 1) + facing as i32;
    Ok(format!("Final password is {password}"))
}

pub fn part2(input: &str) -> Result<String> {
    let (grid, path) = parse(input)?;
    let s = face_size(&grid);
    let blocks = find_blocks(&grid, s);
    let faces = build_faces(&blocks, s);
    let (r, c, facing) = walk_cube(&grid, &faces, s, &path);
    let password = 1000 * (r + 1) + 4 * (c + 1) + facing;
    Ok(format!("Final password is {password}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n10R5L5R10L4R5L5\n";

    #[test]
    fn part1_sample() {
        assert!(part1(SAMPLE).unwrap().contains("6032"));
    }

    #[test]
    fn part2_sample() {
        assert!(part2(SAMPLE).unwrap().contains("5031"));
    }

    #[test]
    fn errors_on_missing_blank() {
        assert!(part1("foo").is_err());
    }

    #[test]
    fn errors_on_invalid_path_char() {
        let bad = "..\n..\n\n10X5";
        assert!(part1(bad).is_err());
    }
}
