// cargo build --target wasm32-wasi --release

#[link(wasm_import_module = "env")]
extern "C" {
}

#[no_mangle]
pub fn run() {
  println!("test");
}

/*
wasi preview1
https://github.com/WebAssembly/WASI/blob/a7be582112b35e281058f1df7d8628bb30a69c3f/legacy/preview1/docs.md

  args_get(...)          : コマンドライン引数データを読み取り
  args_sizes_get(...)    : コマンドライン引数のデータ サイズ
  environ_get(...)       : 環境変数データを読み取り
  environ_sizes_get(...) : 環境変数のデータ サイズ
  ...
  fd_write(...)          : ファイル記述子に書き込みます
  proc_exit(...)         : プロセスを通常どおり終了します

  必要に応じてimportが生成される
*/
