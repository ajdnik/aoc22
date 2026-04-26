use anyhow::{bail, Result};

fn snafu_to_decimal(s: &str) -> Result<i64> {
    let mut n = 0i64;
    for ch in s.chars() {
        let d = match ch {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => bail!("invalid SNAFU digit {ch:?}"),
        };
        n = n * 5 + d;
    }
    Ok(n)
}

fn decimal_to_snafu(mut n: i64) -> String {
    if n == 0 {
        return "0".into();
    }
    let mut digits = Vec::new();
    while n != 0 {
        let r = n.rem_euclid(5);
        let (ch, carry) = match r {
            0 => ('0', 0),
            1 => ('1', 0),
            2 => ('2', 0),
            3 => ('=', 1),
            4 => ('-', 1),
            _ => unreachable!(),
        };
        digits.push(ch);
        n = n / 5 + carry;
    }
    digits.iter().rev().collect()
}

pub fn part1(input: &str) -> Result<String> {
    let mut sum = 0i64;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        sum += snafu_to_decimal(line)?;
    }
    let snafu = decimal_to_snafu(sum);
    Ok(format!("Fuel requirement in SNAFU is {snafu}"))
}

pub fn part2(_input: &str) -> Result<String> {
    Ok("Merry Christmas!".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str =
        "1=-0-2\n12111\n2=0=\n21\n2=01\n111\n20012\n112\n1=-1=\n1-12\n12\n1=\n122\n";

    #[test]
    fn part1_sample() {
        assert!(part1(SAMPLE).unwrap().contains("2=-1=0"));
    }

    #[test]
    fn snafu_decode() {
        assert_eq!(snafu_to_decimal("1=-0-2").unwrap(), 1747);
        assert_eq!(snafu_to_decimal("1121-1110-1=0").unwrap(), 314159265);
    }

    #[test]
    fn snafu_encode() {
        assert_eq!(decimal_to_snafu(1), "1");
        assert_eq!(decimal_to_snafu(2), "2");
        assert_eq!(decimal_to_snafu(3), "1=");
        assert_eq!(decimal_to_snafu(4), "1-");
        assert_eq!(decimal_to_snafu(5), "10");
        assert_eq!(decimal_to_snafu(976), "2=-01");
        assert_eq!(decimal_to_snafu(4890), "2=-1=0");
    }

    #[test]
    fn rejects_invalid() {
        assert!(snafu_to_decimal("12X").is_err());
    }
}
