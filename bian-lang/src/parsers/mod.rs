use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, one_of, space0, line_ending},
    combinator::{eof, map, map_res, recognize},
    error::ParseError,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};

use crate::ast::{EvalExpr, Expr, Operator, Statement, Block};

use self::{flows::if_else_parser, compute::compute_parser};

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(space0, inner, space0)
}


fn line<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(many0(line_ending), inner, many1(line_ending))
}


fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}
pub fn liberal_parse(i: &str) -> IResult<&str, EvalExpr> {
    map_res(decimal, |i| {
        let n = i.parse::<i32>();
        n.map(|n| EvalExpr::Literal(n))
    })(i)
}
pub fn operatee(input: &str) -> IResult<&str, EvalExpr> {
    alt((liberal_parse, variable_parser))(input)
}
pub fn operator(i: &str) -> IResult<&str, Operator> {
    map(ws(one_of("+-")), |m| match m {
        '+' => Operator::Plus,
        '-' => Operator::Minus,
        _ => panic!(),
    })(i)
}


fn identifier(input: &str) -> IResult<&str, &str> {
    //TODO 排除keyword：let、return
    recognize(pair(alpha1, many0(alt((alphanumeric1, tag("_"))))))(input)
}
pub fn variable_parser(input: &str) -> IResult<&str, EvalExpr> {
    map(ws(identifier), |ident| {
        EvalExpr::Variable(ident.to_string())
    })(input)
}
pub fn def_parser(i: &str) -> IResult<&str, Expr> {
    map(preceded(tag("let"), ws(identifier)), |id| {
        Expr::VarDef(id.to_string())
    })(i)
}

pub fn return_parser(i: &str) -> IResult<&str, Expr> {
    map(
        preceded(tag("return"), ws(alt((compute_parser, variable_parser)))),
        |ev| Expr::Return(Box::new(ev)),
    )(i)
}
pub fn assign_par(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((identifier, ws(tag("=")), compute_parser)),
        |(i, _v, e)| Expr::Assign(i.to_string(), Box::new(e)),
    )(input)
}

pub fn def_and_assign_par(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((
            ws(tag("let")),
            ws(identifier),
            ws(tag("=")),
            ws(compute_parser),
        )),
        |(_l, i, _v, e)| {
            Expr::Seq(vec![
                Expr::VarDef(i.to_string()),
                Expr::Assign(i.to_string(), Box::new(e)),
            ])
        },
    )(input)
}
pub fn eval_parse(input: &str) -> IResult<&str, Expr> {
    map(
        ws(alt((compute_parser, variable_parser))),
        |eval: EvalExpr| Expr::Eval(Box::new(eval)),
    )(input)
}
pub fn statement(input: &str) -> IResult<&str, Expr> {
    line(ws(alt((
        if_else_parser,
        return_parser,
        def_and_assign_par,
        def_parser,
        assign_par,
        eval_parse,
    ))))(input)
}
pub fn block(i: &str) -> IResult<&str, Vec<Expr>> {
    ws(many1(statement))(i)
}
pub fn program(input: &str) -> IResult<&str, Block> {
    map(terminated(block, eof), |ve| {
        Block(ve.into_iter().map(Statement).collect())
    })(input)
}

pub mod flows;
pub mod compute;

#[cfg(test)]
pub mod test;
