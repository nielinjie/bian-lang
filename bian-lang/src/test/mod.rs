use metered_wasmi::RuntimeValue;

use crate::ast::compile::{Compile, Compiling};
use crate::ast::transform::{BlockTransform, Transform};
use crate::parsers::program;
use crate::wasm::{module_by_compiling, run_module};
#[test]
fn string_to_result_sample() {
    string_to_result("return 1+3-2\n", 2)
}

fn string_to_result(s: &str, re: i32) {
    let exp = program(s);
    assert!(exp.is_ok());
    let r = exp.unwrap();
    println!("{:?}", r);
    let block = r.1;

    let transformed = BlockTransform::transform(&block);

    let ins = transformed.unwrap().unwrap().compile(&Compiling::default());
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

mod error;
mod flows;
mod variable;
