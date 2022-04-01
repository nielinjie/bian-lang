import { invoke } from "@tauri-apps/api/tauri";

export function compute(code){
    return invoke("compute_and_represent",{code});
}