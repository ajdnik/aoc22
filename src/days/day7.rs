use crate::utils::file;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::ops::AddAssign;

fn directory_sizes<N>(filesystem: &[(file::FilesystemType, String, N)]) -> HashMap<String, N>
where
    N: AddAssign + Default + Copy,
{
    let mut sizes: HashMap<String, N> = filesystem
        .iter()
        .filter(|(kind, _, _)| matches!(kind, file::FilesystemType::Dir))
        .map(|(_, path, _)| (path.clone(), N::default()))
        .collect();
    for (kind, path, size) in filesystem {
        if !matches!(kind, file::FilesystemType::File) {
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
    let filesystem = file::parse_filesystem::<u32, _>(file::lines_of(input))?;
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
    let filesystem = file::parse_filesystem::<u32, _>(file::lines_of(input))?;
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
