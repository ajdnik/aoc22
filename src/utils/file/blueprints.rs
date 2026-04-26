use anyhow::{bail, Context, Result};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blueprint<N> {
    pub id: N,
    pub costs: [[N; 3]; 4],
}

pub fn to_blueprints<N, I>(lines: I) -> Result<Vec<Blueprint<N>>>
where
    N: FromStr + Default + Copy,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let nums: Vec<N> = line
                .split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .map(|s| {
                    s.parse::<N>()
                        .with_context(|| format!("parsing number {s:?} in {line:?}"))
                })
                .collect::<Result<Vec<_>>>()?;
            if nums.len() != 7 {
                bail!(
                    "expected 7 numbers in blueprint, got {} in {line:?}",
                    nums.len()
                );
            }
            let zero = N::default();
            Ok(Blueprint {
                id: nums[0],
                costs: [
                    [nums[1], zero, zero],
                    [nums[2], zero, zero],
                    [nums[3], nums[4], zero],
                    [nums[5], zero, nums[6]],
                ],
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";

    #[test]
    fn parses_blueprint() {
        let bp = to_blueprints::<u32, _>([SAMPLE.to_string()]).unwrap();
        assert_eq!(bp.len(), 1);
        assert_eq!(bp[0].id, 1);
        assert_eq!(bp[0].costs[0], [4, 0, 0]);
        assert_eq!(bp[0].costs[1], [2, 0, 0]);
        assert_eq!(bp[0].costs[2], [3, 14, 0]);
        assert_eq!(bp[0].costs[3], [2, 0, 7]);
    }

    #[test]
    fn errors_on_missing_numbers() {
        assert!(to_blueprints::<u32, _>(["Blueprint 1: foo".to_string()]).is_err());
    }

    #[test]
    fn errors_on_too_many_numbers() {
        let line = "Blueprint 1 2 3 4 5 6 7 8".to_string();
        assert!(to_blueprints::<u32, _>([line]).is_err());
    }
}
