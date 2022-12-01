use std::env;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Ok(lines) = utils::read_lines(&args[1]) {
       let numbers = utils::lines_to_numbers(lines);
       let counts = utils::sum_continuous_numbers(numbers);
       let max = counts.iter().max().unwrap();
       println!("Maximum Calories = {}", max);
    }
}
