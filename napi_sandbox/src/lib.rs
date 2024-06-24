use napi::*;
use napi_derive::napi;

#[napi]
pub fn main(env: Env) {
  env.spawn_local(|env| async {
    println!("hello");
  });
  println!("Hello, world!");
}
