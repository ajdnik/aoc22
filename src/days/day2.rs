use crate::utils::file;
use std::{
    fs::File,
    io::{
        Lines,
        BufReader,
    },
};
use log::{debug, info};

pub fn to_scores(lines: Lines<BufReader<File>>) -> Vec<Option<Vec<i32>>> { 
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

fn total_score(scores: Vec<Option<Vec<i32>>>) -> i32 {
    let mut total_score = 0;
    for score in scores.iter() {
        if let Some(itm) = score {
            total_score += itm[1];
            if itm[0] == itm[1] {
                total_score += 3;
            } else if itm[0] + 1 == itm[1] {
                total_score += 6;
            } else if itm[0] == 3 && itm[1] == 1 {
                total_score += 6;
            }
        }
    }
    total_score
}

fn fixed_match(scores: Vec<Option<Vec<i32>>>) -> i32 {
    let mut total_score = 0;
    for score in scores.iter() {
        if let Some(itm) = score {
            match itm[1] {
                1 => {
                    let mut a = itm[0] - 1;
                    if a == 0 {
                        a = 3;
                    }
                    total_score += a;
                },
                2 => {
                    total_score += itm[0] + 3;
                },
                3 => {
                    let mut a = itm[0] + 1;
                    if a == 4 {
                        a = 1;
                    }
                    total_score += a + 6;
                },
                _ => (),
            }
        }
    }
    total_score
}

pub fn task1(path: &str) {
    if let Ok(rounds) = file::read_lines(path) {
        let scores = to_scores(rounds);
        debug!("Found strategies for {} rounds", scores.len());
        let total = total_score(scores);
        info!("Total score is {}", total);
    }
}

pub fn task2(path: &str) {
    if let Ok(rounds) = file::read_lines(path) {
        let scores = to_scores(rounds);
        debug!("Found strategies for {} rounds", scores.len());
        let fixed_total = fixed_match(scores);
        info!("Total fixed score is {}", fixed_total);
    }

}
