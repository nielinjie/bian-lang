use super::{compute_parser, assign_par, def_and_assign_par, def_parser, test::ok_eq, variable_parser};
use crate::ast::{Expr, Operator};
use Expr::*;

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
    ok_eq(
        ast,
        BinaryExpr {
            op: Operator::Plus,
            left: Box::new(Variable("a".to_string())),
            right: Box::new(Int(1)),
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
    ok_eq(
        ast,
        BinaryExpr {
            op: Operator::Plus,
            left: Box::new(ast1),
            right: Box::new(Int(1)),
        },
    );
}
#[test]
fn assign_simple() {
    let s = "a = b+1";
    let ast = assign_par(s);
    ok_eq(
        ast,
        Assign(
            "a".to_string(),
            Box::new(BinaryExpr {
                op: Operator::Plus,
                left: Box::new(Variable("b".to_string())),
                right: Box::new(Int(1)),
            }),
        ),
    );
}

#[test]
fn def_and_assign() {
    let s = "let a = b+1";
    let ast = def_and_assign_par(s);
    ok_eq(
        ast,
        Block(vec![
            VarDef("a".to_string()),
            Assign(
                "a".to_string(),
                Box::new(BinaryExpr {
                    op: Operator::Plus,
                    left: Box::new(Variable("b".to_string())),
                    right: Box::new(Int(1)),
                }),
            ),
        ]),
    );
}
