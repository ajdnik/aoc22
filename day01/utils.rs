use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::option::Option;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn lines_to_numbers(lines: io::Lines<io::BufReader<File>>) -> Vec<Option<i32>> {
    lines.map(|line| {
        match line {
            Err(_) => None,
            Ok(itm) => {
                match itm.parse::<i32>() {
                    Err(_) => None,
                    Ok(num) => Some(num),
                }
            },
        }
    }).collect()
}

pub fn sum_continuous_numbers(numbers: Vec<Option<i32>>) -> Vec<i32> {
   return numbers.iter().fold(vec![0], |mut acc, itm| {
        match itm {
            None => {
                acc.push(0);
                acc
            },
            Some(val) => {
                let sz = acc.len();
                acc[sz - 1] += val;
                acc
            },
        }
   });
}
