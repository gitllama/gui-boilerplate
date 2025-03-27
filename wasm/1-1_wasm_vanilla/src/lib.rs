// cargo build --target wasm32-unknown-unknown --release
// cargo build --target wasm32-wasip1 --release

/* 標準がenvなので、#[link(wasm_import_module = "env")]は省略可 */
#[link(wasm_import_module = "env")]
extern "C" {
  fn hello();                  // wat -> (import "env" "hello" (func $... (type $t0)))
  fn add_one(src:i32) -> i32;  // wat -> (import "env" "add_one" (func $... (type $t1)))
  fn date_now() -> f64;        // wat -> (import "env" "date_now" (func $... (type $t2)))
  // fn console_log(ptr: *const u16, len: usize);
}

static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
// wat -> (global $__stack_pointer (mut i32) (i32.const 1048576))

#[no_mangle]
pub fn run() {
  COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
  unsafe { hello(); }
}
// wat -> (func $run (export "run") (type $t0)

#[no_mangle]
pub fn add(left:i32, right:i32) -> i32 {
  let c = COUNT.load(std::sync::atomic::Ordering::Relaxed);
  unsafe { add_one(left + right + c as i32) }
}
// (func $add (export "add") (type $t3) (param $p0 i32) (param $p1 i32) (result i32)
// (call $_ZN12wasm_vanilla7add_one17hb497febf2ea7a629E
//   (i32.add
//     (i32.add
//       (local.get $p1)　                      <- stackされてる引数の取り出し
//       (local.get $p0))
//     (i32.load offset=1048576 (i32.const 0))  <- globalの取り出し 初期値0 スタックポインタのアドレス1048576
//   ))
// )

#[no_mangle]
pub fn timestamp() -> f64 {
  unsafe { date_now() }
}

// #[no_mangle]
// pub fn console_write() {
//   let utf16: Vec<u16> = String::from("こんにちわ").encode_utf16().collect();
//   unsafe { console_log(utf16.as_ptr(), utf16.len()); }
// }