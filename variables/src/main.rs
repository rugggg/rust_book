fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // give consts all uppercase + _ names
    let mut x = 5; // mut is keyword to allow variable to be mutable
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");

    tuples();
}

fn float_type_hints() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

fn artithmetic() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}

fn chars() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻'; // rust supports emojis!
}

fn tuples() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup; // destructuring

    println!("The value of y is: {y}");

    // dot notation
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

fn arrays() {
    let a = [1, 2, 3, 4, 5];
    let months = [
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
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // typed array i32, 5 elements
    let a = [3; 5]; // [3, 3, 3, ,3, 3]

    //arrays allocate on the stack
    //arrays are size imutable!
    //if you want to grow/shrink, use vector instead

    //array accessing
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    // rust will handle index out of bounds by panicking, throwing err and exiting
}
