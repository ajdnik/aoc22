use std::env;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Ok(lines) = utils::read_lines(&args[1]) {
       let numbers = utils::lines_to_numbers(lines);
       let mut counts = utils::sum_continuous_numbers(numbers);
       
       counts.sort_by(|a, b| b.cmp(a));
       let total = counts[0] + counts[1] + counts[2];
       println!("Top 3, Total Calories = {}", total);
    }
}
