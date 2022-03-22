use parity_wasm::elements::Instruction::*;

use crate::{ast::Operator, Error};

use super::super::{
    compile::{Compile, Compiling},
    Expr,
};

#[test]
fn var_def_compile() {
    let ast = Expr::VarDef("foo".to_string());
    let result = ast.compile(Compiling::default());
    assert_eq!(
        result,
        Compiling {
            locals: vec!["foo".to_string()],
            ..Compiling::default()
        }
    );
}
#[test]
fn variable_compile() {
    let ast = Expr::VarDef("foo".to_string());
    let ast2 = Expr::VarDef("bar".to_string());
    let result = ast.compile(Compiling::default());
    let r = ast2.compile(result);
    assert_eq!(
        r,
        Compiling {
            locals: vec!["foo".to_string(), "bar".to_string()],
            ..Compiling::default()
        }
    );
}
#[test]
fn variable_existed_error() {
    let ast = Expr::VarDef("foo".to_string());
    let ast2 = Expr::VarDef("foo".to_string());
    let result = ast.compile(Compiling::default());
    let r = ast2.compile(result);
    assert_eq!(
        r,
        Compiling {
            locals: vec!["foo".to_string()],
            errors: vec![Error::CompileError("existed var - foo".to_string())],
            ..Compiling::default()
        }
    );
}

#[test]
fn variable_and_get() {
    let ast = Expr::VarDef("foo".to_string());
    let ast2 = Expr::Variable("foo".to_string());
    let result = ast.compile(Compiling::default());
    let r = ast2.compile(result);
    assert_eq!(
        r,
        Compiling {
            instructions: vec![GetLocal(0)],
            locals: vec!["foo".to_string()],
            ..Compiling::default()
        }
    );
}

#[test]
fn variable_and_unknown() {
    let ast = Expr::VarDef("foo".to_string());
    let ast2 = Expr::Variable("bar".to_string());
    let result = ast.compile(Compiling::default());
    let r = ast2.compile(result);
    assert_eq!(
        r,
        Compiling {
            locals: vec!["foo".to_string()],
            errors: vec![Error::CompileError("unknown var - bar".to_string())],
            ..Compiling::default()
        }
    );
}

#[test]
fn variable_set() {
    let ast = Expr::VarDef("foo".to_string());

    let ast2 = Expr::Assign("foo".to_string(), Box::new(Expr::Int(42)));
    let result = ast.compile(Compiling::default());
    let r = ast2.compile(result);
    assert_eq!(
        r,
        Compiling {
            instructions: vec![I32Const(42), SetLocal(0)],
            locals: vec!["foo".to_string()],
            ..Compiling::default()
        }
    );
}
#[test]
fn variable_set_unknown() {
    let ast = Expr::VarDef("foo".to_string());

    let ast2 = Expr::Assign("bar".to_string(), Box::new(Expr::Int(42)));
    let result = ast.compile(Compiling::default());
    let r = ast2.compile(result);
    assert_eq!(
        r,
        Compiling {
            instructions: vec![I32Const(42)],
            locals: vec!["foo".to_string()],
            errors: vec![Error::CompileError("unknown var - bar".to_string())],
        }
    );
}
#[test]
fn variable_scene() {
    use Expr::*;
    /*
    let a
    let b
    b=42
    a  = b
    a
     */

    let a1 = VarDef("a".to_string());
    let a2 = VarDef("b".to_string());
    let a3 = Assign("b".to_string(), Box::new(Int(42)));
    let a4 = Assign("a".to_string(), Box::new(Variable("b".to_string())));
    let a5 = Variable("a".to_string());
    let c = vec![a1, a2, a3, a4, a5]
        .into_iter()
        .fold(Compiling::default(), |c, a| a.compile(c));
    assert_eq!(
        c,
        Compiling {
            instructions: vec![
                I32Const(42),
                SetLocal(1),
                GetLocal(1),
                SetLocal(0),
                GetLocal(0)
            ],
            locals: vec!["a".to_string(), "b".to_string()],
            ..Compiling::default()
        }
    )
}
#[test]
fn variable_scene_plus() {
    use Expr::*;
    /*
    let a
    let b
    b=42
    a  = b +7
    a
     */

    let a1 = VarDef("a".to_string());
    let a2 = VarDef("b".to_string());
    let a3 = Assign("b".to_string(), Box::new(Int(42)));
    let a4 = Assign(
        "a".to_string(),
        Box::new(BinaryExpr {
            op: Operator::Plus,
            left: Box::new(Variable("b".to_string())),
            right: Box::new(Int(7)),
        }),
    );
    let a5 = Variable("a".to_string());
    let b = Block(vec![a1, a2, a3, a4, a5]);
    let c = b.compile(Compiling::default());
    assert_eq!(
        c,
        Compiling {
            instructions: vec![
                I32Const(42),
                SetLocal(1),
                GetLocal(1),
                I32Const(7),
                I32Add,
                SetLocal(0),
                GetLocal(0)
            ],
            locals: vec!["a".to_string(), "b".to_string()],
            ..Compiling::default()
        }
    )
}
#[test]
fn variable_scene_plus_three() {
    use Expr::*;
    /*
    let a
    let b
    b=42
    a  = b +7
    c =  a+b-21
    c
     */

    let a1 = VarDef("a".to_string());
    let a2 = VarDef("b".to_string());
    let a = VarDef("c".to_string());
    let a3 = Assign("b".to_string(), Box::new(Int(42)));
    let a4 = Assign(
        "a".to_string(),
        Box::new(BinaryExpr {
            op: Operator::Plus,
            left: Box::new(Variable("b".to_string())),
            right: Box::new(Int(7)),
        }),
    );
    let a41 = Assign(
        "c".to_string(),
        Box::new(BinaryExpr {
            op: Operator::Minus,
            right: Box::new(Int(21)),
            left: Box::new(BinaryExpr {
                op: Operator::Plus,
                left: Box::new(Variable("a".to_string())),
                right: Box::new(Variable("b".to_string())),
            }),
        }),
    );
    let a5 = Variable("c".to_string());

    let b = Block(vec![a1, a2, a, a3, a4, a41, a5]);
    let c = b.compile(Compiling::default());
    assert_eq!(
        c,
        Compiling {
            instructions: vec![
                I32Const(42),
                SetLocal(1),
                GetLocal(1),
                I32Const(7),
                I32Add,
                SetLocal(0),
                GetLocal(0),
                GetLocal(1),
                I32Add,
                I32Const(21),
                I32Sub,
                SetLocal(2),
                GetLocal(2)
            ],
            locals: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            ..Compiling::default()
        }
    )
}
