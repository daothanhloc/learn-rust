fn main() {
    // Imutable variable
    let x = 6;
    // x = 7; // error: cannot assign to immutable variable `x`
    println!("{}", x);
    // Mutable variable
    let mut y = 6;
    y = 7; // You can assign to mutable variables
    println!("{}", y);

    // Constants
    const THREE: i32 = 3;
    print!("{}", THREE);
    // You can declare a immutable and assign a value to it
    // but you can not declare a const variable
    // if you no assign a value

    // Shadowing
    let x = 5;
    let x = x + 1;

    // Shadowing different with mutable variable is: it can assign a value with different type

    let spaces = "   ";
    let spaces = spaces.len();

    // let mut spaces = "   ";
    // spaces = spaces.len();
}
