use crate::utils::{file, vec};
use anyhow::Result;

fn to_priority_sum(s: &str) -> i32 {
    let mut sum: i32 = 0;
    for char in s.chars() {
        let ascii = char as u8;
        if ascii > 96 {
            sum += i32::from(ascii) - 96;
        } else if ascii < 91 {
            sum += i32::from(ascii) - 38;
        }
    }
    sum
}

pub fn part1(input: &str) -> Result<String> {
    let mut sum = 0;
    for line in file::lines_of(input) {
        let sz = line.len();
        let (first, last) = line.split_at(sz / 2);
        let duplicates = vec::find_duplicate_chars(&[first.to_string(), last.to_string()]);
        sum += to_priority_sum(&duplicates);
    }
    Ok(format!("The priority sum of all duplicates is {sum}"))
}

pub fn part2(input: &str) -> Result<String> {
    let groups = file::to_groups(file::lines_of(input), 3);
    let mut sum = 0;
    for group in &groups {
        let duplicates = vec::find_duplicate_chars(group);
        sum += to_priority_sum(&duplicates);
    }
    Ok(format!("The priority sum of all the badges is {sum}"))
}
