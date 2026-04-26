use crate::utils::file;
use anyhow::Result;

const GEODE: usize = 3;

fn max_consumption(bp: &file::Blueprint<u32>) -> [u32; 3] {
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
    bp: &file::Blueprint<u32>,
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

fn max_geodes(bp: &file::Blueprint<u32>, t: u32) -> u32 {
    let max_c = max_consumption(bp);
    let mut best = 0u32;
    let bots = [1, 0, 0, 0];
    dfs(bp, &max_c, t, [0; 4], bots, &mut best);
    best
}

pub fn part1(input: &str) -> Result<String> {
    let blueprints = file::to_blueprints::<u32, _>(file::lines_of(input))?;
    let total: u32 = blueprints.iter().map(|bp| bp.id * max_geodes(bp, 24)).sum();
    Ok(format!("Sum of quality levels is {total}"))
}

pub fn part2(input: &str) -> Result<String> {
    let blueprints = file::to_blueprints::<u32, _>(file::lines_of(input))?;
    let product: u32 = blueprints
        .iter()
        .take(3)
        .map(|bp| max_geodes(bp, 32))
        .product();
    Ok(format!(
        "Product of geodes from first 3 blueprints is {product}"
    ))
}
