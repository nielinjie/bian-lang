use crate::{
    ast::{Expr, Operator,EvalExpr},
    parsers::*,
};
use core::fmt::Debug;
use Expr::*;
use EvalExpr::*;
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

#[test]
fn add_sub_simple() {
    let r = compute_parser("1+2");
    ok_eq(r, binary_expr("+", 1, 2));
    let r1 = compute_parser("1 + 3");
    assert!(r1.is_ok());
    let r2 = compute_parser("1 +3 -2");
    assert!(r2.is_ok());
    ok_eq(r2, sample_node());
}

#[test]
fn some_case_for_ws() {
    let strings = vec!["1+3-2\n", " 1+3-2 \n", "1+ 3-2\n "];
    for text in strings.into_iter() {
        let r2 = statement(text);
        ok_eq(r2, Eval(Box::new(sample_node())));
    }
}
#[test]
fn statement_all() {
    let str = " 1+2 \n";
    let ast = statement(str);
    let r = binary_expr("+", 1, 2);
    ok_eq(ast, Eval(Box::new(r)));
    let str = " 1+2 \r\n";
    let ast = statement(str);
    let r = binary_expr("+", 1, 2);
    ok_eq(ast, Eval(Box::new(r)));
}

// #[test]
// fn block_simple() {
//     let s = "2+1
//     1+2
//     2+1
//     6+7";
//     let ast = block(s);
//     let re = Expr::Block(vec![
//         Expr::binary_expr("+", 2, 1),
//         Expr::binary_expr("+", 1, 2),
//         Expr::binary_expr("+", 2, 1),
//         Expr::binary_expr("+", 6, 7),
//     ]);
//     ok_eq(ast, re);
// }
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

pub fn ok_eq<T, E>(r: Result<(&str, T), E>, eq: T)
where
    T: PartialEq + Debug,
    E: Debug,
{
    println!("{:?}", r);
    assert!(r.is_ok());
    println!("{:?}", eq);
    assert_eq!(eq, r.ok().unwrap().1);
}

mod variable;
mod error;

mod statement;

mod flows;