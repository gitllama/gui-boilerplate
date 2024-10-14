// cargo build --target wasm32-wasi --release
// use wasm_bindgen::prelude::*;

#[link(wasm_import_module = "env")]
extern "C" {
  pub fn console_log(ptr: *const u8, len: usize);
}

fn console_log_(src: &str) { unsafe { console_log(src.as_ptr(), src.len()); } }

#[no_mangle]
pub fn run(src:i32) {
  let code = format!("hello wasm {}", src);
  console_log_(code.as_str());
}

#[no_mangle]
pub fn print(ptr: *const u8, len: usize) {
  unsafe {
    let slice = std::slice::from_raw_parts(ptr as *const u8, len);
    let str_slice = std::str::from_utf8_unchecked(slice);
    println!("Hello, {}", str_slice);
  }
}

/* wasm_bindgenでのstringやり取り時のコード */

#[no_mangle]
pub fn allocate(size: usize) -> *mut u8 {
  let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
  unsafe {
    let ptr: *mut u8 = std::alloc::alloc(layout);
    if ptr.is_null() {
      std::alloc::handle_alloc_error(layout);
      // let mut buffer: Vec<u8> = Vec::with_capacity(size);
      // let ptr = buffer.as_mut_ptr();
      // std::mem::forget(buffer);
    }
    ptr
  }
}

#[no_mangle]
pub fn dealloc(ptr: *mut u8, size: usize) {
  if ptr.is_null() { return; }
  let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
  unsafe { std::alloc::dealloc(ptr, layout); }
}

/* unsafe使いたくないのでグローバル変数を使用したコード */
use std::sync::Mutex;

static LINEAR_MEMORY: Mutex<Vec<u8>> = Mutex::new(Vec::new()); // mutable global variable

#[no_mangle]
pub fn resize(size: usize) {
  let mut vec = LINEAR_MEMORY.lock().unwrap();
  vec.resize_with(size, Default::default);
}

#[no_mangle]
pub fn get_ptr() -> *const u8 {
  let vec = LINEAR_MEMORY.lock().unwrap();
  vec.as_ptr()
}

#[no_mangle]
pub fn get_size() -> i32 {
  let vec = LINEAR_MEMORY.lock().unwrap();
  vec.len() as i32
}

/* wasm_bindgenでのstringやり取り時のコード 

// stringの受け渡し前に事前に呼ばれる
function passStringToWasm0(arg, malloc, realloc) {
  if (realloc === undefined) { ... }
  
  // 単純にメモリの確保
  let len = arg.length;
  let ptr = malloc(len, 1) >>> 0; // >>> 0 で正の整数化 (負, 非整数は0)

  // メモリの呼び出し + 書き込み (非ASCIIコードがでるとbreak)
  const mem = getUint8Memory0();
  let offset = 0;
  for (; offset < len; offset++) {
    const code = arg.charCodeAt(offset);
    if (code > 0x7F) break;
    mem[ptr + offset] = code;
  }

  // 非ASCIIコードあった場合の処理 (文字列の切り上げ,UTF8のためにx3倍の長さを確保)
  if (offset !== len) {
    if (offset !== 0) { arg = arg.slice(offset); }
    ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
    const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
    const ret = encodeString(arg, view);
    offset += ret.written;
  }

  WASM_VECTOR_LEN = offset;  // -> wasmで使用する長さ
  return ptr;                // -> wasmで使用するptr
}

*/

#[no_mangle]
pub extern "C" fn __wbindgen_malloc(size: usize, align: usize) -> *mut u8 {
  if let Ok(layout) = std::alloc::Layout::from_size_align(size, align) {
    unsafe {
      if layout.size() > 0 {
        let ptr = std::alloc::alloc(layout);
        if !ptr.is_null() { return ptr; }
      } else {
        return align as *mut u8;
      }
    }
  }
  panic!("malloc_failure"); //malloc_failure();
}

#[no_mangle]
pub unsafe extern "C" fn __wbindgen_realloc(ptr: *mut u8, old_size: usize, new_size: usize, align: usize) -> *mut u8 {
  debug_assert!(old_size > 0);
  debug_assert!(new_size > 0);
  if let Ok(layout) = std::alloc::Layout::from_size_align(old_size, align) {
    let ptr = std::alloc::realloc(ptr, layout, new_size);
    if !ptr.is_null() {
      return ptr;
    }
  }
  panic!("malloc_failure"); //malloc_failure();
}

// #[cold]
// fn malloc_failure() -> ! {
//   cfg_if::cfg_if! {
//     if #[cfg(debug_assertions)] {
//       super::throw_str("invalid malloc request")
//     } else if #[cfg(feature = "std")] {
//       std::process::abort();
//     } else if #[cfg(all(
//       target_arch = "wasm32",
//       target_os = "unknown"
//     ))] {
//       core::arch::wasm32::unreachable();
//     } else {
//       unreachable!()
//     }
//   }
// }

#[no_mangle]
pub unsafe extern "C" fn __wbindgen_free(ptr: *mut u8, size: usize, align: usize) {
  // This happens for zero-length slices, and in that case `ptr` is
  // likely bogus so don't actually send this to the system allocator
  if size == 0 { return; }
  let layout = std::alloc::Layout::from_size_align_unchecked(size, align);
  std::alloc::dealloc(ptr, layout);
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