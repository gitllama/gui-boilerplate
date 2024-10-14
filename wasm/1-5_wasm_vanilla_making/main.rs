use wasmtime::*;
use wasmtime_wasi::*;

// https://keens.github.io/blog/2020/06/07/wasmtimenoimportsnirustnokansuuwosashikomu/

rusty_v8 = "0.32.1" -> v8に変更
https://github.com/denoland/rusty_v8
use rusty_v8 as v8;

fn main() {

  let mut linker = Linker::<GlobalState>::new(&engine);
  wasmtime_wasi::preview1::add_to_linker_sync(&mut linker, |state: &mut GlobalState| &mut state.wasi).unwrap();
  linker.func_wrap("env", "call_globalstate", |mut caller: Caller<'_, GlobalState>| {
    caller.data_mut().count
  }).unwrap();
  linker.func_wrap("env", "console_log", |mut _caller: Caller<'_, GlobalState>, (ptr, len):(i32, i32)| {
    println!("console_log called with ptr: {}, len: {}", ptr, len);
  }).unwrap();


  let platform = v8::new_default_platform(0, false).make_shared();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();

  let isolate = &mut v8::Isolate::new(Default::default());

  let scope = &mut v8::HandleScope::new(isolate);
  let context = v8::Context::new(scope);
  let scope = &mut v8::ContextScope::new(scope, context);

  let rust_bytes: Vec<u8> = vec![1u8,2,3,4,5];
  let local_bytes = v8::ArrayBuffer::new(scope, rust_bytes.len());
  {
    let mut backing_store = local_bytes.get_backing_store();
    backing_store.
  }
  let mut buf = v8::Uint8Array::new(scope, local_bytes, 0, local_bytes.byte_length()).unwrap();
  
  let mut buf_data = buf
  // let mut buf_data = buf.copy_contents(rust_bytes.as_mut_slice());

  // buf_data.copy_from_slice(&rust_bytes);

  // Uint8Arrayオブジェクトの作成
  let u8_array = v8::Local::<v8::Uint8Array>::new(scope, buf);

  // JavaScript コード
  let src = r#"
      const vs = [1, 2, 3, 4, 5]
      console.log(vs)
      vs.reduce((acc, v) => acc + v, 0)
  "#;

  v8::String::new(scope, src)
      .map(|code| {
          println!("code: {}", code.to_rust_string_lossy(scope));
          code
      })
      .and_then(|code| v8::Script::compile(scope, code, None)) //コンパイル
      .and_then(|script| script.run(scope)) //実行
      .and_then(|value| value.to_string(scope)) // rusty_v8::Value を rusty_v8::String へ
      .iter()
      .for_each(|s| println!("result: {}", s.to_rust_string_lossy(scope)));
}
