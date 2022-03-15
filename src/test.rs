use metered_wasmi::RuntimeValue;

use crate::parsers::{add_sub};
use crate::ast::compile::Compilable;
use crate::wasm::{module_with_single_function, run_module};
#[test]
fn string_to_result(){
    let s = "1+3-2";
    let exp = add_sub(s);
    assert!(exp.is_ok());
    let ins = exp.unwrap().1.to_instructions();
    assert!(ins.len()>0);
    let module = module_with_single_function(ins,None);
    assert!(module.sections().len()>0);
    let result = run_module(module);
    assert_eq!(result,RuntimeValue::I32(2));
}