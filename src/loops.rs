// Loops - Used to iterate until a condition is met

pub fn run() {
  let mut count = 0;

  // Infinite loop
  loop {
    count += 1;
    println!("Number: {}", count);

    if count == 20 {
      break;
    }
  }

  // While loop (FizzBuzz)
  count = 0;
  while count < 100 {
    count += 1;
    if count % 3 === 0 && count % 5 == 0 {
      println!("{}: {}", count, "fizzbuzz");
    } else if count % 3 == 0 {
      println!("{}: {}", count, "fizz")
    } else if count % 5 == 0 {
      println!("{}: {}", count, "buzz")
    } else {
      println!("{}: {}", count, "Nothing Special")
    }
  }

}