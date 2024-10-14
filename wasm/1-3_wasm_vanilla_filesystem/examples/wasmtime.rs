use wasmtime::*;
use wasmtime_wasi::*;

struct GlobalState {
  pub string : String,
  pub count : i32,
  pub wasi : wasmtime_wasi::preview1::WasiP1Ctx
}

fn main() {

  let wasm_binary = include_bytes!("../target/wasm32-wasi/release/wasm_vanilla.wasm");

  let mut config = Config::new();
  Config::async_support(&mut config, false);
  // config.wasm_backtrace_details(WasmBacktraceDetails::Enable);
  // config.wasm_component_model(true);
  
  let engine = Engine::new(&config).unwrap();
  let module = Module::new(&engine, wasm_binary).unwrap();

  let mut linker = Linker::<GlobalState>::new(&engine);
  wasmtime_wasi::preview1::add_to_linker_sync(&mut linker, |state: &mut GlobalState| &mut state.wasi).unwrap();
  linker.func_wrap("env", "call_globalstate", |mut caller: Caller<'_, GlobalState>| {
    caller.data_mut().count
  }).unwrap();
  linker.func_wrap("env", "console_log", |mut _caller: Caller<'_, GlobalState>, ptr: i32, len: i32| {
    // let slice = unsafe { std::slice::from_raw_parts(ptr as *const u16, len as usize) };
    // let string = String::from_utf16_lossy(slice);
    println!("console_log called with ptr: {}, len: {}, {}", ptr, len);
    // println!("{}", string.to_string());

  }).unwrap();

  let wasi = WasiCtxBuilder::new()
    .inherit_stdio()
    .inherit_stderr()
    // .envs(&[
    //   ("FOO", "bar"),
    //   ("HOME", "/somewhere"),
    // ])
    .inherit_env()
    .args(&[
      "--flag",
      "true",
    ])
    // .inherit_args()
    .preopened_dir(std::path::PathBuf::from("."), ".", DirPerms::all(), FilePerms::all()).unwrap()
    .build_p1();
  let mut store = Store::new( &engine, GlobalState {
    string : "hello world".to_string(),
    count : 3,
    wasi
  });
  let instance = linker.instantiate(&mut store, &module).unwrap();

  let func = instance.get_typed_func::<(), ()>(&mut store, "run").unwrap();
  let _ = func.call(&mut store, ()).unwrap();

}
