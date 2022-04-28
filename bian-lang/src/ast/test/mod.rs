use parity_wasm::elements::Instruction;

use super::compile::*;
use crate::ast::{EvalExpr, Operator};

use Instruction::*;
use super::super::parsers::test::binary_expr;
use EvalExpr::*;
#[test]
fn const_test() {
    let exp = Literal(27);
    let v = exp.compile(&Compiling::default());
    assert_eq!(v, vec![I32Const(27)].into())
}
#[test]
fn const_add() {
    let exp = binary_expr("+", 1, 3);
    let v = exp.compile(&Compiling::default());
    assert_eq!(
        v,
        vec![
            I32Const(1),
            I32Const(3),
            I32Add
        ]
        .into()
    )
}
#[test]

fn const_add_sub() {
    let exp = BinaryExpr {
        op: Operator::Minus,
        left: Box::new(binary_expr("+", 1, 3)),
        right: Box::new(Literal(2)),
    };
    let v = exp.compile(&Compiling::default());
    assert_eq!(
        v,
        vec![
            I32Const(1),
            I32Const(3),
            I32Add,
            I32Const(2),
            I32Sub
        ]
        .into()
    )
}

mod variable;
mod flows;