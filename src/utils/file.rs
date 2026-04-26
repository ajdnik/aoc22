use log::error;
use num::Num;
use std::{collections::HashMap, ops::Range, str::FromStr};

pub fn lines_of(input: &str) -> impl Iterator<Item = String> + '_ {
    input.lines().map(String::from)
}

pub fn lines_to_numbers<N, I>(lines: I) -> Vec<Option<N>>
where
    N: FromStr,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .map(|line| line.parse::<N>().ok())
        .collect()
}

pub fn to_groups<I>(lines: I, size: usize) -> Vec<Vec<String>>
where
    I: IntoIterator<Item = String>,
{
    let mut groups: Vec<Vec<String>> = vec![Vec::new()];
    for line in lines {
        if groups.last().map(|g| g.len()).unwrap_or(0) >= size {
            groups.push(Vec::new());
        }
        groups.last_mut().unwrap().push(line);
    }
    groups
}

pub fn to_range_touple<N, I>(lines: I) -> Vec<(Range<N>, Range<N>)>
where
    N: FromStr + Copy,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .filter_map(|line| {
            let ranges: Vec<&str> = line.split(',').collect();
            let first: Vec<&str> = ranges[0].split('-').collect();
            let second: Vec<&str> = ranges[1].split('-').collect();
            let a = first[0].parse::<N>().ok()?;
            let b = first[1].parse::<N>().ok()?;
            let c = second[0].parse::<N>().ok()?;
            let d = second[1].parse::<N>().ok()?;
            Some((a..b, c..d))
        })
        .collect()
}

#[derive(Clone, PartialEq)]
pub enum FilesystemType {
    Dir,
    File,
}

pub fn parse_filesystem<N, I>(std_output: I) -> Vec<(FilesystemType, String, N)>
where
    N: FromStr + Num,
    I: IntoIterator<Item = String>,
{
    let mut is_ls = false;
    let mut working_directory = String::new();
    let mut filesystem = Vec::<(FilesystemType, String, N)>::new();
    for line in std_output {
        if line == "$ cd .." {
            is_ls = false;
            let mut parts: Vec<&str> = working_directory.split('/').collect();
            parts.pop();
            parts.pop();
            working_directory = parts.join("/");
            working_directory.push('/');
        } else if line == "$ ls" {
            is_ls = true;
        } else if !line.starts_with('$') && is_ls {
            if !line.starts_with("dir") {
                let stats: Vec<&str> = line.split(' ').collect();
                if let Ok(size) = stats[0].parse::<N>() {
                    filesystem.push((
                        FilesystemType::File,
                        working_directory.clone() + stats[1],
                        size,
                    ));
                }
            }
        } else if line.starts_with("$ cd") {
            is_ls = false;
            let (_, dir_name) = line.split_at(5);
            if working_directory.is_empty() {
                working_directory = String::from(dir_name);
            } else {
                working_directory.push_str(dir_name);
                working_directory.push('/');
            }
            filesystem.push((FilesystemType::Dir, working_directory.clone(), N::zero()));
        }
    }
    filesystem
}

pub fn to_matrix<N, I>(lines: I) -> Vec<Vec<N>>
where
    N: FromStr,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .map(|line| {
            line.chars()
                .filter_map(|chr| chr.to_string().parse::<N>().ok())
                .collect()
        })
        .collect()
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn to_movements<I>(lines: I) -> Vec<Direction>
where
    I: IntoIterator<Item = String>,
{
    let mut movements = Vec::new();
    for line in lines {
        let (direction, length) = line.split_at(2);
        let Ok(val) = length.parse::<usize>() else {
            continue;
        };
        let dir = match direction {
            "U " => Direction::Up,
            "D " => Direction::Down,
            "L " => Direction::Left,
            "R " => Direction::Right,
            _ => {
                error!("Unknown direction {}", direction);
                continue;
            }
        };
        for _ in 0..val {
            movements.push(dir);
        }
    }
    movements
}

pub enum CPUCommand {
    Addx,
    Noop,
}

pub fn to_commands<N, I>(input: I) -> Vec<(CPUCommand, N)>
where
    N: FromStr + Num,
    I: IntoIterator<Item = String>,
{
    let mut commands = Vec::new();
    for line in input {
        if line.starts_with("noop") {
            commands.push((CPUCommand::Noop, N::zero()));
        } else {
            let (_, num) = line.split_at(5);
            if let Ok(val) = num.parse::<N>() {
                commands.push((CPUCommand::Addx, val));
            }
        }
    }
    commands
}

pub enum Operation<N> {
    Add(N),
    Multiply(N),
    Pow2,
    Unknown,
}

pub struct Monkey<N, T> {
    pub items: Vec<N>,
    pub op: Operation<N>,
    pub test_divisible: N,
    pub test_true: T,
    pub test_false: T,
}

