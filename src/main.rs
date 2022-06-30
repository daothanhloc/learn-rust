// fn main() {
//     // Ownership of variables
//     let x = 5;
//     let y = x;

//     let a = String::from("hello");
//     let b = a;
//     print!("{} - {}", x, y);
//     println!("{} - {}", a, b); // error: cannot move `a` into `b` because they both require unique ownership
// }

// Ownership of function
// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len(); // len() trả về độ dài của một String

//     (s, length)
// }

// ======================
// Borrowing
// fn main() {
//     let s1 = String::from("hello");
//     let len = calculate_length(&s1);
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

// Borrowing mutable

// fn main() {
//     let mut s = String::from("hello");

//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// // Slices
// fn main() {
//     let s = String::from("hello");

//     let len = s.len();

//     let slice = &s[0..len];
//     let slice = &s[..];
// }
