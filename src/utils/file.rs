use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
        Result,
        Lines,
    },
    path::Path,
    option::Option,
    str::FromStr,
    ops::Range,
};
use num::Num;
use log::error;

pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn lines_to_numbers<N>(lines: Lines<BufReader<File>>) -> Vec<Option<N>>
where N: FromStr {
    lines.map(|line| {
        match line {
            Err(_) => None,
            Ok(itm) => {
                match itm.parse::<N>() {
                    Err(_) => None,
                    Ok(num) => Some(num),
                }
            },
        }
    }).collect()
}

pub fn to_groups(lines: Lines<BufReader<File>>, size: usize) -> Vec<Vec<String>> {
    let mut groups = Vec::new();
    groups.push(Vec::new());
    lines.fold(groups, |mut groups, line| {
        if let Ok(itm) = line {
            if let Some(last) = groups.last() {
                if last.len() >= size {
                    groups.push(Vec::new());
                }
            }
            if let Some(last) = groups.last_mut() {
                last.push(itm);
            }
        }
        groups
    }) 
}

pub fn to_range_touple<N>(lines: Lines<BufReader<File>>) -> Vec<(Range<N>, Range<N>)>
where N: FromStr + Copy {
    lines.fold(Vec::new(), |mut acc, line| {
        if let Ok(itm) = line {
            let ranges: Vec<&str> = itm.split(",").collect();
            let first_range: Vec<&str> = ranges[0].split("-").collect();
            let mut res: Vec<N> = Vec::new();
            if let Ok(idx) = first_range[0].parse::<N>() {
                res.push(idx);
            }
            if let Ok(idx) = first_range[1].parse::<N>() {
                res.push(idx);
            }
            let second_range: Vec<&str> = ranges[1].split("-").collect();
            if let Ok(idx) = second_range[0].parse::<N>() {
                res.push(idx);
            }
            if let Ok(idx) = second_range[1].parse::<N>() {
                res.push(idx);
            }
            acc.push(((res[0]..res[1]), (res[2]..res[3])));
        }
        acc
    })
}

#[derive(Clone)]
#[derive(PartialEq)]
pub enum FilesystemType {
    Dir,
    File,
}

pub fn parse_filesystem<N>(std_output: Lines<BufReader<File>>) -> Vec<(FilesystemType, String, N)>
where N: FromStr + Num {
    let mut is_ls = false;
    let mut working_directory = String::from("");
    std_output.fold(Vec::<(FilesystemType, String, N)>::new(), |mut filesystem, itm| {
        if let Ok(output_line) = itm {
            if output_line.eq("$ cd ..") {
                is_ls = false;
                let mut path_parts: Vec<&str> = working_directory.split("/").collect();
                path_parts.pop();
                path_parts.pop();
                working_directory = path_parts.join("/");
                working_directory.push('/');
            } else if output_line.eq("$ ls") {
                is_ls = true;
            } else if !output_line.starts_with("$") && is_ls {
                if !output_line.starts_with("dir") {
                    let file_stats: Vec<&str> = output_line.split(" ").collect();
                    if let Ok(file_size) = file_stats[0].parse::<N>() {
                        filesystem.push((FilesystemType::File, working_directory.clone() + file_stats[1], file_size));
                    }
                }
            } else if output_line.starts_with("$ cd") {
                is_ls = false;
                let (_, dir_name) = output_line.split_at(5);
                if working_directory.len() == 0 {
                    working_directory = String::from(dir_name);
                } else {
                    working_directory.push_str(dir_name);
                    working_directory.push('/');
                }
                filesystem.push((FilesystemType::Dir, working_directory.clone(), N::zero()));
            }
        }
        filesystem
    })
}

pub fn to_matrix<N>(lines: Lines<BufReader<File>>) -> Vec<Vec<N>>
where N: FromStr {
    lines.fold(Vec::new(), |mut matrix, itm| {
        if let Ok(line) = itm {
            matrix.push(line.chars().fold(Vec::new(), |mut row, chr| {
                if let Ok(val) = chr.to_string().parse::<N>() {
                    row.push(val);
                }
                row
            }));
        }
        matrix
    })
}

#[derive(Copy)]
#[derive(Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn to_movements(lines: Lines<BufReader<File>>) -> Vec<Direction> {
    lines.fold(Vec::new(), |mut movements, itm| {
        if let Ok(line) = itm {
            let (direction, length) = line.split_at(2);
            if let Ok(val) = length.parse::<usize>() {
                match direction {
                    "U " => {
                        for _ in 0..val {
                            movements.push(Direction::Up);
                        }
                    },
                    "D " => {
                        for _ in 0..val {
                            movements.push(Direction::Down);
                        }
                    },
                    "L " => {
                        for _ in 0..val {
                            movements.push(Direction::Left);
                        }
                    },
                    "R " => {
                        for _ in 0..val {
                            movements.push(Direction::Right);
                        }
                    },
                    &_ => error!("Unknown direction {}", direction), 
                }
            }
        }
        movements
    })
}

