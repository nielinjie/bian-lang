use crate::{
    ast::{Expr, Operator},
    parsers::*,
};
use core::fmt::Debug;

#[test]
fn number_simple() {
    let r = number("1234");
    ok_eq(r, Expr::Int(1234));
    let r0 = number("02");
    ok_eq(r0, Expr::Int(2));
    let re = number("K");
    assert!(re.is_err());
}
#[test]
fn symbol_simple() {
    let r = symbol("+");
    ok_eq(r, Operator::Plus);
}

fn sample_node() -> Expr {
    (Expr::BinaryExpr {
        op: Operator::Minus,
        left: Box::new(Expr::binary_expr("+", 1, 3)),
        right: Box::new(Expr::Int(2)),
    })
    .clone()
}
fn sample_expr() -> Expr {
    Expr::Block(vec![sample_node()]).clone()
}
#[test]
fn add_sub_simple() {
    let r = add_sub("1+2");
    ok_eq(r, Expr::binary_expr("+", 1, 2));
    let r1 = add_sub("1 + 3");
    assert!(r1.is_ok());
    let r2 = add_sub("1 +3 -2");
    assert!(r2.is_ok());
    ok_eq(r2, sample_node());
}

#[test]
fn some_case_for_ws() {
    let strings = vec!["1+3-2", " 1+3-2 ", "1+ 3-2"];
    for text in strings.into_iter() {
        let r2 = statement(text);
        ok_eq(r2, sample_node());
    }
}
#[test]
fn statement_all() {
    let str = " 1+2 ";
    let ast = statement(str);
    let r = Expr::binary_expr("+", 1, 2);
    ok_eq(ast, r);
    let str = " 1+2 \r\n";
    let ast = statement(str);
    let r = Expr::binary_expr("+", 1, 2);
    ok_eq(ast, r);
}

#[test]
fn block_simple() {
    let s = "2+1
    1+2
    2+1
    6+7";
    let ast = block(s);
    let re = Expr::Block(vec![
        Expr::binary_expr("+", 2, 1),
        Expr::binary_expr("+", 1, 2),
        Expr::binary_expr("+", 2, 1),
        Expr::binary_expr("+", 6, 7),
    ]);
    ok_eq(ast, re);
}
fn expr_int(i: i32) -> Box<Expr> {
    Box::new(Expr::Int(i))
}
impl Expr {
    pub fn binary_expr(op: &str, l: i32, r: i32) -> Self {
        Expr::BinaryExpr {
            op: match op {
                "+" => Operator::Plus,
                "-" => Operator::Minus,
                _ => panic!(),
            },
            left: expr_int(l),
            right: expr_int(r),
        }
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
