use crate::utils::file;
use anyhow::{bail, Context, Result};
use std::fmt::Write;
use std::str::FromStr;

enum CPUCommand {
    Addx,
    Noop,
}

fn to_commands<N, I>(input: I) -> Result<Vec<(CPUCommand, N)>>
where
    N: FromStr + Default,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    let mut commands = Vec::new();
    for line in input {
        if line.is_empty() {
            continue;
        }
        if line == "noop" {
            commands.push((CPUCommand::Noop, N::default()));
        } else if let Some(num) = line.strip_prefix("addx ") {
            let val: N = num
                .parse()
                .with_context(|| format!("parsing addx operand {num:?}"))?;
            commands.push((CPUCommand::Addx, val));
        } else {
            bail!("unknown CPU command {line:?}");
        }
    }
    Ok(commands)
}

fn simulate_cpu(commands: &[(CPUCommand, i32)]) -> Vec<i32> {
    let mut register = 1;
    commands
        .iter()
        .fold(Vec::new(), |mut cycles, (command, val)| {
            match command {
                CPUCommand::Noop => cycles.push(register),
                CPUCommand::Addx => {
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
    let commands = to_commands::<i32, _>(file::lines_of(input))?;
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
    let commands = to_commands::<i32, _>(file::lines_of(input))?;
    let cycles = simulate_cpu(&commands);
    let crt = to_crt(&cycles);
    let mut out = String::from("CRT Output:");
    for line in &crt {
        write!(out, "\n{line}").unwrap();
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_noop_and_addx() {
        let lines = ["noop", "addx 3", "addx -5"].map(String::from);
        let cmds = to_commands::<i32, _>(lines).unwrap();
        assert_eq!(cmds.len(), 3);
        assert!(matches!(cmds[0], (CPUCommand::Noop, 0)));
        assert!(matches!(cmds[1], (CPUCommand::Addx, 3)));
        assert!(matches!(cmds[2], (CPUCommand::Addx, -5)));
    }

    #[test]
    fn unknown_command_errors() {
        let lines = ["jump 5"].map(String::from);
        assert!(to_commands::<i32, _>(lines).is_err());
    }
}
