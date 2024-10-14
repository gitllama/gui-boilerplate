wit_bindgen::generate!("sample");

struct Component;

impl Sample for Component {
  // エクスポートする関数の実装
  fn run() {
    // インポート関数 log の呼び出し
    log(LogLevel::Debug, "called run");
    // インポート関数 getitem の呼び出し
    let r = getitem("item-1");

    log(LogLevel::Info, &format!("item: {:?}", r));
  }
}

export_sample!(Component);

/*
> cargo build --release --target wasm32-unknown-unknown
> cargo install wasm-tools
> wasm-tools component new target/wasm32-unknown-unknown/release/wasm_vanilla.wasm -o wasm-component.wasm
*/

// wat -> wit
// Component
use wit_wit_bindgen::*;

wit_bindgen::generate!({
  // the name of the world in the `*.wit` input file
  world: "host",
});

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.
struct MyHost;

impl Guest for MyHost {
  fn run() {
    print("Hello, world!");
  }
}