pub fn to_monkeys<N, T, I>(input: I) -> Vec<Monkey<N, T>>
where
    N: FromStr + Num,
    T: FromStr + Num,
    I: IntoIterator<Item = String>,
{
    let mut monkeys: Vec<Monkey<N, T>> = Vec::new();
    for line in input {
        if line.starts_with("Monkey") {
            monkeys.push(Monkey {
                items: Vec::new(),
                op: Operation::Unknown,
                test_divisible: N::zero(),
                test_true: T::zero(),
                test_false: T::zero(),
            });
        } else if line.starts_with("  Starting items:") {
            let (_, nums) = line.split_at(18);
            let last_monkey = monkeys.last_mut().unwrap();
            for item in nums.split(", ") {
                if let Ok(val) = item.parse::<N>() {
                    last_monkey.items.push(val);
                }
            }
        } else if line.starts_with("  Operation:") {
            let (_, calc) = line.split_at(19);
            let calc_parts: Vec<&str> = calc.split(' ').collect();
            let last_monkey = monkeys.last_mut().unwrap();
            match calc_parts[1] {
                "+" => {
                    if let Ok(val) = calc_parts[2].parse::<N>() {
                        last_monkey.op = Operation::Add(val);
                    }
                }
                "*" => {
                    if calc_parts[2] == "old" {
                        last_monkey.op = Operation::Pow2;
                    } else if let Ok(val) = calc_parts[2].parse::<N>() {
                        last_monkey.op = Operation::Multiply(val);
                    }
                }
                _ => error!("Unsupported operation {}", calc),
            }
        } else if line.starts_with("  Test:") {
            let (_, divisible) = line.split_at(21);
            let last_monkey = monkeys.last_mut().unwrap();
            if let Ok(val) = divisible.parse::<N>() {
                last_monkey.test_divisible = val;
            }
        } else if line.starts_with("    If true:") {
            let (_, monkey) = line.split_at(29);
            let last_monkey = monkeys.last_mut().unwrap();
            if let Ok(val) = monkey.parse::<T>() {
                last_monkey.test_true = val;
            }
        } else if line.starts_with("    If false:") {
            let (_, monkey) = line.split_at(30);
            let last_monkey = monkeys.last_mut().unwrap();
            if let Ok(val) = monkey.parse::<T>() {
                last_monkey.test_false = val;
            }
        }
    }
    monkeys
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Position<N> {
    pub x: N,
    pub y: N,
}

pub fn to_elevation_map<N, I>(input: I) -> (Vec<Vec<N>>, Position<usize>, Position<usize>)
where
    N: From<u8>,
    I: IntoIterator<Item = String>,
{
    let mut elevation: Vec<Vec<N>> = Vec::new();
    let mut start = Position { x: 0, y: 0 };
    let mut end = Position { x: 0, y: 0 };
    for (y, line) in input.into_iter().enumerate() {
        let mut row: Vec<N> = Vec::new();
        for (x, chr) in line.chars().enumerate() {
            let mut ascii = chr as u8;
            if chr == 'S' {
                start = Position { x, y };
                ascii = b'a';
            } else if chr == 'E' {
                end = Position { x, y };
                ascii = b'z';
            }
            row.push(ascii.into());
        }
        elevation.push(row);
    }
    (elevation, start, end)
}

#[derive(Clone, Copy, PartialEq)]
pub enum SignalParts<N> {
    Start,
    End,
    Next,
    Number(N),
}

pub fn to_signals<N, I>(lines: I) -> Vec<Vec<SignalParts<N>>>
where
    N: FromStr,
    I: IntoIterator<Item = String>,
{
    let mut signals = Vec::new();
    for line in lines {
        if !line.starts_with('[') {
            continue;
        }
        let mut signal = Vec::new();
        let mut num_buffer = String::new();
        for chr in line.chars() {
            match chr {
                '[' => signal.push(SignalParts::Start),
                ']' => {
                    if !num_buffer.is_empty() {
                        if let Ok(val) = num_buffer.parse::<N>() {
                            signal.push(SignalParts::Number(val));
                        }
                        num_buffer.clear();
                    }
                    signal.push(SignalParts::End);
                }
                ',' => {
                    if !num_buffer.is_empty() {
                        if let Ok(val) = num_buffer.parse::<N>() {
                            signal.push(SignalParts::Number(val));
                        }
                        num_buffer.clear();
                    }
                    signal.push(SignalParts::Next);
                }
                c => num_buffer.push(c),
            }
        }
        signals.push(signal);
    }
    signals
}

pub fn to_walls<N, I>(lines: I) -> Vec<Vec<Position<N>>>
where
    N: FromStr,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .map(|line| {
            line.split(" -> ")
                .filter_map(|point| {
                    let dim: Vec<&str> = point.split(',').collect();
                    let x = dim[0].parse::<N>().ok()?;
                    let y = dim[1].parse::<N>().ok()?;
                    Some(Position { x, y })
                })
                .collect()
        })
        .collect()
}

pub fn to_sensor_data<N, I>(lines: I) -> Vec<(Position<N>, Position<N>)>
where
    N: FromStr,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let (_, sensor_loc) = parts[0].split_at(10);
            let (_, beacon_loc) = parts[1].split_at(22);
            let sp: Vec<&str> = sensor_loc.split(", ").collect();
            let bp: Vec<&str> = beacon_loc.split(", ").collect();
            let sensor_x = sp[0][2..].parse::<N>().ok()?;
            let sensor_y = sp[1][2..].parse::<N>().ok()?;
            let beacon_x = bp[0][2..].parse::<N>().ok()?;
            let beacon_y = bp[1][2..].parse::<N>().ok()?;
            Some((
                Position {
                    x: sensor_x,
                    y: sensor_y,
                },
                Position {
                    x: beacon_x,
                    y: beacon_y,
                },
            ))
        })
        .collect()
}

pub fn to_valves<N, I>(lines: I) -> HashMap<String, (N, Vec<String>)>
where
    N: FromStr,
    I: IntoIterator<Item = String>,
{
    let mut valves = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split("; ").collect();
        let first_parts: Vec<&str> = parts[0].split(' ').collect();
        let (_, rate_str) = first_parts[4].split_at(5);
        let offset = if parts[1].starts_with("tunnels") {
            23
        } else {
            22
        };
        let (_, second_parts) = parts[1].split_at(offset);
        let other_valves: Vec<String> = second_parts.split(", ").map(String::from).collect();
        if let Ok(rate) = rate_str.parse::<N>() {
            valves.insert(first_parts[1].to_string(), (rate, other_valves));
        }
    }
    valves
}
