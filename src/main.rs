mod temperature;
mod fibonacci;
mod twelve_christmas;

use std::io;

fn main() {
  loop {
    let mut option = String::new();
    println!("Please input options for choices below\n1. Nth Fibonacci\n2. Temperature Conversion\n3. Twelve days of christmas");
  
    io::stdin()
      .read_line(&mut option)
      .expect("Failed to read line");
  
    let option: u32 = match option.trim().parse() {
      Err(e) => {
        println!("Invalid Input: {}", e);
        continue;
      },
      Ok(num) => {
        if (num > 0) && (num < 4) {
          num
        } else {
          println!("Invalid Input: {}", num);
          continue;
        }
      },
    };

    if option == 1 {
      fibonacci::nth_fibonacci();
    } else if option == 2 {
      temperature::convert();
    } else {
     twelve_christmas::twelve_christmas();
    }

    break;
  }
}
