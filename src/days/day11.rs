use crate::utils::file;
use anyhow::Result;
use num::integer::{div_floor, lcm};

fn find_lcm(values: &[u64]) -> u64 {
    values.iter().fold(1, |prev, val| lcm(*val, prev))
}

fn compute_monkey_business(
    monkeys: &[file::Monkey<u64, usize>],
    rounds: usize,
    relief: Option<u64>,
) -> u64 {
    let mut items: Vec<Vec<u64>> = monkeys.iter().map(|m| m.items.clone()).collect();
    let mut inspect_count = vec![0u64; monkeys.len()];
    let divisors: Vec<u64> = monkeys.iter().map(|m| m.test_divisible).collect();
    let lcm = find_lcm(&divisors);
    for _ in 0..rounds {
        for monkey_num in 0..monkeys.len() {
            for item_num in 0..items[monkey_num].len() {
                inspect_count[monkey_num] += 1;
                let item = items[monkey_num][item_num];
                let mut new_item = match monkeys[monkey_num].op {
                    file::Operation::Add(val) => item.wrapping_add(val),
                    file::Operation::Multiply(val) => item.wrapping_mul(val),
                    file::Operation::Pow2 => item.wrapping_pow(2),
                };
                new_item %= lcm;
                if let Some(val) = relief {
                    new_item = div_floor(new_item, val);
                }
                if new_item.wrapping_rem(monkeys[monkey_num].test_divisible) == 0 {
                    items[monkeys[monkey_num].test_true].push(new_item);
                } else {
                    items[monkeys[monkey_num].test_false].push(new_item);
                }
            }
            items[monkey_num] = Vec::new();
        }
    }
    inspect_count.sort();
    inspect_count[inspect_count.len() - 1] * inspect_count[inspect_count.len() - 2]
}

pub fn part1(input: &str) -> Result<String> {
    let monkeys = file::to_monkeys::<u64, usize, _>(file::lines_of(input))?;
    let monkey_business = compute_monkey_business(&monkeys, 20, Some(3));
    Ok(format!("Monkey business level is {}", monkey_business))
}

pub fn part2(input: &str) -> Result<String> {
    let monkeys = file::to_monkeys::<u64, usize, _>(file::lines_of(input))?;
    let monkey_business = compute_monkey_business(&monkeys, 10000, None);
    Ok(format!("Monkey business level is {}", monkey_business))
}
