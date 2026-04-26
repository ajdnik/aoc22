use anyhow::{Context, Result};
use std::{collections::HashMap, str::FromStr};

pub fn to_valves<N, I>(lines: I) -> Result<HashMap<String, (N, Vec<String>)>>
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
            .with_context(|| format!("malformed valve line {:?}", line))?;
        let header = header
            .strip_prefix("Valve ")
            .with_context(|| format!("missing 'Valve' prefix {:?}", header))?;
        let (name, rate_str) = header
            .split_once(" has flow rate=")
            .with_context(|| format!("malformed valve header {:?}", header))?;
        let neighbors_str = tail
            .strip_prefix("tunnels lead to valves ")
            .or_else(|| tail.strip_prefix("tunnel leads to valve "))
            .with_context(|| format!("malformed tunnels {:?}", tail))?;
        let neighbors: Vec<String> = neighbors_str.split(", ").map(String::from).collect();
        let rate: N = rate_str
            .parse()
            .with_context(|| format!("parsing valve rate {:?}", rate_str))?;
        valves.insert(name.to_string(), (rate, neighbors));
    }
    Ok(valves)
}
