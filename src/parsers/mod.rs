use crate::ast::*;
use nom::{
    character::complete::{char, multispace0, one_of},
    combinator::recognize,
    error::{ErrorKind, FromExternalError, ParseError},
    multi::{many0, many1},
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}
fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}
pub fn number(i: &str) -> IResult<&str, Expr> {
    decimal(i).and_then(|(i, o)| {
        let n = o.parse::<i32>();
        n.map(|n| (i, Expr::Int(n)))
            .map_err(|err| external_err(i, err))
    })
}
pub fn symbol(i: &str) -> IResult<&str, Operator> {
    ws(one_of("+-"))(i).map(|(i, m)| {
        (
            i,
            match m {
                '+' => Operator::Plus,
                '-' => Operator::Minus,
                _ => panic!(),
            },
        )
    })
}
pub fn add_sub(i: &str) -> IResult<&str, Expr> {
    tuple((number, many0(pair(ws(symbol), number))))(i).map(|(i, (first, v))| {
        (
            i,
            v.into_iter().fold(first, |a, b| Expr::BinaryExpr {
                op: b.0,
                left: Box::new(a),
                right: Box::new(b.1),
            }),
        )
    })
}
pub fn expr(i: &str) -> IResult<&str,Expr> {
    ws(many1(add_sub))(i).map(|(i, v)| {(i,Expr::Block(v))})
}

fn external_err(i: &str, err: std::num::ParseIntError) -> nom::Err<nom::error::Error<&str>> {
    nom::Err::Error(FromExternalError::from_external_error(
        i,
        ErrorKind::MapRes,
        err,
    ))
}

#[cfg(test)]
mod test;
#[cfg(test)]
mod test_variable;