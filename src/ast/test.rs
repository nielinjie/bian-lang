use parity_wasm::elements::Instruction;

use super::compile::*;
use crate::ast::{Expr, Operator};
#[test]
fn const_test() {
    let exp = Expr::Int(27);
    let v = exp.to_instructions();
    assert_eq!(v, vec![Instruction::I32Const(27)])
}
#[test]
fn const_add_test() {
    let exp = Expr::binary_expr("+", 1, 3);
    let v = exp.to_instructions();
    assert_eq!(
        v,
        vec![
            Instruction::I32Const(1),
            Instruction::I32Const(3),
            Instruction::I32Add
        ]
    )
}
#[test]

fn const_add_sub_test() {
    let exp = Expr::BinaryExpr {
        op: Operator::Minus,
        left: Box::new(Expr::binary_expr("+", 1, 3)),
        right: Box::new(Expr::Int(2)),
    };
    let v = exp.to_instructions();
    assert_eq!(
        v,
        vec![
            Instruction::I32Const(1),
            Instruction::I32Const(3),
            Instruction::I32Add,
            Instruction::I32Const(2),
            Instruction::I32Sub
        ]
    )
}
