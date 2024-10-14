#[cfg(test)]
use mockall::{automock, mock, predicate::*};

use fake::{Dummy, Fake, Faker, faker::name::en::*};

#[derive(Debug, Dummy)]
pub struct Customer {
  #[dummy(faker = "0..u64::MAX")]
  pub id: u64,
  #[dummy(faker = "Name()")]
  pub name: String,
}

#[cfg_attr(test, automock)]
pub trait Hoge {
  fn foo(&self, x: u32) -> u32;
  fn bar(&self) -> u32;
}

pub fn test_method<T>(src: T) where T : Hoge  {
  println!("{} {}", src.foo(4), src.bar());
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn mytest() {
    let customer : Customer = Faker.fake();
    println!("{:?}", customer);

    let mut mock = MockHoge::new();
    /*
      .with(eq(4)) : 4が入るはず
      .times(1)    : 1度だけ呼ばれる
      .returning   : ||{} を返す
    */
    mock.expect_foo()
      .with(eq(4))
      .times(1)
      .returning(|x| x + 1);
    mock.expect_bar()
      .times(1)
      .returning(|| 3);

    test_method(mock);
  }
}

fn main() {

}
