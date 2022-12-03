use std::env;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Ok(backpacks) = utils::read_lines(&args[1]) {
        let sum = backpacks.fold(0, |mut sum, backpack| {
            if let Ok(itm) = backpack {
                let sz = itm.len();
                let (first, last) = itm.split_at(sz / 2);
                let duplicates = utils::find_duplicates(&vec![first.to_string(), last.to_string()]);
                let priority = utils::to_priority_sum(duplicates);
                sum += priority;
            }
            sum
        });
        println!("Total Sum = {}", sum);
    }
}
