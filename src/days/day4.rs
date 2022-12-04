use crate::utils::file;
use log::{debug, info};

pub fn task1(path: &str) {
    if let Ok(lines) = file::read_lines(path) {
        let pairs = file::to_range_touple::<i32>(lines);
        debug!("Found {} pairs", pairs.len());
        let mut count = 0;
        for pair in pairs.iter() {
            let (first, second) = pair;
            if first.start <= second.start && first.end >= second.end {
                count += 1;
            } else if second.start <= first.start && second.end >= first.end {
                count += 1;
            }
        }
        info!("Found {} matching pairs", count);
    }
}

pub fn task2(path: &str) {
    if let Ok(lines) = file::read_lines(path) {
        let pairs = file::to_range_touple::<i32>(lines);
        debug!("Found {} pairs", pairs.len());
        let mut count = 0;
        for pair in pairs.iter() {
            let (first, second) = pair;
            if first.start <= second.end && second.start <= first.end {
                count += 1;
            }
        }
        info!("Found {} overlapping pairs", count);
    } 
}
