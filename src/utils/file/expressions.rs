use anyhow::{bail, Context, Result};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Number(i64),
    Op(String, Op, String),
}

pub fn to_expressions<I>(lines: I) -> Result<HashMap<String, Expr>>
where
    I: IntoIterator<Item = String>,
{
    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (name, rest) = line
                .split_once(": ")
                .with_context(|| format!("missing ': ' in {line:?}"))?;
            let parts: Vec<&str> = rest.split_whitespace().collect();
            let expr = match parts.as_slice() {
                [n] => {
                    let val: i64 = n
                        .parse()
                        .with_context(|| format!("parsing number {n:?} in {line:?}"))?;
                    Expr::Number(val)
                }
                [a, op, b] => {
                    let op = match *op {
                        "+" => Op::Add,
                        "-" => Op::Sub,
                        "*" => Op::Mul,
                        "/" => Op::Div,
                        other => bail!("unknown operator {other:?} in {line:?}"),
                    };
                    Expr::Op((*a).to_string(), op, (*b).to_string())
                }
                _ => bail!("malformed expression in {line:?}"),
            };
            Ok((name.to_string(), expr))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_number_and_op() {
        let lines = ["root: pppw + sjmn", "dbpl: 5"].map(String::from);
        let m = to_expressions(lines).unwrap();
        assert_eq!(m.get("dbpl"), Some(&Expr::Number(5)));
        assert_eq!(
            m.get("root"),
            Some(&Expr::Op("pppw".into(), Op::Add, "sjmn".into()))
        );
    }

    #[test]
    fn errors_on_missing_colon() {
        let lines = ["root pppw + sjmn".to_string()];
        assert!(to_expressions(lines).is_err());
    }

    #[test]
    fn errors_on_unknown_operator() {
        let lines = ["root: a ^ b".to_string()];
        assert!(to_expressions(lines).is_err());
    }
}
