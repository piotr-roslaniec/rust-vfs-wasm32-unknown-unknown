use vfs::MemoryFS;
use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn reproduce_bug() {
    set_panic_hook();

    MemoryFS::new();
}
