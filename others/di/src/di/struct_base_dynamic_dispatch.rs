// structベース、動的ディスパッチ
// - 実行時解決なので肥大化しない
// - コンパイル時にサイズが決定できないオブジェクトも管理できる。
// - 型引数の管理が楽
// - 最適化がかかりにくい
// - 遅い

use std::sync::Arc;

pub trait Character: Send + Sync + 'static {
  fn name(&self) -> String;
  fn call(&self) -> String;
}

pub trait Description {
  fn description(&self) -> String;
}

pub struct Animal {
  breed: Arc<dyn Character>,
}

impl Animal {
  pub fn new(breed: Arc<dyn Character>) -> Self { Self{ breed } }
}

impl Description for Animal {
  fn description(&self) -> String {
    format!("{} {}", self.breed.name(), self.breed.call())
  }
}

pub fn use_animal(src: Animal) {
  println!("{}", src.description());
}
