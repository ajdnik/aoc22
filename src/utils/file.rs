use anyhow::{anyhow, bail, Context, Result};
use num::Num;
use std::{collections::HashMap, ops::Range, str::FromStr};

pub fn lines_of(input: &str) -> impl Iterator<Item = String> + '_ {
    input.lines().map(String::from)
}

/// Splits input on blank lines, parsing each non-blank line as `N`.
pub fn to_number_groups<N, I>(lines: I) -> Result<Vec<Vec<N>>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    let mut groups: Vec<Vec<N>> = vec![Vec::new()];
    for line in lines {
        if line.is_empty() {
            groups.push(Vec::new());
            continue;
        }
        let val: N = line
            .parse()
            .with_context(|| format!("parsing number {:?}", line))?;
        groups.last_mut().unwrap().push(val);
    }
    Ok(groups)
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

pub fn to_range_touple<N, I>(lines: I) -> Result<Vec<(Range<N>, Range<N>)>>
where
    N: FromStr + Copy,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let ranges: Vec<&str> = line.split(',').collect();
            if ranges.len() != 2 {
                bail!("expected 2 ranges in line {:?}", line);
            }
            let first: Vec<&str> = ranges[0].split('-').collect();
            let second: Vec<&str> = ranges[1].split('-').collect();
            if first.len() != 2 || second.len() != 2 {
                bail!("malformed range in line {:?}", line);
            }
            let a: N = first[0]
                .parse()
                .with_context(|| format!("parsing {:?}", first[0]))?;
            let b: N = first[1]
                .parse()
                .with_context(|| format!("parsing {:?}", first[1]))?;
            let c: N = second[0]
                .parse()
                .with_context(|| format!("parsing {:?}", second[0]))?;
            let d: N = second[1]
                .parse()
                .with_context(|| format!("parsing {:?}", second[1]))?;
            Ok((a..b, c..d))
        })
        .collect()
}

#[derive(Clone, PartialEq)]
pub enum FilesystemType {
    Dir,
    File,
}

pub fn parse_filesystem<N, I>(std_output: I) -> Result<Vec<(FilesystemType, String, N)>>
where
    N: FromStr + Num,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
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
                if stats.len() < 2 {
                    bail!("malformed ls entry {:?}", line);
                }
                let size: N = stats[0]
                    .parse()
                    .with_context(|| format!("parsing file size {:?}", stats[0]))?;
                filesystem.push((
                    FilesystemType::File,
                    working_directory.clone() + stats[1],
                    size,
                ));
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
    Ok(filesystem)
}

pub fn to_matrix<N, I>(lines: I) -> Result<Vec<Vec<N>>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.chars()
                .map(|chr| {
                    let s = chr.to_string();
                    s.parse::<N>()
                        .with_context(|| format!("parsing digit {:?}", s))
                })
                .collect::<Result<Vec<_>>>()
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

pub fn to_movements<I>(lines: I) -> Result<Vec<Direction>>
where
    I: IntoIterator<Item = String>,
{
    let mut movements = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.len() < 3 {
            bail!("malformed movement line {:?}", line);
        }
        let (direction, length) = line.split_at(2);
        let val: usize = length
            .parse()
            .with_context(|| format!("parsing movement length {:?}", length))?;
        let dir = match direction {
            "U " => Direction::Up,
            "D " => Direction::Down,
            "L " => Direction::Left,
            "R " => Direction::Right,
            _ => bail!("unknown direction {:?}", direction),
        };
        for _ in 0..val {
            movements.push(dir);
        }
    }
    Ok(movements)
}

pub enum CPUCommand {
    Addx,
    Noop,
}

pub fn to_commands<N, I>(input: I) -> Result<Vec<(CPUCommand, N)>>
where
    N: FromStr + Num,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    let mut commands = Vec::new();
    for line in input {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("noop") {
            commands.push((CPUCommand::Noop, N::zero()));
        } else if line.starts_with("addx ") {
            let (_, num) = line.split_at(5);
            let val: N = num
                .parse()
                .with_context(|| format!("parsing addx operand {:?}", num))?;
            commands.push((CPUCommand::Addx, val));
        } else {
            bail!("unknown CPU command {:?}", line);
        }
    }
    Ok(commands)
}

pub enum Operation<N> {
    Add(N),
    Multiply(N),
    Pow2,
}

pub struct Monkey<N, T> {
    pub items: Vec<N>,
    pub op: Operation<N>,
    pub test_divisible: N,
    pub test_true: T,
    pub test_false: T,
}

