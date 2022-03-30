extern crate bian_lang;
extern crate wasm_bindgen;
use bian_lang::ast::compile::Compiling;
use bian_lang::*;
use wasm_bindgen::prelude::*;

// use bian_lang::*;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let s = bian_lang::greet();
    alert(format!("String from Rust - {}", s).as_str());
}
pub fn compute(e: String) -> (Result<Compiling, Error>, Result<i32, Error>) {
    let compiling = compile(e.as_str());
    let running = compiling
        .clone()
        .and_then(|c| run(e.as_str(), Some(c.clone())));
    (compiling, running)
}
#[wasm_bindgen]
pub fn compute_and_represent(e: String) -> JsValue {
    let results = compute(e);
    let compiling_r = results.0.map(|c| CompilingRepresent::from_compiling(&c));
    let re = (compiling_r, results.1);
    JsValue::from_serde(&re).unwrap()
}
