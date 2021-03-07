// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run() {
  let mut numbers: [i32; 4] = [1, 2, 3, 4];

  // Use debug interpolation
  println!("{:?}", numbers);

  // Get single value
  println!("{}", numbers[2]);

  // Re-assign value
  numbers[2] = 20;
  println!("{:?}", numbers);

  // Get array length
  println!("Array length: {}", numbers.len());

  // Arrays are stacked allocated
  // println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
  println!("Array occupies {} bytes", mem::size_of_val(&numbers));

  // Get slice of array
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);

}