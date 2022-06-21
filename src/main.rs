mod temperature;
mod fibonacci;

use std::io;

fn main() {
  loop {
    let mut option = String::new();
    println!("Please input options for choices below\n1. Nth Fibonacci\n2. Temperature Conversion");
  
    io::stdin()
      .read_line(&mut option)
      .expect("Failed to read line");
  
    let option: u32 = match option.trim().parse() {
      Err(e) => {
        println!("Invalid Input: {}", e);
        continue;
      },
      Ok(num) => {
        if (num > 0) && (num < 3) {
          num
        } else {
          println!("Invalid Input: {}", num);
          continue;
        }
      },
    };

    if option == 1 {
      fibonacci::nth_fibonacci();
    } else {
      temperature::convert();
    }

    break;
  }
}
