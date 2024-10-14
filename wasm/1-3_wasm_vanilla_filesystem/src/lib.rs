// cargo build --target wasm32-wasi --release

#[link(wasm_import_module = "env")]
extern "C" {
  fn call_globalstate() -> i32;
}

#[no_mangle]
pub fn run() {
  for arg in std::env::args() {
    println!("args : {arg}");
  }

  println!("param : {}", unsafe { call_globalstate() });

  // let target_path = r".\";
  let target_path = std::env::var("TARGET_PATH").unwrap_or_else(|_| ".".to_string());
  let target = std::path::PathBuf::from(target_path);
  let files = target.read_dir().expect("path not found");
  for dir_entry in files {
    let file_path = dir_entry.unwrap().path();
    println!("{:?}", file_path);
  }
}