use std::{ 
    fs::File,
    io::{
        BufReader,
        Lines,
    },
    str::FromStr,
};
use crate::utils::file;
use log::{debug, info};

pub fn parse_crates<N>(lines: Lines<BufReader<File>>) -> (Vec<String>, Vec<(N, N, N)>)
where N: FromStr + Copy {
    let mut stacks: Vec<String> = Vec::new();
    let mut operations = Vec::new();
    for itm in lines {
        if let Ok(line) = itm {
            if line.trim().starts_with("[") {
                let columns = line.split(" ").collect::<Vec<&str>>();
                let mut space_count = 0;
                let mut column_idx = 0;
                for column in columns {
                    if column.starts_with("[") {
                        if stacks.len() <= column_idx {
                            for _ in stacks.len()..column_idx+1 {
                                stacks.push(String::from(""));
                            }
                        }
                        stacks[column_idx].push(column.chars().nth(1).unwrap());
                        column_idx += 1;
                        space_count = 0;
                    } else {
                        space_count += 1;
                        if space_count == 4 {
                            column_idx += 1;
                            space_count = 0;
                        }
                    }
                }
            } else if line.starts_with("move") {
                let operation_parts = line.split(" ").collect::<Vec<&str>>();
                let mut numbers: Vec<N> = Vec::new();
                if let Ok(num) = operation_parts[1].parse::<N>() {
                    numbers.push(num);
                }
                if let Ok(num) = operation_parts[3].parse::<N>() {
                    numbers.push(num);
                }
                if let Ok(num) = operation_parts[5].parse::<N>() {
                    numbers.push(num);
                }
                operations.push((numbers[0], numbers[1], numbers[2]));
            }
        }
    }
    (stacks, operations)
}

fn move_crates(mut stacks: Vec<String>, operations: Vec<(usize, usize, usize)>, all_at_once: Option<bool>) -> Vec<String> {
    for operation in operations.iter() {
        let (size, from, to) = operation;
        let (first, rest) = stacks[from - 1].split_at(*size);
        let mut new_column: String = first.to_string();
        if !all_at_once.unwrap_or(false) {
            new_column = new_column.chars().rev().collect();
        }
        new_column.push_str(stacks[to - 1].as_str());
        stacks[from - 1] = rest.to_string();
        stacks[to - 1] = new_column;
    }
    stacks
}

fn get_top_crates(stacks: Vec<String>) -> String {
    stacks.iter().fold(String::from(""), |mut result, stack| {
        result.push(stack.chars().nth(0).unwrap());
        result
    })
}

pub fn task1(path: &str) {
    if let Ok(lines) = file::read_lines(path) {
        let (stacks, operations) = parse_crates(lines);
        debug!("Found {} stacks", stacks.len());
        debug!("Found {} operations", operations.len());
        let reordered_crates = move_crates(stacks, operations, None);
        let top_crates = get_top_crates(reordered_crates);
        info!("After reordering the top crates are {}", top_crates);
    }
}

pub fn task2(path: &str) {
    if let Ok(lines) = file::read_lines(path) {
        let (stacks, operations) = parse_crates(lines);
        debug!("Found {} stacks", stacks.len());
        debug!("Found {} operations", operations.len());
        let reordered_crates = move_crates(stacks, operations, Some(true));
        let top_crates = get_top_crates(reordered_crates);
        info!("After reordering the top crates are {}", top_crates);
    }
}
