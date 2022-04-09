use metered_wasmi::RuntimeValue;


use crate::ast::compile::{Compile, Compiling};
use crate::parsers::program;
use crate::{wasm::{module_by_compiling, run_module}};
#[test]
fn string_to_result_sample() {
    string_to_result("1+3-2", 2)
}

fn string_to_result(s: &str, re: i32) {
    let exp = program(s);
    assert!(exp.is_ok());
    let r = exp.unwrap();
    println!("{:?}", r);
    let ins = r.1.compile(Compiling::default());
    println!("{:?}", ins.instructions);
    println!("{:?}", ins.locals);
    let module = module_by_compiling(ins);
    assert!(module.sections().len() > 0);
    let result = run_module(module);
    match result {
        Ok(RuntimeValue::I32(rv)) => assert_eq!(rv, re),
        r => panic!("{:?}", r),
    }
}

mod variable;
mod error;
mod flows;
