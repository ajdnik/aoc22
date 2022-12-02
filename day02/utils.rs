use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn to_scores(lines: io::Lines<io::BufReader<File>>) -> Vec<Option<Vec<i32>>> { 
    lines.map(|line| {
        match line {
            Err(_) => None,
            Ok(itm) => {
                let mut scores = vec![0, 0];
                match itm.chars().nth(0) {
                    Some('A') => scores[0] = 1,
                    Some('B') => scores[0] = 2,
                    Some('C') => scores[0] = 3,
                    _ => (),
                }
                match itm.chars().nth(2) {
                    Some('X') => scores[1] = 1,
                    Some('Y') => scores[1] = 2,
                    Some('Z') => scores[1] = 3,
                    _ => (),
                }
                Some(scores)
            },
        }
    }).collect()
}
