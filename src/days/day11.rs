use crate::utils::file;
use log::{debug, info, error};
use num::integer::{div_floor, lcm};

fn find_lcm(values: &Vec<u64>) -> u64 {
    values.iter().fold(1, |prev, val| {
        lcm(*val, prev)
    })
}

fn compute_monkey_business(monkeys: &Vec<file::Monkey<u64, usize>>, rounds: usize, relief: Option<u64>) -> u64 {
    let mut items = monkeys.iter().fold(Vec::new(), |mut items, monkey| {
        items.push(monkey.items.clone());
        items
    });
    let mut inspect_count = (0..monkeys.len()).fold(Vec::new(), |mut res, _| {
        res.push(0);
        res
    });
    let lcm = find_lcm(&monkeys.iter().map(|monkey| {
        monkey.test_divisible
    }).collect());
    for _ in 0..rounds {
        for monkey_num in 0..monkeys.len() {
            for item_num in 0..items[monkey_num].len() {
                inspect_count[monkey_num] += 1;
                let mut new_item = 0;
                match monkeys[monkey_num].op {
                    file::Operation::Add(val) => new_item = items[monkey_num][item_num].wrapping_add(val),
                    file::Operation::Multiply(val) => new_item = items[monkey_num][item_num].wrapping_mul(val),
                    file::Operation::Pow2 => new_item = items[monkey_num][item_num].wrapping_pow(2),
                    file::Operation::Unknown => error!("Unknown operation"),
                }
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
    return inspect_count[inspect_count.len() - 1] * inspect_count[inspect_count.len() - 2];
}

pub fn task1(path: &str) {
    if let Ok(lines) = file::read_lines(path) {
        let monkeys = file::to_monkeys(lines);
        debug!("Found {} monkeys", monkeys.len());
        let monkey_business = compute_monkey_business(&monkeys, 20, Some(3));
        info!("Monkey business level is {}", monkey_business);
    }
}

pub fn task2(path: &str) {
    if let Ok(lines) = file::read_lines(path) {
        let monkeys = file::to_monkeys(lines);
        debug!("Found {} monkeys", monkeys.len());
        let monkey_business = compute_monkey_business(&monkeys, 10000, None);
        info!("Monkey business level is {}", monkey_business);
    }
}
