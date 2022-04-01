#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]



extern crate bian_lang;
use bian_lang::ast::compile::Compiling;
use bian_lang::*;

fn main() {
    tauri::Builder::default().invoke_handler(tauri::generate_handler![compute_and_represent])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub fn compute(e: String) -> (Result<Compiling, Error>, Result<i32, Error>) {
    let compiling = compile(e.as_str());
    let running = compiling
        .clone()
        .and_then(|c| run(e.as_str(), Some(c.clone())));
    (compiling, running)
}

#[tauri::command]
fn compute_and_represent(code: String) -> (Result<CompilingRepresent, Error>, Result<i32, Error>) {
    let results = compute(code);
    let compiling_r = results.0.map(|c| CompilingRepresent::from_compiling(&c));
    (compiling_r, results.1)
}
