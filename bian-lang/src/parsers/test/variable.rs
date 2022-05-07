use super::{
    assign_par, compute_parser, def_and_assign_par, def_parser, test::ok_eq, variable_parser,
};
use core::fmt::Debug;

use crate::ast::{EvalExpr, Expr, Operator, transform::{EvalExprTransform, ExpressionTransform, Transform,TransformResult}};
use EvalExpr::*;
use Expr::*;
use TransformResult::*;

#[test]
fn simple_let() {
    let s = "let a";
    let ast = def_parser(s);
    ok_eq(ast, VarDef("a".to_string()));
}
#[test]
fn simple_variable() {
    let s = " a";
    let ast = variable_parser(s);
    ok_eq(ast, Variable("a".to_string()));
}

#[test]
fn var_in_add_sub() {
    let s = " a+1";
    let ast = compute_parser(s);
    ok_eq_transform(
        ast,
        BinaryExpr {
            op: Operator::Plus,
            left: Box::new(Variable("a".to_string())),
            right: Box::new(Literal(1)),
        },
    );
}
#[test]
fn var_in_add_sub_three() {
    let s = "b- a+1";
    let ast = compute_parser(s);
    let ast1 = BinaryExpr {
        op: Operator::Minus,
        left: Box::new(Variable("b".to_string())),
        right: Box::new(Variable("a".to_string())),
    };
    ok_eq_transform(
        ast,
        BinaryExpr {
            op: Operator::Plus,
            left: Box::new(ast1),
            right: Box::new(Literal(1)),
        },
    );
}
#[test]
fn assign_simple() {
    let s = "a = b+1";
    let ast = assign_par(s);
    ok_eq_transform_ex(
        ast,
        Assign(
            "a".to_string(),
            Box::new(BinaryExpr {
                op: Operator::Plus,
                left: Box::new(Variable("b".to_string())),
                right: Box::new(Literal(1)),
            }),
        ),
    );
}

#[test]
fn def_and_assign() {
    let s = "let a = b+1";
    let ast = def_and_assign_par(s);
    ok_eq_transform_ex(
        ast,
        Seq(vec![
            VarDef("a".to_string()),
            Assign(
                "a".to_string(),
                Box::new(BinaryExpr {
                    op: Operator::Plus,
                    left: Box::new(Variable("b".to_string())),
                    right: Box::new(Literal(1)),
                }),
            ),
        ]),
    );
}
pub fn ok_eq_transform<E>(r: Result<(&str, EvalExpr), E>, eq: EvalExpr)
where
    E: Debug,
{
    println!("{:?}", r);
    assert!(r.is_ok());
    println!("{:?}", eq);
    let re = r.ok().unwrap().1;
    let trans = EvalExprTransform::transform(&re);
    match trans {
        Success(e, _) => assert_eq!(e, eq),
        _ => panic!(),
    }
}
pub fn ok_eq_transform_ex<E>(r: Result<(&str, Expr), E>, eq: Expr)
where
    E: Debug,
{
    println!("{:?}", r);
    assert!(r.is_ok());
    println!("{:?}", eq);
    let re = r.ok().unwrap().1;
    let trans = ExpressionTransform::transform(&re);
    match trans {
        Success(e, _) => assert_eq!(e, eq),
        _ => panic!(),
    }
}