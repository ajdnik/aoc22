use anyhow::{bail, Context, Result};
use num::Num;
use std::str::FromStr;

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
        } else if let Some(dir_name) = line.strip_prefix("$ cd ") {
            is_ls = false;
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
