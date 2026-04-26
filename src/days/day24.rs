use anyhow::{bail, Result};
use std::collections::{HashSet, VecDeque};

const DR: [i32; 4] = [-1, 1, 0, 0];
const DC: [i32; 4] = [0, 0, -1, 1];

struct Valley {
    h: usize,
    w: usize,
    period: usize,
    blocked: Vec<Vec<bool>>,
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn parse(input: &str) -> Result<Valley> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() < 3 {
        bail!("input too short");
    }
    let h = lines.len() - 2;
    let w = lines[0].len() - 2;
    if w == 0 || h == 0 {
        bail!("empty inner valley");
    }
    let mut blizzards = Vec::new();
    for (r, line) in lines.iter().skip(1).take(h).enumerate() {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() != w + 2 {
            bail!("inconsistent row width");
        }
        for c in 0..w {
            let ch = chars[c + 1];
            let dir = match ch {
                '^' => Some(0),
                'v' => Some(1),
                '<' => Some(2),
                '>' => Some(3),
                '.' => None,
                _ => bail!("unexpected char {ch:?}"),
            };
            if let Some(d) = dir {
                blizzards.push((r as i32, c as i32, d));
            }
        }
    }
    let period = lcm(h, w);
    let mut blocked = vec![vec![false; h * w]; period];
    for (t, layer) in blocked.iter_mut().enumerate() {
        for &(r, c, d) in &blizzards {
            let nr = (r + DR[d] * t as i32).rem_euclid(h as i32) as usize;
            let nc = (c + DC[d] * t as i32).rem_euclid(w as i32) as usize;
            layer[nr * w + nc] = true;
        }
    }
    Ok(Valley {
        h,
        w,
        period,
        blocked,
    })
}

fn shortest(v: &Valley, start: (i32, i32), end: (i32, i32), t0: usize) -> Result<usize> {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    q.push_back((start.0, start.1, t0));
    visited.insert((start.0, start.1, t0 % v.period));
    while let Some((r, c, t)) = q.pop_front() {
        let nt = t + 1;
        let nt_mod = nt % v.period;
        for k in 0..5 {
            let (dr, dc) = if k < 4 { (DR[k], DC[k]) } else { (0, 0) };
            let nr = r + dr;
            let nc = c + dc;
            if (nr, nc) == end {
                return Ok(nt);
            }
            if (nr, nc) == start {
                if visited.insert((nr, nc, nt_mod)) {
                    q.push_back((nr, nc, nt));
                }
                continue;
            }
            if nr < 0 || nr >= v.h as i32 || nc < 0 || nc >= v.w as i32 {
                continue;
            }
            if v.blocked[nt_mod][nr as usize * v.w + nc as usize] {
                continue;
            }
            if visited.insert((nr, nc, nt_mod)) {
                q.push_back((nr, nc, nt));
            }
        }
    }
    bail!("no path from {:?} to {:?}", start, end)
}

pub fn part1(input: &str) -> Result<String> {
    let v = parse(input)?;
    let start = (-1i32, 0i32);
    let end = (v.h as i32, v.w as i32 - 1);
    let t = shortest(&v, start, end, 0)?;
    Ok(format!("Fastest path to goal takes {t} minutes"))
}

pub fn part2(input: &str) -> Result<String> {
    let v = parse(input)?;
    let start = (-1i32, 0i32);
    let end = (v.h as i32, v.w as i32 - 1);
    let t1 = shortest(&v, start, end, 0)?;
    let t2 = shortest(&v, end, start, t1)?;
    let t3 = shortest(&v, start, end, t2)?;
    Ok(format!("Round trip with snack takes {t3} minutes"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#.######\n#>>.<^<#\n#.<..<<#\n#>v.><>#\n#<^v^^>#\n######.#\n";

    #[test]
    fn part1_sample() {
        assert!(part1(SAMPLE).unwrap().contains("18"));
    }

    #[test]
    fn part2_sample() {
        assert!(part2(SAMPLE).unwrap().contains("54"));
    }

    #[test]
    fn parses_grid() {
        let v = parse(SAMPLE).unwrap();
        assert_eq!(v.h, 4);
        assert_eq!(v.w, 6);
        assert_eq!(v.period, 12);
    }

    #[test]
    fn rejects_short() {
        assert!(parse("#.#\n#.#\n").is_err());
    }
}
