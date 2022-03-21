use metered_wasmi::RuntimeValue;

use crate::ast::compile::{Compile, Compiling};
use crate::parsers::block;
use crate::wasm::{module_by_compiling, run_module};

#[test]
fn string_to_result_sample() {
    string_to_result("1+3-2", 2)
}
#[test]
fn var() {
    string_to_result(
        "let a
    a=2
    a",
        2,
    )
}
#[test]
fn var_more() {
    string_to_result(
        "let a
    a=2
    let b
    b = a+3
    a",
        2,
    )
}
#[test]
fn var_more_2() {
    string_to_result(
        "let a
    a=2
    let b
    b = a+3
    b",
        5,
    )
}
#[test]
fn var_three() {
    string_to_result(
        "let a
    a=2
    let b
    b = a+3
    let c
    c = 1+a+ b -7
    c",
        1,
    )
}
#[test]
fn var_three_short() {
    string_to_result(
        "let a
    a=2
    let b = a+3
    let c = 1+a+ b -7
    c",
        1,
    )
}
fn string_to_result(s: &str, re: i32) {
    let exp = block(s);
    assert!(exp.is_ok());
    let r = exp.unwrap();
    println!("{:?}", r);
    let ins = r.1.compile(Compiling::default());
    println!("{:?}", ins.instructions);
    println!("{:?}", ins.locals);
    let module = module_by_compiling(ins);
    assert!(module.sections().len() > 0);
    let result = run_module(module);
    assert_eq!(result, RuntimeValue::I32(re));
}
