use std::iter::once;

use nom::{
    branch::alt,
    character::complete::one_of,
    combinator::map,
    multi::many0,
    sequence::{pair, tuple},
    IResult,
};

use crate::ast::{compute::ComputeSeq, EvalExpr, Operator};
use EvalExpr::*;

use super::{liberal_parse, variable_parser, ws};

pub fn operand_parser(input: &str) -> IResult<&str, EvalExpr> {
    alt((liberal_parse, variable_parser))(input)
}
pub fn operator(i: &str) -> IResult<&str, Operator> {
    map(ws(one_of("+-")), |m| match m {
        '+' => Operator::Plus,
        '-' => Operator::Minus,
        _ => unimplemented!(),
    })(i)
}
pub fn compute_parser(i: &str) -> IResult<&str, EvalExpr> {
    map(
        tuple((operand_parser, many0(pair(ws(operator), operand_parser)))),
        |(first, v)| {
            let operators = v.iter().map(|(o, _)| o.clone()).collect();
            let operands = v.into_iter().map(|(_, e)| e);
            Compute(ComputeSeq {
                operators: operators,
                operands: once(first).chain(operands).collect(),
            })
        },
    )(i)
}
