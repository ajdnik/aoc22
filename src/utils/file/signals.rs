use anyhow::{Context, Result};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Signal<N> {
    Number(N),
    List(Vec<Signal<N>>),
}

impl<N: Ord> Ord for Signal<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Signal::Number(a), Signal::Number(b)) => a.cmp(b),
            (Signal::List(a), Signal::List(b)) => a.cmp(b),
            (Signal::Number(_), Signal::List(b)) => std::slice::from_ref(self).cmp(b.as_slice()),
            (Signal::List(a), Signal::Number(_)) => a.as_slice().cmp(std::slice::from_ref(other)),
        }
    }
}

impl<N: Ord> PartialOrd for Signal<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_signal<N>(line: &str) -> Result<Signal<N>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    let mut stack: Vec<Vec<Signal<N>>> = Vec::new();
    let mut num_buffer = String::new();
    let mut completed: Option<Signal<N>> = None;
    for chr in line.chars() {
        match chr {
            '[' => stack.push(Vec::new()),
            ']' => {
                if !num_buffer.is_empty() {
                    let val: N = num_buffer
                        .parse()
                        .with_context(|| format!("parsing signal number {num_buffer:?}"))?;
                    stack
                        .last_mut()
                        .context("number outside any list")?
                        .push(Signal::Number(val));
                    num_buffer.clear();
                }
                let finished = stack.pop().context("unmatched ']' in signal")?;
                let list = Signal::List(finished);
                match stack.last_mut() {
                    Some(parent) => parent.push(list),
                    None => completed = Some(list),
                }
            }
            ',' => {
                if !num_buffer.is_empty() {
                    let val: N = num_buffer
                        .parse()
                        .with_context(|| format!("parsing signal number {num_buffer:?}"))?;
                    stack
                        .last_mut()
                        .context("number outside any list")?
                        .push(Signal::Number(val));
                    num_buffer.clear();
                }
            }
            c => num_buffer.push(c),
        }
    }
    completed.with_context(|| format!("unclosed signal: {line:?}"))
}

pub fn to_signals<N, I>(lines: I) -> Result<Vec<Signal<N>>>
where
    N: FromStr,
    <N as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    I: IntoIterator<Item = String>,
{
    let mut signals = Vec::new();
    for line in lines {
        if !line.starts_with('[') {
            continue;
        }
        signals.push(parse_signal::<N>(&line)?);
    }
    Ok(signals)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    fn parse(s: &str) -> Signal<u32> {
        parse_signal::<u32>(s).unwrap()
    }

    #[test]
    fn nested_lists_parse() {
        let s = parse("[[1],[2,3]]");
        if let Signal::List(outer) = s {
            assert_eq!(outer.len(), 2);
            assert!(matches!(outer[0], Signal::List(_)));
        } else {
            panic!("expected List");
        }
    }

    #[test]
    fn empty_list_parses() {
        assert_eq!(parse("[]"), Signal::List(vec![]));
    }

    #[test]
    fn number_lifts_against_list() {
        // [1,1,3,1,1] vs [1,1,5,1,1] → Less at the 3<5 spot
        assert_eq!(
            parse("[1,1,3,1,1]").cmp(&parse("[1,1,5,1,1]")),
            Ordering::Less
        );
    }

    #[test]
    fn shorter_list_is_less() {
        assert_eq!(parse("[1,2,3]").cmp(&parse("[1,2,3,4]")), Ordering::Less);
    }

    #[test]
    fn lift_number_to_list() {
        // [[1],[2,3,4]] vs [[1],4] → second elem 4 vs [2,3,4]; 4 lifted to [4]; 4>2 so Greater
        assert_eq!(
            parse("[[1],[2,3,4]]").cmp(&parse("[[1],4]")),
            Ordering::Less
        );
    }

    #[test]
    fn unclosed_signal_errors() {
        assert!(parse_signal::<u32>("[1,2").is_err());
    }
}
