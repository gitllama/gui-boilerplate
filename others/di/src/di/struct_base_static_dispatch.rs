pub trait Character {
  fn name(&self) -> String;
  fn call(&self) -> String;
}

pub struct Animal<T: Character> {
  pub breed: T,
}

pub trait Description {
  fn description(&self) -> String;
}

impl<T: Character> Description for Animal<T> {
  fn description(&self) -> String { format!("{} {}", self.breed.name(), self.breed.call()) }
}


pub fn use_animal<T: Description>(b: T) {
  println!("{}", b.description());
}
