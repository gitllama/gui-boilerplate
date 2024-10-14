use include_dir::{include_dir, Dir};

static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/examples");

fn main() {
  for i in PROJECT_DIR.files() {
    println!("{:?}", i.path());
  }
  for i in PROJECT_DIR.dirs() {
    println!("{:?}", i.path());
  }
  let lib_rs = PROJECT_DIR.get_file("main.rs").unwrap();
  let body = lib_rs.contents_utf8().unwrap();
  println!("{}", body);
}
