use crate::utils::file;
use anyhow::Result;
use std::fmt::Write;

fn simulate_cpu(commands: &[(file::CPUCommand, i32)]) -> Vec<i32> {
    let mut register = 1;
    commands
        .iter()
        .fold(Vec::new(), |mut cycles, (command, val)| {
            match command {
                file::CPUCommand::Noop => cycles.push(register),
                file::CPUCommand::Addx => {
                    cycles.push(register);
                    cycles.push(register);
                    register += val;
                }
            }
            cycles
        })
}

fn to_crt(cycles: &[i32]) -> Vec<String> {
    let mut cycle = 1;
    let mut cursor = 0;
    cycles.iter().fold(Vec::new(), |mut crt, register| {
        if cycle % 40 == 1 {
            crt.push(String::new())
        }
        let mut row = String::from(crt.pop().unwrap().as_str());
        let sprite = [*register - 1, *register, *register + 1];
        if sprite[0] == cursor || sprite[1] == cursor || sprite[2] == cursor {
            row.push('#');
        } else {
            row.push('.');
        }
        crt.push(row);
        cycle += 1;
        cursor += 1;
        if cursor == 40 {
            cursor = 0;
        }
        crt
    })
}

pub fn part1(input: &str) -> Result<String> {
    let commands = file::to_commands::<i32, _>(file::lines_of(input))?;
    let cycles = simulate_cpu(&commands);
    let total = cycles[19] * 20
        + cycles[59] * 60
        + cycles[99] * 100
        + cycles[139] * 140
        + cycles[179] * 180
        + cycles[219] * 220;
    Ok(format!("Sum of cycles {total}"))
}

pub fn part2(input: &str) -> Result<String> {
    let commands = file::to_commands::<i32, _>(file::lines_of(input))?;
    let cycles = simulate_cpu(&commands);
    let crt = to_crt(&cycles);
    let mut out = String::from("CRT Output:");
    for line in &crt {
        write!(out, "\n{line}").unwrap();
    }
    Ok(out)
}
