use extism::*;

pub struct GlobalParams {
  pub hello : String,
}

host_fn!(hello_world(user_data: GlobalParams; key: String) -> String {
  let data = user_data.get()?;
  let hello = &data.lock().unwrap().hello;
  Ok(format!("{} {}", hello, key))
});

fn main() {
  let url = Wasm::file(r".\target\wasm32-unknown-unknown\release\wasm_extism.wasm");

  let store = UserData::new(GlobalParams{ hello : "hello!".to_string()});
  let manifest = Manifest::new([url]);
    // let manifest = Manifest::new([Wasm::File { ... }, ]);
  // let mut plugin = Plugin::new(&manifest, [], true).unwrap();
  let mut plugin = PluginBuilder::new(manifest)
    .with_wasi(true)
    .with_function("hello_world", [PTR], [PTR], store.clone(), hello_world)
    .build()
    .unwrap();

  {
    let res = plugin.call::<_, &str>("call_host", ()).unwrap();
    println!("{}", res);
  }
  {
    // できない？
    // let res = plugin.call::<(i32, i32), i32>("sum", (1, 2)).unwrap();
    // println!("{}", res);
  }

}