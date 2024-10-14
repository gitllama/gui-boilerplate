
#[cfg(feature = "pdk")]
pub mod pdk {
  #[link(wasm_import_module = "env")]
  extern "C" {
    pub fn get_args_size() -> i32;
    pub fn get_args(ptr:*const u8, len: usize);
    pub fn set_result(ptr:*const u8, len: usize);
  }

  pub fn args_value() -> serde_json::Value {
    let len = unsafe { get_args_size() as usize };
    let line_memory = vec![0u8; len];
    unsafe { get_args(line_memory.as_ptr(), line_memory.len()); }
    let json = std::str::from_utf8(&line_memory).unwrap();
    let de: serde_json::Value = serde_json::from_str(&json).unwrap();
    de
  }

  pub fn args_string() -> String {
    let len = unsafe { get_args_size() as usize };
    let line_memory = vec![0u8; len];
    unsafe { get_args(line_memory.as_ptr(), line_memory.len()); }
    let dst = std::str::from_utf8(&line_memory).unwrap();
    dst.to_string()
  }

  pub fn result(dst: serde_json::Value) {
    let dst = serde_json::to_string(&dst).unwrap();
    unsafe { set_result(dst.as_ptr(), dst.len()); }
  }  

}

#[cfg(feature = "sdk")]
pub mod sdk {
}
