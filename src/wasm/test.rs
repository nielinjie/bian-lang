extern crate metered_wasmi;
extern crate wabt;

use super::*;
use metered_wasmi::{RuntimeValue};
use parity_wasm::elements::Instruction;

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

fn sample_wasm() -> Vec<u8> {
    wabt::wat2wasm(
        r#"
            (module
                (func (export "test") (result i32)
                    i32.const 1337
                )
            )
            "#,
    )
    .unwrap()
}
fn sample_by_builder() -> Vec<u8> {
    module_with_single_function(vec![Instruction::I32Const(1337)], None)
        .to_bytes()
        .unwrap()
}
fn sample_add_by_builder() -> Vec<u8> {
    module_with_single_function(
        vec![
            Instruction::I32Const(1),
            Instruction::I32Const(2),
            Instruction::I32Add,
        ],
        None,
    )
    .to_bytes()
    .unwrap()
}

#[test]
fn sample_run() {
    assert_eq!(run(sample_wasm()), RuntimeValue::I32(1337));
}
#[test]
fn sample_builder_run() {
    assert_eq!(run(sample_by_builder()), RuntimeValue::I32(1337));
}
#[test]
fn sample_add_builder_run() {
    assert_eq!(run(sample_add_by_builder()), RuntimeValue::I32(3));
}
