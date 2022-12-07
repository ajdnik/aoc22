use std::ops::AddAssign;
use crate::utils::file;
use log::{debug, info};
use num::Num;

fn directory_size<N>(working_directory: &String, filesystem: &Vec<(file::FilesystemType, String, N)>) -> N
where N: Clone + AddAssign + Num + Copy {
    filesystem.to_vec().iter().fold(N::zero(), |mut sum, filesystem_item| {
        let (item_type, path, size) = filesystem_item;
        if path.starts_with(working_directory) && !path.eq(working_directory) {
            if let file::FilesystemType::File = item_type {
                sum += *size;
            }
        }
        sum
    })
}

pub fn task1(path: &str) {
    if let Ok(terminal_output) = file::read_lines(path) {
        let filesystem = file::parse_filesystem::<u32>(terminal_output);
        debug!("Found {} files and directories in the output", filesystem.len());
        let copy = filesystem.to_vec();
        let threshold = 100000;
        let sum_under_threshold = filesystem.iter().fold(0, |mut sum, filesystem_item| {
            let (item_type, path, _) = filesystem_item;
            if let file::FilesystemType::Dir = item_type {
                let size = directory_size(&path, &copy);
                if size <= threshold {
                    sum += size;
                }
            }
            sum
        });
        info!("The size sum of all directories whose size is under {} is {}", threshold, sum_under_threshold);
    }
}

pub fn task2(path: &str) {
    if let Ok(terminal_output) = file::read_lines(path) {
        let filesystem = file::parse_filesystem::<u32>(terminal_output);
        debug!("Found {} files and directories in the output", filesystem.len());
        let copy = filesystem.to_vec();
        let used_space = directory_size(&filesystem[0].1, &copy);
        debug!("The filesystem uses up {}", used_space);
        let free_space = 70000000 - used_space;
        debug!("The filesystem has {} free space", free_space);
        let free_space_needed = 30000000;
        let need_to_free = free_space_needed - free_space;
        debug!("In order to update we need {} free space, so we need to free  up {}", free_space_needed, need_to_free);
        let freed_up_size = filesystem.iter().fold(used_space, |mut min_size, filesystem_item| {
            let (item_type, path, _) = filesystem_item;
            if let file::FilesystemType::Dir = item_type {
                let size = directory_size(&path, &copy);
                if size >= need_to_free {
                    if min_size > size {
                        min_size = size;
                    }
                }
            }
            min_size
        });
        info!("Freed up {} to prepare for update", freed_up_size);
    }
}
