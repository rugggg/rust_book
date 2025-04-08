fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // give consts all uppercase + _ names
    let mut x = 5; // mut is keyword to allow variable to be mutable
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");
}

fn floatTypeHints() {
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
