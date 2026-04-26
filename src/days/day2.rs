use crate::utils::file;
use anyhow::Result;

fn to_scores<I>(lines: I) -> Vec<Vec<i32>>
where
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .map(|itm| {
            let mut scores = vec![0, 0];
            match itm.chars().next() {
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
            scores
        })
        .collect()
}

fn total_score(scores: &[Vec<i32>]) -> i32 {
    let mut total = 0;
    for itm in scores {
        total += itm[1];
        if itm[0] == itm[1] {
            total += 3;
        } else if itm[0] + 1 == itm[1] || (itm[0] == 3 && itm[1] == 1) {
            total += 6;
        }
    }
    total
}

fn fixed_match(scores: &[Vec<i32>]) -> i32 {
    let mut total = 0;
    for itm in scores {
        match itm[1] {
            1 => {
                let mut a = itm[0] - 1;
                if a == 0 {
                    a = 3;
                }
                total += a;
            }
            2 => {
                total += itm[0] + 3;
            }
            3 => {
                let mut a = itm[0] + 1;
                if a == 4 {
                    a = 1;
                }
                total += a + 6;
            }
            _ => (),
        }
    }
    total
}

pub fn part1(input: &str) -> Result<String> {
    let scores = to_scores(file::lines_of(input));
    let total = total_score(&scores);
    Ok(format!("Total score is {total}"))
}

pub fn part2(input: &str) -> Result<String> {
    let scores = to_scores(file::lines_of(input));
    let fixed_total = fixed_match(&scores);
    Ok(format!("Total fixed score is {fixed_total}"))
}
