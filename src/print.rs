pub fn run() {
  // Print to console
  println!("Hello from the print.rs file");

  // Variable interpolation
  println!("Number: {}", 1);

  // Basic Formatting. Multiple placeholders
  println!("{} is from {}", "Brad", "Mass");

  // Positional Arguments
  println!("{0} is from {1} and {0} like to {2}", "Brad", "Mass", "code");

  // Named Arguments
  println!("{name} likes to play {activity}", name = "John", activity = "Baseball" );
}