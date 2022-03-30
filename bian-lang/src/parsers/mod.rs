use crate::ast::*;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, newline, one_of, space0},
    combinator::{eof, map, map_res, recognize},
    error::{ParseError},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
};
fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(space0, inner, space0)
}
fn decimal(input: &str) -> IResult<&str, &str> {
    recognize(many1(terminated(one_of("0123456789"), many0(char('_')))))(input)
}
pub fn number(i: &str) -> IResult<&str, Expr> {
    map_res(decimal, |i| {
        let n = i.parse::<i32>();
        n.map(|n| Expr::Int(n))
    })(i)
}
pub fn operatee(input: &str) -> IResult<&str, Expr> {
    alt((number, variable_parser))(input)
}
pub fn symbol(i: &str) -> IResult<&str, Operator> {
    map(ws(one_of("+-")), |m| match m {
        '+' => Operator::Plus,
        '-' => Operator::Minus,
        _ => panic!(),
    })(i)
}
pub fn add_sub(i: &str) -> IResult<&str, Expr> {
    tuple((operatee, many0(pair(ws(symbol), operatee))))(i).map(|(i, (first, v))| {
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

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(alpha1, many0(alt((alphanumeric1, tag("_"))))))(input)
}
pub fn variable_parser(input: &str) -> IResult<&str, Expr> {
    map(ws(identifier), |ident| Expr::Variable(ident.to_string()))(input)
}
pub fn def_parser(i: &str) -> IResult<&str, Expr> {
    map(preceded(tag("let"), ws(identifier)), |id| {
        Expr::VarDef(id.to_string())
    })(i)
}
pub fn assign_par(input: &str) -> IResult<&str, Expr> {
    map(tuple((identifier, ws(tag("=")), add_sub)), |(i, _v, e)| {
        Expr::Assign(i.to_string(), Box::new(e))
    })(input)
}

pub fn def_and_assign_par(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((ws(tag("let")), ws(identifier), ws(tag("=")), ws(add_sub))),
        |(_l, i, _v, e)| {
            Expr::Block(vec![
                Expr::VarDef(i.to_string()),
                Expr::Assign(i.to_string(), Box::new(e)),
            ])
        },
    )(input)
}
pub fn statement(input: &str) -> IResult<&str, Expr> {
    ws(alt((
        def_and_assign_par,
        def_parser,
        assign_par,
        add_sub,
        variable_parser,
    )))(input)
}
pub fn block(i: &str) -> IResult<&str, Expr> {
    map(separated_list1(newline, statement), |v| Expr::Block(v))(i)
}
pub fn program(input: &str) -> IResult<&str, Expr> {
    terminated(block, eof)(input)
}

#[cfg(test)]
mod test;
