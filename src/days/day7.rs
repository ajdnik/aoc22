use crate::utils::file;
use anyhow::Result;
use num::Num;
use std::ops::AddAssign;

fn directory_size<N>(working_directory: &str, filesystem: &[(file::FilesystemType, String, N)]) -> N
where
    N: AddAssign + Num + Copy,
{
    filesystem
        .iter()
        .fold(N::zero(), |mut sum, (item_type, path, size)| {
            if path.starts_with(working_directory) && path != working_directory {
                if let file::FilesystemType::File = item_type {
                    sum += *size;
                }
            }
            sum
        })
}

pub fn part1(input: &str) -> Result<String> {
    let filesystem = file::parse_filesystem::<u32, _>(file::lines_of(input));
    let threshold = 100000;
    let sum: u32 = filesystem
        .iter()
        .filter_map(|(item_type, path, _)| {
            if let file::FilesystemType::Dir = item_type {
                let size = directory_size(path, &filesystem);
                if size <= threshold {
                    return Some(size);
                }
            }
            None
        })
        .sum();
    Ok(format!(
        "The size sum of all directories whose size is under {} is {}",
        threshold, sum
    ))
}

pub fn part2(input: &str) -> Result<String> {
    let filesystem = file::parse_filesystem::<u32, _>(file::lines_of(input));
    let used_space = directory_size(&filesystem[0].1, &filesystem);
    let free_space = 70000000 - used_space;
    let need_to_free = 30000000 - free_space;
    let freed_up = filesystem
        .iter()
        .fold(used_space, |mut min_size, (item_type, path, _)| {
            if let file::FilesystemType::Dir = item_type {
                let size = directory_size(path, &filesystem);
                if size >= need_to_free && min_size > size {
                    min_size = size;
                }
            }
            min_size
        });
    Ok(format!("Freed up {} to prepare for update", freed_up))
}
