use parity_wasm::elements::Instruction;

use super::{Expr, Operator};

pub trait Compile {
    fn to_instructions(&self) -> Vec<Instruction>;
}

impl Compile for Expr {
    fn to_instructions(&self) -> Vec<Instruction> {
        match self {
            Expr::Int(i) => vec![Instruction::I32Const(*i)],
            Expr::BinaryExpr { op, left, right } => {
                let ops = &mut left.to_instructions();
                ops.append(&mut right.to_instructions());
                ops.push(match op {
                    &Operator::Plus => Instruction::I32Add,
                    &Operator::Minus => Instruction::I32Sub,
                });
                ops.to_owned()
            }
            _ => unimplemented!(),
        }
    }
}
