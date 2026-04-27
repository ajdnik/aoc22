use crate::utils::file;
use anyhow::{Context, Result};
use rustc_hash::FxHashMap;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

fn to_valves<N, I>(lines: I) -> Result<HashMap<String, (N, Vec<String>)>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    let mut valves = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (header, tail) = line
            .split_once("; ")
            .with_context(|| format!("malformed valve line {line:?}"))?;
        let header = header
            .strip_prefix("Valve ")
            .with_context(|| format!("missing 'Valve' prefix {header:?}"))?;
        let (name, rate_str) = header
            .split_once(" has flow rate=")
            .with_context(|| format!("malformed valve header {header:?}"))?;
        let neighbors_str = tail
            .strip_prefix("tunnels lead to valves ")
            .or_else(|| tail.strip_prefix("tunnel leads to valve "))
            .with_context(|| format!("malformed tunnels {tail:?}"))?;
        let neighbors: Vec<String> = neighbors_str.split(", ").map(String::from).collect();
        let rate: N = rate_str
            .parse()
            .with_context(|| format!("parsing valve rate {rate_str:?}"))?;
        valves.insert(name.to_string(), (rate, neighbors));
    }
    Ok(valves)
}

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
    best: &mut FxHashMap<u32, u32>,
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
) -> Result<FxHashMap<u32, u32>> {
    let (flow_rates, adj, idx) = build_indexed(valves);
    let start = *idx.get("AA").context("missing valve AA")?;
    let dist = shortest_distances(&adj);
    let useful: Vec<usize> = (0..flow_rates.len())
        .filter(|&i| flow_rates[i] > 0)
        .collect();
    let bits: Vec<u32> = (0..useful.len()).map(|i| 1u32 << i).collect();
    let mut best = FxHashMap::default();
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

fn best_parallel(best: &FxHashMap<u32, u32>) -> u32 {
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
    let valves = to_valves::<u32, _>(file::lines_of(input))?;
    let best = best_per_mask(&valves, minutes)?;
    let max_flow = best.values().copied().max().unwrap_or(0);
    Ok(format!("Released {max_flow} pressure in {minutes} minutes"))
}

pub fn part2(input: &str, minutes: u32) -> Result<String> {
    let valves = to_valves::<u32, _>(file::lines_of(input))?;
    let best = best_per_mask(&valves, minutes)?;
    let best_flow = best_parallel(&best);
    Ok(format!(
        "Released {best_flow} pressure in {minutes} minutes when working with 1 elephant"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plural_tunnels() {
        let lines = ["Valve AA has flow rate=0; tunnels lead to valves BB, CC"].map(String::from);
        let v = to_valves::<u32, _>(lines).unwrap();
        let (rate, neighbors) = &v["AA"];
        assert_eq!(*rate, 0);
        assert_eq!(neighbors, &vec!["BB".to_string(), "CC".to_string()]);
    }

    #[test]
    fn parses_singular_tunnel() {
        let lines = ["Valve HH has flow rate=22; tunnel leads to valve GG"].map(String::from);
        let v = to_valves::<u32, _>(lines).unwrap();
        let (rate, neighbors) = &v["HH"];
        assert_eq!(*rate, 22);
        assert_eq!(neighbors, &vec!["GG".to_string()]);
    }

    #[test]
    fn missing_valve_prefix_errors() {
        let lines = ["AA has flow rate=0; tunnel leads to valve BB"].map(String::from);
        assert!(to_valves::<u32, _>(lines).is_err());
    }
}
