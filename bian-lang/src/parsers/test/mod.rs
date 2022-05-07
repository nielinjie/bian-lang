use crate::{
    ast::{compute::ComputeSeq, EvalExpr, Expr, Operator},
    parsers::*,
};
use core::fmt::Debug;
use EvalExpr::*;
use Expr::*;
use Operator::*;
#[test]
fn number_simple() {
    let r = liberal_parse("1234");
    ok_eq(r, Literal(1234));
    let r0 = liberal_parse("02");
    ok_eq(r0, Literal(2));
    let re = liberal_parse("K");
    assert!(re.is_err());
}
#[test]
fn symbol_simple() {
    let r = operator("+");
    ok_eq(r, Operator::Plus);
}

fn sample_node() -> EvalExpr {
    (BinaryExpr {
        op: Operator::Minus,
        left: Box::new(binary_expr("+", 1, 3)),
        right: Box::new(Literal(2)),
    })
    .clone()
}

fn sample_node_compute() -> EvalExpr {
    Compute(ComputeSeq {
        operators: vec![Plus, Minus],
        operands: vec![Literal(1), Literal(3), Literal(2)],
    })
}

#[test]
fn add_sub_simple() {
    let r = compute_parser("1+2");
    ok_eq(r, binary_expr_compute("+", 1, 2));
    let r1 = compute_parser("1 + 3");
    assert!(r1.is_ok());
    let r2 = compute_parser("1 +3 -2");
    assert!(r2.is_ok());
    ok_eq(r2, sample_node_compute());
}

#[test]
fn some_case_for_ws() {
    let strings = vec!["1+3-2\n", " 1+3-2 \n", "1+ 3-2\n "];
    for text in strings.into_iter() {
        let r2 = statement(text);
        ok_eq(r2, Eval(Box::new(sample_node_compute())));
    }
}
#[test]
fn statement_all() {
    let str = " 1+2 \n";
    let ast = statement(str);
    let r = binary_expr_compute("+", 1, 2);
    ok_eq(ast, Eval(Box::new(r)));
    let str = " 1+2 \r\n";
    let ast = statement(str);
    let r = binary_expr_compute("+", 1, 2);
    ok_eq(ast, Eval(Box::new(r)));
}

fn expr_int(i: i32) -> Box<EvalExpr> {
    Box::new(Literal(i))
}
pub fn binary_expr(op: &str, l: i32, r: i32) -> EvalExpr {
    BinaryExpr {
        op: match op {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            _ => panic!(),
        },
        left: expr_int(l),
        right: expr_int(r),
    }
}
pub fn binary_expr_compute(op: &str, l: i32, r: i32) -> EvalExpr {
    Compute(ComputeSeq {
        operators: vec![match op {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            _ => panic!(),
        }],
        operands: vec![Literal(l), Literal(r)],
    })
}
pub fn ok_eq<T, E>(r: Result<(&str, T), E>, eq: T)
where
    T: PartialEq + Debug,
    E: Debug,
{
    println!("{:?}", r);
    assert!(r.is_ok());
    println!("{:?}", eq);
    assert_eq!(r.ok().unwrap().1, eq);
}

mod error;
mod variable;

mod statement;

mod flows;
