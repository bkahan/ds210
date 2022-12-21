use crate::utils::foo;

pub fn say_something() {
  println!("Utils Bar");
  foo::say_something();
}
