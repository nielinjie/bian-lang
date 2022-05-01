use crate::parsers::{statement, program};

#[test]
fn statement_simple() {
    let s = "hello\n";
    let ast = statement(s);
    assert!(ast.is_ok());
    let s = "hello";
    let ast = statement(s);
    assert!(ast.is_err());
}
#[test]
fn block_simple() {
    let s = "hello\n";
    let ast = program(s);
    assert!(ast.is_ok());
    let s = "hello\nworld\n";
    let ast = program(s);
    assert!(ast.is_ok());
    let s = "hello\nworld";
    let ast = program(s);
    assert!(!ast.is_ok())
}

#[test]
fn program_statement() {
    let s ="let a
    a=2
    return a\n";
    let ast  = program(s);
    println!("{:#?}",ast);
    assert!(ast.is_ok());
}

#[test]
fn program_if_else() {
    let s = "let a
    a=2
    let b
    if (a) {
        b =1
    } else {
        b = 2
    }
    return b\n";
    let ast = program(s);
    println!("{:#?}",ast);
    assert!(ast.is_ok());
}