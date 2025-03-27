// wasm-pack build --target web

use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_timer::Delay;


#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
  println!("hello");
  a + b
}

#[wasm_bindgen]
pub async fn sleep_millis(numbers: u16) -> js_sys::Promise {
  let millis: u64 = u64::from(numbers);
  Delay::new(Duration::from_millis(millis)).await.unwrap();
  let promise = js_sys::Promise::resolve(&numbers.into());
  return promise;
}

#[wasm_bindgen]
pub async fn print(src: &str) {
  println!("{}", src);
}

/*
1. wasm_bindgenから生成されるpassStringToWasm0でptrを取得
  1-1. __wbindgen_malloc  : メモリを割り当てる（確保する）wasm method
  1-2. __wbindgen_realloc : メモリブロックを再割り当て（サイズを変更）

static new(name, age) {
  const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
  const len0 = WASM_VECTOR_LEN;
  _assertNum(age);
  const ret = wasm.person_new(ptr0, len0, age);
  return Person.__wrap(ret);
}
*/

