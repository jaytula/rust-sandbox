// Vectors - Resizeable arrays
use std::mem;

pub fn run() {
  let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

  // Add on to vector
  numbers.push(5);
  numbers.push(6);

  // Pop off last value
  numbers.pop();

  // Use debug interpolation
  println!("{:?}", numbers);

  // Get single value
  println!("{}", numbers[2]);

  // Re-assign value
  numbers[2] = 20;
  println!("{:?}", numbers);

  // Get vector length
  println!("Vector length: {}", numbers.len());

  // Vectors are stacked allocated
  // println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
  println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice of array
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in numbers.iter() {
    println!("Number: {}", x);
  }

  // Loop and mutate values
  for x in numbers.iter_mut() {
    *x *= 2;
  }

  println!("Numbers Vec: {:?}", numbers);
}