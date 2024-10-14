use wasmtime::*;
use wasmtime_wasi::*;

fn main() {

  let wasm_binary = include_bytes!("../target/wasm32-wasi/release/wasm_vanilla.wasm");

  let mut config = Config::new();
  Config::async_support(&mut config, false);
  // config.wasm_backtrace_details(WasmBacktraceDetails::Enable);
  // config.wasm_component_model(true);
  
  let engine = Engine::new(&config).unwrap();
  let module = Module::new(&engine, wasm_binary).unwrap();

  let wasi = WasiCtxBuilder::new()
    .inherit_stdio()
    .inherit_args()
    .build_p1();

  let mut linker = Linker::<wasmtime_wasi::preview1::WasiP1Ctx>::new(&engine);
  wasmtime_wasi::preview1::add_to_linker_sync(&mut linker, |t| t).unwrap();
 
  let mut store = Store::new( &engine, wasi);
  let instance = linker.instantiate(&mut store, &module).unwrap();

  let func = instance.get_typed_func::<(), ()>(&mut store, "run").unwrap();
  let _ = func.call(&mut store, ()).unwrap();

}