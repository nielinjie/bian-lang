use metered_wasmi::RuntimeValue;
use nom::Finish;

use crate::ast::compile::{Compile, Compiling};
use crate::parsers::program;
use crate::wasm::{module_by_compiling, run_module};
use crate::Error;
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
fn run(s: &str) -> Result<i32, Error> {
    let (_input, exp) = program(s).finish()?;
    let compiling = exp.compile(Compiling::default());
    if compiling.errors.is_empty() {
        let module = module_by_compiling(compiling);
        let re = run_module(module);
        match re {
            Ok(RuntimeValue::I32(i)) => Ok(i),
            _ => Err(Error::RuntimeError(format!(
                "not supported type - {:?}",
                re
            ))),
        }
    } else {
        Err(Error::CompileError(format!(
            "compiling error - {:?}",
            compiling.errors
        )))
    }
}
impl From<nom::error::Error<&str>> for Error {
    fn from(e: nom::error::Error<&str>) -> Self {
        Error::ParseError(format!("{:?}", e))
    }
}
mod variable;
mod error;