pub fn to_monkeys<N, T, I>(input: I) -> Result<Vec<Monkey<N, T>>>
where
    N: FromStr + Num,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    T: FromStr + Num,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    struct Builder<N, T> {
        items: Vec<N>,
        op: Option<Operation<N>>,
        test_divisible: Option<N>,
        test_true: Option<T>,
        test_false: Option<T>,
    }
    impl<N, T> Builder<N, T> {
        fn new() -> Self {
            Self {
                items: Vec::new(),
                op: None,
                test_divisible: None,
                test_true: None,
                test_false: None,
            }
        }
        fn finish(self, idx: usize) -> Result<Monkey<N, T>> {
            Ok(Monkey {
                items: self.items,
                op: self
                    .op
                    .ok_or_else(|| anyhow!("monkey {} missing op", idx))?,
                test_divisible: self
                    .test_divisible
                    .ok_or_else(|| anyhow!("monkey {} missing test divisor", idx))?,
                test_true: self
                    .test_true
                    .ok_or_else(|| anyhow!("monkey {} missing test_true", idx))?,
                test_false: self
                    .test_false
                    .ok_or_else(|| anyhow!("monkey {} missing test_false", idx))?,
            })
        }
    }

    let mut monkeys: Vec<Monkey<N, T>> = Vec::new();
    let mut current: Option<Builder<N, T>> = None;
    for line in input {
        if line.starts_with("Monkey") {
            if let Some(b) = current.take() {
                monkeys.push(b.finish(monkeys.len())?);
            }
            current = Some(Builder::new());
        } else if let Some(stripped) = line.strip_prefix("  Starting items: ") {
            let b = current
                .as_mut()
                .context("'Starting items' before Monkey header")?;
            for item in stripped.split(", ") {
                b.items.push(
                    item.parse()
                        .with_context(|| format!("parsing monkey item {:?}", item))?,
                );
            }
        } else if let Some(stripped) = line.strip_prefix("  Operation: new = old ") {
            let b = current
                .as_mut()
                .context("'Operation' before Monkey header")?;
            let parts: Vec<&str> = stripped.split(' ').collect();
            if parts.len() != 2 {
                bail!("malformed operation {:?}", stripped);
            }
            b.op = Some(match (parts[0], parts[1]) {
                ("+", val) => Operation::Add(
                    val.parse()
                        .with_context(|| format!("parsing operand {:?}", val))?,
                ),
                ("*", "old") => Operation::Pow2,
                ("*", val) => Operation::Multiply(
                    val.parse()
                        .with_context(|| format!("parsing operand {:?}", val))?,
                ),
                _ => bail!("unsupported operation {:?}", stripped),
            });
        } else if let Some(stripped) = line.strip_prefix("  Test: divisible by ") {
            let b = current.as_mut().context("'Test' before Monkey header")?;
            b.test_divisible = Some(
                stripped
                    .parse()
                    .with_context(|| format!("parsing test divisor {:?}", stripped))?,
            );
        } else if let Some(stripped) = line.strip_prefix("    If true: throw to monkey ") {
            let b = current.as_mut().context("'If true' before Monkey header")?;
            b.test_true = Some(
                stripped
                    .parse()
                    .with_context(|| format!("parsing test_true {:?}", stripped))?,
            );
        } else if let Some(stripped) = line.strip_prefix("    If false: throw to monkey ") {
            let b = current
                .as_mut()
                .context("'If false' before Monkey header")?;
            b.test_false = Some(
                stripped
                    .parse()
                    .with_context(|| format!("parsing test_false {:?}", stripped))?,
            );
        }
    }
    if let Some(b) = current.take() {
        monkeys.push(b.finish(monkeys.len())?);
    }
    Ok(monkeys)
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

#[derive(Clone, PartialEq, Eq)]
pub enum Signal<N> {
    Number(N),
    List(Vec<Signal<N>>),
}

impl<N: Ord> Ord for Signal<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Signal::Number(a), Signal::Number(b)) => a.cmp(b),
            (Signal::List(a), Signal::List(b)) => a.cmp(b),
            (Signal::Number(_), Signal::List(b)) => std::slice::from_ref(self).cmp(b.as_slice()),
            (Signal::List(a), Signal::Number(_)) => a.as_slice().cmp(std::slice::from_ref(other)),
        }
    }
}

