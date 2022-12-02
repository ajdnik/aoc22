use std::env;
use std::option::Option;
mod utils;


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

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Ok(lines) = utils::read_lines(&args[1]) {
        let scores = utils::to_scores(lines);
        let total = total_score(scores);
        println!("Total Score = {}", total);
    }
}
