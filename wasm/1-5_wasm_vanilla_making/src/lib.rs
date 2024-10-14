// cargo build --target wasm32-wasi --release

use serde_json::json;
use wasm_library::pdk::*;

#[no_mangle]
pub fn run() {
  println!("args : {:?}", args_value());
  result(json!({ "result" : false }));
}


// #[wasm_bindgen]
// struct GlobalState {
//   name: String,
//   count: usize,
// }

// #[no_mangle]
// pub fn console_write() {
//   let utf16: Vec<u16> = String::from("こんにちわ").encode_utf16().collect();
//   unsafe { console_log(utf16.as_ptr(), utf16.len()); }
// }