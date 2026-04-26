use crate::utils::file;
use anyhow::{Context, Result};
use std::collections::{HashMap, VecDeque};

fn build_indexed(
    valves: &HashMap<String, (u32, Vec<String>)>,
) -> (Vec<u32>, Vec<Vec<usize>>, HashMap<String, usize>) {
    let mut names: Vec<String> = valves.keys().cloned().collect();
    names.sort();
    let idx: HashMap<String, usize> = names
        .iter()
        .enumerate()
        .map(|(i, n)| (n.clone(), i))
        .collect();
    let n = names.len();
    let mut flow_rates = vec![0u32; n];
    let mut adj = vec![Vec::new(); n];
    for (i, name) in names.iter().enumerate() {
        let (rate, neighbors) = &valves[name];
        flow_rates[i] = *rate;
        adj[i] = neighbors.iter().map(|nb| idx[nb]).collect();
    }
    (flow_rates, adj, idx)
}

fn shortest_distances(adj: &[Vec<usize>]) -> Vec<Vec<u32>> {
    let n = adj.len();
    (0..n)
        .map(|src| {
            let mut row = vec![u32::MAX; n];
            row[src] = 0;
            let mut queue = VecDeque::new();
            queue.push_back(src);
            while let Some(cur) = queue.pop_front() {
                for &next in &adj[cur] {
                    if row[next] == u32::MAX {
                        row[next] = row[cur] + 1;
                        queue.push_back(next);
                    }
                }
            }
            row
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn explore(
    flow_rates: &[u32],
    dist: &[Vec<u32>],
    useful: &[usize],
    bits: &[u32],
    current: usize,
    time_left: u32,
    opened: u32,
    flow: u32,
    best: &mut HashMap<u32, u32>,
) {
    let entry = best.entry(opened).or_insert(0);
    if flow > *entry {
        *entry = flow;
    }
    for (i, &v) in useful.iter().enumerate() {
        let bit = bits[i];
        if opened & bit != 0 {
            continue;
        }
        let cost = dist[current][v] + 1;
        if cost >= time_left {
            continue;
        }
        let new_time = time_left - cost;
        let new_flow = flow + new_time * flow_rates[v];
        explore(
            flow_rates,
            dist,
            useful,
            bits,
            v,
            new_time,
            opened | bit,
            new_flow,
            best,
        );
    }
}

fn best_per_mask(
    valves: &HashMap<String, (u32, Vec<String>)>,
    minutes: u32,
) -> Result<HashMap<u32, u32>> {
    let (flow_rates, adj, idx) = build_indexed(valves);
    let start = *idx.get("AA").context("missing valve AA")?;
    let dist = shortest_distances(&adj);
    let useful: Vec<usize> = (0..flow_rates.len())
        .filter(|&i| flow_rates[i] > 0)
        .collect();
    let bits: Vec<u32> = (0..useful.len()).map(|i| 1u32 << i).collect();
    let mut best = HashMap::new();
    explore(
        &flow_rates,
        &dist,
        &useful,
        &bits,
        start,
        minutes,
        0,
        0,
        &mut best,
    );
    Ok(best)
}

fn best_parallel(best: &HashMap<u32, u32>) -> u32 {
    let entries: Vec<(u32, u32)> = best.iter().map(|(&m, &f)| (m, f)).collect();
    let mut max = 0;
    for i in 0..entries.len() {
        let (mask_a, flow_a) = entries[i];
        for &(mask_b, flow_b) in &entries[i..] {
            if mask_a & mask_b == 0 && flow_a + flow_b > max {
                max = flow_a + flow_b;
            }
        }
    }
    max
}

pub fn part1(input: &str, minutes: u32) -> Result<String> {
    let valves = file::to_valves::<u32, _>(file::lines_of(input));
    let best = best_per_mask(&valves, minutes)?;
    let max_flow = best.values().copied().max().unwrap_or(0);
    Ok(format!(
        "Released {} pressure in {} minutes",
        max_flow, minutes
    ))
}

pub fn part2(input: &str, minutes: u32) -> Result<String> {
    let valves = file::to_valves::<u32, _>(file::lines_of(input));
    let best = best_per_mask(&valves, minutes)?;
    let best_flow = best_parallel(&best);
    Ok(format!(
        "Released {} pressure in {} minutes when working with 1 elephant",
        best_flow, minutes
    ))
}
