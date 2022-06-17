fn main() {
    //Scalar types

    // Integer literal
    let x: i32 = 21;
    print!("{}", x);

    // Floating-point literal
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // Boolean literal
    let f = true;
    let t: bool = false;

    // Character literal
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    // Compound types
    // Tuple
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    // Array

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
}
