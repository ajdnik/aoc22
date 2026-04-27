use crate::utils::file;
use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Clone, PartialEq)]
enum FilesystemType {
    Dir,
    File,
}

fn parse_filesystem<N, I>(std_output: I) -> Result<Vec<(FilesystemType, String, N)>>
where
    N: FromStr + Default,
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
                    bail!("malformed ls entry {line:?}");
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
            filesystem.push((FilesystemType::Dir, working_directory.clone(), N::default()));
        }
    }
    Ok(filesystem)
}

fn directory_sizes<N>(filesystem: &[(FilesystemType, String, N)]) -> HashMap<String, N>
where
    N: AddAssign + Default + Copy,
{
    let mut sizes: HashMap<String, N> = filesystem
        .iter()
        .filter(|(kind, _, _)| matches!(kind, FilesystemType::Dir))
        .map(|(_, path, _)| (path.clone(), N::default()))
        .collect();
    for (kind, path, size) in filesystem {
        if !matches!(kind, FilesystemType::File) {
            continue;
        }
        for (idx, _) in path.match_indices('/') {
            let prefix = &path[..=idx];
            sizes
                .entry(prefix.to_string())
                .and_modify(|s| *s += *size)
                .or_insert(*size);
        }
    }
    sizes
}

pub fn part1(input: &str) -> Result<String> {
    let filesystem = parse_filesystem::<u32, _>(file::lines_of(input))?;
    let threshold = 100_000;
    let sum: u32 = directory_sizes(&filesystem)
        .values()
        .filter(|&&s| s <= threshold)
        .sum();
    Ok(format!(
        "The size sum of all directories whose size is under {threshold} is {sum}"
    ))
}

pub fn part2(input: &str) -> Result<String> {
    let filesystem = parse_filesystem::<u32, _>(file::lines_of(input))?;
    let sizes = directory_sizes(&filesystem);
    let root = &filesystem.first().context("filesystem is empty")?.1;
    let used = *sizes.get(root).context("root size not computed")?;
    let need_to_free = 30_000_000 - (70_000_000 - used);
    let freed_up = sizes
        .values()
        .copied()
        .filter(|&s| s >= need_to_free)
        .min()
        .context("no directory large enough to free required space")?;
    Ok(format!("Freed up {freed_up} to prepare for update"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_cd_ls_dir_file() {
        let lines = [
            "$ cd /",
            "$ ls",
            "dir a",
            "100 b.txt",
            "$ cd a",
            "$ ls",
            "50 c.txt",
            "$ cd ..",
            "$ ls",
        ]
        .map(String::from);
        let fs = parse_filesystem::<u32, _>(lines).unwrap();
        let dirs: Vec<_> = fs
            .iter()
            .filter(|(k, _, _)| matches!(k, FilesystemType::Dir))
            .map(|(_, p, _)| p.clone())
            .collect();
        assert_eq!(dirs, vec!["/".to_string(), "/a/".to_string()]);
        let files: Vec<_> = fs
            .iter()
            .filter(|(k, _, _)| matches!(k, FilesystemType::File))
            .map(|(_, p, s)| (p.clone(), *s))
            .collect();
        assert_eq!(
            files,
            vec![("/b.txt".to_string(), 100), ("/a/c.txt".to_string(), 50)]
        );
    }
}
