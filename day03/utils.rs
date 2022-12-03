use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn to_groups(lines: io::Lines<io::BufReader<File>>, size: usize) -> Vec<Vec<String>> {
    let mut groups = Vec::new();
    groups.push(Vec::new());
    lines.fold(groups, |mut groups, line| {
        if let Ok(itm) = line {
            if let Some(last) = groups.last() {
                if last.len() >= size {
                    groups.push(Vec::new());
                }
            }
            if let Some(last) = groups.last_mut() {
                last.push(itm);
            }
        }
        groups
    }) 
}

pub fn find_duplicates(strings: &Vec<String>) -> String { 
    let mut duplicates = String::from("");
    if let Some((first, rest)) = strings.split_first() {
        for char in first.chars() {
            let mut count_found = 0;
            for itm in rest {
                if let Some(_) = itm.find(char) {
                    count_found += 1;
                }
            }
            if count_found == rest.len() {
                if let None = duplicates.find(char) {
                    duplicates += &char.to_string();
                }
            }
        }
    }
    duplicates
}

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
