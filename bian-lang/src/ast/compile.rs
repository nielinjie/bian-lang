use std::iter::empty;

use parity_wasm::elements::{BlockType, Instruction};

use super::{Block, EvalExpr, Expr, Operator};
use super::{EvalExpr::*, Statement};
use crate::Error;
use Expr::*;
use Instruction::*;
pub trait Compile {
    fn compile(&self, compiling: &Compiling) -> Compiling;
}
impl Compile for Statement {
    fn compile(&self, compiling: &Compiling) -> Compiling {
        let Statement(e) = self;
        match e {
            Eval(_) => e.compile(compiling).merge(vec![Drop].into()),
            _ => e.compile(compiling),
        }
    }
}
impl Compile for Block {
    fn compile(&self, compiling: &Compiling) -> Compiling {
        (&self.0)
            .into_iter()
            .fold(compiling.clone(), |c, a| a.compile(&c))
    }
}

impl Compile for EvalExpr {
    fn compile(&self, compiling: &Compiling) -> Compiling {
        match self {
            Literal(i) => compiling.merge(vec![I32Const(*i)].into()),
            BinaryExpr { op, left, right } => {
                let left_compiled = &left.compile(compiling);
                let right_compiled = right.compile(left_compiled);
                right_compiled.merge(
                    vec![match op {
                        &Operator::Plus => I32Add,
                        &Operator::Minus => I32Sub,
                        _ => unimplemented!()
                    }]
                    .into(),
                )
            }
            Variable(name) => {
                let new = match compiling.local_index(name) {
                    Some(index) => Compiling {
                        instructions: vec![GetLocal(index)],
                        ..Compiling::default()
                    },
                    None => Compiling {
                        errors: vec![Error::CompileError(format!("unknown var - {}", name))],
                        ..Compiling::default()
                    },
                };
                compiling.merge(new)
            }
        }
    }
}

impl Compile for Expr {
    fn compile(&self, compiling: &Compiling) -> Compiling {
        match self {
            Eval(eval) => eval.compile(compiling),
            Expr::Return(ret) => ret.compile(compiling),
            VarDef(name) => {
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

            Assign(name, value) => {
                let value_compiled = value.compile(compiling);
                let new = match compiling.local_index(name) {
                    Some(index) => Compiling {
                        instructions: vec![SetLocal(index)],
                        ..Compiling::default()
                    },
                    None => Compiling {
                        errors: vec![Error::CompileError(format!("unknown var - {}", name))],
                        ..Compiling::default()
                    },
                };
                value_compiled.merge(new)
            }
            Seq(v) => v.into_iter().fold(compiling.clone(), |c, a| a.compile(&c)),
            IfElse(cond, then_b, else_b) => {
                let cond_instructions = &cond.compile(compiling);
                let then_b_instructions =
                    then_b.compile(&cond_instructions.merge(If(BlockType::NoResult).into()));
                let else_b_instructions = else_b.compile(&then_b_instructions.merge(Else.into()));
                else_b_instructions.merge(End.into())
            }
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
    pub fn merge(&self, other: Compiling) -> Compiling {
        Compiling {
            instructions: self
                .instructions
                .clone()
                .into_iter()
                .chain(other.instructions)
                .collect::<Vec<Instruction>>(),
            locals: {
                let mut re = self
                    .locals
                    .clone()
                    .into_iter()
                    .chain(other.locals)
                    .collect::<Vec<String>>();
                re.dedup();
                re
            },
            errors: self
                .errors
                .clone()
                .into_iter()
                .chain(other.errors)
                .collect(),
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
impl Into<Compiling> for Instruction {
    fn into(self) -> Compiling {
        Compiling {
            instructions: vec![self],
            ..Compiling::default()
        }
    }
}