pub enum CPUCommand {
    Addx,
    Noop,
}

pub fn to_commands<N>(input: Lines<BufReader<File>>) -> Vec<(CPUCommand, N)>
where N: FromStr + Num {
    input.fold(Vec::new(), |mut commands, itm| {
        if let Ok(line) = itm {
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
    })
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

pub fn to_monkeys<N, T>(input: Lines<BufReader<File>>) -> Vec<Monkey<N,T>>
where N: FromStr + Num, T: FromStr + Num {
    input.fold(Vec::new(), |mut monkeys, itm| {
        if let Ok(line) = itm {
            if line.starts_with("Monkey") {
                monkeys.push(Monkey{items:Vec::new(), op: Operation::Unknown, test_divisible: N::zero(), test_true: T::zero(), test_false: T::zero()});
            } else if line.starts_with("  Starting items:") {
                let (_, nums) = line.split_at(18);
                let items = nums.split(", ");
                let last_monkey = monkeys.last_mut().unwrap();
                for item in items {
                    if let Ok(val) = item.parse::<N>() {
                        last_monkey.items.push(val);
                    }
                }
            } else if line.starts_with("  Operation:") {
                let (_, calc) = line.split_at(19);
                let calc_parts: Vec<&str> = calc.split(" ").collect();
                let last_monkey = monkeys.last_mut().unwrap();
                match calc_parts[1] {
                    "+" => {
                        if let Ok(val) = calc_parts[2].parse::<N>() {
                            last_monkey.op = Operation::Add(val);
                        }
                    },
                    "*" => {
                        if calc_parts[2] == "old" {
                            last_monkey.op = Operation::Pow2;
                        } else if let Ok(val) = calc_parts[2].parse::<N>() {
                            last_monkey.op = Operation::Multiply(val);
                        }
                    },
                    &_ => error!("Unsupported operation {}", calc),
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
    })
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Position<N> {
    pub x: N,
    pub y: N,
}

pub fn to_elevation_map<N>(input: Lines<BufReader<File>>) -> (Vec<Vec<N>>, Position<usize>, Position<usize>)
where N: From<u8> {
    let mut x = 0;
    let mut y = 0;
    input.fold((Vec::new(), Position{x:0, y:0}, Position{x:0, y:0}), |output, itm| {
        let (mut elevation, mut start, mut end) = output;
        if let Ok(line) = itm {
            x = 0;
            elevation.push(line.chars().map(|chr| {
                let mut ascii = chr as u8;
                if chr == 'S' {
                    start.x = x;
                    start.y = y;
                    ascii = 'a' as u8;
                } else if chr == 'E' {
                    end.x = x;
                    end.y = y;
                    ascii = 'z' as u8;
                }
                x += 1;
                ascii.into()
            }).collect());
            y += 1;
        }
        (elevation, start, end)
    })
}

#[derive(Clone, Copy, PartialEq)]
pub enum SignalParts<N> {
    Start,
    End,
    Next,
    Number(N),
}

pub fn to_signals<N>(lines: Lines<BufReader<File>>) -> Vec<Vec<SignalParts<N>>>
where N: FromStr {
    lines.fold(Vec::new(), |mut signals, itm| {
        if let Ok(line) = itm {
            if line.starts_with("[") {
                let mut num_buffer = String::from("");
                signals.push(line.chars().fold(Vec::new(), |mut signal, chr| {
                    match chr {
                        '[' => signal.push(SignalParts::Start),
                        ']' => {
                            if num_buffer.len() > 0 {
                                if let Ok(val) = num_buffer.parse::<N>() {
                                    signal.push(SignalParts::Number(val));
                                }
                                num_buffer = String::from("");
                            }
                            signal.push(SignalParts::End);
                        },
                        ',' => {
                            if num_buffer.len() > 0 {
                                if let Ok(val) = num_buffer.parse::<N>() {
                                    signal.push(SignalParts::Number(val));
                                }
                                num_buffer = String::from("");
                            }
                            signal.push(SignalParts::Next);
                        },
                        c => num_buffer.push(c),
                    }
                    signal
                }));
            }
        }
        signals
    })
}

pub fn to_walls<N>(lines: Lines<BufReader<File>>) -> Vec<Vec<Position<N>>>
where N: FromStr {
    lines.fold(Vec::new(), |mut walls, itm| {
        if let Ok(line) = itm {
            walls.push(line.split(" -> ").fold(Vec::new(), |mut points, point| {
                let dim: Vec<&str> = point.split(",").collect();
                if let (Ok(x), Ok(y)) = (dim[0].parse::<N>(), dim[1].parse::<N>()) {
                    points.push(Position{x,y});
                }
                points
            }));
        }
        walls
    })
}
