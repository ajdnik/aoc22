use crate::utils::{
    file,
    vec,
};
use log::{debug, info};

pub fn to_priority_sum(str: String) -> i32 { 
    let mut sum: i32 = 0;
    for char in str.chars() {
        let ascii = char as u8;
        if ascii > 96 {
            sum += i32::from(ascii) - 96;
        } else if ascii < 91 {
            sum += i32::from(ascii) - 38;
        }
    }
    sum
}

pub fn task1(path: &str) {
    if let Ok(backpacks) = file::read_lines(path) {
        let sum = backpacks.fold(0, |mut sum, backpack| {
            if let Ok(itm) = backpack {
                let sz = itm.len();
                let (first, last) = itm.split_at(sz / 2);
                let duplicates = vec::find_duplicate_chars(&vec![first.to_string(), last.to_string()]);
                let priority = to_priority_sum(duplicates);
                sum += priority;
            }
            sum
        });
        info!("The priority sum of all duplicates is {}", sum);
    }
}

pub fn task2(path: &str) {
    if let Ok(backpacks) = file::read_lines(path) {
        let groups = file::to_groups(backpacks, 3);
        debug!("Grouped backpacks into {} groups of 3", groups.len());
        let mut sum = 0;
        for group in groups.iter() {
            let duplicates = vec::find_duplicate_chars(group);
            let priority = to_priority_sum(duplicates);
            sum += priority;
        }
        info!("The priority sum of all the badges is {}", sum);
    }
}
