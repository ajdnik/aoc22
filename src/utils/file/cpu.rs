use anyhow::{bail, Context, Result};
use num::Num;
use std::str::FromStr;

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
        if line == "noop" {
            commands.push((CPUCommand::Noop, N::zero()));
        } else if let Some(num) = line.strip_prefix("addx ") {
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
