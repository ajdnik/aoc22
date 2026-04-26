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
