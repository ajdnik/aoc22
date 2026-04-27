use crate::utils::file;
use anyhow::{bail, Context, Result};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Blueprint<N> {
    id: N,
    costs: [[N; 3]; 4],
}

fn to_blueprints<N, I>(lines: I) -> Result<Vec<Blueprint<N>>>
where
    N: FromStr + Default + Copy,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let nums: Vec<N> = line
                .split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.parse::<N>()
                        .with_context(|| format!("parsing number {s:?} in {line:?}"))
                })
                .collect::<Result<Vec<_>>>()?;
            if nums.len() != 7 {
                bail!(
                    "expected 7 numbers in blueprint, got {} in {line:?}",
                    nums.len()
                );
            }
            let zero = N::default();
            Ok(Blueprint {
                id: nums[0],
                costs: [
                    [nums[1], zero, zero],
                    [nums[2], zero, zero],
                    [nums[3], nums[4], zero],
                    [nums[5], zero, nums[6]],
                ],
            })
        })
        .collect()
}

const GEODE: usize = 3;

fn max_consumption(bp: &Blueprint<u32>) -> [u32; 3] {
    let mut max = [0u32; 3];
    for cost in &bp.costs {
        for i in 0..3 {
            if cost[i] > max[i] {
                max[i] = cost[i];
            }
        }
    }
    max
}

fn dfs(
    bp: &Blueprint<u32>,
    max_c: &[u32; 3],
    t: u32,
    res: [u32; 4],
    bots: [u32; 4],
    best: &mut u32,
) {
    let baseline = res[GEODE] + bots[GEODE] * t;
    if baseline > *best {
        *best = baseline;
    }
    let upper = baseline + t * t.saturating_sub(1) / 2;
    if upper <= *best {
        return;
    }
    for (r, cost) in bp.costs.iter().enumerate() {
        if r != GEODE && bots[r] >= max_c[r] {
            continue;
        }
        let mut wait = 0u32;
        let mut impossible = false;
        for i in 0..3 {
            if cost[i] == 0 {
                continue;
            }
            if bots[i] == 0 {
                impossible = true;
                break;
            }
            if cost[i] > res[i] {
                let need = cost[i] - res[i];
                let w = need.div_ceil(bots[i]);
                if w > wait {
                    wait = w;
                }
            }
        }
        if impossible {
            continue;
        }
        let total_wait = wait + 1;
        if total_wait >= t {
            continue;
        }
        let new_t = t - total_wait;
        let mut new_res = res;
        let mut new_bots = bots;
        for i in 0..4 {
            new_res[i] += bots[i] * total_wait;
        }
        for i in 0..3 {
            new_res[i] -= cost[i];
        }
        new_bots[r] += 1;
        dfs(bp, max_c, new_t, new_res, new_bots, best);
    }
}

fn max_geodes(bp: &Blueprint<u32>, t: u32) -> u32 {
    let max_c = max_consumption(bp);
    let mut best = 0u32;
    let bots = [1, 0, 0, 0];
    dfs(bp, &max_c, t, [0; 4], bots, &mut best);
    best
}

pub fn part1(input: &str) -> Result<String> {
    let blueprints = to_blueprints::<u32, _>(file::lines_of(input))?;
    let total: u32 = blueprints.iter().map(|bp| bp.id * max_geodes(bp, 24)).sum();
    Ok(format!("Sum of quality levels is {total}"))
}

pub fn part2(input: &str) -> Result<String> {
    let blueprints = to_blueprints::<u32, _>(file::lines_of(input))?;
    let product: u32 = blueprints
        .iter()
        .take(3)
        .map(|bp| max_geodes(bp, 32))
        .product();
    Ok(format!(
        "Product of geodes from first 3 blueprints is {product}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";

    #[test]
    fn parses_blueprint() {
        let bp = to_blueprints::<u32, _>([SAMPLE.to_string()]).unwrap();
        assert_eq!(bp.len(), 1);
        assert_eq!(bp[0].id, 1);
        assert_eq!(bp[0].costs[0], [4, 0, 0]);
        assert_eq!(bp[0].costs[1], [2, 0, 0]);
        assert_eq!(bp[0].costs[2], [3, 14, 0]);
        assert_eq!(bp[0].costs[3], [2, 0, 7]);
    }

    #[test]
    fn errors_on_missing_numbers() {
        assert!(to_blueprints::<u32, _>(["Blueprint 1: foo".to_string()]).is_err());
    }

    #[test]
    fn errors_on_too_many_numbers() {
        let line = "Blueprint 1 2 3 4 5 6 7 8".to_string();
        assert!(to_blueprints::<u32, _>([line]).is_err());
    }
}
