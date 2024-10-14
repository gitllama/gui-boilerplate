use wasmtime::*;

fn main() {

  let wasm_binary = include_bytes!("../target/wasm32-unknown-unknown/release/wasm_vanilla.wasm");

  println!("Compiling module...");
  let engine = Engine::default();
  let module = Module::new(&engine, wasm_binary).unwrap();

  println!("Initializing...");
  let mut store = Store::new(&engine, ());

  println!("Creating callback...");
  let hello = Func::wrap(&mut store, || { println!("Hello World"); });
  let add_one = Func::wrap(&mut store, |src:i32| { Ok(src+1) });
  let date_now = Func::wrap(&mut store, || { Ok(0f64) });

  println!("Instantiating module...");
  let imports = [hello.into(), add_one.into(), date_now.into()];
  let instance = Instance::new(&mut store, &module, &imports).unwrap();

  println!("Extracting export...");
  let run = instance.get_typed_func::<(), ()>(&mut store, "run").unwrap();
  let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add").unwrap();

  println!("Calling export...");
  let _ = run.call(&mut store, ()).unwrap();
  println!(" 3 + 4 + 1 = {}", add.call(&mut store, (3, 4)).unwrap());
  let _ = run.call(&mut store, ()).unwrap();
  println!(" 3 + 4 + 1 = {}", add.call(&mut store, (3, 4)).unwrap());

}