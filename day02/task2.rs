use std::env;
use std::option::Option;
mod utils;

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

fn main() {
    let args: Vec<String> = env::args().collect();
    if let Ok(lines) = utils::read_lines(&args[1]) {
        let scores = utils::to_scores(lines);
        let fixed_total = fixed_match(scores);
        println!("Fixed Score = {}", fixed_total);
    }
}
