// コンテナもどき

use std::sync::Arc;
use std::sync::OnceLock;

pub struct Container { }

impl Container {
  
}

fn computation() -> &'static Animal {
  static COMPUTATION: OnceLock<Animal> = OnceLock::new();
  COMPUTATION.get_or_init(|| Animal::new(Arc::new(Dog{})))
}

static COMPUTATION_2: OnceLock<Animal> = OnceLock::new();

pub trait Character : Send + Sync + 'static {
  fn call(&self) -> String;
}

pub struct Animal {
  breed: Arc<dyn Character>,
}

impl Animal {
  fn new(src: Arc<dyn Character>) -> Self {
    Self { breed: src, }
  }
  fn description(&self) -> String {
    format!("{}, call: {}", "description", self.breed.call())
  }
}

pub struct Dog {}
impl Character for Dog {
  fn call(&self) -> String { "bowwow".to_owned() }
}

struct Cat {}
impl Character for Cat {
  fn call(&self) -> String { "meow".to_owned() }
}

fn main() {

  COMPUTATION_2.get_or_init(|| Animal::new(Arc::new(Cat{})));

  let animal = computation();
  println!("{}", animal.description());

  use_animal();
}

pub fn use_animal() {
  let animal_1 = computation();
  let animal_2 = COMPUTATION_2.get().unwrap();
  println!("[use] {}", animal_1.description());
  println!("[use] {}", animal_2.description());
}
