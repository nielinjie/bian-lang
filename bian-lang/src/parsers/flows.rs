use nom::{
    bytes::complete::tag,
    combinator::{map},
    sequence::{delimited, tuple},
    IResult,
};

use crate::ast::{Block, EvalExpr, Expr, Statement};
use Expr::*;

use super::{block, compute_parser, ws};

pub fn condition_expr_parser(input: &str) -> IResult<&str, EvalExpr> {
    delimited(tag("("), compute_parser, tag(")"))(input)
}

pub fn branch_parser(input: &str) -> IResult<&str, Block> {
    map(delimited(tag("{"), ws(block), tag("}")), |ve| {
        Block(ve.into_iter().map(Statement).collect())
    })(input)
}

pub fn if_else_parser(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            tag("if"),
            ws(condition_expr_parser),
            ws(branch_parser),
            tag("else"),
            ws(branch_parser),
        )),
        |(_i, cond, branch1, _e, branch2)| {
            IfElse(Box::new(cond), Box::new(branch1), Box::new(branch2))
        },
    )(input)
}
