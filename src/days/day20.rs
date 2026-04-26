use anyhow::{Context, Result};

const DECRYPTION_KEY: i64 = 811_589_153;

fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            l.trim()
                .parse::<i64>()
                .with_context(|| format!("parsing number {l:?}"))
        })
        .collect()
}

fn mix(numbers: &[i64], rounds: usize) -> Vec<i64> {
    let n = numbers.len();
    let modulus = n as i64 - 1;
    let mut order: Vec<usize> = (0..n).collect();
    for _ in 0..rounds {
        for (i, &num) in numbers.iter().enumerate() {
            let pos = order.iter().position(|&x| x == i).unwrap();
            order.remove(pos);
            let new_pos = (pos as i64 + num).rem_euclid(modulus) as usize;
            order.insert(new_pos, i);
        }
    }
    order.into_iter().map(|i| numbers[i]).collect()
}

fn grove_sum(mixed: &[i64]) -> Result<i64> {
    let zero_pos = mixed
        .iter()
        .position(|&x| x == 0)
        .context("no zero in input")?;
    let n = mixed.len();
    Ok([1000, 2000, 3000]
        .iter()
        .map(|&k| mixed[(zero_pos + k) % n])
        .sum())
}

pub fn part1(input: &str) -> Result<String> {
    let numbers = parse(input)?;
    let mixed = mix(&numbers, 1);
    let sum = grove_sum(&mixed)?;
    Ok(format!("Grove coordinates sum is {sum}"))
}

pub fn part2(input: &str) -> Result<String> {
    let numbers: Vec<i64> = parse(input)?
        .into_iter()
        .map(|x| x * DECRYPTION_KEY)
        .collect();
    let mixed = mix(&numbers, 10);
    let sum = grove_sum(&mixed)?;
    Ok(format!("Grove coordinates sum is {sum}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "1\n2\n-3\n3\n-2\n0\n4\n";

    #[test]
    fn part1_sample() {
        assert!(part1(SAMPLE).unwrap().contains("is 3"));
    }

    #[test]
    fn part2_sample() {
        assert!(part2(SAMPLE).unwrap().contains("is 1623178306"));
    }

    #[test]
    fn errors_on_bad_number() {
        assert!(part1("1\nabc\n2").is_err());
    }

    #[test]
    fn errors_when_no_zero() {
        assert!(part1("1\n2\n3").is_err());
    }
}
