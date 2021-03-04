// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data
pub fn run() {
  let hello = "Hello. Immutable";
  let mut hello_growable = String::from("I am growable");

  println!("hello: {}", hello);
  
  // Get length
  println!("Length: {}", hello_growable.len());
  println!("{}", hello_growable);

  // .push for pushing char's
  hello_growable.push('W');
  println!("{}", hello_growable);

  // .pushStr from pushing strings
  hello_growable.push_str("More");
  println!("{}", hello_growable);
}