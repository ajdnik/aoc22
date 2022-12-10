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
