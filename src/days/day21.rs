use crate::utils::file;
use anyhow::{bail, Context, Result};
use std::collections::HashMap;

fn apply(op: file::Op, a: i64, b: i64) -> i64 {
    match op {
        file::Op::Add => a + b,
        file::Op::Sub => a - b,
        file::Op::Mul => a * b,
        file::Op::Div => a / b,
    }
}

fn eval(
    name: &str,
    exprs: &HashMap<String, file::Expr>,
    cache: &mut HashMap<String, i64>,
) -> Result<i64> {
    if let Some(&v) = cache.get(name) {
        return Ok(v);
    }
    let expr = exprs
        .get(name)
        .with_context(|| format!("missing monkey {name:?}"))?;
    let val = match expr {
        file::Expr::Number(n) => *n,
        file::Expr::Op(a, op, b) => {
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
    exprs: &HashMap<String, file::Expr>,
    cache: &mut HashMap<String, bool>,
) -> bool {
    if name == "humn" {
        return true;
    }
    if let Some(&v) = cache.get(name) {
        return v;
    }
    let v = match exprs.get(name) {
        Some(file::Expr::Number(_)) | None => false,
        Some(file::Expr::Op(a, _, b)) => {
            contains_humn(a, exprs, cache) || contains_humn(b, exprs, cache)
        }
    };
    cache.insert(name.to_string(), v);
    v
}

fn solve(
    name: &str,
    target: i64,
    exprs: &HashMap<String, file::Expr>,
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
        file::Expr::Number(_) => bail!("monkey {name:?} is a number but not humn"),
        file::Expr::Op(a, op, b) => (a.clone(), *op, b.clone()),
    };
    if contains_humn(&a, exprs, humn_cache) {
        let bv = eval(&b, exprs, eval_cache)?;
        let new_target = match op {
            file::Op::Add => target - bv,
            file::Op::Sub => target + bv,
            file::Op::Mul => target / bv,
            file::Op::Div => target * bv,
        };
        solve(&a, new_target, exprs, eval_cache, humn_cache)
    } else {
        let av = eval(&a, exprs, eval_cache)?;
        let new_target = match op {
            file::Op::Add => target - av,
            file::Op::Sub => av - target,
            file::Op::Mul => target / av,
            file::Op::Div => av / target,
        };
        solve(&b, new_target, exprs, eval_cache, humn_cache)
    }
}

pub fn part1(input: &str) -> Result<String> {
    let exprs = file::to_expressions(file::lines_of(input))?;
    let mut cache = HashMap::new();
    let val = eval("root", &exprs, &mut cache)?;
    Ok(format!("Root yells {val}"))
}

pub fn part2(input: &str) -> Result<String> {
    let exprs = file::to_expressions(file::lines_of(input))?;
    let root = exprs.get("root").context("missing root monkey")?;
    let (a, b) = match root {
        file::Expr::Op(a, _, b) => (a.clone(), b.clone()),
        file::Expr::Number(_) => bail!("root must be an operation"),
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
