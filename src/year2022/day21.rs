use std::str::FromStr;

use anyhow::bail;
use aoc::ProblemOutput;
use itertools::Itertools;
use rustc_hash::FxHashMap;

use crate::util::err::NoneErr;

aoc::register!(solve, 2022, 21);

pub fn solve(input: &str, out: &mut ProblemOutput<'_>) -> anyhow::Result<()> {
    fn inner(ctx: &mut FxHashMap<String, Expr>, out: &mut ProblemOutput<'_>) -> Option<()> {
        out.set_part1(ctx.get("root")?.eval(ctx)?);

        ctx.insert("humn".to_string(), Expr::Unknown);
        if let Expr::Op(op, ..) = ctx.get_mut("root")? {
            *op = Op::Sub;
        };
        out.set_part2(ctx.get("root")?.reverse_eval(ctx, 0)?);

        Some(())
    }

    let mut ctx: FxHashMap<String, Expr> = input.lines().map(Expr::from_str).try_collect()?;
    inner(&mut ctx, out).ok_or(NoneErr)?;
    Ok(())
}

#[derive(Debug, Clone)]
enum Expr {
    Unknown,
    Number(i64),
    Op(Op, String, String),
}

impl Expr {
    fn eval(&self, ctx: &FxHashMap<String, Expr>) -> Option<i64> {
        Some(match self {
            Expr::Unknown => return None,
            &Expr::Number(n) => n,
            Expr::Op(op, lhs, rhs) => {
                let lhs = ctx.get(lhs)?.eval(ctx)?;
                let rhs = ctx.get(rhs)?.eval(ctx)?;
                op.eval(lhs, rhs)
            }
        })
    }

    fn reverse_eval(&self, ctx: &FxHashMap<String, Expr>, target: i64) -> Option<i64> {
        match self {
            Expr::Unknown => Some(target),
            Expr::Number(_) => None,
            Expr::Op(op, lhs, rhs) => {
                let lhs = ctx.get(lhs)?;
                let rhs = ctx.get(rhs)?;
                if let Some(lhs_val) = lhs.eval(ctx) {
                    let new_target = op.reverse_eval_rhs(target, lhs_val);
                    rhs.reverse_eval(ctx, new_target)
                } else if let Some(rhs_val) = rhs.eval(ctx) {
                    let new_target = op.reverse_eval_lhs(target, rhs_val);
                    lhs.reverse_eval(ctx, new_target)
                } else {
                    None
                }
            }
        }
    }

    fn from_str(s: &str) -> anyhow::Result<(String, Self)> {
        let (name, raw_expr) = s.split_once(':').ok_or(NoneErr)?;
        let raw_expr = raw_expr.trim();
        let expr = if raw_expr.bytes().all(|b| b.is_ascii_digit()) {
            Self::Number(raw_expr.parse()?)
        } else {
            let (lhs, raw_op, rhs) = raw_expr.splitn(3, ' ').collect_tuple().ok_or(NoneErr)?;
            Self::Op(raw_op.parse()?, lhs.to_string(), rhs.to_string())
        };
        Ok((name.to_string(), expr))
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn eval(self, lhs: i64, rhs: i64) -> i64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Sub => lhs - rhs,
            Self::Mul => lhs * rhs,
            Self::Div => lhs / rhs,
        }
    }

    fn reverse_eval_lhs(self, target: i64, rhs: i64) -> i64 {
        match self {
            Self::Add => target - rhs,
            Self::Sub => target + rhs,
            Self::Mul => target / rhs, // not exact
            Self::Div => target * rhs,
        }
    }

    fn reverse_eval_rhs(self, target: i64, lhs: i64) -> i64 {
        match self {
            Self::Add => target - lhs,
            Self::Sub => lhs - target,
            Self::Mul => target / lhs, // not exact
            Self::Div => lhs / target,
        }
    }
}

impl FromStr for Op {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Ok(match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mul,
            "/" => Self::Div,
            _ => bail!("Invalid operator: {s}"),
        })
    }
}
