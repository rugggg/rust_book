fn main() {
    println!("Hello, world!");
    another_function(42, "psi");
}

fn another_function(value: i32, unit_label: &str) {
    println!("value: {value} {unit_label}");
}
