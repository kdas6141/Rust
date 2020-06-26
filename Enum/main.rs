enum IpAddrKind{
	V4,
	V6,
}

enum IpAddr2 {
	V4(String),
	V6(String),
}

enum Message {
	Quit,
	Move {x: i32, y: i32},
	Write(String),
	ChangeColor(i32, i32, i32),
}

enum Option<T> {
	Some(T),
	None,
}

#[derive(Debug)]
enum UsState {
	Alabama,
	Alaska,
}

enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}

struct IpAddr {
	kind: IpAddrKind,
	address: String,
}

struct QuitMessage; //unit struct
struct MoveMessage {
	x: i32,
	y: i32,
}
struct WriteMessage(String); //tuple struct
struct ChangeColorMessage(i32, i32, i32);

fn route(ip_kind: IpAddrKind) {
	
}

impl Message {
	fn call(&self) {

	}
}

fn value_in_cents(coin: Coin) -> u8 {
	match coin {
		Coin::Penny => {
			println!("Lucky penny!");
			1
		}
		Coin::Nickel => 5,
		Coin::Dime => 10,
		Coin::Quarter(state) => {
			println!("State quarter from {:?}", state);
			25
		}
	}
}
/*
fn plus_one(x: Option<i32>) -> Option<i32> {
	match x {
		None => None,
		Some(i) => Some(i + 1),
	}
}
*/

fn main() {
	let four = IpAddrKind::V4;
	let six = IpAddrKind::V6;

	let home = IpAddr {
		kind: IpAddrKind::V4,
		address: String::from("127.0.0.1"),
	};

	let loopback = IpAddr {
		kind: IpAddrKind::V6,
		address: String::from("::1"),
	};

	let home2 = IpAddr2::V4(String::from("127.0.0.1"));
	let loopback2 = IpAddr2::V6(String::from("::1"));

	let m = Message::Write(String::from("hello"));
	m.call();
	let some_number = Some(5);
	let some_string = Some("a string");
	let absent_number: Option<i32> = Option::None;
//	let x: i8 = 5;
//	let y: Option<i8> = Some(5);
//	let sum = x + y;
	let some_u8_value = 0u8;
	match some_u8_value {
		1 => println!("one"),
		3 => println!("three"),
		5 => println!("five"),
		7 => println!("seven"),
		_ => (),
	}

	let some_u8_value2 = Some(0u8);
	match some_u8_value2 {
		Some(3) => println!("there"),
		_ => (),
	}

	if let Some(3) = some_u8_value2 {
		println!("there");
	}

	let mut coin: Coin = Coin::Dime; 
	let mut count = 0;
	match coin {
		Coin::Quarter(state) => println!("State quarter from {:?}!", state),
		_ => count += 1,
	}

	let mut coin2: Coin = Coin::Penny;
	if let Coin::Quarter(state) = coin2 {
		println!("State quarter from {:?}", state);
	} else {
		count += 1;
	}
}