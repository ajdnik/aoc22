use std::collections::{HashMap, HashSet, VecDeque};
use crate::utils::file;
use log::{debug, info};

fn find_shortest_path(valves: &HashMap<String, (u32, Vec<String>)>, start: &String, end: &String) -> Vec<String> {
    if start.eq(end) {
        return vec![start.clone()];
    }
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start.clone());
    let mut queue: VecDeque<Vec<String>> = VecDeque::new();
    queue.push_front(vec![start.clone()]);
    while let Some(path) = queue.pop_front() {
        if let Some(last) = path.last() {
            if let Some(valve_data) = valves.get(last) {
                let (_, neighbors) = valve_data;
                for neighbor in neighbors.iter() {
                    if end.eq(neighbor) {
                        let mut new_path = path.clone();
                        new_path.push(neighbor.clone());
                        return new_path;
                    }
                    if !visited.contains(neighbor) {
                        let mut new_path = path.clone();
                        new_path.push(neighbor.clone());
                        queue.push_back(new_path);
                        visited.insert(neighbor.clone());
                    }
                }
            }
        }
    }
    vec![]
}

fn get_distances(valves: &HashMap<String, (u32, Vec<String>)>) -> HashMap<String, HashMap<String, u32>> {
    let names = valves.iter().fold(Vec::new(), |mut names, valve| {
        let (name, _) = valve;
        names.push(name.clone());
        names
    });
    let mut distances = HashMap::new();
    for start_name in names.iter() {
        let mut dist = HashMap::new();
        for end_name in names.iter() {
            let path = find_shortest_path(valves, start_name, end_name);
            dist.insert(end_name.clone(), path.len() as u32);
        }
        distances.insert(start_name.clone(), dist);
    }
    distances
}

fn get_unopened(valves: &HashMap<String, (u32, Vec<String>)>) -> Vec<String> {
    valves.iter().fold(Vec::new(), |mut unopened, valve_data| {
        let (name, data) = valve_data;
        let (flow_rate, _) = data;
        if *flow_rate > 0 {
            unopened.push(name.clone());
        }
        unopened
    })
}

fn compute_all_flows(valves: &HashMap<String, (u32, Vec<String>)>, unopened: &Vec<String>, path: &Vec<String>, distances: &HashMap<String, HashMap<String, u32>>, time_left: u32, flow: u32) -> Vec<(Vec<String>, u32)> {
    let mut flows = Vec::new();
    flows.push((path.clone(), flow));
    if let Some(last) = path.last() {
        if let Some(last_connections) = distances.get(last) {
            for valve in unopened.iter() {
                if let (Some(valve_distance), Some(valve_data)) = (last_connections.get(valve), valves.get(valve)) {
                    let new_time_left = time_left as i32 - *valve_distance as i32;
                    if new_time_left <= 0 {
                        continue;
                    }
                    let (flow_rate, _) = valve_data;
                    let new_flow = flow + new_time_left as u32 * *flow_rate;
                    let mut new_path = path.clone();
                    new_path.push(valve.clone());
                    let new_unopened = unopened.iter().fold(Vec::new(), |mut unopened, v| {
                        if !v.eq(valve) {
                            unopened.push(v.clone());
                        }
                        unopened
                    });
                    let new_flows = compute_all_flows(valves, &new_unopened, &new_path, distances, new_time_left as u32, new_flow);
                    for flow in new_flows.iter() {
                        flows.push(flow.clone());
                    }
                }
            }
        }
    }
    flows
}

fn find_best_parallel_solutions(solutions: &Vec<(Vec<String>, u32)>, start: &String) -> u32 {
    let mut max_flow = u32::MIN;
    for a in 0..solutions.len() {
        if let Some(a_item) = solutions.get(a) {
            let (a_path, a_flow) = a_item;
            let unique_a: HashSet<String> = a_path.clone().into_iter().collect();
            for b in a..solutions.len() {
                if let Some(b_item) = solutions.get(b) {
                    let (b_path, b_flow) = b_item;
                    let mut found = false;
                    for node in b_path.iter() {
                        if node.eq(start) {
                            continue;
                        }
                        if unique_a.contains(node) {
                            found = true;
                            break;
                        }
                    }
                    let ttl_flow = a_flow + b_flow;
                    if !found && ttl_flow > max_flow {
                        max_flow = ttl_flow;
                    }
                }
            }
        }
    }
    max_flow
}

pub fn task1(path: &str, minutes: u32) {
    if let Ok(lines) = file::read_lines(path) {
        let valves = file::to_valves::<u32>(lines);
        debug!("Found {} valves", valves.len());
        let distances = get_distances(&valves);
        let unopened = get_unopened(&valves);
        debug!("Found {} unopened valves", unopened.len());
        let all_flows = compute_all_flows(&valves, &unopened, &vec![String::from("AA")], &distances, minutes, 0);
        debug!("Found {} possible solutions", all_flows.len());
        let max_flow = all_flows.iter().fold(0, |max, result| {
            let (_, flow) = result;
            if *flow > max {
                *flow
            } else {
                max
            }
        });
        info!("Released {:?} pressure in {} minutes", max_flow, minutes);
    }
}

pub fn task2(path: &str, minutes: u32) {
    if let Ok(lines) = file::read_lines(path) {
        let valves = file::to_valves::<u32>(lines);
        debug!("Found {} valves", valves.len());
        let distances = get_distances(&valves);
        let unopened = get_unopened(&valves);
        debug!("Found {} unopened valves", unopened.len());
        let all_flows = compute_all_flows(&valves, &unopened, &vec![String::from("AA")], &distances, minutes, 0);
        debug!("Found {} possible solutions", all_flows.len());
        let best_flow = find_best_parallel_solutions(&all_flows, &String::from("AA"));
        info!("Released {} pressure in {} minutes when working with 1 elephant", best_flow, minutes);
    } 
}
