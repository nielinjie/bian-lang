use metered_wasmi::RuntimeValue;
use metered_wasmi::{ImportsBuilder, ModuleInstance, NopExternals};
use parity_wasm::elements::{
    ExportEntry, FuncBody, Instruction, Instructions, Internal, Local, Module, ValueType,
};

fn append_to_new<I: Clone>(v: Vec<I>, item: I) -> Vec<I> {
    let mut re = v.clone();
    re.push(item);
    re
    // Vector::from(v).push_back(item).
}
pub fn module_with_single_function(
    codes: Vec<Instruction>,
    name: Option<&str>,
    locals: Option<Vec<Local>>,
) -> Module {
    use parity_wasm::builder;

    builder::module()
        .function()
        .signature()
        .with_return_type(Some(ValueType::I32))
        .build()
        .with_body(FuncBody::new(
            locals.unwrap_or_default(),
            Instructions::new(append_to_new(codes, Instruction::End)),
        ))
        .build()
        .with_export(ExportEntry::new(
            name.unwrap_or("test").to_string(),
            Internal::Function(0),
        ))
        .build()
}
pub fn run_module(module: Module) -> RuntimeValue {
    module.to_bytes().map(|b| run(b)).unwrap()
}
fn run(wasm: Vec<u8>) -> RuntimeValue {
    let module = metered_wasmi::Module::from_buffer(&wasm).expect("failed to load wasm");

    // Instantiate a module with empty imports and
    // assert that there is no `start` function.
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    // Finally, invoke the exported function "test" with no parameters
    // and empty external function executor.
    let result = instance
        .invoke_export("test", &[], &mut NopExternals)
        .unwrap()
        .unwrap();
    result
}
#[cfg(test)]
pub mod test;
#[cfg(test)]
pub mod test_variable;
