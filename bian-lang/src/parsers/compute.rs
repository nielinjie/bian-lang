use nom::{
    branch::alt,
    character::complete::one_of,
    combinator::map,
    multi::many0,
    sequence::{pair, tuple},
    IResult,
};

use crate::ast::{EvalExpr, Operator};

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
            //TODO 在这里处理优先级的问题。
            //从一个operator的序列变换成一个树，以优先级为根据。
            v.into_iter().fold(first, |a, b| EvalExpr::BinaryExpr {
                op: b.0,
                left: Box::new(a),
                right: Box::new(b.1),
            })
        },
    )(i)
}
