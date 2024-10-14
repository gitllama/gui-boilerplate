use wasmtime::*;
use wasmtime_wasi::*;

struct GlobalState {
  pub string : String,
  pub count : i32,
  pub wasi : wasmtime_wasi::preview1::WasiP1Ctx
}

/*
線形メモリへの直接書き込み

wasm-bindgenによる抽象化

  #[wasm_bindgen]
  pub fn greet(message: &str) { }

WIT 

*/


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
  linker.func_wrap("env", "console_log", |mut caller: Caller<'_, GlobalState>, ptr: i32, len: i32| {
    println!("console_log called with ptr: {}, len: {}", ptr, len);

    let memory = caller.get_export("memory").unwrap().into_memory().unwrap();
    let data = memory.data(&caller);
    let key = &data[ptr as usize..(ptr + len) as usize];
    let key_str = std::str::from_utf8(key).unwrap();

    // let linear_memory = memory.data_mut(&mut caller).as_mut_ptr();
    // unsafe {
    //     // the problem happened around here, but I don't know how to fix it.
    //     let add: *mut extern "C" fn(i32, i32) -> i32 = linear_memory.add(_add as usize).cast();
    //     test_func(*add)
    // }   

    println!("{}", key_str);
  }).unwrap();

  let wasi = WasiCtxBuilder::new()
    .inherit_stdio()
    .inherit_args()
    .build_p1();
  let mut store = Store::new( &engine, GlobalState {
    string : "hello world".to_string(),
    count : 3,
    wasi
  });

  let instance = linker.instantiate(&mut store, &module).unwrap();
  // let memory = instance.get_memory(&mut store, "memory").unwrap();
  // let memory = instance.get_memory("memory").unwrap();

  let _run = instance.get_typed_func::<i32, ()>(&mut store, "run").unwrap();
  let print = instance.get_typed_func::<(i32, i32), ()>(&mut store, "print").unwrap();
  let allocate = instance.get_typed_func::<i32, i32>(&mut store, "allocate").unwrap();
  let dealloc = instance.get_typed_func::<(i32, i32), ()>(&mut store, "dealloc").unwrap();
  let memory = instance.get_memory(&mut store, "memory").unwrap();
  let current_pages = memory.size(&store);
  println!("current_pages : {}", current_pages); // memory.grow(&mut store, additional_pages)?;

  let src = "hello world";
  {
    let ptr = allocate.call(&mut store, src.len() as i32).unwrap() as usize;

    {
      let data = memory.data_mut(&mut store);
      let slice_copy: Vec<u8> = data[ptr..ptr + src.len()].to_vec();
      let a = std::str::from_utf8(&slice_copy).unwrap();
      println!("{} {:?} {}", ptr, slice_copy, a);
    }
    memory.write(&mut store, ptr as usize, src.as_bytes()).unwrap();
    {
      let data = memory.data_mut(&mut store);
      let slice_copy: Vec<u8> = data[ptr..ptr + src.len()].to_vec();
      let a = std::str::from_utf8(&slice_copy).unwrap();
      println!("{} {:?} {}", ptr, slice_copy, a);
    }
    
    let _ = print.call(&mut store, (ptr as i32, src.len() as i32)).unwrap();
    let _ = dealloc.call(&mut store, (ptr as i32, src.len() as i32)).unwrap();
    {
      let data = memory.data_mut(&mut store);
      let slice_copy: Vec<u8> = data[ptr..ptr + src.len()].to_vec();
      // let a = std::str::from_utf8(&slice_copy).unwrap();
      println!("{} {:?}", ptr, slice_copy);
    }
  }
  let src = "goodbye world";
  {
    let ptr = allocate.call(&mut store, src.len() as i32).unwrap() as usize;
    memory.write(&mut store, ptr as usize, src.as_bytes()).unwrap();
    let _ = print.call(&mut store, (ptr as i32, src.len() as i32)).unwrap();
    let _ = dealloc.call(&mut store, (ptr as i32, src.len() as i32)).unwrap();
  }



}


  // let ty = MemoryType::new(0, None);  
  // let memory = Memory::new(&mut store, ty).unwrap();
  // let mut linker = Linker::new(&engine);
  // linker.define(&store, "host", "mem", memory).unwrap();
  // let person = linker.instantiate(&mut store, &module).unwrap();


/*
  - WebAssembly内
    - 線形メモリ
      - 線形メモリの初期サイズと最大サイズを定義できる。
      - 64 KB (65536 byte) の Page 単位
      - アドレス空間は 32 bit 
      - 各 instance は線形 Memory を最大で 1 個しか使えない
      - 常に先頭は0番
      - Alignment を（普通は）考慮しないのでメモリアクセスは遅い
    - グローバル変数
      - 可変、又は不変
      - 変数の型（上記 8 種類の Primitive な型のいずれか）
      - 初期値（又は初期化関数）
      - ポインターは使用できないのでコンパイル時に書き換え = パフォーマンスの改善に寄与しない
  - wasm外
    - 渡された Memory の塊は設定の範囲内で後から拡張可能

*/