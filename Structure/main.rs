struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

#[derive(Debug)]
struct Rectangle {
	width: u32,
	height: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.width * self.height
	}

	fn can_hold(&self, other: &Rectangle)->bool {
		self.width > other.width && self.height > other.height
	}

	fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
	let mut user1 = User {
		email: String::from("someone@example.com"),
		username: String::from("someone@example.com"),
		active: true,
		sign_in_count: 1,
	};

    println!("Hello, world!");
    user1.email = String::from("anotheremail@example.com");
	let user2 = build_user(
		String::from("someone@example.com"),
		String::from("someusername123"),
		);
	let user3 = User {
		email: String::from("someone@example.com"),
		username: String::from("someusername123"),
		active: true,
		sign_in_count: 1,		
	};
	let user4 = User {
		email: String::from("another@example.com"),
		username: String::from("anotherusername567"),
		active: user3.active,
		sign_in_count: user3.sign_in_count,		
	};
	let user5 = User {
		email: String::from("another2@example.com"),
		username: String::from("anotherusername5672"),
		..user3
	};
	struct Color(i32, i32, i32, i32, i32);
	struct Point(i32, i32, i32);
	let black = Color(0, 0, 0, 0, 0);
	let origin = Point(0, 0, 0);
	let rect1 = Rectangle {
		width: 30,
		height: 50,
	};

	println!("rect1 is {:#?}", rect1);
	println!(
		"The Area of the rectangle is {} square pixels.",
		area(&rect1)
	);
	println!(
		"The Area of the rectangle is {} square pixels.",
		rect1.area()
	);
	#[derive(Debug, Copy, Clone)]
	struct Point2 {
		x: f64,
		y: f64,
	}

	impl Point2 {
		fn distance(&self, &other: &Point2) -> f64 {
			let x_squared = f64::powi(other.x - self.x, 2);
			let y_squared = f64::powi(other.y - self.y, 2);
			f64::sqrt(x_squared + y_squared)
		}
	}

	let p1 = Point2 { x:0.0, y:0.0 };
	let p2 = Point2 { x:5.0, y:6.5 };
	p1.distance(&p2);
	(&p1).distance(&p2);
	let rect2 = Rectangle {
		width: 10,
		height: 40,
	};
	let rect3 = Rectangle {
		width: 60,
		height: 45,
	};

	println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
	println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

	let sq = Rectangle::square(3);
	//println!("Size of the square: {}", Rectangle::square(3));
}

fn build_user(email: String, username: String) -> User {
	User {
		email: email,
		username: username,
		active: true,
		sign_in_count: 1,
	}
} 

fn area(R: &Rectangle) -> u32 {
	R.width * R.height
}
