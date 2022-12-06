use crate::utils::{
    file,
    vec,
};
use log::{debug, info};

pub fn task1(path: &str) {
    if let Ok(buffers) = file::read_lines(path) {
        for (idx, itm) in buffers.enumerate() {
            if let Ok(buffer) = itm {
                debug!("Processing buffer at position {} with size {}", idx, buffer.len());
                let start = vec::find_first_distinct_substring(buffer, 4);
                info!("The first distinct 4 character substring for buffer at position {} starts at {}", idx, start);
            }
        }
    }
}

pub fn task2(path: &str) {
    if let Ok(buffers) = file::read_lines(path) {
        for (idx, itm) in buffers.enumerate() {
            if let Ok(buffer) = itm {
                debug!("Processing buffer at position {} with size {}", idx, buffer.len());
                let start = vec::find_first_distinct_substring(buffer, 14);
                info!("The first distinct 14 character substring for buffer at position {} starts at {}", idx, start);
            }
        }
    }
}
