use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess number!");
    // thread_rng gives you a THREAD local rand number gen
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input guess:");
        println!("The secret number is: {secret_number}");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Guess is : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
