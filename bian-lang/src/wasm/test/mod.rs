extern crate metered_wasmi;

use super::*;
use metered_wasmi::RuntimeValue;
use parity_wasm::elements::Instruction;
use Instruction::*;

#[test]
fn simple() {
    use parity_wasm::builder;

    let module = builder::module()
        .function()
        .signature()
        .param()
        .i32()
        .build()
        .body()
        .build()
        .build()
        .build();

    assert_eq!(
        module
            .type_section()
            .expect("type section to exist")
            .types()
            .len(),
        1
    );
    assert_eq!(
        module
            .function_section()
            .expect("function section to exist")
            .entries()
            .len(),
        1
    );
    assert_eq!(
        module
            .code_section()
            .expect("code section to exist")
            .bodies()
            .len(),
        1
    );
}


fn sample_by_builder() -> Vec<u8> {
    module_with_single_function(vec![I32Const(1337)], None, None)
        .to_bytes()
        .unwrap()
}
fn sample_add_by_builder() -> Vec<u8> {
    module_with_single_function(vec![I32Const(1), I32Const(2), I32Add], None, None)
        .to_bytes()
        .unwrap()
}


#[test]
fn sample_builder_run() {
    assert_run(sample_by_builder(), RuntimeValue::I32(1337));
}
#[test]
fn sample_add_builder_run() {
    assert_run(sample_add_by_builder(), RuntimeValue::I32(3));
}

#[test]
fn sample_variable_run() {
    with_instructions(
        vec![I32Const(1337), SetLocal(0), GetLocal(0)],
        vec![Local::new(1, ValueType::I32)],
        RuntimeValue::I32(1337),
    )
}

pub fn with_instructions(instructions: Vec<Instruction>, locals: Vec<Local>, result: RuntimeValue) {
    assert_run(
            module_with_single_function(instructions, None, Some(locals))
                .to_bytes()
                .unwrap()
        ,
        result
    )
}

fn assert_run(module:Vec<u8>,re:RuntimeValue){
        let r=run(module);
        match r{
             Ok(rv) => assert_eq!(rv,re),
             r => panic!("{:?}",r)
        }

}

mod variable;
mod flows;
