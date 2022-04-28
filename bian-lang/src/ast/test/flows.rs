use crate::ast::compile::{Compile, Compiling};
use crate::ast::{Block, EvalExpr, Statement};
use parity_wasm::elements::{BlockType, Instruction};

use super::super::Expr;
use super::EvalExpr::*;
use Expr::*;
use Instruction::*;
#[test]
fn simple_if_else() {
    let simple = IfElse(
        Box::new(Literal(1)),
        Box::new(Block(vec![Literal(2).into()])),
        Box::new(Block(vec![Literal(3).into()])),
    );
    let compiled = Statement(simple).compile(&Compiling::default());
    println!("{:?}", compiled.instructions);
    let expected = vec![
        I32Const(1),
        If(BlockType::NoResult),
        I32Const(2),
        Drop,
        Else,
        I32Const(3),
        Drop,
        End,
    ];
    assert_eq!(compiled.instructions, expected);
}
#[test]
fn context_if_else() {
    let context = Statement(Seq(vec![
        VarDef("a".to_string()),
        Assign("a".to_string(), Box::new(Literal(1))),
    ]));
    let simple = IfElse(
        Box::new(Variable("a".to_string())),
        Box::new(Block(vec![Literal(2).into()])),
        Box::new(Block(vec![Literal(3).into()])),
    );
    let compiled = Block(vec![context, Statement(simple)]).compile(&Compiling::default());
    println!("{:?}", compiled.instructions);
    let expected = vec![
        I32Const(1),
        SetLocal(0),
        GetLocal(0),
        If(BlockType::NoResult),
        I32Const(2),
        Drop,
        Else,
        I32Const(3),
        Drop,
        End,
    ];
    assert_eq!(compiled.instructions, expected);
}
#[test]
fn set_if_else() {
    let context = vec![
        Statement(Seq(vec![
            VarDef("a".to_string()),
            Assign("a".to_string(), Box::new(Literal(1))),
        ])),
        Statement(Seq(vec![
            VarDef("b".to_string()),
            Assign("b".to_string(), Box::new(Literal(1))),
        ])),
    ];
    let simple = IfElse(
        Box::new(Variable("a".to_string())),
        Box::new(Block(vec![Statement(Assign(
            "b".to_string(),
            Box::new(Literal(2)),
        ))])),
        Box::new(Block(vec![Statement(Assign(
            "b".to_string(),
            Box::new(Literal(3)),
        ))])),
    );
    let compiled = Block(context.into_iter().chain(Some(Statement(simple))).collect())
        .compile(&Compiling::default());
    println!("{:?}", compiled.instructions);
    let expected = vec![
        I32Const(1),
        SetLocal(0),
        I32Const(1),
        SetLocal(1),
        GetLocal(0),
        If(BlockType::NoResult),
        I32Const(2),
        SetLocal(1),
        Else,
        I32Const(3),
        SetLocal(1),
        End,
    ];
    assert_eq!(compiled.instructions, expected);
}

impl Into<Statement> for EvalExpr {
    fn into(self) -> Statement {
        Statement(Eval(Box::new(self)))
    }
}
