pub trait Character {
  fn name(&self) -> String { "shiba".to_owned() }
  fn call(&self) -> String { "bowwow".to_owned() }
}

pub trait Description : Character {
  fn description(&self) -> String {
    format!("{} {}", self.name(), self.call())
  }
}

pub fn use_animal<T: Description>(b: T) {
  println!("{}", b.description());
}
