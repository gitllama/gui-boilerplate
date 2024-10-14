mod struct_base_static_dispatch;
mod trait_base;
mod struct_base_dynamic_dispatch;

#[cfg(test)]
mod tests {

  #[test]
  fn struct_base_static_dispatch_works() {
    /*
      コンストラクタで依存関係を受け取る
    */
    use super::struct_base_static_dispatch::*;
    
    struct Cat {}
    impl Character for Cat {
      fn name(&self) -> String { "mike".to_owned() }
      fn call(&self) -> String { "meow".to_owned() }
    }
  
    struct Dog {}
    impl Character for Dog {
      fn name(&self) -> String { "shiba".to_owned() }
      fn call(&self) -> String { "bowwow".to_owned() }
    }
  
    let animal_1 = Animal { breed: Cat {} };
    let animal_2 = Animal { breed: Dog {} };
    
    use_animal(animal_1);
    use_animal(animal_2);
  }

  #[test]
  fn trait_base_works() {
    /*
      差し替えのimplを実装するだけ
    */
    use super::trait_base::*;
    
    struct Cat {}
    impl Description for Cat {}
    impl Character for Cat {
      fn name(&self) -> String { "mike".to_owned() }
      fn call(&self) -> String { "meow".to_owned() }
    }

    struct Dog {}
    impl Description for Dog {}
    impl Character for Dog {}
    
    let animal_1 = Cat {};
    let animal_2 = Dog {};
    
    use_animal(animal_1);
    use_animal(animal_2);
  }

  #[test]
  fn struct_base_dynamic_dispatch_works() {
    /*
    */
    use super::struct_base_dynamic_dispatch::*;
    use std::sync::Arc;
    
    struct Cat {}
    impl Character for Cat {
      fn name(&self) -> String { "mike".to_owned() }
      fn call(&self) -> String { "meow".to_owned() }
    }
  
    struct Dog {}
    impl Character for Dog {
      fn name(&self) -> String { "shiba".to_owned() }
      fn call(&self) -> String { "bowwow".to_owned() }
    }
  
    let animal_1 = Animal::new(Arc::new(Cat {} ));
    let animal_2 = Animal::new(Arc::new(Dog {} ));
    
    use_animal(animal_1);
    use_animal(animal_2);
  }

}


