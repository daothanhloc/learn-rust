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
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() trả về độ dài của một String

    (s, length)
}
