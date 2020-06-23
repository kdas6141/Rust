//const MAX_POINTS: u32 = 100000;
fn main() {
	let mut x = 5;
	println!("The value of x is: {}", x);
	x = 6;
	println!("The value of x is: {}", x);
	let y =5;
	let y = y + 1;
	let y = y * 2;
	println!("The value of y is: {}", y);
	let spaces = "   ";
	let spaces = spaces.len();
	println!("The value of spaces.len() is: {}", spaces);
	//let guess: u32 = "42".parse().expect("Not a number!");
	let fx = 2.0;
	let fy: f32 = 3.0;
	let sum = 5 + 10;
	let difference = 95.5 - 4.3;
	let product = 4 * 30;
	let quotient = 56.7 / 32.2;
	let reminder = 43 % 5;
	let t = true;
	let f: bool = false;
	let c = 'z';
	let z = 'Z';
	let heart_eyed_cat = '😻';
	let tup: (i32, f64, u8) = (500, 6.4, 1);
	let (tx, ty, tz) = tup;
	println!("The value of y is: {}", ty);
	let five_hundred = tup.0;
	let six_point_four = tup.1;
	let one = tup.2;
	let a = [1, 2, 3, 4, 5];
	let months = ["January", "February", "March", "April", "May", "June", "July",
					"August", "September", "October", "November", "December"];
	let a: [i32; 5] = [1, 2, 3, 4, 5];
	let a = [3; 5];
	let first = a[0];
	let second = a[1];
	another_function(5, 6);
	let mut x5 = 5;
	let y5 = {
		let x3 = 3;
		x3 + 1
	};
	println!("The value of y5 is: {}", y5);
	x5 = five();
	println!("The value of x5 is: {}", x5);
	x5 = plus_one(5);
	println!("The value of x5+1 is: {}", x5);
	let mut number = 3;
	if number < 5 {
		println!("Condition is true");
	} else {
		println!("Condition was false");
	}
	let condition = true;
	number = if condition {5} else {6};
	if number % 4 == 0 {
		println!("Number is divisible by 4");	
	} else if number % 3 == 0 {
		println!("Number is divisible by 3");
	} else if number % 2 == 0 {
		println!("Number is divisible by 2");
	} else {
		println!("Number not divisible by 4, 3, or 2");
	}
	let mut counter = 0;
	let result = loop {
		counter += 1;
		if counter == 10 {
			break counter * 2;
		}
	};

	println!("The result is {}", result);

	while number != 0 {
		println!("{}!", number);
		number -= 1;
	}
	println!("LIFTOFF!!!");

	for element in a.iter() {
		println!("The value is: {}", element);
	}

	for number in (1..4).rev() {
		println!("{}!", number);
	}
	println!("LIFTOFF!!!");
}

fn another_function(x: i32, y: i32) {
	println!("The value of x is {}", x);
	println!("The value of y is {}", y);

}

fn five()->i32 {
	5
}

fn plus_one(x: i32) -> i32 {
	x + 1
}
