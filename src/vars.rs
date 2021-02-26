// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "Brad";
  let mut age = 45;
  println!("My name is {} and I am {}", name, age);

  age = 46;
  println!("My name is {} and I am {}", name, age);

  // Define constant. Not often used. Type is required.
  const ID: i32 = 001;
  println!("ID: {}", ID);
}