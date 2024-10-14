use serde_json::json;
use wasmtime::*;
use wasmtime_wasi::*;

#[allow(dead_code)]
struct GlobalState {
  pub args : Option<String>,
  pub dst : Option<String>,
  pub wasi : wasmtime_wasi::preview1::WasiP1Ctx,
  // pub ptr : i32,
  // pub len : i32
}

impl GlobalState {
  fn new() -> GlobalState {
    GlobalState {
      args : None,
      dst : None,
      wasi : WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .build_p1()
    }
  }
  // fn get_vec(memory: &Memory) -> Vec<u8> {
  //   let data = memory.data_mut(&mut store);
  //   let slice_copy: Vec<u8> = data[ptr..ptr + len].to_vec();
  //   let json = std::str::from_utf8(&slice_copy).unwrap();  
  // }
}

trait Ffi {
  fn set_args(&mut self, src: serde_json::Value);
  fn get_return(&mut self) -> serde_json::Value;
  fn set_memory(&mut self, memory: Memory, src: Vec<u8>);
  fn get_memory(&mut self, memory: Memory) -> Vec<u8>;
}

impl Ffi for Store<GlobalState> {
  fn set_args(&mut self, src: serde_json::Value) {
    let data = self.data_mut();
    data.args = Some(src.to_string());
  }
  fn get_return(&mut self) -> serde_json::Value {
    let data = self.data();
    match &data.dst {
      Some(n) => serde_json::from_str(n).unwrap(),
      None => json!({})
    }
  }
  fn set_memory(&mut self, memory: Memory, src: Vec<u8>) {
    /*
      let memory = instance.get_memory(&mut store, "memory").unwrap();
      let ptr = resize.call(&mut store, src_len).unwrap();
      memory.write(&mut store, ptr as usize, src_bytes).unwrap();
    */
  }
  fn get_memory(&mut self, memory: Memory) -> Vec<u8> {
    /* let memory = instance.get_memory(&mut store, "memory").unwrap(); */
    let ptr = 0; // get_ptr.call(&mut store, ()).unwrap() as usize;
    let len = 1; // get_size.call(&mut store, ()).unwrap() as usize;
    let data = memory.data_mut(self);
    let slice_copy: Vec<u8> = data[ptr..ptr + len].to_vec();
    slice_copy
  }
}

trait GlobalStateLinker {
  fn set_imports(&mut self);
}

impl GlobalStateLinker for Linker<GlobalState> {
  fn set_imports(&mut self) {
    self.func_wrap("env", "get_args_size", |mut caller: Caller<'_, GlobalState>| -> i32 {
      match &caller.data_mut().args {
        Some(n) => n.len() as i32,
        None => 2
      }
    }).unwrap();
    
    self.func_wrap("env", "get_args", |mut caller: Caller<'_, GlobalState>, ptr: i32, len: i32| {
      let args = match &caller.data_mut().args {
        Some(n) => n.to_string(),
        None => "{}".to_string()
      };
      let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
      memory.write(&mut caller, ptr as usize, args.as_bytes()).unwrap();
    }).unwrap();
  
    self.func_wrap("env", "set_result", |mut caller: Caller<'_, GlobalState>, ptr: i32, len: i32| {
  
      let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
      let data = memory.data(&caller);
      let key = &data[ptr as usize..(ptr + len) as usize];
      let key_str = std::str::from_utf8(key).unwrap();
  
      caller.data_mut().dst = Some(key_str.to_string());
    }).unwrap();
  }
}


fn main() {

  let wasm_binary = include_bytes!("../target/wasm32-wasi/release/wasm_vanilla.wasm");

  let mut config = Config::new();
  Config::async_support(&mut config, false);
  
  let engine = Engine::new(&config).unwrap();
  let module = Module::new(&engine, wasm_binary).unwrap();
  let mut store = Store::new( &engine, GlobalState::new());

  let mut linker = Linker::<GlobalState>::new(&engine);
  wasmtime_wasi::preview1::add_to_linker_sync(&mut linker, |state: &mut GlobalState| &mut state.wasi).unwrap();
  linker.set_imports();

  let instance = linker.instantiate(&mut store, &module).unwrap();

  let run = instance.get_typed_func::<(), ()>(&mut store, "run").unwrap();
  let mut call_run = |src: serde_json::Value| -> serde_json::Value {
    store.set_args(src);
    let _ = run.call(&mut store, ()).unwrap();
    store.get_return()
  };

  let dst = call_run(json!({ "input" : 10 }));
  println!("{:?}", dst);
  let dst = call_run(json!({ "input" : 40 }));
  println!("{:?}", dst);

}

