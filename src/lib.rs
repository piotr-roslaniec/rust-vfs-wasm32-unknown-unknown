use vfs::MemoryFS;
use wasm_bindgen::prelude::*;

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}


// #[wasm_bindgen]
// struct WasmVFS(MemoryFS);
//
// #[wasm_bindgen]
// pub fn setup_vfs(files: JsValue) -> WasmVFS {
//     set_panic_hook();
//     wasm_logger::init(wasm_logger::Config::default());
//
//     // Deserialize the JsValue into a HashMap<String, Vec<u8>>
//     let files: HashMap<String, Vec<u8>> = serde_wasm_bindgen::from_value(files).unwrap();
//
//     // Create a new virtual file system using the embedded files
//     let fs = MemoryFS::new();
//
//     // Map the requested files to virtual file system paths
//     for (path, content) in files {
//         fs.create_file(&path).unwrap().write_all(&content).unwrap();
//     }
//
//     WasmVFS(fs)
// }

#[wasm_bindgen]
pub fn reproduce_bug() {
    set_panic_hook();

    MemoryFS::new();
}
