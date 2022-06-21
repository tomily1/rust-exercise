// use std::cmp::Ordering;
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
			nth_fibonacci();
		} else {
			temperature();
		}

		break;
	}
}

fn nth_fibonacci() {
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

fn temperature() {
	'temperature: loop {
		let mut option = String::new();
		println!("Press 1 to convert from F to C, or 2 to convert from C to F");

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

		loop {
			let mut temp = String::new();
			println!("How many degrees?");
		
			io::stdin()
				.read_line(&mut temp)
				.expect("Failed to read line");
		
			let temp: f32 = match temp.trim().parse() {
				Err(e) => {
					println!("Invalid Input: {}", e);
					continue;
				},
				Ok(num) => num,
			};

			if option == 1 {
				println!("Converted Temperature in celsius is {}°C", c_to_f(temp));
			} else {
				println!("Converted Temperature in fahrenheit is {}°F", f_to_c(temp));
			}

			break 'temperature;
		}
	}
}

fn c_to_f(temp: f32) -> u32 {
	let celsius: f32 =  (temp - 32. ) * 0.5556;

	return celsius as u32;
}

fn f_to_c(temp: f32) -> u32 {
	let fahrenheit: f32 =  (temp * 1.8 ) + 32.;

	return fahrenheit as u32;
}