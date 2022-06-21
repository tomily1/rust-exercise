use std::io;

pub fn nth_fibonacci() {
  let mut number = String::new();
  
  println!("Please input the nth number to fetch the fibonacci");

  io::stdin()
    .read_line(&mut number)
    .expect("Failed to read line");
  
  let number: u32 = match number.trim().parse() {
    Err(e) => {
      println!("Invalid Input: {}", e);
      return;
    },
    Ok(num) => num,
  };

  println!("The nth fibonacci of {} is {}", number, fibonacci(number));
}

fn fibonacci(n: u32) -> u32 {
  if [0, 1].contains(&n) { return n; }

  return fibonacci(n - 1) + fibonacci (n - 2);
}