impl<N: Ord> PartialOrd for Signal<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_signal<N>(line: &str) -> Result<Signal<N>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    let mut stack: Vec<Vec<Signal<N>>> = Vec::new();
    let mut num_buffer = String::new();
    let mut completed: Option<Signal<N>> = None;
    for chr in line.chars() {
        match chr {
            '[' => stack.push(Vec::new()),
            ']' => {
                if !num_buffer.is_empty() {
                    let val: N = num_buffer
                        .parse()
                        .with_context(|| format!("parsing signal number {:?}", num_buffer))?;
                    stack
                        .last_mut()
                        .context("number outside any list")?
                        .push(Signal::Number(val));
                    num_buffer.clear();
                }
                let finished = stack.pop().context("unmatched ']' in signal")?;
                let list = Signal::List(finished);
                match stack.last_mut() {
                    Some(parent) => parent.push(list),
                    None => completed = Some(list),
                }
            }
            ',' => {
                if !num_buffer.is_empty() {
                    let val: N = num_buffer
                        .parse()
                        .with_context(|| format!("parsing signal number {:?}", num_buffer))?;
                    stack
                        .last_mut()
                        .context("number outside any list")?
                        .push(Signal::Number(val));
                    num_buffer.clear();
                }
            }
            c => num_buffer.push(c),
        }
    }
    completed.with_context(|| format!("unclosed signal: {:?}", line))
}

pub fn to_signals<N, I>(lines: I) -> Result<Vec<Signal<N>>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    let mut signals = Vec::new();
    for line in lines {
        if !line.starts_with('[') {
            continue;
        }
        signals.push(parse_signal::<N>(&line)?);
    }
    Ok(signals)
}

pub fn to_walls<N, I>(lines: I) -> Result<Vec<Vec<Position<N>>>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let dim: Vec<&str> = point.split(',').collect();
                    if dim.len() != 2 {
                        bail!("malformed point {:?}", point);
                    }
                    let x: N = dim[0]
                        .parse()
                        .with_context(|| format!("parsing x {:?}", dim[0]))?;
                    let y: N = dim[1]
                        .parse()
                        .with_context(|| format!("parsing y {:?}", dim[1]))?;
                    Ok(Position { x, y })
                })
                .collect::<Result<Vec<_>>>()
        })
        .collect()
}

pub fn to_sensor_data<N, I>(lines: I) -> Result<Vec<(Position<N>, Position<N>)>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 2 {
                bail!("malformed sensor line {:?}", line);
            }
            if parts[0].len() < 10 || parts[1].len() < 22 {
                bail!("sensor line too short {:?}", line);
            }
            let (_, sensor_loc) = parts[0].split_at(10);
            let (_, beacon_loc) = parts[1].split_at(22);
            let sp: Vec<&str> = sensor_loc.split(", ").collect();
            let bp: Vec<&str> = beacon_loc.split(", ").collect();
            if sp.len() != 2 || bp.len() != 2 {
                bail!("malformed sensor coordinates {:?}", line);
            }
            let sensor_x: N = sp[0][2..]
                .parse()
                .with_context(|| format!("sensor x {:?}", &sp[0][2..]))?;
            let sensor_y: N = sp[1][2..]
                .parse()
                .with_context(|| format!("sensor y {:?}", &sp[1][2..]))?;
            let beacon_x: N = bp[0][2..]
                .parse()
                .with_context(|| format!("beacon x {:?}", &bp[0][2..]))?;
            let beacon_y: N = bp[1][2..]
                .parse()
                .with_context(|| format!("beacon y {:?}", &bp[1][2..]))?;
            Ok((
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
        let parts: Vec<&str> = line.split("; ").collect();
        if parts.len() != 2 {
            bail!("malformed valve line {:?}", line);
        }
        let first_parts: Vec<&str> = parts[0].split(' ').collect();
        if first_parts.len() < 5 || first_parts[4].len() < 5 {
            bail!("malformed valve header {:?}", line);
        }
        let (_, rate_str) = first_parts[4].split_at(5);
        let offset = if parts[1].starts_with("tunnels") {
            23
        } else {
            22
        };
        if parts[1].len() < offset {
            bail!("malformed valve tunnels {:?}", line);
        }
        let (_, second_parts) = parts[1].split_at(offset);
        let other_valves: Vec<String> = second_parts.split(", ").map(String::from).collect();
        let rate: N = rate_str
            .parse()
            .with_context(|| format!("parsing valve rate {:?}", rate_str))?;
        valves.insert(first_parts[1].to_string(), (rate, other_valves));
    }
    Ok(valves)
}
