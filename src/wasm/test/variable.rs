use metered_wasmi::RuntimeValue;
use parity_wasm::elements::{Instruction, Local, ValueType};
use Instruction::*;

use super::test::with_instructions;
#[test]
fn two_variables_run() {
    with_instructions(
        vec![
            I32Const(1337),
            SetLocal(0),
            GetLocal(0),
            Drop,
            I32Const(42),
            SetLocal(1),
            GetLocal(1),
        ],
        vec![Local::new(2, ValueType::I32)],
        RuntimeValue::I32(42),
    );
    with_instructions(
        vec![
            I32Const(1337),
            SetLocal(0),
            GetLocal(0),
            I32Const(42),
            SetLocal(1),
            GetLocal(1),
            Drop,
        ],
        vec![Local::new(2, ValueType::I32)],
        RuntimeValue::I32(1337),
    );
    with_instructions(
        vec![
            I32Const(1337),
            SetLocal(0),
            I32Const(42),
            SetLocal(1),
            GetLocal(1),
        ],
        vec![Local::new(2, ValueType::I32)],
        RuntimeValue::I32(42),
    );
    with_instructions(
        vec![
            I32Const(1337),
            SetLocal(0),
            I32Const(42),
            SetLocal(1),
            GetLocal(1),
        ],
        vec![Local::new(2, ValueType::I32)],
        RuntimeValue::I32(42),
    )
}
#[test]
fn need_no_init() {
    with_instructions(
        vec![GetLocal(0)],
        vec![Local::new(2, ValueType::I32)],
        RuntimeValue::I32(0),
    );
}
#[test]
fn variable_scene() {
    with_instructions(
        /*
        let a
        let b
        b=42
        a  = b
        a
         */
        vec![
            I32Const(42),
            SetLocal(1),
            GetLocal(1),
            SetLocal(0),
            GetLocal(0),
        ],
        vec![Local::new(2, ValueType::I32)],
        RuntimeValue::I32(42),
    )
}
#[test]
fn variable_scene_plus() {
    with_instructions(
        /*
        let a
        let b
        b=42
        a  = b+7
        a
         */
        vec![
            I32Const(42),
            SetLocal(1),
            GetLocal(1),
            I32Const(7),
            I32Add,
            SetLocal(0),
            GetLocal(0),
        ],
        vec![Local::new(2, ValueType::I32)],
        RuntimeValue::I32(49),
    )
}
#[test]
fn variable_scene_plus_three() {
    with_instructions(
        /*
        let a
        let b
        let c
        b=42
        a  = b+7
        c = a+b -21
        c
         */
        vec![
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
            GetLocal(2),
        ],
        vec![Local::new(3, ValueType::I32)],
        RuntimeValue::I32(70),
    )
}
