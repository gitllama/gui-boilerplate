
use extism_pdk::*;

/* call host */

#[host_fn]
extern "ExtismHost" {
  fn hello_world(n: String) -> String;
}
// (import "extism:host/user" "hello_world" (func $_... (type (func (param i64) (result i64)) )))

#[plugin_fn]
pub fn call_host() -> FnResult<String> {
  let dst = unsafe { hello_world("extism".to_string())? };
  Ok(dst)
}
// -> (type (result (func (result i32)) )) (result i32)

/* primitive */

#[plugin_fn]
pub fn add_pi(input: f32) -> FnResult<f64> {
  Ok(input as f64 + 3.14f64)
}
// -> (type (result (func (result i32)) )) (result i32)

#[no_mangle]
pub fn sum(left: i32, right: i32) -> i32 {
  left + right
}

#[plugin_fn]
pub fn process_bytes(input: Vec<u8>) -> FnResult<Vec<u8>> {
  // process bytes here
  Ok(input)
}
// -> (type (result (func (result i32)) )) (result i32)



/* greet */

// #[plugin_fn]
// pub fn greet(name: String) -> FnResult<String> { 
//   Ok(format!("Hello, {}!", name)) 
// }

/* global */

// #[plugin_fn]
// pub fn set(value: i32) -> FnResult<()> {
//   var::set("global", value)?;
//   Ok(())
// }

// #[plugin_fn]
// pub fn get() -> FnResult<i32> {
//   // let mut c = var::get("vec")?.unwrap_or(0);
//   let c = var::get("global")?.unwrap_or(0);
//   Ok(c)
// }