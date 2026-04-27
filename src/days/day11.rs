use crate::utils::file;
use anyhow::{anyhow, bail, Context, Result};
use std::str::FromStr;

enum Operation<N> {
    Add(N),
    Multiply(N),
    Pow2,
}

struct Monkey<N, T> {
    items: Vec<N>,
    op: Operation<N>,
    test_divisible: N,
    test_true: T,
    test_false: T,
}

fn to_monkeys<N, T, I>(input: I) -> Result<Vec<Monkey<N, T>>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    struct Builder<N, T> {
        items: Vec<N>,
        op: Option<Operation<N>>,
        test_divisible: Option<N>,
        test_true: Option<T>,
        test_false: Option<T>,
    }
    impl<N, T> Builder<N, T> {
        fn new() -> Self {
            Self {
                items: Vec::new(),
                op: None,
                test_divisible: None,
                test_true: None,
                test_false: None,
            }
        }
        fn finish(self, idx: usize) -> Result<Monkey<N, T>> {
            Ok(Monkey {
                items: self.items,
                op: self.op.ok_or_else(|| anyhow!("monkey {idx} missing op"))?,
                test_divisible: self
                    .test_divisible
                    .ok_or_else(|| anyhow!("monkey {idx} missing test divisor"))?,
                test_true: self
                    .test_true
                    .ok_or_else(|| anyhow!("monkey {idx} missing test_true"))?,
                test_false: self
                    .test_false
                    .ok_or_else(|| anyhow!("monkey {idx} missing test_false"))?,
            })
        }
    }

    let mut monkeys: Vec<Monkey<N, T>> = Vec::new();
    let mut current: Option<Builder<N, T>> = None;
    for line in input {
        if line.starts_with("Monkey") {
            if let Some(b) = current.take() {
                monkeys.push(b.finish(monkeys.len())?);
            }
            current = Some(Builder::new());
        } else if let Some(stripped) = line.strip_prefix("  Starting items: ") {
            let b = current
                .as_mut()
                .context("'Starting items' before Monkey header")?;
            for item in stripped.split(", ") {
                b.items.push(
                    item.parse()
                        .with_context(|| format!("parsing monkey item {item:?}"))?,
                );
            }
        } else if let Some(stripped) = line.strip_prefix("  Operation: new = old ") {
            let b = current
                .as_mut()
                .context("'Operation' before Monkey header")?;
            let parts: Vec<&str> = stripped.split(' ').collect();
            if parts.len() != 2 {
                bail!("malformed operation {stripped:?}");
            }
            b.op = Some(match (parts[0], parts[1]) {
                ("+", val) => Operation::Add(
                    val.parse()
                        .with_context(|| format!("parsing operand {val:?}"))?,
                ),
                ("*", "old") => Operation::Pow2,
                ("*", val) => Operation::Multiply(
                    val.parse()
                        .with_context(|| format!("parsing operand {val:?}"))?,
                ),
                _ => bail!("unsupported operation {stripped:?}"),
            });
        } else if let Some(stripped) = line.strip_prefix("  Test: divisible by ") {
            let b = current.as_mut().context("'Test' before Monkey header")?;
            b.test_divisible = Some(
                stripped
                    .parse()
                    .with_context(|| format!("parsing test divisor {stripped:?}"))?,
            );
        } else if let Some(stripped) = line.strip_prefix("    If true: throw to monkey ") {
            let b = current.as_mut().context("'If true' before Monkey header")?;
            b.test_true = Some(
                stripped
                    .parse()
                    .with_context(|| format!("parsing test_true {stripped:?}"))?,
            );
        } else if let Some(stripped) = line.strip_prefix("    If false: throw to monkey ") {
            let b = current
                .as_mut()
                .context("'If false' before Monkey header")?;
            b.test_false = Some(
                stripped
                    .parse()
                    .with_context(|| format!("parsing test_false {stripped:?}"))?,
            );
        }
    }
    if let Some(b) = current.take() {
        monkeys.push(b.finish(monkeys.len())?);
    }
    Ok(monkeys)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn find_lcm(values: &[u64]) -> u64 {
    values.iter().fold(1, |prev, val| lcm(*val, prev))
}

fn compute_monkey_business(
    monkeys: &[Monkey<u64, usize>],
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
                    Operation::Add(val) => item.wrapping_add(val),
                    Operation::Multiply(val) => item.wrapping_mul(val),
                    Operation::Pow2 => item.wrapping_pow(2),
                };
                new_item %= lcm;
                if let Some(val) = relief {
                    new_item /= val;
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
    let monkeys = to_monkeys::<u64, usize, _>(file::lines_of(input))?;
    let monkey_business = compute_monkey_business(&monkeys, 20, Some(3));
    Ok(format!("Monkey business level is {monkey_business}"))
}

pub fn part2(input: &str) -> Result<String> {
    let monkeys = to_monkeys::<u64, usize, _>(file::lines_of(input))?;
    let monkey_business = compute_monkey_business(&monkeys, 10_000, None);
    Ok(format!("Monkey business level is {monkey_business}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<String> {
        [
            "Monkey 0:",
            "  Starting items: 79, 98",
            "  Operation: new = old * 19",
            "  Test: divisible by 23",
            "    If true: throw to monkey 2",
            "    If false: throw to monkey 3",
        ]
        .map(String::from)
        .to_vec()
    }

    #[test]
    fn parses_full_monkey() {
        let m = to_monkeys::<u64, usize, _>(sample()).unwrap();
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].items, vec![79, 98]);
        assert!(matches!(m[0].op, Operation::Multiply(19)));
        assert_eq!(m[0].test_divisible, 23);
        assert_eq!(m[0].test_true, 2);
        assert_eq!(m[0].test_false, 3);
    }

    #[test]
    fn pow2_recognized() {
        let mut s = sample();
        s[2] = String::from("  Operation: new = old * old");
        let m = to_monkeys::<u64, usize, _>(s).unwrap();
        assert!(matches!(m[0].op, Operation::Pow2));
    }

    #[test]
    fn missing_op_errors() {
        let mut s = sample();
        s.remove(2);
        assert!(to_monkeys::<u64, usize, _>(s).is_err());
    }
}
