# WASM

## 0. detail

- 1_wasm_vanilla
  - 1-1 wasm_vanilla
  - 1-2 wasm_vanilla_wasi
- 2_wasm_pack
- 3_wasm_extism

### build target

- wat
  - WebAssembly テキスト形式
- wasm32-unknown-unknown
  - JavaScript API, Web API
  - 純粋な計算機能
- wasm32-wasi
  - JavaScript API, Web API + WASI API
  - システムコールやファイル システム、ネットワークなどに関連する機能を含む完全なOSをエミュレート
  - ブラウザは通常 WASI をサポートしていない
  - [version](https://github.com/WebAssembly/WASI/blob/main/legacy/)
    - Preview 0 : ファイルシステムアクセス、プロセス管理など
    - Preview 1 : ファイルシステムの拡張、環境変数など
    - Preview 2 : コンポーネントモデル
    - Preview 3

### Runtime

- wasi 
  - wasmtime
  - wasmer
  - ~~lucet~~ -> wasmtime
  - wasmedge
- for web
  - wasm-pack : wasm-bindgenのWrapper
  - Emscripten
- extism

## 1. wasm_vanilla

- PDK : vanilla rust
- SDK
  - rust : wasmtime
  - javascript : vanilla

### build

```ps1
ps> rustup target list
# wasm32-wasi : 標準ライブラリに統合したスタンドアロンバイナリ
# wasm32-unknown-unknown : 標準ライブラリの想定無し(unknown)
# wasm32-unknown-emscripten : ウェブブラウザ向け
ps> rustup target add wasm32-unknown-unknown
ps> rustup target add wasm32-wasi

ps> cargo build --target wasm32-unknown-unknown --release
ps> cargo build --target wasm32-wasi --release
```

### 内容

importされるobjectの内容がwasm32-unknown-unknownに対してwasiはwasi_snapshot_preview1が追加される  
追加がなければ```Import #0 module="wasi_snapshot_preview1" error```となる

**wasm32-unknown-unknown**
```json
{
  "[[Exports]]" : [
    { "name" : "memory", "kind" : "memory" },
    { "name" : "get_timestamp", "kind" : "function" },
    { "name" : "add", "kind" : "function" },
    { "name" : "console_write", "kind" : "function" },
    { "name" : "__data_end", "kind" : "global" },
    { "name" : "__heap_base", "kind" : "global" },
  ],
  "[[Imports]]" : [
    { "module" : "env", "name" : "data_now" },
    { "module" : "env", "name" : "console_log" }
  ]
}
```

**wasm32-wasi**
```json
{
  "[[Exports]]" : [
    { "name" : "memory", "kind" : "memory" },
    { "name" : "get_timestamp", "kind" : "function" },
    { "name" : "add", "kind" : "function" },
    { "name" : "console_write", "kind" : "function" },
  ],
  "[[Imports]]" : [
    { "module" : "env", "name" : "data_now" },
    { "module" : "env", "name" : "console_log" },
    { "module" : "wasi_snapshot_preview1", "name" : "fd_write" }
    { "module" : "wasi_snapshot_preview1", "name" : "environ_get" }
    { "module" : "wasi_snapshot_preview1", "name" : "environ_sizes_get" }
    { "module" : "wasi_snapshot_preview1", "name" : "proc_exit" }
  ]
}
```

適当にDummy入れれば動かなくもないが、wasm_packやwasmtime_wasiで自動的に生成させた方が楽

**dummy**
```javascript
const importObject = {
  env: {
    console_log: () => { console.log("Hello WebAssembly!"); },
  },
  wasi_snapshot_preview1: {
    fd_write: (i32a, i32b, i32c, i32d) => { return 0 },
    environ_get: (i32a, i32b) => { return 0 },
    environ_sizes_get: (i32a, i32b) => { return 0 },
    proc_exit: (i32) => console.log(i32),
  },
};
```
**wasmtime_wasi**
```rust
let mut linker = Linker::<wasmtime_wasi::preview1::WasiP1Ctx>::new(&engine);
wasmtime_wasi::preview1::add_to_linker_sync(&mut linker, |t| t).unwrap();
```


## 2. wasm_pack

vanillaだとimportは面倒なので、wasmpackを使用

- PDK : wasmpack / wasm-bindgen / js-sys
- SDK
  - rust : ?
  - javascript : wasmpack

wasm-bindgenをwasmtimeでloadする場合、カスタムメモリやグローバル変数, DOM操作みたいな共有できない機能との切り分けは必要

### build

```ps1
ps> cargo install wasm-pack
ps> wasm-pack build --target web
```

-target
  - nodejs
  - bundler(webpack)
  - deno
  - web
  - no-module : webとほぼ同等

## 3. wasm_extism

- PDK : extism
- SDK
  - rust : extism
  - javascript : extism

### build

```powershell
ps> cargo build --target wasm32-unknown-unknown
```

## 97. Other

### Wasmer

making

```javascript
  <script type="module">
    import { init, Wasmer } from "https://unpkg.com/@wasmer/sdk@latest?module";
    // await init();
    // const pkg = await Wasmer.getImports(module);
    console.log(Wasmer);
    // const a = Wasmer.fromFile(fetch('/target/wasm32-unknown-unknown/release/wasm_vanilla.wasm'))
    // console.log(a);
  </script>
```

[Rust で WebAssembly から console.log](https://zenn.dev/a24k/articles/20221012-wasmple-simple-console)
[WebAssembly と JavaScript との間で自在にデータをやりとり](https://zenn.dev/a24k/articles/20221107-wasmple-passing-buffer)

### .NET WebAssembly Browser app

```ps1
ps> dotnet workload install wasm-experimental
ps> dotnet new wasmbrowser -o myApp
```

#### Build

You can build the app from Visual Studio or from the command-line:

```ps1
ps> dotnet build -c Debug
ps> dotnet build -c Release
```

After building the app, the result is in the `bin/$(Configuration)/net7.0/browser-wasm/AppBundle` directory.

#### Run

You can build the app from Visual Studio or the command-line:

```ps1
ps> dotnet run -c Debug/Release
```

Or you can start any static file server from the AppBundle directory:

```ps1
ps> dotnet tool install dotnet-serve
ps> dotnet serve -d:bin/$(Configuration)/net7.0/browser-wasm/AppBundle
```


### .NET WebAssembly Browser app

```powershell
ps> dotnet workload install wasi-experimental
ps> dotnet new wasiconsole -o MyPlugin
ps> dotnet add package Extism.Pdk --version 1.0.3

```


## 98. grammar 

### Primitive な変数型

WebAssemblyのPrimitiveな変数の型は、下記の6種類のみ

- i32（符号付き 32 bit 整数）
- f32（符号付き 32 bit 浮動小数）
- i64（符号付き 64 bit 整数）
- f64（符号付き 64 bit 浮動小数）
- v128（128 bit byte 列）
- 抽象ポインター
  - 関数ポインター, オブジェクトを指すポインターのみサポート
  - 抽象ポインターの指す番地の byte 列を読むことは出来ない

### WebAssembly.Memory

```javascript
const importObject = {
  env: {
    console_log: (ptr, len) => {
      const chars = new Uint16Array(instance_2.exports.memory.buffer, ptr, len);
      console.log(String.fromCharCode(...chars));
    },
  },
};
```

```javascript
const memory = new WebAssembly.Memory({
  initial: 10,
  maximum: 100
});
const instance = await WebAssembly.instantiateStreaming(fetch("memory.wasm"), { js: { mem: memory } })
const summands = new Uint32Array(memory.buffer);
for (let i = 0; i < 10; i++) {
  summands[i] = i;
}
sum = instance.exports.accumulate(0, 10);
console.log(sum);
```


```
                                                                                       
  ┌──────┐      ┌───────┐   ┌─────────┐   ┌───────────────────────────────────────┐    
  │Device│      │Device │   │Operating│   │Application                            │    
  │      │      │Driver │   │System   │   │ ┌───────────────────────┐   ┌───────┐ │    
  │      │◄─────┤       │◄──┤         │◄──┤ │Rust                   │   │Vue.js │ │    
  │      │      │       │   │         │   │ │                       │   │       │ │    
  │      │      │       │   │         │   │ │ ┌────────┐                │       │ │    
  │      ├─────►│       ├──►│         ├──►│ │ │decoder │◄───────────────┤       │ │    
  │      │      │       │   │         │   │ │ │        │                │       │ │    
  │      │      │       │   │         │   │ │ │        ├───────────────►│       │ │    
  └──────┘      └───────┘   │         │   │ │ │        │                │       │ │    
                            │         │   │ │ │        │    ┌─────┐     │       │ │    
                ┌───────┐   │         │   │ │ │        ├───►│Pyo3 ├────►│       │ │    
                │storage│◄──┤         │   │ │ │        │    │Wasm │     │       │ │    
                │       │   │         │   │ │ │        │    │     │ │   │       │ │    
                │       ├──►│         │   │ │ └────────┘    └─────┘ │   │       │ │    
                │       │   │         │   │ │                       │   │       │ │    
                │       │   │         │   │ └───────────────────────┘   └───────┘ │    
                │       │   │         │   │                                       │    
                └───────┘   └─────────┘   └───────────────────────────────────────┘    
                                                                                       
```

## 98. reference

[暗黙の型変換](https://wasm-dev-book.netlify.app/hello-wasm.html#%E6%9A%97%E9%BB%99%E3%81%AE%E5%9E%8B%E5%A4%89%E6%8F%9B)
[linear-memory](https://rustwasm.github.io/docs/book/what-is-webassembly.html#linear-memory)