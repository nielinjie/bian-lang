use std::iter::empty;

use parity_wasm::elements::Instruction;

use crate::Error;

use super::{Expr, Operator};

pub trait Compile {
    fn compile(&self, compiling: Compiling) -> Compiling;
}
impl Compile for Expr {
    fn compile(&self, compiling: Compiling) -> Compiling {
        match self {
            Expr::Int(i) => compiling.merge(vec![Instruction::I32Const(*i)].into()),
            Expr::BinaryExpr { op, left, right } => {
                let left_compiled = left.compile(compiling.clone());
                let right_compiled = right.compile(left_compiled.clone());
                right_compiled.merge(
                    vec![match op {
                        &Operator::Plus => Instruction::I32Add,
                        &Operator::Minus => Instruction::I32Sub,
                    }]
                    .into(),
                )
            }
            Expr::VarDef(name) => {
                let new = match compiling.local_index(name) {
                    None => Compiling {
                        locals: vec![name.clone()],
                        ..Compiling::default()
                    },
                    Some(_) => Compiling {
                        errors: vec![Error::CompileError(format!("existed var - {}", name))],
                        ..Compiling::default()
                    },
                };
                compiling.merge(new)
            }
            Expr::Variable(name) => {
                let new = match compiling.local_index(name) {
                    Some(index) => Compiling {
                        instructions: vec![Instruction::GetLocal(index)],
                        ..Compiling::default()
                    },
                    None => Compiling {
                        errors: vec![Error::CompileError(format!("unknown var - {}", name))],
                        ..Compiling::default()
                    },
                };
                compiling.merge(new)
            }
            Expr::Assign(name, value) => {
                let value_compiled = value.compile(compiling.clone());
                let new = match compiling.local_index(name) {
                    Some(index) => Compiling {
                        instructions: vec![Instruction::SetLocal(index)],
                        ..Compiling::default()
                    },
                    None => Compiling {
                        errors: vec![Error::CompileError(format!("unknown var - {}", name))],
                        ..Compiling::default()
                    },
                };
                value_compiled.merge(new)
            }
            Expr::Block(v) => v.into_iter().fold(compiling, |c, a| a.compile(c)),
            // _ => unimplemented!(),
        }
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Compiling {
    pub instructions: Vec<Instruction>,
    pub locals: Vec<String>,
    pub errors: Vec<Error>,
}
impl Compiling {
    pub fn merge(self, other: Compiling) -> Compiling {
        Compiling {
            instructions: self
                .instructions
                .into_iter()
                .chain(other.instructions)
                .collect(),
            locals: {
                let mut re = self
                    .locals
                    .into_iter()
                    .chain(other.locals)
                    .collect::<Vec<String>>();
                re.dedup();
                re
            },
            errors: self.errors.into_iter().chain(other.errors).collect(),
        }
    }
    fn local_index(&self, name: &str) -> Option<u32> {
        self.locals
            .iter()
            .position(|l| l == name)
            .map(|i| i.try_into().unwrap())
    }
}
impl Into<Compiling> for Vec<Instruction> {
    fn into(self) -> Compiling {
        Compiling {
            instructions: empty().chain(self).collect(),
            ..Compiling::default()
        }
    }
}
