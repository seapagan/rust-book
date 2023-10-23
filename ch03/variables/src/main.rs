#[allow(unused_variables)]
fn main() {
    /* ------------------------------- mutable ------------------------------ */
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    /* ------------------------------ shadowing ----------------------------- */
    let y: i32 = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    /* --------------------------- floating point --------------------------- */
    let f = 2.0; // f64
    let f: f32 = 3.0; // f32

    /* ------------------------- numeric operations ------------------------- */

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // -1

    // remainder
    let remainder = 43 % 5;

    /* ------------------------------- boolean ------------------------------ */
    let t = true;
    let f: bool = false; // with explicit type annotation

    /* --------------------------- character types -------------------------- */
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    /* ------------------------------- tuples ------------------------------- */
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("{five_hundred}, {six_point_four}, {one}");

    /* ------------------------------- arrays ------------------------------- */
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("{first}, {second}");

    let b = [3; 5]; // [3, 3, 3, 3, 3]
}
