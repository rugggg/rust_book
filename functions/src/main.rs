fn main() {
    println!("Hello, world!");
    another_function(42, "psi");
    let y = {
        let x = 3;
        x + 1 // note the lack of semicolon here, that makes it an expression! not a statement,
              // therefor it returns a value!
    }; // y is 4 because the function returns x + 1 which is 4
    println!("{y}");
    five();
}

fn another_function(value: i32, unit_label: &str) {
    println!("value: {value} {unit_label}");
}

fn five() -> i32 {
    5 // this is the return value implicit. again, not semi colon!
}
