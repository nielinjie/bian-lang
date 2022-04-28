use metered_wasmi::RuntimeValue;
use parity_wasm::elements::{BlockType, Instruction, Local, ValueType};
use Instruction::*;

use super::with_instructions;

#[test]
fn simple_if_else() {
    with_instructions(
        vec![
            I32Const(1),
            If(BlockType::NoResult),
            I32Const(2),
            Drop,
            Else,
            I32Const(3),
            Drop,
            End,
            I32Const(4),
        ],
        vec![],
        RuntimeValue::I32(4),
    )
}
#[test]
fn context_if_else() {
    with_instructions(
        vec![
            I32Const(1),
            SetLocal(0),
            GetLocal(0),
            If(BlockType::NoResult),
            I32Const(2),
            Drop,
            Else,
            I32Const(3),
            Drop,
            End,
            GetLocal(0),
        ],
        vec![Local::new(1, ValueType::I32)],
        RuntimeValue::I32(1),
    )
}

#[test]
fn set_if_else() {
    with_instructions(
        vec![
            I32Const(1),
            SetLocal(0),
            I32Const(1),
            SetLocal(1),
            GetLocal(0),
            If(BlockType::NoResult),
            I32Const(2),
            SetLocal(1),
            Else,
            I32Const(3),
            SetLocal(1),
            End,
            GetLocal(1),
        ],
        vec![Local::new(2, ValueType::I32)],
        RuntimeValue::I32(2),
    )
}
