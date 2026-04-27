use crate::utils::file;
use anyhow::{bail, Context, Result};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Expr {
    Number(i64),
    Op(String, Op, String),
}

fn to_expressions<I>(lines: I) -> Result<HashMap<String, Expr>>
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

fn apply(op: Op, a: i64, b: i64) -> i64 {
    match op {
        Op::Add => a + b,
        Op::Sub => a - b,
        Op::Mul => a * b,
        Op::Div => a / b,
    }
}

fn eval(name: &str, exprs: &HashMap<String, Expr>, cache: &mut HashMap<String, i64>) -> Result<i64> {
    if let Some(&v) = cache.get(name) {
        return Ok(v);
    }
    let expr = exprs
        .get(name)
        .with_context(|| format!("missing monkey {name:?}"))?;
    let val = match expr {
        Expr::Number(n) => *n,
        Expr::Op(a, op, b) => {
            let av = eval(a, exprs, cache)?;
            let bv = eval(b, exprs, cache)?;
            apply(*op, av, bv)
        }
    };
    cache.insert(name.to_string(), val);
    Ok(val)
}

fn contains_humn(
    name: &str,
    exprs: &HashMap<String, Expr>,
    cache: &mut HashMap<String, bool>,
) -> bool {
    if name == "humn" {
        return true;
    }
    if let Some(&v) = cache.get(name) {
        return v;
    }
    let v = match exprs.get(name) {
        Some(Expr::Number(_)) | None => false,
        Some(Expr::Op(a, _, b)) => contains_humn(a, exprs, cache) || contains_humn(b, exprs, cache),
    };
    cache.insert(name.to_string(), v);
    v
}

fn solve(
    name: &str,
    target: i64,
    exprs: &HashMap<String, Expr>,
    eval_cache: &mut HashMap<String, i64>,
    humn_cache: &mut HashMap<String, bool>,
) -> Result<i64> {
    if name == "humn" {
        return Ok(target);
    }
    let expr = exprs
        .get(name)
        .with_context(|| format!("missing monkey {name:?}"))?;
    let (a, op, b) = match expr {
        Expr::Number(_) => bail!("monkey {name:?} is a number but not humn"),
        Expr::Op(a, op, b) => (a.clone(), *op, b.clone()),
    };
    if contains_humn(&a, exprs, humn_cache) {
        let bv = eval(&b, exprs, eval_cache)?;
        let new_target = match op {
            Op::Add => target - bv,
            Op::Sub => target + bv,
            Op::Mul => target / bv,
            Op::Div => target * bv,
        };
        solve(&a, new_target, exprs, eval_cache, humn_cache)
    } else {
        let av = eval(&a, exprs, eval_cache)?;
        let new_target = match op {
            Op::Add => target - av,
            Op::Sub => av - target,
            Op::Mul => target / av,
            Op::Div => av / target,
        };
        solve(&b, new_target, exprs, eval_cache, humn_cache)
    }
}

pub fn part1(input: &str) -> Result<String> {
    let exprs = to_expressions(file::lines_of(input))?;
    let mut cache = HashMap::new();
    let val = eval("root", &exprs, &mut cache)?;
    Ok(format!("Root yells {val}"))
}

pub fn part2(input: &str) -> Result<String> {
    let exprs = to_expressions(file::lines_of(input))?;
    let root = exprs.get("root").context("missing root monkey")?;
    let (a, b) = match root {
        Expr::Op(a, _, b) => (a.clone(), b.clone()),
        Expr::Number(_) => bail!("root must be an operation"),
    };
    let mut eval_cache = HashMap::new();
    let mut humn_cache = HashMap::new();
    let (humn_side, value_side) = if contains_humn(&a, &exprs, &mut humn_cache) {
        (a, b)
    } else {
        (b, a)
    };
    let target = eval(&value_side, &exprs, &mut eval_cache)?;
    let humn = solve(&humn_side, target, &exprs, &mut eval_cache, &mut humn_cache)?;
    Ok(format!("humn must yell {humn}"))
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
