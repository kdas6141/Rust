fn main() {
	let mut s = String::from("hello");
	s.push_str(", world!");
    println!("{}", s);
    let s2 = s.clone();
    println!("s = {}, s1 = {}", s, s2);
    takes_ownership(s);
    let x = 5;
    makes_copy(x);
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let mut s3 = takes_and_gives_back(s2);
    let (s4, len) = calculate_length(s1);
    let len2 = calculate_length2(&s3);
    let mut s = String::from("hello");
    change(&mut s3);
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
}

fn takes_ownership(some_string: String) {
	println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
	println!("{}", some_integer);
}

fn gives_ownership() -> String {
	let some_string = String::from("hello");
	some_string
}

fn takes_and_gives_back(a_string: String) -> String {
	a_string
}

fn calculate_length(s: String) -> (String, usize) {
	let length = s.len();
	(s, length)
}

fn calculate_length2(s: &String) -> usize {
	s.len()
}

fn change(some_string: &mut String) {
	some_string.push_str(", world");
}
/*
fn first_word(s: &String) -> usize {
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return i;
		}
	}
	s.len()
}
*/
fn first_word(s: &String) -> &str {
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate() {
		if item == b' ' {
			return &s[0..i]
		}
	}

	&s[..]
}