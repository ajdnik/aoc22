use std::env;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Ok(backpacks) = utils::read_lines(&args[1]) {
        let groups = utils::to_groups(backpacks, 3);
        let mut sum = 0;
        for group in groups.iter() {
            let duplicates = utils::find_duplicates(group);
            let priority = utils::to_priority_sum(duplicates);
            sum += priority;
        }
        println!("Total Grouped Sum = {}", sum);
    }
}
