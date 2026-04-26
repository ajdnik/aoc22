use anyhow::{Context, Result};
use std::str::FromStr;

pub fn lines_of(input: &str) -> impl Iterator<Item = String> + '_ {
    input.lines().map(String::from)
}

/// Splits input on blank lines, parsing each non-blank line as `N`.
pub fn to_number_groups<N, I>(lines: I) -> Result<Vec<Vec<N>>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    let mut groups: Vec<Vec<N>> = vec![Vec::new()];
    for line in lines {
        if line.is_empty() {
            groups.push(Vec::new());
            continue;
        }
        let val: N = line
            .parse()
            .with_context(|| format!("parsing number {:?}", line))?;
        groups.last_mut().unwrap().push(val);
    }
    Ok(groups)
}

pub fn to_groups<I>(lines: I, size: usize) -> Vec<Vec<String>>
where
    I: IntoIterator<Item = String>,
{
    let mut groups: Vec<Vec<String>> = vec![Vec::new()];
    for line in lines {
        if groups.last().map(|g| g.len()).unwrap_or(0) >= size {
            groups.push(Vec::new());
        }
        groups.last_mut().unwrap().push(line);
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lines_of_splits_on_newlines() {
        let v: Vec<String> = lines_of("a\nb\n\nc").collect();
        assert_eq!(v, vec!["a", "b", "", "c"]);
    }

    #[test]
    fn number_groups_separates_on_blank() {
        let lines = ["1", "2", "", "3", "", "4", "5"].map(String::from);
        let g = to_number_groups::<i32, _>(lines).unwrap();
        assert_eq!(g, vec![vec![1, 2], vec![3], vec![4, 5]]);
    }

    #[test]
    fn number_groups_errors_on_unparsable() {
        let lines = ["1", "abc"].map(String::from);
        assert!(to_number_groups::<i32, _>(lines).is_err());
    }

    #[test]
    fn groups_chunks_by_size() {
        let lines = ["a", "b", "c", "d", "e"].map(String::from);
        assert_eq!(
            to_groups(lines, 2),
            vec![vec!["a", "b"], vec!["c", "d"], vec!["e"]]
        );
    }
}
